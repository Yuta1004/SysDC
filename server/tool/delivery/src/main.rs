mod s3;

use actix_web::middleware::Logger;
use actix_web::http::{header, StatusCode};
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(||
            App::new()
                .service(get_tool_req_fix_path)
                .service(get_tool)
                .service(get_tool_files)
                .wrap(Logger::default())
        )
        .bind(("0.0.0.0", 50000))?
        .run()
        .await
}

#[get("/{author}/{tool}/{version}")]
async fn get_tool_req_fix_path(req: HttpRequest) -> impl Responder {
    HttpResponse::TemporaryRedirect()
        .append_header((
            header::LOCATION,
            format!("/tool/delivery{}/", req.uri().path())
        ))
        .finish()
}

#[get("/{author}/{tool}/{version}/")]
async fn get_tool(req: HttpRequest) -> impl Responder {
    let path = format!("{}/index.html", req.uri().path());
    create_f_response(&path).await
}

#[get("/{author}/{tool}/{version}/{file:.*}")]
async fn get_tool_files(req: HttpRequest) -> impl Responder {
    create_f_response(req.uri().path()).await
}

async fn create_f_response(path: &str) -> HttpResponse {
    match s3::get_file(path).await {
        Ok((mime, body)) =>
            HttpResponse::build(StatusCode::OK)
                .content_type(mime)
                .body(body),
        Err(_) =>
            HttpResponse::build(StatusCode::NOT_FOUND)
                .body("The specified file is not found.")
    }
}
