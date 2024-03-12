use axum::{routing::get, Router};
use handler::handler::get_handler;
mod handler;
mod pool;
mod service;
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_handler().await));
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
