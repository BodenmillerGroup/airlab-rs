use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

impl TagBmc {
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
  is_metal boolean DEFAULT false NOT NULL,
  is_fluorophore boolean DEFAULT false NOT NULL,
  is_enzyme boolean DEFAULT false NOT NULL,
  is_biotin boolean DEFAULT false NOT NULL,
  is_other boolean DEFAULT false NOT NULL,
  mw smallint,
  emission smallint,
  excitation smallint,
  status smallint DEFAULT 0 NOT NULL,
  meta jsonb,
  created_at timestamp with time zone DEFAULT now() NOT NULL
);
ALTER TABLE ONLY tag
  ADD CONSTRAINT "UQ_tag_group_id_and_name_and_mw" UNIQUE (group_id, name, mw);
CREATE INDEX "IDX_tag_group_id" ON tag USING btree (group_id);
        "##,
            if drop_table {
                format!("drop table if exists {table};")
            } else {
                String::new()
            }
        )
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default)]
pub struct Tag {
    pub id: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "isMetal")]
    pub is_metal: bool,
    #[serde(rename = "isFluorophore")]
    pub is_fluorophore: bool,
    #[serde(rename = "isEnzyme")]
    pub is_enzyme: bool,
    #[serde(rename = "isBiotin")]
    pub is_biotin: bool,
    #[serde(rename = "isOther")]
    pub is_other: bool,
    pub mw: Option<i16>,
    pub emission: Option<i16>,
    pub excitation: Option<i16>,
    pub status: Option<i16>,
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Fields, Deserialize, Clone, Debug)]
pub struct TagForCreate {
    pub name: String,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    pub description: Option<String>,
    #[serde(rename = "isMetal")]
    pub is_metal: bool,
    #[serde(rename = "isFluorophore")]
    pub is_fluorophore: bool,
    #[serde(rename = "isEnzyme")]
    pub is_enzyme: bool,
    #[serde(rename = "isBiotin")]
    pub is_biotin: bool,
    #[serde(rename = "isOther")]
    pub is_other: bool,
    pub mw: Option<i16>,
    pub emission: Option<i16>,
    pub excitation: Option<i16>,
    pub status: Option<i16>,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Fields, Default, Deserialize, Debug)]
pub struct TagForUpdate {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "isMetal")]
    pub is_metal: bool,
    #[serde(rename = "isFluorophore")]
    pub is_fluorophore: bool,
    #[serde(rename = "isEnzyme")]
    pub is_enzyme: bool,
    #[serde(rename = "isBiotin")]
    pub is_biotin: bool,
    #[serde(rename = "isOther")]
    pub is_other: bool,
    pub mw: Option<i16>,
    pub emission: Option<i16>,
    pub excitation: Option<i16>,
    pub status: Option<i16>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct TagFilter {
    id: Option<OpValsInt64>,
    group_id: Option<OpValsInt64>,

    name: Option<OpValsString>,
}

pub struct TagBmc;

impl DbBmc for TagBmc {
    const TABLE: &'static str = "tag";
}

impl TagBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, tag_c: TagForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, tag_c).await
    }
    pub async fn create_full(ctx: &Ctx, mm: &ModelManager, tag_c: Tag) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, tag_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Tag> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<TagFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Tag>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(ctx: &Ctx, mm: &ModelManager, id: i32, tag_u: TagForUpdate) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, tag_u).await
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
    async fn test_tag_create_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let fx_name = "test_create_ok name";

        let tag_c = TagForCreate {
            name: fx_name.to_string(),
            group_id: 1,
            description: None,
            is_metal: false,
            is_fluorophore: false,
            is_enzyme: false,
            is_biotin: false,
            is_other: false,
            mw: None,
            emission: None,
            excitation: None,
            status: Some(0),
        };
        let id = TagBmc::create(&ctx, &mm, tag_c).await?;

        let tag = TagBmc::get(&ctx, &mm, id).await?;
        assert_eq!(tag.name, fx_name);

        TagBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_tag_get_err_not_found() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = TagBmc::get(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "tag",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_tag_list_all_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let tname = "test_tag_list_all_ok";
        let seeds = _dev_utils::get_tag_seed(tname);
        _dev_utils::seed_tags(&ctx, &mm, &seeds).await?;

        let tags = TagBmc::list(&ctx, &mm, None, None).await?;

        let tags: Vec<Tag> = tags
            .into_iter()
            .filter(|t| t.name.starts_with("test_list_all_ok-tag"))
            .collect();
        assert_eq!(tags.len(), 4, "number of seeded tags.");

        if false {
            for tag in tags.iter() {
                TagBmc::delete(&ctx, &mm, tag.id).await?;
            }
        }

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_tag_list_by_filter_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let tname = "test_tag_list_by_filter_ok";
        let seeds = _dev_utils::get_tag_seed(tname);
        _dev_utils::seed_tags(&ctx, &mm, &seeds).await?;

        let filters: Vec<TagFilter> = serde_json::from_value(json!([
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
        let tags = TagBmc::list(&ctx, &mm, Some(filters), Some(list_options)).await?;

        assert_eq!(tags.len(), 3);
        assert!(tags[0].name.ends_with("03"));
        assert!(tags[1].name.ends_with("02.a"));
        assert!(tags[2].name.ends_with("01.a"));

        if false {
            let tags = TagBmc::list(
                &ctx,
                &mm,
                Some(serde_json::from_value(json!([{
                    "name": {"$startsWith": "test_list_by_filter_ok"}
                }]))?),
                None,
            )
            .await?;
            assert_eq!(tags.len(), 5);
            for tag in tags.iter() {
                TagBmc::delete(&ctx, &mm, tag.id).await?;
            }
        }

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_tag_update_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let tname = "test_tag_update_ok";
        let seeds = _dev_utils::get_tag_seed(tname);
        let fx_tag = _dev_utils::seed_tags(&ctx, &mm, &seeds).await?.remove(0);

        TagBmc::update(
            &ctx,
            &mm,
            fx_tag.id,
            TagForUpdate {
                name: tname.to_string(),
                ..Default::default()
            },
        )
        .await?;

        let tag = TagBmc::get(&ctx, &mm, fx_tag.id).await?;
        assert_eq!(tag.name, tname);

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_tag_delete_err_not_found() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = TagBmc::delete(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "tag",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
