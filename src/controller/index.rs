use actix_web::{web, HttpResponse, HttpRequest};

pub async fn index(req : HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json("success")
}