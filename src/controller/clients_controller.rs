use actix_web::{delete, get, post, HttpResponse, Responder};

use crate::services;

#[get("clients")]
pub async fn get_clients() -> impl Responder {
    match services::clients_service::get_clients_service() {
        Ok(clients) => HttpResponse::Ok().json(clients),
        Err(_) => HttpResponse::InternalServerError().json("Error"),
    }
}

#[get("clients/{id}")]
pub async fn get_clients_by_id() -> impl Responder {
    HttpResponse::Ok().json("")
}

#[post("clients")]
pub async fn create_clients() -> impl Responder {
    HttpResponse::Ok().json("")
}

#[delete("clients")]
pub async fn delete_clients() -> impl Responder {
    HttpResponse::Ok().json("")
}
