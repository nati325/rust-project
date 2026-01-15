use axum::{routing::{get, post, delete, put}, Router};
use crate::controllers::workerController::{get_all_workers, get_worker_by_id, create_worker, delete_worker, update_worker};

pub fn worker_router() -> Router<sqlx::SqlitePool> {
    Router::new()
        .route("/", get(get_all_workers).post(create_worker))
        .route("/:id", get(get_worker_by_id).delete(delete_worker).put(update_worker))
}
