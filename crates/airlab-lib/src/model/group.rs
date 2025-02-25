use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::Result;
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

impl GroupBmc {
    #[must_use]
    pub fn get_create_sql(drop_table: bool) -> String {
        let table = Self::TABLE;
        format!(
            r##"{}
create table if not exists "{table}" (
  id serial primary key,
  name character varying NOT NULL,
  institution character varying,
  description character varying,
  location character varying,
  tags character varying(64)[],
  url character varying,
  is_open boolean DEFAULT false NOT NULL,
  meta jsonb,
  created_at timestamp with time zone DEFAULT now() NOT NULL
);
        "##,
            if drop_table {
                format!("drop table if exists {table};")
            } else {
                String::new()
            }
        )
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize)]
pub struct AirLabGroup {
    pub id: i32,

    pub name: String,
    pub institution: String,
    pub url: Option<String>,
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "isOpen")]
    pub is_open: bool,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize)]
pub struct Group {
    pub id: i32,

    pub name: String,
    pub institution: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub url: Option<String>,
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "isOpen")]
    pub is_open: bool,
    pub tags: Option<Vec<String>>,
}

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct GroupForCreate {
    pub name: String,
    pub institution: String,
    pub url: Option<String>,
    #[serde(rename = "isOpen")]
    pub is_open: bool,
    pub tags: Option<Vec<String>>,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct GroupForUpdate {
    pub name: String,
    pub institution: String,
    pub url: String,
    //pub meta: Option<String>,
    #[serde(rename = "isOpen")]
    pub is_open: bool,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct GroupFilter {
    id: Option<OpValsInt64>,

    name: Option<OpValsString>,
}

pub struct GroupBmc;

impl DbBmc for GroupBmc {
    const TABLE: &'static str = "group";
}

impl GroupBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, group_c: GroupForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, group_c).await
    }
    pub async fn create_full(ctx: &Ctx, mm: &ModelManager, group_c: Group) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, group_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Group> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<GroupFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Group>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn airlab_list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<GroupFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<AirLabGroup>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        group_u: GroupForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, group_u).await
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

    #[tokio::test]
    async fn test_group_create_ok() -> Result<()> {
        let mm = ModelManager::new().await?;

        let ctx = Ctx::root_ctx();
        let fx_name = "test_create_ok name";

        let group_c = GroupForCreate {
            name: fx_name.to_string(),
            institution: "inst 01".to_string(),
            url: Some("url 01".to_string()),
            is_open: false,
            tags: None,
        };
        let id = GroupBmc::create(&ctx, &mm, group_c).await?;

        let group = GroupBmc::get(&ctx, &mm, id).await?;
        assert_eq!(group.name, fx_name);

        GroupBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_group_get_err_not_found() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = GroupBmc::get(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "group",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_group_list_all_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let tname = "test_group_list_all_ok";
        let gseeds = _dev_utils::get_group_seed(tname);
        let _groups = _dev_utils::seed_groups(&ctx, &mm, &gseeds).await?;

        let groups = GroupBmc::list(&ctx, &mm, None, None).await?;

        let groups: Vec<Group> = groups
            .into_iter()
            .filter(|t| t.name.starts_with(tname))
            .collect();
        assert_eq!(groups.len(), gseeds.len(), "number of seeded groups.");

        for group in groups.iter() {
            GroupBmc::delete(&ctx, &mm, group.id).await?;
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_group_list_by_filter_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let tname = "test_group_list_by_filter_ok";
        let fx_names = _dev_utils::get_group_seed(tname);
        _dev_utils::seed_groups(&ctx, &mm, &fx_names).await?;

        let filters: Vec<GroupFilter> = serde_json::from_value(json!([
            {
                "name": {
                    "$contains": tname,
                    "$containsAny": ["01"]
                }
            }
        ]))?;
        let list_options = serde_json::from_value(json!({
            "order_bys": "!id"
        }))?;
        let groups = GroupBmc::list(&ctx, &mm, Some(filters), Some(list_options)).await?;

        assert_eq!(groups.len(), 1);
        assert!(groups[0].name.ends_with("01"));

        let groups = GroupBmc::list(
            &ctx,
            &mm,
            Some(serde_json::from_value(json!([{
                "name": {"$startsWith": tname}
            }]))?),
            None,
        )
        .await?;
        for group in groups.iter() {
            GroupBmc::delete(&ctx, &mm, group.id).await?;
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_group_update_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let tname = "test_group_update_ok";
        let fx_names = _dev_utils::get_group_seed(tname);
        let fx_name_new = "test_update_ok - group 01 - new";
        let fx_group = _dev_utils::seed_groups(&ctx, &mm, &fx_names)
            .await?
            .remove(0);

        // -- Exec
        GroupBmc::update(
            &ctx,
            &mm,
            fx_group.id,
            GroupForUpdate {
                name: fx_name_new.to_string(),
                ..Default::default()
            },
        )
        .await?;

        // -- Check
        let group = GroupBmc::get(&ctx, &mm, fx_group.id).await?;
        assert_eq!(group.name, fx_name_new);

        Ok(())
    }

    #[tokio::test]
    async fn test_group_delete_err_not_found() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = GroupBmc::delete(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "group",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
