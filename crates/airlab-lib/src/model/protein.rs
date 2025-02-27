use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

impl ProteinBmc {
    #[must_use]
    pub fn get_create_sql(drop_table: bool) -> String {
        let table = Self::TABLE;
        format!(
            r##"{}
create table if not exists "{table}" (
  id serial primary key,
  group_id integer NOT NULL,
  created_by integer NOT NULL,
  name character varying NOT NULL,
  description character varying,
  meta jsonb,
  created_at timestamp with time zone DEFAULT now() NOT NULL
);
CREATE INDEX "IDX_protein_created_by" ON protein USING btree (created_by);
CREATE INDEX "IDX_protein_group_id" ON protein USING btree (group_id);
CREATE INDEX "IDX_protein_name" ON protein USING btree (name);
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
pub struct Protein {
    pub id: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,

    #[serde(rename = "createdBy")]
    pub created_by: i32,
    pub name: String,
    pub description: Option<String>,
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>, //String
}

#[derive(Fields, Deserialize, Clone)]
pub struct ProteinForCreate {
    pub name: String,
    pub description: Option<String>,
    pub group_id: i32,
    pub created_by: i32,
}

#[derive(Fields, Default, Deserialize)]
pub struct ProteinForUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ProteinFilter {
    id: Option<OpValsInt64>,
    group_id: Option<OpValsInt64>,
    name: Option<OpValsString>,
    description: Option<OpValsString>,
}

pub struct ProteinBmc;

impl DbBmc for ProteinBmc {
    const TABLE: &'static str = "protein";
}

impl ProteinBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, protein_c: ProteinForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, protein_c).await
    }
    pub async fn create_full(ctx: &Ctx, mm: &ModelManager, protein_c: Protein) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, protein_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Protein> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<ProteinFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Protein>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        protein_u: ProteinForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, protein_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
