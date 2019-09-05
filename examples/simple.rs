use actix_service::{ Transform};
use actix_web::middleware::{NormalizePath};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use actix_web_condition_middleware::Condition;
use env_logger;


pub fn main() {
    use actix_web::{App, HttpServer};
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        // App::new().wrap(Logger::default())
        App::new().wrap(Condition::new(true, NormalizePath))
    })
    .bind("127.0.0.1:8888")
    .unwrap()
    .run()
    .unwrap();
}
