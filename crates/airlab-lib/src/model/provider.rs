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
pub struct Provider {
    pub id: i64,
    #[serde(rename = "groupId")]
    pub group_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct ProviderForCreate {
    pub name: String,
    #[serde(rename = "groupId")]
    pub group_id: i64,
    pub description: Option<String>,
    pub url: Option<String>,
}

impl From<Value> for ProviderForCreate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        ProviderForCreate {
            name: string_or(&obj, "name"),
            group_id: i64_or(&obj, "groupId", 0),
            description: opt_string(&obj, "description"),
            url: opt_string(&obj, "url"),
        }
    }
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct ProviderForUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
}

impl From<Value> for ProviderForUpdate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        ProviderForUpdate {
            name: opt_string(&obj, "name"),
            description: opt_string(&obj, "description"),
            url: opt_string(&obj, "url"),
        }
    }
}

#[derive(FilterNodes, Deserialize, Default, Debug, Clone)]
pub struct ProviderFilter {
    id: Option<OpValsInt64>,
    group_id: Option<OpValsInt64>,
    name: Option<OpValsString>,
}

pub struct ProviderBmc;

impl DbBmc for ProviderBmc {
    const TABLE: &'static str = "provider";
}

impl ProviderBmc {
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        provider_c: ProviderForCreate,
    ) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, provider_c).await
    }
    pub async fn create_full(ctx: &Ctx, mm: &ModelManager, provider_c: Provider) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, provider_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Provider> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<ProviderFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Provider>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn count(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<ProviderFilter>>,
    ) -> Result<i64> {
        base::count::<Self, _>(ctx, mm, filters).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        provider_u: ProviderForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, provider_u).await
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
    async fn test_provider_create_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_name = "test_create_ok name";

        let provider_c = ProviderForCreate {
            name: fx_name.to_string(),
            description: None,
            url: None,
            group_id: 1,
        };
        let id = ProviderBmc::create(&ctx, &mm, provider_c).await?;

        let provider = ProviderBmc::get(&ctx, &mm, id).await?;
        assert_eq!(provider.name, fx_name);

        ProviderBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_provider_get_err_not_found() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = ProviderBmc::get(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "provider",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_provider_list_all_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_provider_list_all_ok";
        let seeds = _dev_utils::get_provider_seed(tname);
        _dev_utils::seed_providers(&ctx, &mm, &seeds).await?;

        let providers = ProviderBmc::list(&ctx, &mm, None, None).await?;

        let providers: Vec<Provider> = providers
            .into_iter()
            .filter(|t| t.name.starts_with(tname))
            .collect();
        assert_eq!(providers.len(), 4, "number of seeded providers.");

        if false {
            for provider in providers.iter() {
                ProviderBmc::delete(&ctx, &mm, provider.id).await?;
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_provider_list_by_filter_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_provider_list_by_filter_ok";
        let seeds = _dev_utils::get_provider_seed(tname);
        _dev_utils::seed_providers(&ctx, &mm, &seeds).await?;

        let filters: Vec<ProviderFilter> = serde_json::from_value(json!([
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
        let providers = ProviderBmc::list(&ctx, &mm, Some(filters), Some(list_options)).await?;

        assert_eq!(providers.len(), 3);
        assert!(providers[0].name.ends_with("03"));
        assert!(providers[1].name.ends_with("02.a"));
        assert!(providers[2].name.ends_with("01.a"));

        if false {
            let providers = ProviderBmc::list(
                &ctx,
                &mm,
                Some(serde_json::from_value(json!([{
                    "name": {"$startsWith": "test_list_by_filter_ok"}
                }]))?),
                None,
            )
            .await?;
            assert_eq!(providers.len(), 5);
            for provider in providers.iter() {
                ProviderBmc::delete(&ctx, &mm, provider.id).await?;
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_provider_update_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        let tname = "test_provider_update_ok";
        let seeds = _dev_utils::get_provider_seed(tname);
        let fx_provider = _dev_utils::seed_providers(&ctx, &mm, &seeds)
            .await?
            .remove(0);

        ProviderBmc::update(
            &ctx,
            &mm,
            fx_provider.id,
            ProviderForUpdate {
                name: Some(tname.to_string()),
                ..Default::default()
            },
        )
        .await?;

        let provider = ProviderBmc::get(&ctx, &mm, fx_provider.id).await?;
        assert_eq!(provider.name, tname);

        Ok(())
    }

    #[tokio::test]
    async fn test_provider_delete_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = ProviderBmc::delete(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "provider",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
