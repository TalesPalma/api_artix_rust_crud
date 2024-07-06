mod controller;
mod database;
mod models;
mod repository;
mod schema;
mod services;

use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};
use config::{Config, ConfigError, File};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
struct AppConfig {
    server: ServerConfig,
}

fn load_config() -> Result<AppConfig, ConfigError> {
    let cfg = Config::builder()
        .set_default("default", "1")?
        .add_source(File::with_name("Actix"))
        .build()?;

    cfg.try_deserialize()
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_config().unwrap();
    let server_cfg = config.server;
    let andress = format!("{}:{}", server_cfg.host, server_cfg.port);

    println!("Server running on {}", andress);
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(index)
            .service(controller::clients_controller::get_clients)
            .service(controller::clients_controller::get_clients_by_id)
            .service(controller::clients_controller::create_clients)
            .service(controller::clients_controller::delete_clients)
    })
    .bind(&andress)?
    .run()
    .await
}
