use actix_web::{get, web::Json, Responder};

#[get("/hello")]
pub async fn hello() -> impl Responder {
    Json("Hello world")
}
