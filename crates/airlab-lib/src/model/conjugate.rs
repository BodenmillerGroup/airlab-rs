use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
use crate::model::helpers::{i64_or, opt_bool, opt_datetime, opt_f64, opt_i64, opt_string};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default)]
pub struct Conjugate {
    pub id: i64,
    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "createdBy")]
    pub created_by: i64,
    #[serde(rename = "labeledBy")]
    pub labeled_by: Option<i64>,
    #[serde(rename = "finishedBy")]
    pub finished_by: Option<i64>,
    #[serde(rename = "lotId")]
    pub lot_id: i64,
    #[serde(rename = "tagId")]
    pub tag_id: i64,
    #[serde(rename = "storageId")]
    pub storage_id: Option<i64>,
    pub status: i64,
    #[serde(rename = "tubeNumber")]
    pub tube_number: i64,
    pub concentration: Option<f64>,
    pub description: Option<String>,
    #[serde(rename = "finishedAt")]
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "isArchived")]
    pub is_archived: Option<bool>,
    #[serde(rename = "customId")]
    pub custom_id: Option<String>,
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct ConjugateForCreate {
    #[serde(rename = "groupId")]
    pub group_id: i64,
    #[serde(rename = "createdBy")]
    pub created_by: Option<i64>,
    #[serde(rename = "labeledBy")]
    pub labeled_by: Option<i64>,
    #[serde(rename = "finishedBy")]
    pub finished_by: Option<i64>,
    #[serde(rename = "lotId")]
    pub lot_id: i64,
    #[serde(rename = "tagId")]
    pub tag_id: i64,
    #[serde(rename = "storageId")]
    pub storage_id: Option<i64>,
    pub status: Option<i64>,
    pub concentration: Option<f64>,
    pub description: Option<String>,
    #[serde(rename = "finishedAt")]
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "isArchived")]
    pub is_archived: Option<bool>,
    #[serde(rename = "customId")]
    pub custom_id: Option<String>,
}

impl From<Value> for ConjugateForCreate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        ConjugateForCreate {
            group_id: i64_or(&obj, "groupId", 0),
            created_by: opt_i64(&obj, "createdBy"),
            labeled_by: opt_i64(&obj, "labeledBy"),
            finished_by: opt_i64(&obj, "finishedBy"),
            lot_id: i64_or(&obj, "lotId", 0),
            tag_id: i64_or(&obj, "tagId", 0),
            storage_id: opt_i64(&obj, "storageId").or_else(|| opt_i64(&obj, "storage_id")),
            status: opt_i64(&obj, "status"),
            concentration: opt_f64(&obj, "concentration"),
            description: opt_string(&obj, "description"),
            finished_at: opt_datetime(&obj, "finishedAt"),
            is_archived: opt_bool(&obj, "isArchived"),
            custom_id: opt_string(&obj, "customId"),
        }
    }
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct ConjugateForUpdate {
    #[serde(rename = "labeledBy")]
    pub labeled_by: Option<i64>,
    #[serde(rename = "finishedBy")]
    pub finished_by: Option<i64>,
    #[serde(rename = "lotId")]
    pub lot_id: Option<i64>,
    #[serde(rename = "tagId")]
    pub tag_id: Option<i64>,
    #[serde(rename = "storageId")]
    pub storage_id: Option<i64>,
    pub status: Option<i64>,
    #[serde(rename = "tubeNumber")]
    pub tube_number: Option<i64>,
    pub concentration: Option<f64>,
    pub description: Option<String>,
    #[serde(rename = "finishedAt")]
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "isArchived")]
    pub is_archived: Option<bool>,
    #[serde(rename = "customId")]
    pub custom_id: Option<String>,
}
impl From<Value> for ConjugateForUpdate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        ConjugateForUpdate {
            labeled_by: opt_i64(&obj, "labeledBy"),
            finished_by: opt_i64(&obj, "finishedBy"),
            lot_id: opt_i64(&obj, "lotId"),
            tag_id: opt_i64(&obj, "tagId"),
            storage_id: opt_i64(&obj, "storageId").or_else(|| opt_i64(&obj, "storage_id")),
            status: opt_i64(&obj, "status"),
            tube_number: opt_i64(&obj, "tubeNumber"),
            concentration: opt_f64(&obj, "concentration"),
            description: opt_string(&obj, "description"),
            finished_at: opt_datetime(&obj, "finishedAt"),
            is_archived: opt_bool(&obj, "isArchived"),
            custom_id: opt_string(&obj, "customId"),
        }
    }
}

#[derive(FilterNodes, Deserialize, Default, Debug, Clone)]
pub struct ConjugateFilter {
    id: Option<OpValsInt64>,
    lot_id: Option<OpValsInt64>,
    group_id: Option<OpValsInt64>,
    storage_id: Option<OpValsInt64>,

    description: Option<OpValsString>,
}

pub struct ConjugateBmc;

impl DbBmc for ConjugateBmc {
    const TABLE: &'static str = "conjugate";
}

impl ConjugateBmc {
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        mut conjugate_c: ConjugateForCreate,
    ) -> Result<i64> {
        conjugate_c.status = Some(0);
        base::create::<Self, _>(ctx, mm, conjugate_c).await
    }
    pub async fn create_full(ctx: &Ctx, mm: &ModelManager, conjugate_c: Conjugate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, conjugate_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Conjugate> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<ConjugateFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Conjugate>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn count(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<ConjugateFilter>>,
    ) -> Result<i64> {
        base::count::<Self, _>(ctx, mm, filters).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        conjugate_u: ConjugateForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, conjugate_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    use crate::model::Error;
    use serde_json::json;

    type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    #[tokio::test]
    async fn test_conjugate_create_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_description = "test_create_ok description";

        let conjugate_c = ConjugateForCreate {
            description: Some(fx_description.to_string()),
            group_id: 1,
            created_by: Some(261),
            labeled_by: None,
            finished_by: None,
            lot_id: 5495,
            status: Some(1),
            tag_id: 211,
            storage_id: None,
            concentration: None,
            finished_at: None,
            is_archived: None,
            custom_id: None,
        };
        let id = ConjugateBmc::create(&ctx, &mm, conjugate_c).await?;

        let conjugate = ConjugateBmc::get(&ctx, &mm, id).await?;
        assert_eq!(conjugate.description.unwrap_or("".into()), fx_description);
        assert_eq!(conjugate.status, 0);

        ConjugateBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_conjugate_get_err_not_found() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = ConjugateBmc::get(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "conjugate",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_conjugate_list_all_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_conjugate_list_all_ok";
        let seeds = _dev_utils::get_conjugate_seed(tname);
        _dev_utils::seed_conjugates(&ctx, &mm, &seeds).await?;

        let conjugates = ConjugateBmc::list(&ctx, &mm, None, None).await?;

        let conjugates: Vec<Conjugate> = conjugates
            .into_iter()
            .filter(|t| {
                t.description
                    .as_deref()
                    .is_some_and(|desc| desc.starts_with(tname))
            })
            .collect();
        assert_eq!(conjugates.len(), 4, "number of seeded conjugates.");

        Ok(())
    }

    #[tokio::test]
    async fn test_conjugate_list_by_filter_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_conjugate_list_by_filter_ok";
        let seeds = _dev_utils::get_conjugate_seed(tname);
        _dev_utils::seed_conjugates(&ctx, &mm, &seeds).await?;

        let filters: Vec<ConjugateFilter> = serde_json::from_value(json!([
            {
                "description": {
                    "$endsWith": ".a",
                    "$containsAny": ["01", "02"]
                }
            },
            {
                "description": {"$contains": "03"}
            }
        ]))?;
        let list_options = serde_json::from_value(json!({
            "order_bys": "!id"
        }))?;
        let conjugates = ConjugateBmc::list(&ctx, &mm, Some(filters), Some(list_options)).await?;

        assert_eq!(conjugates.len(), 3);
        assert!(
            conjugates[0]
                .description
                .as_deref()
                .is_some_and(|desc| desc.ends_with("03"))
        );
        assert!(
            conjugates[1]
                .description
                .as_deref()
                .is_some_and(|desc| desc.ends_with("02.a"))
        );
        assert!(
            conjugates[2]
                .description
                .as_deref()
                .is_some_and(|desc| desc.ends_with("01.a"))
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_conjugate_update_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_conjugate_update_ok";
        let seeds = _dev_utils::get_conjugate_seed(tname);
        let fx_conjugate = _dev_utils::seed_conjugates(&ctx, &mm, &seeds)
            .await?
            .remove(0);

        ConjugateBmc::update(
            &ctx,
            &mm,
            fx_conjugate.id,
            ConjugateForUpdate {
                description: Some(tname.to_string()),
                ..Default::default()
            },
        )
        .await?;

        let conjugate = ConjugateBmc::get(&ctx, &mm, fx_conjugate.id).await?;
        assert_eq!(conjugate.description, Some(tname.into()));

        Ok(())
    }

    #[tokio::test]
    async fn test_conjugate_delete_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = ConjugateBmc::delete(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "conjugate",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
