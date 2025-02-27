use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

impl PanelElementBmc {
    #[must_use]
    pub fn get_create_sql(drop_table: bool) -> String {
        let table = Self::TABLE;
        format!(
            r##"{}
create table if not exists "{table}" (
  id serial primary key,
  panel_id integer NOT NULL,
  conjugate_id integer NOT NULL,
  dilution_type smallint DEFAULT 0 NOT NULL,
  concentration real
);
ALTER TABLE ONLY panel_element
  ADD CONSTRAINT "UQ_panel_element_panel_id_and_conjugate_id" UNIQUE (panel_id, conjugate_id);
CREATE INDEX "IDX_panel_element_conjugate_id" ON panel_element USING btree (conjugate_id);
CREATE INDEX "IDX_panel_element_panel_id" ON panel_element USING btree (panel_id);
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
pub struct PanelElement {
    pub id: i32,

    #[serde(rename = "panelId")]
    pub panel_id: i32,
    #[serde(rename = "conjugateId")]
    pub conjugate_id: i32,
    #[serde(rename = "dilutionType")]
    pub dilution_type: i16,
    pub concentration: Option<f32>,
}

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct PanelElementForCreate {
    #[serde(rename = "panelId")]
    pub panel_id: i32,
    #[serde(rename = "conjugateId")]
    pub conjugate_id: i32,
    #[serde(rename = "dilutionType")]
    pub dilution_type: i32,
    pub concentration: Option<f32>,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct PanelElementForUpdate {
    #[serde(rename = "dilutionType")]
    pub dilution_type: i32,
    pub concentration: Option<f32>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct PanelElementFilter {
    id: Option<OpValsInt64>,

    panel_id: Option<OpValsInt64>,
    conjugate_id: Option<OpValsInt64>,
}

pub struct PanelElementBmc;

impl DbBmc for PanelElementBmc {
    const TABLE: &'static str = "panel_element";
}

impl PanelElementBmc {
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        panel_element_c: PanelElementForCreate,
    ) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, panel_element_c).await
    }
    pub async fn create_full(
        ctx: &Ctx,
        mm: &ModelManager,
        panel_element_c: PanelElement,
    ) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, panel_element_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<PanelElement> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<PanelElementFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<PanelElement>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        panel_element_u: PanelElementForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, panel_element_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
