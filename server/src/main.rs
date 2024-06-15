use actix_files::NamedFile;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use include_dir::{include_dir as include_d, Dir};
use std::path::PathBuf;

const FRONTEND_DIR: Dir = include_d!("../client/build");

async fn index(_req: HttpRequest) -> HttpResponse {
    let file = FRONTEND_DIR.get_file("index.html").unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(file.contents())
}

async fn static_files(req: HttpRequest) -> HttpResponse {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let file = FRONTEND_DIR.get_file(&path).unwrap();
    println!("{}", &path.to_str().unwrap());
    HttpResponse::Ok()
        .content_type("application/json")
        .body(file.contents())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/{filename:.*}", web::get().to(static_files))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
