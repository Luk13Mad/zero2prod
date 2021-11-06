use actix_web::{HttpResponse, Responder};

pub async fn health_check() -> impl Responder {
    return HttpResponse::Ok();
}
