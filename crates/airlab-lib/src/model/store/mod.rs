pub mod dbx;

use crate::core_config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub(crate) type Db = Pool<Postgres>;

pub(crate) async fn new_db_pool() -> std::result::Result<Db, String> {
    let config = core_config().map_err(|err| err.to_string())?;
    new_db_pool_from_url(&config.DB_URL).await
}

pub(crate) async fn new_db_pool_from_url(db_con_url: &str) -> std::result::Result<Db, String> {
    let max_connections = 1;

    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(db_con_url)
        .await
        .map_err(|ex| ex.to_string())
}
