use actix_web::{middleware::Logger, App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;

mod api;
mod services;
mod solana;
mod models;
mod config;

use crate::config::SETTINGS;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = &SETTINGS.port;
    let addr = format!("0.0.0.0:{}", port);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .service(api::proof::generate_proof)
    })
    .bind(addr)?
    .run()
    .await
}