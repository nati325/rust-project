use axum::{extract::{Path, State}, Json, http::StatusCode, response::IntoResponse};
use sqlx::SqlitePool;
use crate::models::workerModel::Worker;

pub async fn get_all_workers(State(pool): State<SqlitePool>) -> impl IntoResponse {
    match sqlx::query_as::<_, Worker>("SELECT * FROM workers")
        .fetch_all(&pool)
        .await {
        Ok(workers) => Json(workers).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch workers").into_response(),
    }
}

pub async fn get_worker_by_id(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, Worker>("SELECT * FROM workers WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await {
        Ok(worker) => Json(worker).into_response(),
        Err(sqlx::Error::RowNotFound) => (StatusCode::NOT_FOUND, "Worker not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch worker").into_response(),
    }
}

pub async fn create_worker(
    State(pool): State<SqlitePool>,
    Json(new_worker): Json<Worker>,
) -> impl IntoResponse {
    let city_exists = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM cities WHERE id = ?")
        .bind(new_worker.city_id)
        .fetch_one(&pool)
        .await;
    match city_exists {
        Ok(count) if count > 0 => {
            let result = sqlx::query("INSERT INTO workers (name, role, city_id) VALUES (?, ?, ?)")
                .bind(&new_worker.name)
                .bind(&new_worker.role)
                .bind(new_worker.city_id)
                .execute(&pool)
                .await;
            match result {
                Ok(res) => {
                    let id = res.last_insert_rowid() as i64;
                    let created_worker = Worker {
                        id,
                        name: new_worker.name,
                        role: new_worker.role,
                        city_id: new_worker.city_id,
                    };
                    (StatusCode::CREATED, Json(created_worker)).into_response()
                }
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create worker").into_response(),
            }
        }
        Ok(_) => (StatusCode::BAD_REQUEST, format!("City with id {} does not exist.", new_worker.city_id)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to check city existence").into_response(),
    }
}

pub async fn delete_worker(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    match sqlx::query("DELETE FROM workers WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await {
        Ok(result) => {
            if result.rows_affected() > 0 {
                (StatusCode::OK, Json(true)).into_response()
            } else {
                (StatusCode::NOT_FOUND, Json(false)).into_response()
            }
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete worker").into_response(),
    }
}

pub async fn update_worker(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Json(updated_worker): Json<Worker>,
) -> impl IntoResponse {
    let city_exists = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM cities WHERE id = ?")
        .bind(updated_worker.city_id)
        .fetch_one(&pool)
        .await;
    match city_exists {
        Ok(0) => (StatusCode::BAD_REQUEST, format!("City with id {} does not exist.", updated_worker.city_id)).into_response(),
        Ok(_) => {
            let result = sqlx::query("UPDATE workers SET name = ?, role = ?, city_id = ? WHERE id = ?")
                .bind(&updated_worker.name)
                .bind(&updated_worker.role)
                .bind(updated_worker.city_id)
                .bind(id)
                .execute(&pool)
                .await;
            match result {
                Ok(res) => {
                    if res.rows_affected() == 0 {
                        (StatusCode::NOT_FOUND, "Worker not found").into_response()
                    } else {
                        match sqlx::query_as::<_, Worker>("SELECT * FROM workers WHERE id = ?")
                            .bind(id)
                            .fetch_one(&pool)
                            .await {
                                Ok(worker) => Json(worker).into_response(),
                                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch updated worker").into_response(),
                            }
                    }
                }
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update worker").into_response(),
            }
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to check city existence").into_response(),
    }
}
