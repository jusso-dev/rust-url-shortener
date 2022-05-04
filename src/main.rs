
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod ops;
mod lib;
mod schema;
mod models;

use crate::models::ApiUrl;
use crate::models::UpdateUrl;
use crate::models::Url;
use ops::url_ops;

use actix_web::{get, put, post, delete, web, App, HttpServer, web::Json, Result};

#[get("/")]
async fn ping() -> Json<String> {
    Json("Pong".to_string())
}

#[get("/get-urls")]
async fn get_urls() -> Json<Vec<Url>> {
    let urls = url_ops::get_urls();
    match urls {
        Some(urls) => Json(urls),
        None => Json(vec![])
    }
}

#[put("/update-url")]
async fn update_url(url:Json<UpdateUrl>) -> Result<String> {
    let result = url_ops::update_url(url.into_inner());
    match result {
        Some(true) => Ok("Updated".to_string()),
        Some(false) => Ok("Failed".to_string()),
        None => Ok("Failed".to_string())
    }
}

#[post("/add-url")]
async fn add_url(url: Json<ApiUrl>) -> Result<String> {
    let result = url_ops::create_url(url.into_inner());
    match result {
        Some(true) => Ok("Created".to_string()),
        Some(false) => Ok("Failed".to_string()),
        None => Ok("Failed".to_string())
    }
}

#[delete("/delete-url/{id}")]
async fn delete_url(id:web::Path<i32>) -> Result<String> {
    let result = url_ops::delete_user(id.into_inner());
    match result {
        Some(true) => Ok("Deleted".to_string()),
        Some(false) => Ok("Failed".to_string()),
        None => Ok("Failed".to_string())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 3000;

    println!("Server started on port {}", &port);
    HttpServer::new(|| {
        App::new()
            .service(ping)
            .service(get_urls)
            .service(add_url)
            .service(update_url)
            .service(delete_url)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}