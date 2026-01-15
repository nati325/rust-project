use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Worker {
    #[serde(default)]
    pub id: i64,
    pub name: String,
    pub role: String,
    pub city_id: i64,
} 