
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
use crate::models::Validation;
use actix_web::http;
use ops::url_ops;

use actix_web::{get, put, post, delete, web, App, HttpServer, web::Json, HttpResponse, Responder};

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

#[get("/redirect/{short_url}")]
async fn redirect(short_url:web::Path<String>) -> impl Responder{
    if short_url.to_string() == "" {
        return HttpResponse::BadRequest().json("No short url provided");
    }
    let url = url_ops::get_url(short_url.to_string());
    match url {
        Some(_url) => return HttpResponse::TemporaryRedirect()
        .insert_header((http::header::LOCATION, _url.long_url.to_string()))
        .finish(),
        None => return HttpResponse::NotFound().json("Url not found")
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
    let result = url_ops::create_url(url.into_inner()).await.unwrap();
    match result {
        Validation::UrlDuplicate => return HttpResponse::Conflict().json("Url already exists".to_string()),
        Validation::UrlInvalid => return HttpResponse::BadRequest().json("Invalid URL".to_string()),
        Validation::Success => HttpResponse::Ok().json("Created".to_string())
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
    let port = 3001;

    println!("Server started on port {}", &port);
    HttpServer::new(|| {
        App::new()
            .service(ping)
            .service(get_urls)
            .service(redirect)
            .service(add_url)
            .service(update_url)
            .service(delete_url)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}