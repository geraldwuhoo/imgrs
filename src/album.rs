use actix_session::Session;
use actix_web::{get, web, HttpResponse, Result};
use askama::Template;
use hyper::{body, client::HttpConnector, Client, Method, Request};
use hyper_tls::HttpsConnector;
use log;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json;
use url::Url;

use crate::errors::ImagineError;

#[derive(Deserialize, Debug)]
struct AlbumWrapper {
    data: Album,
}

#[derive(Deserialize, Serialize, Debug, Template)]
#[template(path = "album.html")]
struct Album {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    description: Option<String>,
    images: Vec<AlbumImage>,
}

#[derive(Deserialize, Serialize, Debug)]
struct AlbumImage {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(deserialize_with = "from_link")]
    link: String,
}

pub fn from_link<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if let Ok(url) = Url::parse(&s) {
        Ok(url.path().to_string())
    } else {
        Ok(s)
    }
}

#[get("/a/{uri}")]
pub async fn album_handler(
    uri: web::Path<String>,
    session: Session,
    authorization: web::Data<&str>,
    client: web::Data<Client<HttpsConnector<HttpConnector>>>,
) -> Result<HttpResponse, ImagineError> {
    let uri_key: &str = &format!("album:{}", uri);

    let album = if let Some(a) = session.get::<Album>(uri_key)? {
        log::info!("{} found in cache, serving from cache...", uri_key);
        a
    } else {
        log::info!("{} not found in cache, fetching from remote...", uri_key);

        let authorization: &str = &authorization;

        let req = Request::builder()
            .method(Method::GET)
            .uri(format!("https://api.imgur.com/3/album/{}", uri))
            .header("Authorization", authorization)
            .body(body::Body::empty())?;

        let body = client.request(req).await?.into_body();
        let body = body::to_bytes(body).await?;
        let a: AlbumWrapper = serde_json::from_slice(&body)?;
        let a = a.data;

        session.insert(uri_key, &a)?;
        a
    };

    let album_page = album.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(album_page))
}
