use sqlx::sqlite::SqlitePoolOptions;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_url = "sqlite:resturant.db";
    let pool = SqlitePoolOptions::new()
        .connect(db_url)
        .await
        .expect("Failed to connect to DB");

    // יצירת טבלת ערים
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS cities (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
        "#
    )
    .execute(&pool)
    .await?;

    // יצירת טבלת תפריט
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tafrits (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            item_name TEXT NOT NULL,
            price REAL NOT NULL
        );
        "#
    )
    .execute(&pool)
    .await?;

    // יצירת טבלת עובדים
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS workers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            role TEXT NOT NULL,
            city_id INTEGER NOT NULL,
            FOREIGN KEY (city_id) REFERENCES cities(id)
        );
        "#
    )
    .execute(&pool)
    .await?;

    println!("Tables created successfully!");
    Ok(())
}
