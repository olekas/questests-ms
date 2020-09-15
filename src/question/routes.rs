use actix_web::{
    delete, 
    get, 
    post, 
    put, 
    patch,
    web, 
    HttpRequest, 
    HttpResponse, 
    Responder
};
use serde_json::json;

use super::model::*;
use crate::db;
use crate::errors;

#[get("/questions")]
async fn find_all(request: HttpRequest) -> impl Responder {
    println!("REQ: {:?}", request);
    match db::api::find::<Question>(None, None).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error finding questions: {}", err))
    }    
}

#[get("/questions/{id}")]
async fn find(request: HttpRequest) -> impl Responder {
    println!("REQ: {:?}", request);
    match request.match_info().get("id") {
        Some(id) => match db::api::get::<Question>(String::from(id)).await {
                Ok(data) => HttpResponse::Ok().json(data),
                Err(err) => HttpResponse::InternalServerError().json(format!("Error getting question by id {}: {}", id, err))
            },
        None => HttpResponse::BadRequest().json("Question identifer expected in request URL")
    }
}

#[post("/questions")]
async fn create(path_param: web::Path<String>, payload: web::Json<Question>) -> impl Responder {
    println!("REQ path_param: {:?}", path_param);
    println!("REQ payload: {:?}", payload);
    HttpResponse::Ok().finish()
}

#[patch("/questions/{id}")]
async fn update(request: HttpRequest) -> impl Responder {
    println!("REQ: {:?}", request);
    HttpResponse::Ok().finish()
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
}
