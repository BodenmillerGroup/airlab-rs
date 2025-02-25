use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::Result;
//use chrono::prelude::*;
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsBool, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

impl PanelBmc {
    #[must_use]
    pub fn get_create_sql(drop_table: bool) -> String {
        let table = Self::TABLE;
        format!(
            r##"{}
create table if not exists "{table}" (
  id serial primary key,
  group_id integer NOT NULL,
  created_by integer NOT NULL,
  name character varying,
  description character varying,
  is_fluorophore boolean DEFAULT false NOT NULL,
  is_locked boolean DEFAULT false NOT NULL,
  application integer,
  meta jsonb,
  is_archived boolean DEFAULT false NOT NULL,
  created_at timestamp with time zone DEFAULT now() NOT NULL,
  updated_at timestamp with time zone DEFAULT now() NOT NULL
);
CREATE INDEX "IDX_panel_group_id" ON panel USING btree (group_id);
CREATE INDEX "IDX_panel_created_by" ON panel USING btree (created_by);
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
pub struct Panel {
    pub id: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,

    #[serde(rename = "createdBy")]
    pub created_by: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "isFluorophore")]
    pub is_fluorophore: bool,
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    pub application: Option<i32>,
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "isArchived")]
    pub is_archived: bool,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct PanelForCreate {
    pub name: Option<String>,
    #[serde(rename = "groupId")]
    pub group_id: i32,

    #[serde(rename = "createdBy")]
    pub created_by: Option<i32>,
    pub description: Option<String>,
    #[serde(rename = "isFluorophore")]
    pub is_fluorophore: bool,
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    pub application: Option<i32>,
    //pub meta: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ElementUpdate {
    pub concentration: Option<f32>,
    #[serde(rename = "conjugateId")]
    pub conjugate_id: i32,
    #[serde(rename = "dilutionType")]
    pub dilution_type: i32,
}

#[derive(Default, Deserialize, Debug)]
pub struct PanelPayloadForUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "isFluorophore")]
    pub is_fluorophore: Option<bool>,
    #[serde(rename = "isLocked")]
    pub is_locked: Option<bool>,
    pub application: Option<i32>,
    pub elements: Vec<ElementUpdate>,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct PanelForUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "isFluorophore")]
    pub is_fluorophore: Option<bool>,
    #[serde(rename = "isLocked")]
    pub is_locked: Option<bool>,
    pub is_archived: Option<bool>,
    pub application: Option<i32>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct PanelFilter {
    id: Option<OpValsInt64>,
    group_id: Option<OpValsInt64>,
    //conjugate_id: Option<OpValsInt64>,
    name: Option<OpValsString>,
    is_archived: Option<OpValsBool>,
}

pub struct PanelBmc;

impl DbBmc for PanelBmc {
    const TABLE: &'static str = "panel";
}

impl PanelBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, panel_c: PanelForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, panel_c).await
    }
    pub async fn create_full(ctx: &Ctx, mm: &ModelManager, panel_c: Panel) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, panel_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Panel> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<PanelFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Panel>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        panel_u: PanelForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, panel_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
