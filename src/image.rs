use actix_session::Session;
use actix_web::{get, web, HttpResponse};
use hyper::{body, client::HttpConnector, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};

use crate::album::from_link;
use crate::errors::ImagineError;

#[derive(Deserialize, Debug)]
struct ImageWrapper {
    data: Image,
}

#[derive(Deserialize, Serialize, Debug)]
struct Image {
    #[serde(deserialize_with = "from_link")]
    link: String,
}

#[get("/{uri:\\w+}")]
pub async fn image_handler(
    uri: web::Path<String>,
    session: Session,
    authorization: web::Data<&str>,
    client: web::Data<Client<HttpsConnector<HttpConnector>>>,
) -> Result<HttpResponse, ImagineError> {
    let uri_key: &str = &format!("image:{}", uri);

    let image = if let Some(im) = session.get::<Image>(uri_key)? {
        log::info!("{} found in cache, serving from cache...", uri_key);
        im
    } else {
        log::info!("{} not found in cache, fetching from remote...", uri_key);

        let authorization: &str = &authorization;

        let req = Request::builder()
            .method(Method::GET)
            .uri(format!("https://api.imgur.com/3/image/{}", uri))
            .header("Authorization", authorization)
            .body(body::Body::empty())?;

        let body = client.request(req).await?.into_body();
        let body = body::to_bytes(body).await?;
        let im: ImageWrapper = serde_json::from_slice(&body)?;
        let im = im.data;

        session.insert(uri_key, &im)?;
        im
    };

    log::info!("Redirecting to {}", image.link);
    Ok(HttpResponse::TemporaryRedirect()
        .insert_header(("Location", image.link))
        .body(""))
}
