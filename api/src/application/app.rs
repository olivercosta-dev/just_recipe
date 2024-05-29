use super::state::AppState;
use crate::{
    routes::{
        add_ingredient_handler, add_recipe_handler, add_unit_handler, get_all_ingredients_handler, get_ingredient_by_id_handler, get_ingredients_by_query_handler, get_recipe_by_query_handler, get_recipe_handler, get_unit_handler, get_units_by_query_handler, health_check, remove_ingredient_handler, remove_recipe_handler, remove_unit_handler, update_ingredient_handler, update_recipe_handler, update_unit_handler
    },
    utilities::fetchers::{fetch_all_ingredient_ids, fetch_all_unit_ids},
};
use axum::{
    routing::{get, post, put}, Router
};
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone, Debug)]
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

    /// Initializes the cache inside AppState
    /// and returns the new one with the included cache.
    ///
    /// <b>Note: This function might panic! Initializing the cache is crucial!</b>
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
        let cors = CorsLayer::new()
            .allow_methods(Any)
            .allow_origin(Any)
            .allow_headers(Any);
        Router::new()
            .route("/", get(health_check))
            .route(
                "/units",
                post(add_unit_handler)
                    .delete(remove_unit_handler)
                    .get(get_units_by_query_handler),
            )
            .route(
                "/units/:unit_id",
                put(update_unit_handler).get(get_unit_handler),
            )
            .route(
                "/ingredients",
                post(add_ingredient_handler)
                    .delete(remove_ingredient_handler)
                    .get(get_ingredients_by_query_handler),
            )
            .route("/ingredients/all", get(get_all_ingredients_handler))
            .route(
                "/ingredients/:ingredient_id",
                put(update_ingredient_handler).get(get_ingredient_by_id_handler),
            )
            .route(
                "/recipes",
                post(add_recipe_handler)
                    .delete(remove_recipe_handler)
                    .get(get_recipe_by_query_handler),
            )
            .route(
                "/recipes/:recipe_id",
                put(update_recipe_handler).get(get_recipe_handler),
            )
            .with_state(state)
            .layer(cors)
        // .layer(CatchPanicLayer::new())
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

#[allow(unused)]
mod test {
    use std::{default, sync::Arc};

    use axum::extract::State;
    use sqlx::PgPool;

    use crate::{
        application::{app::App, state::AppState},
        utilities::fetchers::{fetch_all_ingredient_ids, fetch_all_unit_ids},
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
