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
