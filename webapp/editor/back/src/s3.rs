use http::uri::Uri;

use actix_web::web::Bytes;
use aws_sdk_s3::{Client, Endpoint};

const ENDPOINT: &str = "http://tool-storage:9000";
const BUCKET: &str = "sysdc-workspaces";

async fn create_connection() -> Client {
    let ep = Endpoint::immutable(Uri::from_static(ENDPOINT));
    let conf = aws_config::load_from_env().await;
    let s3_conf = aws_sdk_s3::config::Builder::from(&conf)
        .endpoint_resolver(ep)
        .build();
    Client::from_conf(s3_conf)
}

pub async fn get_file_list(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let s3 = create_connection().await;
    let objects = s3
        .list_objects_v2()
        .bucket(BUCKET)
        .prefix(path)
        .send()
        .await?;

    let file_list = objects
        .contents()
        .unwrap_or_default()
        .into_iter()
        .map(|obj| obj.key().unwrap().to_string())
        .collect();

    Ok(file_list)
}

pub async fn get_file(path: &str) -> Result<(String, Bytes), Box<dyn std::error::Error>> {
    let s3 = create_connection().await;
    let resp = s3
        .get_object()
        .bucket(BUCKET)
        .key(path)
        .send()
        .await?;

    let mime = resp
        .content_type()
        .unwrap_or("text/plain")
        .to_string();
    let body = resp
        .body
        .collect()
        .await?
        .into_bytes();

    Ok((mime, body))
}
