#![allow(clippy::module_name_repetitions)]
#![allow(clippy::module_inception)]
pub mod base;
pub mod clone;
pub mod collection;
pub mod conjugate;
mod error;
pub mod group;
pub mod helpers;
pub mod lot;
pub mod member;
pub mod panel;
pub mod panel_element;
pub mod protein;
pub mod provider;
pub mod species;
pub mod storage;
mod store;
pub mod tag;
pub mod user;
pub mod validation;
pub mod validation_file;

use sqlx::{Pool, Postgres};

pub use self::error::{Error, Result};

pub use crate::model::store::dbx::Dbx;
use crate::model::store::{new_db_pool, new_db_pool_from_url};

#[derive(Clone)]
pub struct ModelManager {
    dbx: Dbx,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        let db_pool = new_db_pool()
            .await
            .map_err(|ex| Error::CantCreateModelManagerProvider(ex.to_string()))?;
        Self::from_pool(db_pool)
    }

    pub async fn from_db_url(db_url: &str) -> Result<Self> {
        let db_pool = new_db_pool_from_url(db_url)
            .await
            .map_err(|ex| Error::CantCreateModelManagerProvider(ex.to_string()))?;
        Self::from_pool(db_pool)
    }

    fn from_pool(db_pool: Pool<Postgres>) -> Result<Self> {
        let dbx = Dbx::new(db_pool, true)?;
        Ok(Self { dbx })
    }

    pub const fn db(&self) -> &Pool<Postgres> {
        self.dbx.db()
    }
    pub const fn dbx(&self) -> &Dbx {
        &self.dbx
    }
}
