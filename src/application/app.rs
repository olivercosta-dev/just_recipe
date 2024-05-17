use super::state::AppState;
use crate::{
    fetch_all_ingredient_ids, fetch_all_unit_ids,
    routes::{
        add_ingredient, add_recipe, add_unit, get_ingredient_by_id, get_ingredients_by_query, get_recipe, get_unit, health_check, remove_ingredient, remove_recipe, remove_unit, update_ingredient, update_recipe, update_unit
    },
};
use axum::{
    routing::{get, post, put},
    Router,
};
use tower_http::catch_panic::CatchPanicLayer;

pub struct App {
    url: String,
    port: i32,
    // This router is what owns the AppState
    // It is purposefully not Router<AppState>
    // As recommended by the axum documentation:
    // "We can only call Router::into_make_service on Router<()>, not Router<AppState>"
    // Router<S> means a router that is MISSING a state of type S to be able to handle requests.
    // It does NOT mean a Router that HAS a state of type S.
    // https://docs.rs/axum/latest/axum/routing/struct.Router.html#method.with_state
    pub router: Router,
}

impl App {
    pub async fn new(state: AppState, url: String, port: i32) -> App {
        let state = Self::init_cache(state).await;
        let router = Self::create_router(state);
        App { url, port, router }
    }
    // OPTIMIZE (oliver): This can panic!
    async fn init_cache(state: AppState) -> AppState {
        let unit_ids = fetch_all_unit_ids(&state.pool)
            .await
            .expect("should have fetched the unit_id set");
        let ingredient_ids = fetch_all_ingredient_ids(&state.pool)
            .await
            .expect("should have fetched the ingredient_id set");
        AppState {
            pool: state.pool,
            unit_ids,
            ingredient_ids,
        }
    }

    fn create_router(state: AppState) -> Router {
        Router::new()
            .route("/", get(health_check))
            .route("/units", post(add_unit).delete(remove_unit))
            .route("/units/:unit_id", put(update_unit).get(get_unit))
            .route(
                "/ingredients",
                post(add_ingredient)
                    .delete(remove_ingredient)
                    .get(get_ingredients_by_query),
            )
            .route(
                "/ingredients/:ingredient_id",
                put(update_ingredient).get(get_ingredient_by_id),
            )
            .route("/recipes", post(add_recipe).delete(remove_recipe))
            .route("/recipes/:recipe_id", put(update_recipe).get(get_recipe))
            .layer(CatchPanicLayer::new())
            .with_state(state)
    }

    pub async fn serve(self) {
        let listener: tokio::net::TcpListener =
            tokio::net::TcpListener::bind(format!("{}:{}", self.url, self.port))
                .await
                .unwrap();
        println!("Listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, self.router).await.unwrap();
    }
}

mod test {
    use std::{default, sync::Arc};

    use axum::extract::State;
    use sqlx::PgPool;

    use crate::{
        application::{app::App, state::AppState},
        fetch_all_ingredient_ids, fetch_all_unit_ids,
    };
    #[sqlx::test(fixtures(path = "../../tests/fixtures", scripts("ingredients")))]
    async fn fetches_all_ingredients(pool: PgPool) -> sqlx::Result<()> {
        let fetched_ingredient_ids = fetch_all_ingredient_ids(&pool)
            .await
            .expect("should have filled ingredient_ids");
        let ingredient_ids_from_db = sqlx::query!("SELECT ingredient_id FROM ingredient")
            .fetch_all(&pool)
            .await?;

        assert_eq!(fetched_ingredient_ids.len(), ingredient_ids_from_db.len());

        for rec in ingredient_ids_from_db.iter() {
            assert!(fetched_ingredient_ids.contains(&rec.ingredient_id));
        }
        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../tests/fixtures", scripts("units")))]
    async fn fetches_all_units(pool: PgPool) -> sqlx::Result<()> {
        let fetched_unit_ids = fetch_all_unit_ids(&pool)
            .await
            .expect("should have filled ingredient_ids");
        let unit_ids_from_db = sqlx::query!("SELECT unit_id FROM unit")
            .fetch_all(&pool)
            .await?;

        assert_eq!(fetched_unit_ids.len(), unit_ids_from_db.len());

        for rec in unit_ids_from_db.iter() {
            assert!(fetched_unit_ids.contains(&rec.unit_id));
        }
        Ok(())
    }
}
