use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

impl ConjugateBmc {
    #[must_use]
    pub fn get_create_sql(drop_table: bool) -> String {
        let table = Self::TABLE;
        format!(
            r##"{}
create table if not exists "{table}" (
  id serial primary key,
  group_id integer NOT NULL,
  created_by integer NOT NULL,
  labeled_by integer,
  finished_by integer,
  lot_id integer NOT NULL,
  tag_id integer NOT NULL,
  status smallint DEFAULT 0 NOT NULL,
  tube_number integer NOT NULL,
  concentration real,
  description character varying,
  finished_at timestamp with time zone,
  is_archived boolean DEFAULT false NOT NULL,
  custom_id character varying,
  meta jsonb,
  created_at timestamp with time zone DEFAULT now() NOT NULL,
  updated_at timestamp with time zone DEFAULT now() NOT NULL
);
CREATE INDEX "IDX_conjugate_created_by" ON conjugate USING btree (created_by);
CREATE INDEX "IDX_conjugate_group_id" ON conjugate USING btree (group_id);
CREATE INDEX "IDX_conjugate_lot_id" ON conjugate USING btree (lot_id);
CREATE INDEX "IDX_conjugate_status" ON conjugate USING btree (status);
CREATE INDEX "IDX_conjugate_tag_id" ON conjugate USING btree (tag_id);
CREATE INDEX "IDX_conjugate_tube_number" ON conjugate USING btree (tube_number);
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
pub struct Conjugate {
    pub id: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,

    #[serde(rename = "createdBy")]
    pub created_by: i32,
    #[serde(rename = "labeledBy")]
    pub labeled_by: Option<i32>,
    #[serde(rename = "finishedBy")]
    pub finished_by: Option<i32>,
    #[serde(rename = "lotId")]
    pub lot_id: i32,
    #[serde(rename = "tagId")]
    pub tag_id: i32,
    pub status: i16,
    #[serde(rename = "tubeNumber")]
    pub tube_number: i32,
    pub concentration: Option<f32>,
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
    pub group_id: i32,
    #[serde(rename = "createdBy")]
    pub created_by: Option<i32>,
    #[serde(rename = "labeledBy")]
    pub labeled_by: Option<i32>,
    #[serde(rename = "finishedBy")]
    pub finished_by: Option<i32>,
    #[serde(rename = "lotId")]
    pub lot_id: i32,
    #[serde(rename = "tagId")]
    pub tag_id: i32,
    pub status: Option<i16>,
    pub concentration: Option<f32>,
    pub description: Option<String>,
    #[serde(rename = "finishedAt")]
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "isArchived")]
    pub is_archived: Option<bool>,
    #[serde(rename = "customId")]
    pub custom_id: Option<String>,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct ConjugateForUpdate {
    #[serde(rename = "labeledBy")]
    pub labeled_by: Option<i32>,
    #[serde(rename = "finishedBy")]
    pub finished_by: Option<i32>,
    #[serde(rename = "lotId")]
    pub lot_id: Option<i32>,
    #[serde(rename = "tagId")]
    pub tag_id: Option<i32>,
    pub status: Option<i16>,
    #[serde(rename = "tubeNumber")]
    pub tube_number: Option<i32>,
    pub concentration: Option<f32>,
    pub description: Option<String>,
    #[serde(rename = "finishedAt")]
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "isArchived")]
    pub is_archived: Option<bool>,
    #[serde(rename = "customId")]
    pub custom_id: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ConjugateFilter {
    id: Option<OpValsInt64>,
    lot_id: Option<OpValsInt64>,
    group_id: Option<OpValsInt64>,

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
        conjugate_c: ConjugateForCreate,
    ) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, conjugate_c).await
    }
    pub async fn create_full(ctx: &Ctx, mm: &ModelManager, conjugate_c: Conjugate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, conjugate_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Conjugate> {
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

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        conjugate_u: ConjugateForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, conjugate_u).await
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
    async fn test_conjugate_create_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
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
            concentration: None,
            finished_at: None,
            is_archived: None,
            custom_id: None,
        };
        let id = ConjugateBmc::create(&ctx, &mm, conjugate_c).await?;

        let conjugate = ConjugateBmc::get(&ctx, &mm, id).await?;
        assert_eq!(conjugate.description.unwrap_or("".into()), fx_description);

        ConjugateBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_conjugate_get_err_not_found() -> Result<()> {
        let mm = ModelManager::new().await?;
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

    #[ignore]
    #[tokio::test]
    async fn test_conjugate_list_all_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let tname = "test_conjugate_list_all_ok";
        let seeds = _dev_utils::get_conjugate_seed(tname);
        _dev_utils::seed_conjugates(&ctx, &mm, &seeds).await?;

        let conjugates = ConjugateBmc::list(&ctx, &mm, None, None).await?;

        let conjugates: Vec<Conjugate> = conjugates.into_iter().collect();
        assert_eq!(conjugates.len(), 4, "number of seeded conjugates.");

        for conjugate in conjugates.iter() {
            ConjugateBmc::delete(&ctx, &mm, conjugate.id).await?;
        }

        Ok(())
    }

    #[ignore]
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
        //assert!(conjugates[0].description.unwrap().ends_with("03"));

        let conjugates = ConjugateBmc::list(
            &ctx,
            &mm,
            Some(serde_json::from_value(json!([{
                "description": {"$startsWith": "test_list_by_filter_ok"}
            }]))?),
            None,
        )
        .await?;
        assert_eq!(conjugates.len(), 5);
        for conjugate in conjugates.iter() {
            ConjugateBmc::delete(&ctx, &mm, conjugate.id).await?;
        }

        Ok(())
    }

    #[ignore]
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

    #[ignore]
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
