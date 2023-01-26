mod s3;

use rand::distributions::{Alphanumeric, DistString};
use actix_web::middleware::Logger;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(||
            App::new()
                .service(save_workspace)
                .service(get_workspace_info)
                .service(get_workspace_files)
                .wrap(Logger::default())
        )
        .bind(("0.0.0.0", 50000))?
        .run()
        .await
}

#[post("/workspace")]
async fn save_workspace(mut payload: Multipart) -> impl Responder {
    let workspace = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
    while let Ok(Some(mut field)) = payload.try_next().await {
        let mut body = web::BytesMut::new();
        while let Some(chunk) = field.next().await {
            body.extend_from_slice(&chunk.unwrap())
        }
        let filename = field.name();

        s3::save_file(
            &format!("{}/{}", workspace, filename),
            body.to_vec()
        ).await.unwrap();
    }

    HttpResponse::build(StatusCode::OK).body(workspace)
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
    let path = req.uri().path().replace("/workspace", "");
    match s3::get_file(&path).await {
        Ok((mime, body)) =>
            HttpResponse::build(StatusCode::OK)
                .content_type(mime)
                .body(body),
        Err(_) =>
            HttpResponse::build(StatusCode::NOT_FOUND)
                .body("The specified file is not found.")
    }
}
