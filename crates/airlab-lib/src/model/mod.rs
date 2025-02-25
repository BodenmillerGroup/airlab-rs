#![allow(clippy::module_name_repetitions)]
#![allow(clippy::module_inception)]
mod base;
pub mod clone;
pub mod conjugate;
mod error;
pub mod group;
pub mod lot;
pub mod member;
pub mod panel;
pub mod panel_element;
pub mod protein;
pub mod provider;
pub mod species;
mod store;
pub mod tag;
pub mod user;
pub mod validation;
pub mod validation_file;
pub mod view_application;
pub mod view_clone;
pub mod view_conjugate;
pub mod view_group;
pub mod view_lot;
pub mod view_member;
pub mod view_panel;
pub mod view_panel_element;
pub mod view_validation;

pub use self::error::{Error, Result};

use crate::model::store::{new_db_pool, Db};

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;

        Ok(Self { db })
    }

    pub const fn db(&self) -> &Db {
        &self.db
    }
}
