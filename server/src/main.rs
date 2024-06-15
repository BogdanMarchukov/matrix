use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_files::Files;


async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/", "../../../client/build").index_file("index.html"))
            .default_service(
                web::route().to(|| HttpResponse::NotFound())
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
