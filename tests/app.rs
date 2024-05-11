use just_recipe::{app::AppState, fetch_all_ingredient_ids, fetch_all_unit_ids};
use sqlx::PgPool;


#[sqlx::test(fixtures("ingredients"))]
async fn fetches_all_ingredients(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool).await;
    let ingredient_ids = fetch_all_ingredient_ids(&app_state.pool)
        .await
        .expect("should have filled ingredient_ids");
    assert!(ingredient_ids.len() > 0);
    for id in ingredient_ids.iter() {
        assert!(app_state.ingredient_ids.contains(&id));
    }
    Ok(())
}
#[sqlx::test(fixtures("units"))]
async fn fetches_all_units(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool).await;

    let unit_ids = fetch_all_unit_ids(&app_state.pool)
        .await
        .expect("should have filled unit_ids");
    assert!(unit_ids.len() > 0);
    
    for id in unit_ids.iter() {
        assert!(app_state.unit_ids.contains(&id));
    }
    Ok(())
}

#[sqlx::test(fixtures("units", "ingredients"))]
async fn app_initializes_cache(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool).await;
    let ingredient_ids = fetch_all_ingredient_ids(&app_state.pool)
        .await
        .expect("should have filled ingredient_ids");
    let unit_ids = fetch_all_unit_ids(&app_state.pool)
        .await
        .expect("should have filled unit_ids");
    for id in ingredient_ids.iter() {
        assert!(app_state.ingredient_ids.contains(&id));
    }
    for id in unit_ids.iter() {
        assert!(app_state.unit_ids.contains(&id));
    }
    Ok(())
}


