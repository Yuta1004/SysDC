mod s3;
use actix_web::{get, App, HttpServer, Responder};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(root))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

#[get("/")]
async fn root() -> impl Responder {
    format!("hello!")
}
