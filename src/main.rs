use just_recipe::app::*;

#[tokio::main]
async fn main() {
    let app = new_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
