use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
use crate::model::helpers::{i64_or, opt_i64, opt_string, string_or};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default)]
pub struct Protein {
    pub id: i64,
    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "createdBy")]
    pub created_by: i64,
    pub name: String,
    pub description: Option<String>,
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>, //String
}

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct ProteinForCreate {
    pub name: String,
    pub description: Option<String>,
    pub group_id: i64,
    pub created_by: i64,
}

impl From<Value> for ProteinForCreate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        ProteinForCreate {
            name: string_or(&obj, "name"),
            description: opt_string(&obj, "description"),
            group_id: opt_i64(&obj, "groupId").unwrap_or(i64_or(&obj, "group_id", 0)),
            created_by: opt_i64(&obj, "createdBy").unwrap_or(i64_or(&obj, "created_by", 0)),
        }
    }
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct ProteinForUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
}
impl From<Value> for ProteinForUpdate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        ProteinForUpdate {
            name: opt_string(&obj, "name"),
            description: opt_string(&obj, "description"),
        }
    }
}

#[derive(FilterNodes, Deserialize, Default, Debug, Clone)]
pub struct ProteinFilter {
    id: Option<OpValsInt64>,
    group_id: Option<OpValsInt64>,
    name: Option<OpValsString>,
    description: Option<OpValsString>,
}

pub struct ProteinBmc;

impl DbBmc for ProteinBmc {
    const TABLE: &'static str = "protein";
}

impl ProteinBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, protein_c: ProteinForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, protein_c).await
    }
    pub async fn create_full(ctx: &Ctx, mm: &ModelManager, protein_c: Protein) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, protein_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Protein> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<ProteinFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Protein>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }
    pub async fn count(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<ProteinFilter>>,
    ) -> Result<i64> {
        base::count::<Self, _>(ctx, mm, filters).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        protein_u: ProteinForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, protein_u).await
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
    async fn test_protein_create_ok() -> TestResult {
        let ctx = Ctx::root_ctx();
        let mm = _dev_utils::init_test().await;
        let fx_name = "test_create_ok name";

        let protein_c = ProteinForCreate {
            name: fx_name.to_string(),
            description: Some(fx_name.to_string()),
            group_id: 1,
            created_by: 261,
        };
        let id = ProteinBmc::create(&ctx, &mm, protein_c).await?;

        let protein = ProteinBmc::get(&ctx, &mm, id).await?;
        assert_eq!(protein.name, fx_name);

        ProteinBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_protein_get_err_not_found() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = ProteinBmc::get(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "protein",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_protein_list_all_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_protein_list_all_ok";
        let seeds = _dev_utils::get_protein_seed(tname);
        _dev_utils::seed_proteins(&ctx, &mm, &seeds).await?;

        let proteins = ProteinBmc::list(&ctx, &mm, None, None).await?;

        let proteins: Vec<Protein> = proteins
            .into_iter()
            .filter(|t| t.name.starts_with(tname))
            .collect();
        assert_eq!(proteins.len(), 4, "number of seeded proteins.");

        if false {
            for protein in proteins.iter() {
                ProteinBmc::delete(&ctx, &mm, protein.id).await?;
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_protein_list_by_filter_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_protein_list_by_filter_ok";
        let seeds = _dev_utils::get_protein_seed(tname);
        _dev_utils::seed_proteins(&ctx, &mm, &seeds).await?;

        let filters: Vec<ProteinFilter> = serde_json::from_value(json!([
            {
                "name": {
                    "$endsWith": ".a",
                    "$containsAny": ["01", "02"]
                }
            },
            {
                "name": {"$contains": "03"}
            }
        ]))?;
        let list_options = serde_json::from_value(json!({
            "order_bys": "!id"
        }))?;
        let proteins = ProteinBmc::list(&ctx, &mm, Some(filters), Some(list_options)).await?;

        assert_eq!(proteins.len(), 3);
        assert!(proteins[0].name.ends_with("03"));
        assert!(proteins[1].name.ends_with("02.a"));
        assert!(proteins[2].name.ends_with("01.a"));

        if false {
            let proteins = ProteinBmc::list(
                &ctx,
                &mm,
                Some(serde_json::from_value(json!([{
                    "name": {"$startsWith": "test_list_by_filter_ok"}
                }]))?),
                None,
            )
            .await?;
            assert_eq!(proteins.len(), 5);
            for protein in proteins.iter() {
                ProteinBmc::delete(&ctx, &mm, protein.id).await?;
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_protein_update_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_protein_list_by_filter_ok";
        let seeds = _dev_utils::get_protein_seed(tname);
        let fx_protein = _dev_utils::seed_proteins(&ctx, &mm, &seeds)
            .await?
            .remove(0);

        ProteinBmc::update(
            &ctx,
            &mm,
            fx_protein.id,
            ProteinForUpdate {
                name: Some(tname.to_string()),
                ..Default::default()
            },
        )
        .await?;

        let protein = ProteinBmc::get(&ctx, &mm, fx_protein.id).await?;
        assert_eq!(protein.name, tname);

        Ok(())
    }

    #[tokio::test]
    async fn test_protein_delete_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = ProteinBmc::delete(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "protein",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
