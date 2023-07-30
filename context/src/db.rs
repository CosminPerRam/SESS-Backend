use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::runtime::Runtime;

async fn make_db() -> DatabaseConnection {
    let mut opt = ConnectOptions::new("postgres://root:root@127.0.0.1:5432/sess".to_string());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("sess".to_string()); // Setting default PostgreSQL schema

    let db = Database::connect(opt).await.unwrap();

    return db;
}

pub fn make_db_connection() -> DatabaseConnection {
    let rt = Runtime::new().unwrap();

    rt.block_on(make_db())
}
