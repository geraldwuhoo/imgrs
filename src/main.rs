// #![deny(warnings)]

mod album;
mod directimage;
mod errors;
mod image;

use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, get, middleware, web, App, HttpResponse, HttpServer};
use actix_web_static_files::ResourceFiles;
use hyper::Client;
use hyper_tls::HttpsConnector;

use crate::album::album_handler;
use crate::directimage::direct_image_handler;
use crate::image::image_handler;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[get("/favicon.ico")]
pub async fn favicon_handler() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body("")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    let signing_key = Key::from(&[0; 64]);
    let https = HttpsConnector::new();
    let client = web::Data::new(Client::builder().build::<_, hyper::Body>(https));

    println!("Starting HTTP server...");

    HttpServer::new(move || {
        let generated = generate();
        App::new()
            .app_data(web::Data::new("Client-ID 546c25a59c58ad7"))
            .app_data(web::Data::clone(&client))
            .wrap(middleware::Logger::default())
            .wrap(SessionMiddleware::new(
                RedisActorSessionStore::new("127.0.0.1:6379"),
                signing_key.clone(),
            ))
            .service(ResourceFiles::new("/static", generated))
            .service(
                web::scope("")
                    .service(favicon_handler)
                    .service(image_handler)
                    .service(album_handler)
                    .service(direct_image_handler),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
