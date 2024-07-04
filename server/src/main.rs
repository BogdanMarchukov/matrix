use actix::Addr;
use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQL, GraphQLRequest, GraphQLResponse};
use include_dir::{include_dir as include_d, Dir};
use mime_guess::from_path;
#[path = "common/config/config.rs"]
mod config;
#[path = "common/db_utils/db_utils.rs"]
mod db_utils;
mod get_user;
#[path = "common/gql/gql_schema.rs"]
mod gql_schema;
mod models;
mod schema;
#[path = "user/user.rs"]
mod user;
use crate::gql_schema::Query;
use actix::SyncArbiter;
use db_utils::{get_pool, DbActor};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

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

type GqlSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub struct AppState {
    pub db: Addr<DbActor>,
    pub schema: GqlSchema,
}

async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish())
}

async fn gql_index(app_data: web::Data<AppState>, gql_request: GraphQLRequest) -> GraphQLResponse {
    let schema = app_data.schema.clone();
    let request = gql_request.into_inner();
    let que = serde_json::to_string(&request).unwrap_or(String::from("{}"));
    println!("{}", que);
    let result = schema.execute(request).await;
    GraphQLResponse::from(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = config::get_host();
    let port = config::get_port();
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool();
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();
        App::new()
            .app_data(web::Data::new(AppState {
                db: db_addr.clone(),
                schema: schema.clone(),
            }))
            .route("/", web::post().to(gql_index))
            .route("/", web::get().to(gql_playgound))
    })
    .bind((host, port))?
    .run()
    .await
}
