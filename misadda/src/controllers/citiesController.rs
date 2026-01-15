use axum::{extract::{Path, State}, Json, http::StatusCode, response::IntoResponse}; //תיקשור של השרת
use sqlx::SqlitePool;
use crate::models::citiesModel::City;

pub async fn get_all_cities(State(pool): State<SqlitePool>) -> impl IntoResponse {
    match sqlx::query_as::<_, City>("SELECT * FROM cities")
        .fetch_all(&pool)
        .await {
        Ok(cities) => Json(cities).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch cities").into_response(),
    }
}

pub async fn get_city_by_id(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, City>("SELECT * FROM cities WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await {
        Ok(city) => Json(city).into_response(),
        Err(sqlx::Error::RowNotFound) => (StatusCode::NOT_FOUND, "City not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch city").into_response(),
    }
}

pub async fn create_city(
    State(pool): State<SqlitePool>,
    Json(new_city): Json<City>,  
) -> impl IntoResponse {
    match sqlx::query("INSERT INTO cities (name) VALUES (?)")
        .bind(&new_city.name)
        .execute(&pool)
        .await {
        Ok(result) => {
            let id = result.last_insert_rowid() as i64;
            let created_city = City {
                id,
                name: new_city.name,
            };
            (StatusCode::CREATED, Json(created_city)).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create city").into_response(),
    }
}

pub async fn delete_city(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    match sqlx::query("DELETE FROM cities WHERE id = ?")
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
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete city").into_response(),
    }
}

pub async fn update_city(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Json(updated_city): Json<City>,
) -> impl IntoResponse {
    match sqlx::query("UPDATE cities SET name = ? WHERE id = ?")
        .bind(&updated_city.name)
        .bind(id)
        .execute(&pool)
        .await {
        Ok(result) => {
            if result.rows_affected() == 0 {
                (StatusCode::NOT_FOUND, "City not found").into_response()
            } else {
                let city = City {
                    id,
                    name: updated_city.name,
                };
                Json(city).into_response()
            }
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update city").into_response(),
    }
}