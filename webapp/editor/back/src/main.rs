mod s3;

use actix_web::middleware::Logger;
use actix_web::http::StatusCode;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(||
            App::new()
                .service(get_workspace_info)
                .service(get_workspace_files)
                .wrap(Logger::default())
        )
        .bind(("0.0.0.0", 50000))?
        .run()
        .await
}

#[get("/workspace/{workspace}")]
async fn get_workspace_info(path: web::Path<(String,)>) -> impl Responder {
    let file_list = s3::get_file_list(&path.0).await.unwrap();
    if file_list.len() > 0 {
        HttpResponse::build(StatusCode::OK)
            .content_type("application/json")
            .json(file_list)   
    } else {
        HttpResponse::build(StatusCode::NOT_FOUND)
            .body("The specified workspace is not found.")
    }
}

#[get("/workspace/{workspace}/{file:.*}")]
async fn get_workspace_files(req: HttpRequest) -> impl Responder {
    match s3::get_file(req.uri().path()).await {
        Ok((mime, body)) =>
            HttpResponse::build(StatusCode::OK)
                .content_type(mime)
                .body(body),
        Err(_) =>
            HttpResponse::build(StatusCode::NOT_FOUND)
                .body("The specified file is not found.")
    }
}
