use actix_web::{middleware, web, App, HttpRequest, HttpServer, Responder};
mod common;
mod errors;
mod question;
mod db;

extern crate dotenv;
use dotenv::dotenv;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info");
    std::env::set_var("RUST_BACKTRACE","1");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .configure(question::routes::init_routes)
            .configure(common::routes::init_routes)
    })
    .bind("127.0.0.1:8091")?
    .run()
    .await
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::dev::Service;
//     use actix_web::{http, test, web, App, Error};

//     #[actix_rt::test]
//     async fn test_welcome() -> Result<(), Error> {
//         let app = App::new()
//             .route("/", web::get().to(welcome))
//             .route("/{name}", web::get().to(welcome));
//         let mut app = test::init_service(app).await;

//         let req = test::TestRequest::get().uri("/").to_request();
//         let resp = app.call(req).await.unwrap();

//         assert_eq!(resp.status(), http::StatusCode::OK);

//         let response_body = match resp.response().body().as_ref() {
//             Some(actix_web::body::Body::Bytes(bytes)) => bytes,
//             _ => panic!("Response error"),
//         };

//         assert_eq!(response_body, "Hello World!");

//         let req = test::TestRequest::get().uri("/Rusty").to_request();
//         let resp = app.call(req).await.unwrap();

//         assert_eq!(resp.status(), http::StatusCode::OK);

//         let response_body = match resp.response().body().as_ref() {
//             Some(actix_web::body::Body::Bytes(bytes)) => bytes,
//             _ => panic!("Response error"),
//         };

//         assert_eq!(response_body, "Hello Rusty!");

//         Ok(())
//     }
// }
