use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result};
use include_dir::{Dir, include_dir as include_d};
use actix_files as fs;

const FRONTEND_DIR: Dir = include_d!("../client/build");

async fn index(_req: HttpRequest) -> HttpResponse {
    let file = FRONTEND_DIR.get_file("index.html").unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(file.contents())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(fs::Files::new("/static", ".").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

