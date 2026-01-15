use axum::Router;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::net::SocketAddr;
mod routers;
mod controllers;
mod models;

#[tokio::main]
async fn main() {
    // א. התחברות לבסיס הנתונים
    // א. התחברות לבסיס הנתונים
    let db_url = "sqlite:resturant.db"; // תוקן השם כדי שיתאים לקובץ הקיים
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("נכשל בהתחברות לבסיס הנתונים");

    // ב. בניית הראוטר הראשי וחיבור הראוטרים מהקבצים האחרים
    let app = Router::new()
        // nest לוקח את כל הראוטרים הקטנים ומאחד אותם תחת כתובת גג
        .nest("/cities", routers::citiesRouter::cities_router())
        .nest("/workers", routers::workerRouter::worker_router())
        .nest("/menu", routers::tafritRouter::tafrit_router())
        // הזרקת ה-pool לכל הראוטרים
        .with_state(pool);

    // ג. הגדרת הכתובת והרצת השרת
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
