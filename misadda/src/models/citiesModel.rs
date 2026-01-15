use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct City {
    #[serde(default)]
    pub id: i64,
    pub name: String,
} 