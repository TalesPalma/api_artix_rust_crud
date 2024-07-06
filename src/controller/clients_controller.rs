use actix_web::{delete, get, post, web, HttpResponse, Responder};

use crate::{models::client::Client, services};

#[get("clients")]
pub async fn get_clients() -> impl Responder {
    match services::clients_service::get_clients_service() {
        Ok(clients) => HttpResponse::Ok().json(clients),
        Err(_) => HttpResponse::InternalServerError().json("Error"),
    }
}

#[get("clients/{id}")]
pub async fn get_clients_by_id(id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().json(id.to_string())
}

#[post("clients")]
pub async fn create_clients(req_body: web::Json<Client>) -> impl Responder {
    HttpResponse::Created().json(req_body)
}

#[delete("clients/{id}")]
pub async fn delete_clients() -> impl Responder {
    HttpResponse::NoContent().json("")
}
