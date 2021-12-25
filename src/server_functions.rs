use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async  fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}