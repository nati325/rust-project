use axum::{routing::{get, post, delete, put}, Router};
use crate::controllers::citiesController::{get_all_cities, get_city_by_id, create_city, delete_city, update_city};

pub fn cities_router() -> Router<sqlx::SqlitePool> {
    Router::new()
        .route("/", get(get_all_cities).post(create_city))
        .route("/:id", get(get_city_by_id).delete(delete_city).put(update_city))
}
