
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod ops;
mod lib;
mod schema;
mod models;

use crate::models::Url;
use ops::show_urls;

use actix_web::{get, App, HttpServer, web::Json};

#[get("/")]
async fn hello() -> Json<String> {
    Json("Hello, world!".to_string())
}

#[get("/get-urls")]
async fn get_urls() -> Json<Vec<Url>> {
    let urls = show_urls::get_urls();
    Json(urls)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(get_urls)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}