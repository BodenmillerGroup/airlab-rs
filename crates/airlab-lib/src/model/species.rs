use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
use crate::model::helpers::{i64_or, opt_string, string_or};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default)]
pub struct Species {
    pub id: i64,
    #[serde(rename = "groupId")]
    pub group_id: i64,

    pub name: String,
    pub acronym: String,
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct SpeciesForCreate {
    pub name: String,
    #[serde(rename = "groupId")]
    pub group_id: i64,
    pub acronym: String,
}
impl From<Value> for SpeciesForCreate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        SpeciesForCreate {
            name: string_or(&obj, "name"),
            group_id: i64_or(&obj, "groupId", 0),
            acronym: string_or(&obj, "acronym"),
        }
    }
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct SpeciesForUpdate {
    pub name: Option<String>,
    pub acronym: String,
}
impl From<Value> for SpeciesForUpdate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        SpeciesForUpdate {
            name: opt_string(&obj, "name"),
            acronym: string_or(&obj, "acronym"),
        }
    }
}

#[derive(FilterNodes, Deserialize, Default, Debug, Clone)]
pub struct SpeciesFilter {
    id: Option<OpValsInt64>,
    name: Option<OpValsString>,
    group_id: Option<OpValsInt64>,
}

pub struct SpeciesBmc;

impl DbBmc for SpeciesBmc {
    const TABLE: &'static str = "species";
}

impl SpeciesBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, species_c: SpeciesForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, species_c).await
    }
    pub async fn create_full(ctx: &Ctx, mm: &ModelManager, species_c: Species) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, species_c).await
    }

    pub async fn count(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<SpeciesFilter>>,
    ) -> Result<i64> {
        base::count::<Self, _>(ctx, mm, filters).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Species> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<SpeciesFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Species>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        species_u: SpeciesForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, species_u).await
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
    async fn test_species_create_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_name = "test_create_ok name";

        let species_c = SpeciesForCreate {
            name: fx_name.to_string(),
            group_id: 1,
            acronym: "bbb".to_string(),
        };
        let id = SpeciesBmc::create(&ctx, &mm, species_c).await?;

        let species = SpeciesBmc::get(&ctx, &mm, id).await?;
        assert_eq!(species.name, fx_name);

        SpeciesBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_species_get_err_not_found() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = SpeciesBmc::get(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "species",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_species_list_all_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_species_list_all_ok";
        let seeds = _dev_utils::get_species_seed(tname);
        _dev_utils::seed_species(&ctx, &mm, &seeds).await?;

        let species = SpeciesBmc::list(&ctx, &mm, None, None).await?;

        let species: Vec<Species> = species
            .into_iter()
            .filter(|t| t.name.starts_with(tname))
            .collect();
        assert_eq!(species.len(), 4, "number of seeded species.");

        if false {
            for species in species.iter() {
                SpeciesBmc::delete(&ctx, &mm, species.id).await?;
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_species_list_by_filter_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_species_list_by_filter_ok";
        let seeds = _dev_utils::get_species_seed(tname);
        _dev_utils::seed_species(&ctx, &mm, &seeds).await?;

        let filters: Vec<SpeciesFilter> = serde_json::from_value(json!([
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
        let species = SpeciesBmc::list(&ctx, &mm, Some(filters), Some(list_options)).await?;

        assert_eq!(species.len(), 3);
        assert!(species[0].name.ends_with("03"));
        assert!(species[1].name.ends_with("02.a"));
        assert!(species[2].name.ends_with("01.a"));

        if false {
            let species = SpeciesBmc::list(
                &ctx,
                &mm,
                Some(serde_json::from_value(json!([{
                    "name": {"$startsWith": "test_list_by_filter_ok"}
                }]))?),
                None,
            )
            .await?;
            assert_eq!(species.len(), 5);
            for species in species.iter() {
                SpeciesBmc::delete(&ctx, &mm, species.id).await?;
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_species_update_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_species_update_ok";
        let seeds = _dev_utils::get_species_seed(tname);
        let fx_species = _dev_utils::seed_species(&ctx, &mm, &seeds).await?.remove(0);

        SpeciesBmc::update(
            &ctx,
            &mm,
            fx_species.id,
            SpeciesForUpdate {
                name: Some(tname.to_string()),
                ..Default::default()
            },
        )
        .await?;

        let species = SpeciesBmc::get(&ctx, &mm, fx_species.id).await?;
        assert_eq!(species.name, tname);

        Ok(())
    }

    #[tokio::test]
    async fn test_species_delete_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = SpeciesBmc::delete(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "species",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
