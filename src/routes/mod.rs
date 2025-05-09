use axum::Router;
use super::routes::{generate::generate_proof_route, home::hello};

pub mod generate;
pub mod verify;
pub mod home;

pub fn routes() -> Router {
    Router::new()
        .route("/generate_proof", axum::routing::post(generate_proof_route))
        // .route("/verify_proof", axum::routing::post(verify_proof_route))
        .route("/", axum::routing::get(hello))
}