use actix_session::Session;
use actix_web::{get, web, HttpResponse, Result};
use http::HeaderValue;
use hyper::{body, Client};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use log;

use crate::errors::ImagineError;

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Image {
    content_type: String,
    content: Vec<u8>,
}

#[get("/{uri:\\w+\\.(jpg|jpeg|png|gif|gifv|apng|tiff|mp4|mpeg|avi|webm|ogg)}")]
pub async fn direct_image_handler(
    uri: web::Path<String>,
    session: Session,
) -> Result<HttpResponse, ImagineError> {
    let uri_key: &str = &format!("direct:{}", uri);

    let image = if let Some(im) = session.get::<Image>(uri_key)? {
        log::info!("{} found in cache, serving from cache...", uri_key);
        im
    } else {
        log::info!("{} not found in cache, fetching from remote...", uri_key);
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        let (parts, body) = client
            .get(format!("https://i.imgur.com/{}", uri).parse()?)
            .await?
            .into_parts();

        let content = body::to_bytes(body).await?.to_vec();
        let default_header = HeaderValue::from_static("application/octet-stream");
        let content_type = parts.headers.get("content-type").unwrap_or(&default_header);
        let im = Image {
            content_type: content_type.to_str()?.to_string(),
            content,
        };

        session.insert(uri_key, &im)?;
        im
    };

    Ok(HttpResponse::Ok()
        .content_type(image.content_type)
        .body(image.content))
}
