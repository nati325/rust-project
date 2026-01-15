use serde ::{Deserialize, Serialize};
use sqlx::FromRow;
#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Tafrit {
    #[serde(default)]
    pub id: i64,
    pub item_name: String,
    pub price: f64,
}