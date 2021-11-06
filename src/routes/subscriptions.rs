use actix_web::{web,HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SubscriptionForm {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(form: web::Form<SubscriptionForm>) -> impl Responder {
    HttpResponse::Ok().finish()
}