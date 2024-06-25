use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use include_dir::{include_dir as include_d, Dir};
use mime_guess::from_path;
use std::env;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut host = String::from("127.0.0.1");
    match std::env::var("HOST") {
        Ok(val) => host = val,
        Err(e) => println!("error: {}", e),
    };
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/{filename:.*}", web::get().to(static_files))
    })
    .bind((host, 3000))?
    .run()
    .await
}
