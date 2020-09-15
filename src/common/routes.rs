use super::model::*;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

#[get("/health")]
async fn health(request: HttpRequest) -> impl Responder {
    println!("REQ: {:?}", request);
    HttpResponse::Ok().json(Health {
        healthy: true,
        version: "0.1.0".to_string(),
    })
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(health);
}
