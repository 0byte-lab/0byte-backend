use actix_web::{middleware::Logger, App, HttpServer};
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

    let bind_addr = &SETTINGS.server_addr;

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(api::proof::generate_proof)
    })
    .bind(bind_addr.as_str())?
    .run()
    .await
}