use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

impl ProviderBmc {
    #[must_use]
    pub fn get_create_sql(drop_table: bool) -> String {
        let table = Self::TABLE;
        format!(
            r##"{}
create table if not exists "{table}" (
  id serial primary key,
  group_id integer NOT NULL,
  name character varying NOT NULL,
  description character varying,
  url character varying,
  meta jsonb,
  created_at timestamp with time zone DEFAULT now() NOT NULL
);
ALTER TABLE ONLY provider
  ADD CONSTRAINT "UQ_provider_group_id_and_name" UNIQUE (group_id, name);
CREATE INDEX "IDX_provider_group_id" ON provider USING btree (group_id);
CREATE INDEX "IDX_provider_name" ON provider USING btree (name);
        "##,
            if drop_table {
                format!("drop table if exists {table};")
            } else {
                String::new()
            }
        )
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default)]
pub struct Provider {
    pub id: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,
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
    pub group_id: i32,
    pub description: Option<String>,
    pub url: Option<String>,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct ProviderForUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
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
    ) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, provider_c).await
    }
    pub async fn create_full(ctx: &Ctx, mm: &ModelManager, provider_c: Provider) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, provider_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Provider> {
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

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        provider_u: ProviderForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, provider_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    use crate::model::Error;
    use anyhow::Result;
    use serde_json::json;

    #[ignore]
    #[tokio::test]
    async fn test_provider_create_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
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

    #[ignore]
    #[tokio::test]
    async fn test_provider_get_err_not_found() -> Result<()> {
        let mm = ModelManager::new().await?;
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

    #[ignore]
    #[tokio::test]
    async fn test_provider_list_all_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let tname = "test_provider_list_all_ok";
        let seeds = _dev_utils::get_provider_seed(tname);
        _dev_utils::seed_providers(&ctx, &mm, &seeds).await?;

        let providers = ProviderBmc::list(&ctx, &mm, None, None).await?;

        let providers: Vec<Provider> = providers
            .into_iter()
            .filter(|t| t.name.starts_with("test_list_all_ok-provider"))
            .collect();
        assert_eq!(providers.len(), 4, "number of seeded providers.");

        if false {
            for provider in providers.iter() {
                ProviderBmc::delete(&ctx, &mm, provider.id).await?;
            }
        }

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_provider_list_by_filter_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
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

    #[ignore]
    #[tokio::test]
    async fn test_provider_update_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();

        let providers = ProviderBmc::list(
            &ctx,
            &mm,
            None,
            Some(ListOptions {
                limit: Some(1),
                offset: None,
                order_bys: None,
            }),
        )
        .await?;

        let tname = "test_provider_update_ok";
        let seeds = _dev_utils::get_provider_seed(tname);
        let _fx_provider = _dev_utils::seed_providers(&ctx, &mm, &seeds)
            .await?
            .remove(0);

        ProviderBmc::update(
            &ctx,
            &mm,
            providers[0].id,
            ProviderForUpdate {
                name: Some(tname.to_string()),
                ..Default::default()
            },
        )
        .await?;

        let provider = ProviderBmc::get(&ctx, &mm, providers[0].id).await?;
        assert_eq!(provider.name, tname);

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_provider_delete_err_not_found() -> Result<()> {
        let mm = ModelManager::new().await?;
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
