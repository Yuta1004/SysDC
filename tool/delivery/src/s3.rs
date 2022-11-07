use http::uri::Uri;

use aws_sdk_s3::{ Client, Endpoint };

const ENDPOINT: &str = "http://localhost:9000";
const BUCKET: &str = "sysdc-tools";

async fn create_connection() -> Client {
    let ep = Endpoint::immutable(Uri::from_static(ENDPOINT));
    let conf = aws_config::load_from_env().await;
    let s3_conf = aws_sdk_s3::config::Builder::from(&conf)
        .endpoint_resolver(ep)
        .build();
    Client::from_conf(s3_conf)
}

// pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
//     let s3 = create_connection().await;

//     let resp = s3.list_objects_v2().bucket(BUCKET).send().await?;
//     for obj in resp.contents().unwrap() {
//         println!("* {:?}", obj.key());
//     }

//     Ok(())
// }
