use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use entity::users;
use include_dir::{include_dir as include_d, Dir};
use mime_guess::from_path;
use sea_orm::DatabaseConnection;
use user::user_gql_model::UserGqlModel;
use std::collections::HashMap;
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
mod schema;
#[path = "common/secret/mod.rs"]
mod secret;
#[path = "user/mod.rs"]
mod user;
use crate::gql_schema::Mutation;
use crate::gql_schema::Query;
use crate::secret::secret_service;
use crate::secret::secret_service::JwtPayload;
use crate::user::user_repository;
use db_utils::get_pool;
use jsonwebtoken::TokenData;
#[path = "entity/mod.rs"]
mod entity;
#[path = "common/errors/mod.rs"]
mod errors;
#[path ="common/helpers/mod.rs"]
mod helpers;

const FRONTEND_DIR: Dir = include_d!("../client/build");

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

type GqlSchema = Schema<Query, Mutation, EmptySubscription>;

pub struct AppState {
    pub db: DatabaseConnection,
    pub schema: GqlSchema,
}

pub struct GqlCtx {
    pub db: DatabaseConnection,
    pub headers: HashMap<String, String>,
    pub user: Option<UserGqlModel>,
}

async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/gql").finish())
}

async fn gql_index(
    app_data: web::Data<AppState>,
    gql_request: GraphQLRequest,
    http_request: HttpRequest,
) -> GraphQLResponse {
    let mut headers: HashMap<String, String> = HashMap::new();
    for (key, value) in http_request.headers().iter() {
        headers.insert(key.to_string(), value.to_str().unwrap_or("").to_string());
    }
    let jwt_payload: Option<TokenData<JwtPayload>> = match headers.get("Authorization") {
        Some(token) => match secret_service::verify_jwt(token) {
            Ok(p) => Some(p),
            Err(_) => None,
        },
        None => None,
    };
    let user = match jwt_payload {
        Some(payload) => {
            match user_repository::find_by_id(&payload.claims.sub, &app_data.db).await {
                Ok(user) => user,
                Err(_) => None,
            }
        }
        None => None,
    };
    let schema = app_data.schema.clone();
    let request = gql_request.into_inner().data(GqlCtx {
        db: app_data.db.clone(),
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
    let pool: DatabaseConnection = get_pool().await;
    HttpServer::new(move || {
        let schema = Schema::build(Query, Mutation, EmptySubscription).finish();
        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                schema: schema.clone(),
            }))
            .route("/gql", web::post().to(gql_index))
            .route("/gql", web::get().to(gql_playgound))
            .route("/", web::get().to(index))
            .route("/{filename:.*}", web::get().to(static_files))
    })
    .bind((host, port))?
    .run()
    .await
}
