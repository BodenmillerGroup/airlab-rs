#![allow(clippy::module_name_repetitions)]
use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::user::{User, UserBmc};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tracing::info;

type Db = Pool<Postgres>;

// NOTE: Hardcode to prevent deployed system db update.
const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:welcome@localhost/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@localhost/app_db";

const SQL_RECREATE_DB_FILE_NAME: &str = "00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev_initial";

const DEMO1_PWD: &str = "welcome1";
const DEMO2_PWD: &str = "welcome2";

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
    info!("DEV init_dev_db()");

    let current_dir = std::env::current_dir().unwrap();
    let v: Vec<_> = current_dir.components().collect();
    let path_comp = v.get(v.len().wrapping_sub(3));
    let base_dir = if Some(true) == path_comp.map(|c| c.as_os_str() == "crates") {
        v[..v.len() - 3].iter().collect::<PathBuf>()
    } else {
        current_dir.clone()
    };
    let sql_dir = base_dir.join(SQL_DIR);

    {
        let sql_recreate_db_file = sql_dir.join(SQL_RECREATE_DB_FILE_NAME);
        let root_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
        pexec(&root_db, &sql_recreate_db_file).await?;
    }

    let mut paths: Vec<PathBuf> = fs::read_dir(sql_dir)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();
    paths.sort();

    let app_db = new_db_pool(PG_DEV_APP_URL).await?;

    for path in paths {
        let path_str = path.to_string_lossy();

        if path_str.ends_with(".sql") && !path_str.ends_with(SQL_RECREATE_DB_FILE_NAME) {
            pexec(&app_db, &path).await?;
        }
    }

    let mm = ModelManager::new().await?;
    let ctx = Ctx::root_ctx();

    let demo1_user: User = UserBmc::first_by_username(&ctx, &mm, "demo1@uzh.ch")
        .await?
        .unwrap();
    UserBmc::update_pwd(&ctx, &mm, demo1_user.id, DEMO1_PWD).await?;
    info!("DEV init_dev_db - set demo1 pwd");
    let demo2_user: User = UserBmc::first_by_username(&ctx, &mm, "lars.malmstroem@uzh.ch")
        .await?
        .unwrap();
    UserBmc::update_pwd(&ctx, &mm, demo2_user.id, DEMO2_PWD).await?;
    info!("DEV init_dev_db - set demo2 pwd");
    info!("{:?}", demo2_user);

    Ok(())
}

async fn pexec(db: &Db, file: &Path) -> Result<(), sqlx::Error> {
    info!("DEV pexec: {file:?}");

    let content = fs::read_to_string(file)?;

    let sqls: Vec<&str> = content.split(';').collect();

    for sql in sqls {
        sqlx::query(sql).execute(db).await?;
    }

    Ok(())
}

async fn new_db_pool(db_con_url: &str) -> Result<Db, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_millis(500))
        .connect(db_con_url)
        .await
}
