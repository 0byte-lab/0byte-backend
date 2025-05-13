mod handlers;
mod models;
mod services;
mod utils;
use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use handlers::proof::generate_proof;
use utils::circuits::load_circuit;
use acvm_backend_barretenberg::Barretenberg;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let circuit = load_circuit()
        .expect("Failed to load circuit");
    let backend = Arc::new(Barretenberg::new());
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(circuit.clone()))
            .app_data(web::Data::new(backend.clone()))
            .service(
                web::resource("/generate-proof")
                    .route(web::post().to(generate_proof))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}