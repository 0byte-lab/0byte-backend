use axum;
use routes::routes;
use std::net::SocketAddr;


mod routes;
mod handlers;
mod models;

#[tokio::main]
async fn main() {
    let app = routes();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    
    println!("🚀 0byte server running on http://{}", addr);
    
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service()
    )
    .await
    .unwrap();
}
