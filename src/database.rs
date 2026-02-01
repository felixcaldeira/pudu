use sqlx::{MySqlPool, mysql::MySqlPoolOptions, migrate::MigrateError};

pub async fn setup(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    
    Ok(pool)
}

pub async fn migrate(pool: &MySqlPool) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}