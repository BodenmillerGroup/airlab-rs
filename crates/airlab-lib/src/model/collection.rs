use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::helpers::{i64_or, opt_i64, opt_string, string_or};
use crate::model::{Error, ModelManager, Result};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize)]
pub struct Collection {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub created_by: i64,
}

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct CollectionForCreate {
    pub name: String,
    pub description: Option<String>,
    pub created_by: i64,
}

impl From<Value> for CollectionForCreate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        CollectionForCreate {
            name: string_or(&obj, "name"),
            description: opt_string(&obj, "description"),
            created_by: opt_i64(&obj, "createdBy").unwrap_or(i64_or(&obj, "created_by", 0)),
        }
    }
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct CollectionForUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl From<Value> for CollectionForUpdate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        CollectionForUpdate {
            name: opt_string(&obj, "name"),
            description: opt_string(&obj, "description"),
        }
    }
}

#[derive(FilterNodes, Deserialize, Default, Debug, Clone)]
pub struct CollectionFilter {
    id: Option<OpValsInt64>,
    name: Option<OpValsString>,
    description: Option<OpValsString>,
    created_by: Option<OpValsInt64>,
}

pub struct CollectionBmc;

impl DbBmc for CollectionBmc {
    const TABLE: &'static str = "collection";

    fn has_timestamps() -> bool {
        false
    }
}

impl CollectionBmc {
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        collection_c: CollectionForCreate,
    ) -> Result<i64> {
        let _ = ctx;
        let id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO collection (name, description, created_at, created_by)
            VALUES ($1, $2, NOW(), $3)
            RETURNING id
            "#,
        )
        .bind(collection_c.name)
        .bind(collection_c.description)
        .bind(collection_c.created_by)
        .fetch_one(mm.db())
        .await?;

        Ok(id)
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Collection> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<CollectionFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Collection>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn count(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<CollectionFilter>>,
    ) -> Result<i64> {
        base::count::<Self, _>(ctx, mm, filters).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        collection_u: CollectionForUpdate,
    ) -> Result<()> {
        let _ = ctx;
        let count = sqlx::query(
            r#"
            UPDATE collection
            SET
                name = COALESCE($1, name),
                description = COALESCE($2, description)
            WHERE id = $3
            "#,
        )
        .bind(collection_u.name)
        .bind(collection_u.description)
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
    async fn test_collection_create_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        let id = CollectionBmc::create(
            &ctx,
            &mm,
            CollectionForCreate {
                name: "collection-create".into(),
                description: Some("created in test".into()),
                created_by: 1,
            },
        )
        .await?;

        let collection = CollectionBmc::get(&ctx, &mm, id).await?;
        assert_eq!(collection.name, "collection-create");
        assert_eq!(collection.description.as_deref(), Some("created in test"));

        Ok(())
    }

    #[tokio::test]
    async fn test_collection_list_by_filter_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let prefix = "test_collection_list_by_filter_ok";

        for suffix in ["01.a", "02.a", "03"] {
            CollectionBmc::create(
                &ctx,
                &mm,
                CollectionForCreate {
                    name: format!("{prefix}-{suffix}"),
                    description: None,
                    created_by: 1,
                },
            )
            .await?;
        }

        let filters: Vec<CollectionFilter> = serde_json::from_value(json!([
            {
                "name": {
                    "$startsWith": prefix,
                    "$containsAny": ["01", "02"]
                }
            }
        ]))?;

        let collections = CollectionBmc::list(&ctx, &mm, Some(filters), None).await?;

        assert_eq!(collections.len(), 2);
        assert!(collections.iter().all(|item| item.name.starts_with(prefix)));

        Ok(())
    }

    #[tokio::test]
    async fn test_collection_update_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let id = CollectionBmc::create(
            &ctx,
            &mm,
            CollectionForCreate {
                name: "collection-update-before".into(),
                description: Some("before".into()),
                created_by: 1,
            },
        )
        .await?;

        CollectionBmc::update(
            &ctx,
            &mm,
            id,
            CollectionForUpdate {
                name: Some("collection-update-after".into()),
                description: Some("after".into()),
            },
        )
        .await?;

        let collection = CollectionBmc::get(&ctx, &mm, id).await?;
        assert_eq!(collection.name, "collection-update-after");
        assert_eq!(collection.description.as_deref(), Some("after"));

        Ok(())
    }

    #[tokio::test]
    async fn test_collection_delete_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        let result = CollectionBmc::delete(&ctx, &mm, 999_999).await;

        assert!(matches!(
            result,
            Err(Error::EntityNotFound {
                entity: "collection",
                id: 999_999
            })
        ));

        Ok(())
    }
}
