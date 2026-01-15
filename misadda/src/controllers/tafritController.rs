use axum::{extract::{Path, State}, Json, http::StatusCode, response::IntoResponse};
use sqlx::SqlitePool;
use crate::models::tafritModel::Tafrit;

pub async fn get_all_tafrits(State(pool): State<SqlitePool>) -> impl IntoResponse {
    match sqlx::query_as::<_, Tafrit>("SELECT * FROM tafrits")
        .fetch_all(&pool)
        .await {
        Ok(tafrits) => Json(tafrits).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch tafrits").into_response(),
    }
}

pub async fn get_tafrit_by_id(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, Tafrit>("SELECT * FROM tafrits WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await {
        Ok(tafrit) => Json(tafrit).into_response(),
        Err(sqlx::Error::RowNotFound) => (StatusCode::NOT_FOUND, "Tafrit not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch tafrit").into_response(),
    }
}

pub async fn create_tafrit(
    State(pool): State<SqlitePool>,
    Json(new_tafrit): Json<Tafrit>,
) -> impl IntoResponse {
    match sqlx::query("INSERT INTO tafrits (item_name, price) VALUES (?, ?)")
        .bind(&new_tafrit.item_name)
        .bind(new_tafrit.price)
        .execute(&pool)
        .await {
        Ok(result) => {
            let id = result.last_insert_rowid() as i64;
            let created_tafrit = Tafrit {
                id,
                item_name: new_tafrit.item_name,
                price: new_tafrit.price,
            };
            (StatusCode::CREATED, Json(created_tafrit)).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create tafrit").into_response(),
    }
}

pub async fn delete_tafrit(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    match sqlx::query("DELETE FROM tafrits WHERE id = ?")
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
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete tafrit").into_response(),
    }
}

pub async fn update_tafrit(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Json(update_tafrit): Json<Tafrit>,
) -> impl IntoResponse {
    match sqlx::query("UPDATE tafrits SET item_name = ?, price = ? WHERE id = ?")
        .bind(&update_tafrit.item_name)
        .bind(update_tafrit.price)
        .bind(id)
        .execute(&pool)
        .await {
        Ok(result) => {
            if result.rows_affected() == 0 {
                (StatusCode::NOT_FOUND, "Tafrit not found").into_response()
            } else {
                let tafrit = Tafrit {
                    id,
                    item_name: update_tafrit.item_name,
                    price: update_tafrit.price,
                };
                Json(tafrit).into_response()
            }
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update tafrit").into_response(),
    }
}