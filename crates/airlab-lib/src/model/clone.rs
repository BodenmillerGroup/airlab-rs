use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
use crate::model::helpers::{
    bool_or, i64_or, opt_bool, opt_i64, opt_string, opt_value, opt_vec_i64, string_or,
};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default)]
pub struct CloneId {
    pub id: i64,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default)]
pub struct Clone {
    pub id: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,
    #[serde(rename = "createdBy")]
    pub created_by: i64,
    #[serde(rename = "proteinId")]
    pub protein_id: i64,
    #[serde(rename = "speciesId")]
    pub species_id: Option<i64>,
    pub name: String,
    pub isotype: Option<String>,
    pub epitope: Option<String>,
    #[serde(rename = "isPhospho")]
    pub is_phospho: bool,
    #[serde(rename = "isPolyclonal")]
    pub is_polyclonal: bool,
    pub reactivity: Option<Vec<i64>>,
    pub application: Option<serde_json::Value>,
    #[serde(rename = "isArchived")]
    pub is_archived: bool,
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct CloneForCreate {
    #[serde(rename = "groupId")]
    pub group_id: i64,
    #[serde(rename = "createdBy")]
    pub created_by: Option<i64>,
    #[serde(rename = "proteinId")]
    pub protein_id: i64,
    #[serde(rename = "speciesId")]
    pub species_id: Option<i64>,
    pub name: String,
    pub isotype: String,
    pub epitope: String,
    #[serde(rename = "isPhospho")]
    pub is_phospho: bool,
    #[serde(rename = "isPolyclonal")]
    pub is_polyclonal: bool,
    #[serde(rename = "isArchived")]
    pub is_archived: Option<bool>,
    pub reactivity: Option<Vec<i64>>,
    pub application: Option<serde_json::Value>,
}
impl From<Value> for CloneForCreate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        CloneForCreate {
            group_id: i64_or(&obj, "groupId", 0),
            created_by: opt_i64(&obj, "createdBy"),
            protein_id: i64_or(&obj, "proteinId", 0),
            species_id: opt_i64(&obj, "speciesId"),

            name: string_or(&obj, "name"),
            isotype: string_or(&obj, "isotype"),
            epitope: string_or(&obj, "epitope"),

            is_phospho: bool_or(&obj, "isPhospho", false),
            is_polyclonal: bool_or(&obj, "isPolyclonal", false),
            is_archived: opt_bool(&obj, "isArchived"),

            reactivity: opt_vec_i64(&obj, "reactivity"),

            application: opt_value(&obj, "application"),
        }
    }
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct CloneForUpdate {
    #[serde(rename = "proteinId")]
    pub protein_id: Option<i64>,
    #[serde(rename = "speciesId")]
    pub species_id: Option<i64>,
    pub name: Option<String>,
    pub isotype: Option<String>,
    pub epitope: Option<String>,
    #[serde(rename = "isPhospho")]
    pub is_phospho: bool,
    #[serde(rename = "isPolyclonal")]
    pub is_polyclonal: bool,
    #[serde(rename = "isArchived")]
    pub is_archived: Option<bool>,
    pub reactivity: Option<Vec<i64>>,
    pub application: Option<serde_json::Value>,
}

impl From<Value> for CloneForUpdate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        CloneForUpdate {
            protein_id: opt_i64(&obj, "proteinId"),
            species_id: opt_i64(&obj, "speciesId"),

            name: opt_string(&obj, "name"),
            isotype: opt_string(&obj, "isotype"),
            epitope: opt_string(&obj, "epitope"),

            is_phospho: bool_or(&obj, "isPhospho", false),
            is_polyclonal: bool_or(&obj, "isPolyclonal", false),
            is_archived: opt_bool(&obj, "isArchived"),

            reactivity: opt_vec_i64(&obj, "reactivity"),

            application: opt_value(&obj, "application"),
        }
    }
}

#[derive(FilterNodes, Deserialize, Default, Debug, Clone)]
pub struct CloneFilter {
    id: Option<OpValsInt64>,
    species_id: Option<OpValsInt64>,
    protein_id: Option<OpValsInt64>,
    group_id: Option<OpValsInt64>,
    name: Option<OpValsString>,
}

pub struct CloneBmc;

impl DbBmc for CloneBmc {
    const TABLE: &'static str = "clone";
}

impl CloneBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, clone_c: CloneForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, clone_c).await
    }
    pub async fn create_full(ctx: &Ctx, mm: &ModelManager, clone_c: Clone) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, clone_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Clone> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<CloneFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Clone>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn ids(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<CloneFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<CloneId>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn count(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<CloneFilter>>,
    ) -> Result<i64> {
        base::count::<Self, _>(ctx, mm, filters).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        clone_u: CloneForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, clone_u).await
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

    #[test]
    fn clone_for_update_reads_species_and_protein_ids() {
        let update: CloneForUpdate = json!({
            "proteinId": 123,
            "speciesId": 456,
            "name": "updated-clone"
        })
        .into();

        assert_eq!(update.protein_id, Some(123));
        assert_eq!(update.species_id, Some(456));
        assert_eq!(update.name.as_deref(), Some("updated-clone"));
    }

    #[tokio::test]
    async fn test_clone_create_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_name = "test_create_ok name";

        let clone_c = CloneForCreate {
            name: fx_name.to_string(),
            group_id: 1000,
            created_by: Some(1303),
            epitope: String::new(),
            is_archived: None,
            is_phospho: false,
            reactivity: None,
            application: None,
            is_polyclonal: false,
            isotype: String::new(),
            protein_id: 1002,
            species_id: Some(1004),
        };
        let id = CloneBmc::create(&ctx, &mm, clone_c).await?;

        let clone = CloneBmc::get(&ctx, &mm, id).await?;
        assert_eq!(clone.name, fx_name);

        CloneBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_clone_get_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = CloneBmc::get(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "clone",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_clone_list_all_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_clone_list_all_ok";
        let seeds = _dev_utils::get_clone_seed(tname);
        _dev_utils::seed_clones(&ctx, &mm, &seeds).await?;

        let clones = CloneBmc::list(&ctx, &mm, None, None).await?;

        let clones: Vec<Clone> = clones
            .into_iter()
            .filter(|t| t.name.starts_with(tname))
            .collect();

        for clone in clones.iter() {
            if false {
                CloneBmc::delete(&ctx, &mm, clone.id).await?;
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_clone_list_by_filter_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_clone_list_by_filter_ok";
        let seeds = _dev_utils::get_clone_seed(tname);
        _dev_utils::seed_clones(&ctx, &mm, &seeds).await?;

        let filters: Vec<CloneFilter> = serde_json::from_value(json!([
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
        let clones = CloneBmc::list(&ctx, &mm, Some(filters), Some(list_options)).await?;

        assert_eq!(clones.len(), 3);
        assert!(clones[0].name.ends_with("03"));
        assert!(clones[1].name.ends_with("02.a"));
        assert!(clones[2].name.ends_with("01.a"));

        if false {
            let clones = CloneBmc::list(
                &ctx,
                &mm,
                Some(serde_json::from_value(json!([{
                    "name": {"$startsWith": "test_list_by_filter_ok"}
                }]))?),
                None,
            )
            .await?;
            assert_eq!(clones.len(), 5);
            for clone in clones.iter() {
                CloneBmc::delete(&ctx, &mm, clone.id).await?;
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_clone_update_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_clone_update_ok";
        let seeds = _dev_utils::get_clone_seed(tname);
        _dev_utils::seed_clones(&ctx, &mm, &seeds).await?;
        let fx_clone = _dev_utils::seed_clones(&ctx, &mm, &seeds).await?.remove(0);

        CloneBmc::update(
            &ctx,
            &mm,
            fx_clone.id,
            CloneForUpdate {
                name: Some(tname.to_string()),
                ..Default::default()
            },
        )
        .await?;

        let clone = CloneBmc::get(&ctx, &mm, fx_clone.id).await?;
        assert_eq!(clone.name, tname);

        Ok(())
    }

    #[tokio::test]
    async fn test_clone_delete_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = CloneBmc::delete(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "clone",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
