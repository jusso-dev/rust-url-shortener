
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod ops;
mod lib;
mod schema;
mod models;

use crate::models::HealthCheck;
use crate::models::ApiUrl;
use crate::models::UpdateUrl;
use ops::url_ops;

use actix_web::{get, put, post, delete, web, App, HttpServer, web::Json, Result, HttpResponse, Responder};

#[get("/")]
async fn ping() -> Json<HealthCheck> {
    let response = HealthCheck {
        message: "Pong".to_string()
    };
    Json(response)
}

#[get("/get-urls")]
async fn get_urls() -> impl Responder {
    let urls = url_ops::get_urls();
    match urls {
        Some(urls) => return HttpResponse::Ok().json(urls),
        None => return HttpResponse::InternalServerError().json("Error getting urls")
    }
}

#[put("/update-url")]
async fn update_url(url:Json<UpdateUrl>) -> impl Responder {
    let result = url_ops::update_url(url.into_inner());
    match result {
        Some(true) => return HttpResponse::Ok().json("Updated".to_string()),
        Some(false) => return HttpResponse::InternalServerError().json("Failed".to_string()),
        None => return HttpResponse::InternalServerError().json("Failed".to_string())
    }
}

#[post("/add-url")]
async fn add_url(url: Json<ApiUrl>) -> impl Responder {
    let result = url_ops::create_url(url.into_inner());
    match result {
        Some(true) => return HttpResponse::Ok().json("Created".to_string()),
        Some(false) => return HttpResponse::InternalServerError().json("Failed".to_string()),
        None => return HttpResponse::InternalServerError().json("Failed".to_string())
    }
}

#[delete("/delete-url/{id}")]
async fn delete_url(id:web::Path<i32>) -> impl Responder {
    let result = url_ops::delete_user(id.into_inner());
    match result {
        Some(true) => return HttpResponse::Ok().json("Deleted".to_string()),
        Some(false) => return HttpResponse::InternalServerError().json("Failed".to_string()),
        None => return HttpResponse::InternalServerError().json("Failed".to_string())
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