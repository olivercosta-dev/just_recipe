use just_recipe::application::{app::App, state::AppState};
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let pool = PgPool::connect("postgres://postgres@localhost/just_recipe")
        .await
        .expect("should have connected to the database");
    let state: AppState = AppState::new(pool);
    let app = App::new(state, String::from("0.0.0.0"), 8080).await;
    app.serve().await;
}
