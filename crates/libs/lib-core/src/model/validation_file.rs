use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::Result;
//use chrono::prelude::*;
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

impl ValidationFileBmc {
    #[must_use]
    pub fn get_create_sql(drop_table: bool) -> String {
        let table = Self::TABLE;
        format!(
            r##"{}
create table if not exists "{table}" (
  id serial primary key,
  validation_id integer NOT NULL,
  created_by integer NOT NULL,
  hash character varying NOT NULL,
  size integer,
  name character varying,
  extension character varying NOT NULL,
  description character varying,
  meta jsonb,
  created_at timestamp with time zone DEFAULT now() NOT NULL
);
CREATE INDEX "IDX_validation_file_created_by" ON validation_file USING btree (created_by);
CREATE INDEX "IDX_validation_file_hash" ON validation_file USING btree (hash);
CREATE INDEX "IDX_validation_file_validation_id" ON validation_file USING btree (validation_id);
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
pub struct ValidationFile {
    pub id: i32,

    #[serde(rename = "validationId")]
    pub validation_id: i32,
    #[serde(rename = "createdBy")]
    pub created_by: i32,
    pub hash: String,
    pub size: i32,
    pub name: Option<String>,
    pub extension: String,
    pub description: Option<String>,
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct ValidationFileForCreate {
    #[serde(rename = "validationId")]
    pub validation_id: i32,
    #[serde(rename = "createdBy")]
    pub created_by: i32,
    pub hash: String,
    pub size: i32,
    pub name: Option<String>,
    pub extension: String,
    pub description: Option<String>,
    //pub meta: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct ValidationFileForUpdate {
    pub hash: String,
    pub size: i32,
    pub name: Option<String>,
    pub extension: Option<String>,
    pub description: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ValidationFileFilter {
    id: Option<OpValsInt64>,
    validation_id: Option<OpValsInt64>,

    name: Option<OpValsString>,
}

pub struct ValidationFileBmc;

impl DbBmc for ValidationFileBmc {
    const TABLE: &'static str = "validation_file";
}

impl ValidationFileBmc {
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        validation_file_c: ValidationFileForCreate,
    ) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, validation_file_c).await
    }
    pub async fn create_full(
        ctx: &Ctx,
        mm: &ModelManager,
        validation_file_c: ValidationFile,
    ) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, validation_file_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<ValidationFile> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<ValidationFileFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<ValidationFile>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        validation_file_u: ValidationFileForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, validation_file_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
