use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::Result;
use chrono::prelude::*;
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tracing::warn;

impl LotBmc {
    #[must_use]
    pub fn get_create_sql(drop_table: bool) -> String {
        let table = Self::TABLE;
        format!(
            r##"{}
create table if not exists "{table}" (
  id serial primary key,
  group_id integer NOT NULL,
  created_by integer NOT NULL,
  clone_id integer NOT NULL,
  provider_id integer,
  name character varying NOT NULL,
  reference character varying,
  requested_by integer,
  approved_by integer,
  ordered_by integer,
  received_by integer,
  finished_by integer,
  number character varying,
  status smallint DEFAULT 0 NOT NULL,
  purpose character varying,
  url character varying,
  price character varying,
  note character varying,
  requested_at timestamp with time zone,
  approved_at timestamp with time zone,
  ordered_at timestamp with time zone,
  received_at timestamp with time zone,
  finished_at timestamp with time zone,
  is_archived boolean DEFAULT false NOT NULL,
  meta jsonb,
  created_at timestamp with time zone DEFAULT now() NOT NULL,
  updated_at timestamp with time zone DEFAULT now() NOT NULL
);
CREATE INDEX "IDX_lot_clone_id" ON lot USING btree (clone_id);
CREATE INDEX "IDX_lot_created_by" ON lot USING btree (created_by);
CREATE INDEX "IDX_lot_group_id" ON lot USING btree (group_id);
CREATE INDEX "IDX_lot_provider_id" ON lot USING btree (provider_id);
CREATE INDEX "IDX_lot_status" ON lot USING btree (status);
        "##,
            if drop_table {
                format!("drop table if exists {table};")
            } else {
                String::new()
            }
        )
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Default, Deserialize)]
pub struct Lot {
    pub id: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "createdBy")]
    pub created_by: Option<i32>,

    #[serde(rename = "cloneId")]
    pub clone_id: i32,
    #[serde(rename = "providerId")]
    pub provider_id: Option<i32>,
    pub name: String,
    pub reference: Option<String>,
    #[serde(rename = "requestedBy")]
    pub requested_by: Option<i32>,
    #[serde(rename = "approvedBy")]
    pub approved_by: Option<i32>,
    #[serde(rename = "orderedBy")]
    pub ordered_by: Option<i32>,
    #[serde(rename = "receivedBy")]
    pub received_by: Option<i32>,
    #[serde(rename = "finishedBy")]
    pub finished_by: Option<i32>,
    pub number: Option<String>,
    pub status: Option<i16>,
    pub purpose: Option<String>,
    pub url: Option<String>,
    pub price: Option<String>,
    pub note: Option<String>,
    #[serde(rename = "requestedAt")]
    pub requested_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "orderedAt")]
    pub ordered_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "receivedAt")]
    pub received_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "finishedAt")]
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "isArchived")]
    pub is_archived: bool,
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct LotForCreate {
    pub name: String,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "cloneId")]
    pub clone_id: i32,
    #[serde(rename = "createdBy")]
    pub created_by: Option<i32>,
    #[serde(rename = "providerId")]
    pub provider_id: Option<i32>,
    pub reference: Option<String>,
    #[serde(rename = "requestedBy")]
    pub requested_by: Option<i32>,
    #[serde(rename = "approvedBy")]
    pub approved_by: Option<i32>,
    #[serde(rename = "orderedBy")]
    pub ordered_by: Option<i32>,
    #[serde(rename = "receivedBy")]
    pub received_by: Option<i32>,
    #[serde(rename = "finishedBy")]
    pub finished_by: Option<i32>,
    pub status: Option<i16>,
    pub purpose: Option<String>,
    pub url: Option<String>,
    pub price: Option<String>,
    pub note: Option<String>,
    #[serde(rename = "requestedAt")]
    pub requested_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "orderedAt")]
    pub ordered_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "receivedAt")]
    pub received_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "finishedAt")]
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "isArchived")]
    pub is_archived: Option<bool>,
    //pub meta: Option<String>,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct LotForUpdate {
    pub name: Option<String>,
    pub reference: Option<String>,
    #[serde(rename = "createdBy")]
    pub created_by: Option<i32>,
    #[serde(rename = "requestedBy")]
    pub requested_by: Option<i32>,
    #[serde(rename = "approvedBy")]
    pub approved_by: Option<i32>,
    #[serde(rename = "orderedBy")]
    pub ordered_by: Option<i32>,
    #[serde(rename = "receivedBy")]
    pub received_by: Option<i32>,
    #[serde(rename = "finishedBy")]
    pub finished_by: Option<i32>,
    pub status: Option<i16>,
    pub number: Option<String>,
    pub purpose: Option<String>,
    pub url: Option<String>,
    pub price: Option<String>,
    pub note: Option<String>,
    #[serde(rename = "approvedAt")]
    pub approved_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "requestedAt")]
    pub requested_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "orderedAt")]
    pub ordered_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "receivedAt")]
    pub received_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "finishedAt")]
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "isArchived")]
    pub is_archived: Option<bool>,
    //pub meta: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct LotFilter {
    id: Option<OpValsInt64>,
    group_id: Option<OpValsInt64>,
    clone_id: Option<OpValsInt64>,
    provider_id: Option<OpValsInt64>,
    status: Option<OpValsInt64>,
    name: Option<OpValsString>,
}

pub struct LotBmc;

impl DbBmc for LotBmc {
    const TABLE: &'static str = "lot";
}

impl LotBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, lot_c: LotForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, lot_c).await
    }
    pub async fn create_full(ctx: &Ctx, mm: &ModelManager, lot_c: Lot) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, lot_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Lot> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<LotFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Lot>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        member_id: i32,
        mut lot_u: LotForUpdate,
    ) -> Result<()> {
        warn!("{:?}", lot_u.status);
        if let Some(status) = lot_u.status {
            if status == 1 {
                lot_u.approved_by = Some(member_id);
                lot_u.approved_at = Some(Utc::now());
            } else if status == 3 {
                lot_u.ordered_by = Some(member_id);
                lot_u.ordered_at = Some(Utc::now());
            } else if status == 4 {
                lot_u.received_by = Some(member_id);
                lot_u.received_at = Some(Utc::now());
            } else if status == 6 {
                lot_u.finished_by = Some(member_id);
                lot_u.finished_at = Some(Utc::now());
            }
        }
        lot_u.updated_at = Some(Utc::now());
        base::update::<Self, _>(ctx, mm, id, lot_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
