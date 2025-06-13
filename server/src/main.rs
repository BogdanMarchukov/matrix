use actix_web::{guard, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use async_graphql::{http::GraphiQLSource, Schema};
use async_graphql::{Data, ErrorExtensions};
use async_graphql_actix_web::GraphQLSubscription;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use auth::auth_service;
use errors::gql_error::GqlError;
use gql_schema::Subscription;
use include_dir::{include_dir as include_d, Dir};
use mime_guess::from_path;
use newsletter::newsletter_scheduler::newsletter_scheduler;
use once_cell::sync::Lazy;
use sea_orm::DatabaseConnection;
use secret::secret_service;
use std::collections::HashMap;
use tokio::sync::broadcast::{self};
use user::user_gql_model::{User, UserGqlModel};
#[path = "auth/mod.rs"]
mod auth;
#[path = "common/config/config.rs"]
mod config;
#[path = "common/db_utils/db_utils.rs"]
mod db_utils;
#[path = "common/gql/gql_schema.rs"]
mod gql_schema;
#[path = "common/guards/mod.rs"]
mod guards;
mod offer;
#[path = "common/secret/mod.rs"]
mod secret;
mod tariff_plan;
#[path = "user/mod.rs"]
mod user;
use crate::gql_schema::Mutation;
use crate::gql_schema::Query;
use crate::user::user_repository;
use db_utils::get_pool;
#[path = "entity/mod.rs"]
mod entity;
#[path = "common/errors/mod.rs"]
mod errors;
#[path = "common/helpers/mod.rs"]
mod helpers;
mod newsletter;
mod notify;
use actix_cors::Cors;
use uuid::Uuid;

const FRONTEND_DIR: Dir = include_d!("../client/build");

pub static TX_NOTIFY: Lazy<broadcast::Sender<TxSender>> = Lazy::new(|| {
    let (tx, _rx) = broadcast::channel(100);
    tx
});

async fn index(_req: HttpRequest) -> HttpResponse {
    let file = FRONTEND_DIR.get_file("index.html").unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(file.contents())
}

async fn static_files(req: HttpRequest) -> HttpResponse {
    let path = req.path().trim_start_matches('/');
    if let Some(file) = FRONTEND_DIR.get_file(path) {
        let mime_type = from_path(path).first_or_octet_stream();
        HttpResponse::Ok()
            .content_type(mime_type)
            .body(file.contents())
    } else {
        let file = FRONTEND_DIR.get_file("index.html").unwrap();
        HttpResponse::Ok()
            .content_type("text/html")
            .body(file.contents())
    }
}

type GqlSchema = Schema<Query, Mutation, Subscription>;

#[derive(Clone)]
pub struct TxSender {
    pub id: Uuid,
    pub user_id: Uuid,
}

pub struct AppState {
    pub db: DatabaseConnection,
    pub schema: GqlSchema,
}

impl AppState {
    pub fn new(db: DatabaseConnection, schema: GqlSchema) -> Self {
        Self { db, schema }
    }
}

pub struct GqlCtx {
    pub db: DatabaseConnection,
    pub headers: HashMap<String, String>,
    pub user: Option<User>,
}

async fn gql_playgound() -> HttpResponse {
    HttpResponse::Found()
        .append_header((
            "Location",
            "https://studio.apollographql.com/sandbox/explorer?endpoint=http://localhost:5000/gql",
        ))
        .finish()
}

async fn index_ws(
    app_data: web::Data<AppState>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse, Error> {
    GraphQLSubscription::new(app_data.schema.to_owned())
        .on_connection_init(|value: serde_json::Value| async move {
            let mut data = Data::default();
            if let Some(auth_header) = value.get("Authorization") {
                let headers: HashMap<String, String> = HashMap::new();
                let db = app_data.db.to_owned();
                let token = auth_header.as_str().unwrap_or("");
                if let Ok(jwt_payload) = secret_service::verify_jwt(String::from(token)) {
                    match user_repository::find_by_id(&jwt_payload.claims.sub, &db).await {
                        Ok(user) => {
                            data.insert::<GqlCtx>(GqlCtx { user, headers, db });
                            return Ok(data);
                        }
                        Err(_) => return Err(GqlError::Unauthorized.extend()),
                    };
                }
            }
            Err(GqlError::Unauthorized.extend())
        })
        .start(&req, payload)
}

async fn gql_index(
    app_data: web::Data<AppState>,
    gql_request: GraphQLRequest,
    http_request: HttpRequest,
) -> GraphQLResponse {
    let (headers, user) = auth_service::get_user_from_request(&http_request, &app_data.db).await;
    let schema = app_data.schema.to_owned();
    let request = gql_request.into_inner().data(GqlCtx {
        db: app_data.db.to_owned(),
        headers,
        user,
    });
    let que = serde_json::to_string(&request).unwrap_or(String::from("{}"));
    println!("{}", que);
    let result = schema.execute(request).await;
    GraphQLResponse::from(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = config::get_host();
    let port = config::get_port();
    newsletter_scheduler().await;
    let pool: DatabaseConnection = get_pool().await;
    HttpServer::new(move || {
        let schema = Schema::build(Query, Mutation, Subscription).finish();

        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.to_owned(),
                schema: schema.to_owned(),
            }))
            .wrap(cors)
            .service(
                web::resource("/gql")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .route("/gql", web::post().to(gql_index))
            .route("/gql", web::get().to(gql_playgound))
            .route("/", web::get().to(index))
            .route("/{filename:.*}", web::get().to(static_files))
    })
    .bind((host, port))?
    .run()
    .await
}
