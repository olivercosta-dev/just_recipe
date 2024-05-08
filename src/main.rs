use just_recipe::app::*;
use sqlx::PgPool;

// TODO (oliver): I need to implemenet error propagation to use with the operator "?"
#[tokio::main]
async fn main() {

    let pool = PgPool::connect("postgres://postgres@localhost/just_recipe")
        .await
        .expect("should have connected to the database");

    let app_state = AppState {pool};
    let app = new_app(app_state).await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
