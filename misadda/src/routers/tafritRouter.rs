use axum::{routing::{get, post, delete, put}, Router};
use crate::controllers::tafritController::{get_all_tafrits, get_tafrit_by_id, create_tafrit, delete_tafrit, update_tafrit};

pub fn tafrit_router() -> Router<sqlx::SqlitePool> {
    Router::new()
        .route("/", get(get_all_tafrits).post(create_tafrit))
        .route("/:id", get(get_tafrit_by_id).delete(delete_tafrit).put(update_tafrit))
}
