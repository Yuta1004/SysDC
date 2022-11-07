mod s3;
use actix_web::middleware::Logger;
use actix_web::{get, App, HttpRequest, HttpServer, Responder};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(||
            App::new().service(root).wrap(Logger::default())
        )
        .bind(("0.0.0.0", 50000))?
        .run()
        .await
}

#[get("/")]
async fn root() -> impl Responder {
    format!("hello!")
}
