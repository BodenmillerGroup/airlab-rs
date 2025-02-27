use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

impl CloneBmc {
    #[must_use]
    pub fn get_create_sql(drop_table: bool) -> String {
        let table = Self::TABLE;
        format!(
            r##"{}
create table if not exists "{table}" (
  id serial primary key,
  group_id integer NOT NULL,
  created_by integer NOT NULL,
  protein_id integer NOT NULL,
  species_id integer,
  name character varying NOT NULL,
  isotype character varying,
  epitope character varying,
  is_phospho boolean DEFAULT false NOT NULL,
  is_polyclonal boolean DEFAULT false NOT NULL,
  reactivity integer[],
  application jsonb,
  is_archived boolean DEFAULT false NOT NULL,
  meta jsonb,
  created_at timestamp with time zone DEFAULT now() NOT NULL,
  updated_at timestamp with time zone DEFAULT now() NOT NULL
);
CREATE INDEX "IDX_clone_created_by" ON clone USING btree (created_by);
CREATE INDEX "IDX_clone_group_id" ON clone USING btree (group_id);
CREATE INDEX "IDX_clone_name" ON clone USING btree (name);
CREATE INDEX "IDX_clone_protein_id" ON clone USING btree (protein_id);
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
pub struct Clone {
    pub id: i32,

    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "createdBy")]
    pub created_by: i32,
    #[serde(rename = "proteinId")]
    pub protein_id: i32,
    #[serde(rename = "speciesId")]
    pub species_id: Option<i32>,
    pub name: String,
    pub isotype: Option<String>,
    pub epitope: Option<String>,
    #[serde(rename = "isPhospho")]
    pub is_phospho: bool,
    #[serde(rename = "isPolyclonal")]
    pub is_polyclonal: bool,
    pub reactivity: Option<Vec<i32>>,
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
    pub group_id: i32,
    #[serde(rename = "createdBy")]
    pub created_by: Option<i32>,
    #[serde(rename = "proteinId")]
    pub protein_id: i32,
    #[serde(rename = "speciesId")]
    pub species_id: Option<i32>,
    pub name: String,
    pub isotype: String,
    pub epitope: String,
    #[serde(rename = "isPhospho")]
    pub is_phospho: bool,
    #[serde(rename = "isPolyclonal")]
    pub is_polyclonal: bool,
    #[serde(rename = "isArchived")]
    pub is_archived: Option<bool>,
    pub reactivity: Option<Vec<i32>>,
    pub application: Option<serde_json::Value>,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct CloneForUpdate {
    pub name: Option<String>,
    pub isotype: Option<String>,
    pub epitope: Option<String>,
    #[serde(rename = "isPhospho")]
    pub is_phospho: bool,
    #[serde(rename = "isPolyclonal")]
    pub is_polyclonal: bool,
    #[serde(rename = "isArchived")]
    pub is_archived: Option<bool>,
    pub reactivity: Option<Vec<i32>>,
    pub application: Option<serde_json::Value>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
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
    pub async fn create(ctx: &Ctx, mm: &ModelManager, clone_c: CloneForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, clone_c).await
    }
    pub async fn create_full(ctx: &Ctx, mm: &ModelManager, clone_c: Clone) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, clone_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Clone> {
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

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        clone_u: CloneForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, clone_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
