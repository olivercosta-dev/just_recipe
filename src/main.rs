use axum::{routing::get, Router};
use just_recipe::routes::*;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(health_check));
    
    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

