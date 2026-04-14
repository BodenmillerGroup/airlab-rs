use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::helpers::{bool_or, i64_or, opt_bool, opt_i64, opt_string};
use crate::model::{Error, ModelManager, Result};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsBool, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize)]
pub struct Storage {
    pub id: i64,
    pub name: String,
    #[field(name = "type")]
    pub r#type: String,
    pub location: String,
    pub temperature_c: i64,
    pub active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct StorageForCreate {
    pub name: String,
    #[field(name = "type")]
    pub r#type: String,
    pub location: String,
    pub temperature_c: i64,
    pub active: bool,
}

impl From<Value> for StorageForCreate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        StorageForCreate {
            name: opt_string(&obj, "name").unwrap_or_default(),
            r#type: opt_string(&obj, "type").unwrap_or_default(),
            location: opt_string(&obj, "location").unwrap_or_default(),
            temperature_c: opt_i64(&obj, "temperature_c")
                .or_else(|| opt_i64(&obj, "temperatureC"))
                .unwrap_or(i64_or(&obj, "temperature_c", 0)),
            active: opt_bool(&obj, "active").unwrap_or(bool_or(&obj, "active", true)),
        }
    }
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct StorageForUpdate {
    pub name: Option<String>,
    #[field(name = "type")]
    pub r#type: Option<String>,
    pub location: Option<String>,
    pub temperature_c: Option<i64>,
    pub active: Option<bool>,
}

impl From<Value> for StorageForUpdate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        StorageForUpdate {
            name: opt_string(&obj, "name"),
            r#type: opt_string(&obj, "type"),
            location: opt_string(&obj, "location"),
            temperature_c: opt_i64(&obj, "temperature_c").or_else(|| opt_i64(&obj, "temperatureC")),
            active: opt_bool(&obj, "active"),
        }
    }
}

#[derive(FilterNodes, Deserialize, Default, Debug, Clone)]
pub struct StorageFilter {
    id: Option<OpValsInt64>,
    name: Option<OpValsString>,
    r#type: Option<OpValsString>,
    location: Option<OpValsString>,
    temperature_c: Option<OpValsInt64>,
    active: Option<OpValsBool>,
}

pub struct StorageBmc;

impl DbBmc for StorageBmc {
    const TABLE: &'static str = "storage";

    fn has_timestamps() -> bool {
        false
    }
}

impl StorageBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, storage_c: StorageForCreate) -> Result<i64> {
        let _ = ctx;
        let id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO storage (name, "type", location, temperature_c, active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
            RETURNING id
            "#,
        )
        .bind(storage_c.name)
        .bind(storage_c.r#type)
        .bind(storage_c.location)
        .bind(storage_c.temperature_c)
        .bind(storage_c.active)
        .fetch_one(mm.db())
        .await?;

        Ok(id)
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Storage> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<StorageFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Storage>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn count(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<StorageFilter>>,
    ) -> Result<i64> {
        base::count::<Self, _>(ctx, mm, filters).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        storage_u: StorageForUpdate,
    ) -> Result<()> {
        let _ = ctx;
        let count = sqlx::query(
            r#"
            UPDATE storage
            SET
                name = COALESCE($1, name),
                "type" = COALESCE($2, "type"),
                location = COALESCE($3, location),
                temperature_c = COALESCE($4, temperature_c),
                active = COALESCE($5, active),
                updated_at = NOW()
            WHERE id = $6
            "#,
        )
        .bind(storage_u.name)
        .bind(storage_u.r#type)
        .bind(storage_u.location)
        .bind(storage_u.temperature_c)
        .bind(storage_u.active)
        .bind(id)
        .execute(mm.db())
        .await?
        .rows_affected();

        if count == 0 {
            Err(Error::EntityNotFound {
                entity: Self::TABLE,
                id,
            })
        } else {
            Ok(())
        }
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    use serde_json::json;

    type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    #[tokio::test]
    async fn test_storage_create_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        let id = StorageBmc::create(
            &ctx,
            &mm,
            StorageForCreate {
                name: "storage-create".into(),
                r#type: "freezer".into(),
                location: "Room A".into(),
                temperature_c: -20,
                active: true,
            },
        )
        .await?;

        let storage = StorageBmc::get(&ctx, &mm, id).await?;
        assert_eq!(storage.name, "storage-create");
        assert_eq!(storage.r#type, "freezer");
        assert_eq!(storage.temperature_c, -20);

        Ok(())
    }

    #[tokio::test]
    async fn test_storage_list_by_filter_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let prefix = "test_storage_list_by_filter_ok";

        for (suffix, active) in [("01", true), ("02", true), ("03", false)] {
            StorageBmc::create(
                &ctx,
                &mm,
                StorageForCreate {
                    name: format!("{prefix}-{suffix}"),
                    r#type: "freezer".into(),
                    location: "Room A".into(),
                    temperature_c: -20,
                    active,
                },
            )
            .await?;
        }

        let filters: Vec<StorageFilter> = serde_json::from_value(json!([
            {
                "name": { "$startsWith": prefix },
                "active": { "$eq": true }
            }
        ]))?;

        let storages = StorageBmc::list(&ctx, &mm, Some(filters), None).await?;

        assert_eq!(storages.len(), 2);
        assert!(storages.iter().all(|item| item.active));

        Ok(())
    }

    #[tokio::test]
    async fn test_storage_update_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let id = StorageBmc::create(
            &ctx,
            &mm,
            StorageForCreate {
                name: "storage-update-before".into(),
                r#type: "fridge".into(),
                location: "Room A".into(),
                temperature_c: 4,
                active: true,
            },
        )
        .await?;

        StorageBmc::update(
            &ctx,
            &mm,
            id,
            StorageForUpdate {
                name: Some("storage-update-after".into()),
                r#type: Some("freezer".into()),
                location: Some("Room B".into()),
                temperature_c: Some(-80),
                active: Some(false),
            },
        )
        .await?;

        let storage = StorageBmc::get(&ctx, &mm, id).await?;
        assert_eq!(storage.name, "storage-update-after");
        assert_eq!(storage.r#type, "freezer");
        assert_eq!(storage.location, "Room B");
        assert_eq!(storage.temperature_c, -80);
        assert!(!storage.active);

        Ok(())
    }

    #[tokio::test]
    async fn test_storage_delete_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        let result = StorageBmc::delete(&ctx, &mm, 999_999).await;

        assert!(matches!(
            result,
            Err(Error::EntityNotFound {
                entity: "storage",
                id: 999_999
            })
        ));

        Ok(())
    }
}
