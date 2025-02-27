use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

impl SpeciesBmc {
    #[must_use]
    pub fn get_create_sql(drop_table: bool) -> String {
        let table = Self::TABLE;
        format!(
            r##"{}
create table if not exists "{table}" (
  id serial primary key,
  group_id integer NOT NULL,
  name character varying NOT NULL,
  acronym character varying NOT NULL,
  meta jsonb,
  created_at timestamp with time zone DEFAULT now() NOT NULL
);
ALTER TABLE ONLY species
  ADD CONSTRAINT "UQ_species_group_id_and_acronym" UNIQUE (group_id, acronym);
ALTER TABLE ONLY species
  ADD CONSTRAINT "UQ_species_group_id_and_name" UNIQUE (group_id, name);
CREATE INDEX "IDX_species_group_id" ON species USING btree (group_id);
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
pub struct Species {
    pub id: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,

    pub name: String,
    pub acronym: String,
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct SpeciesForCreate {
    pub name: String,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    pub acronym: String,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct SpeciesForUpdate {
    pub name: Option<String>,
    pub acronym: String,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct SpeciesFilter {
    id: Option<OpValsInt64>,

    name: Option<OpValsString>,
    group_id: Option<OpValsInt64>,
}

pub struct SpeciesBmc;

impl DbBmc for SpeciesBmc {
    const TABLE: &'static str = "species";
}

impl SpeciesBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, species_c: SpeciesForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, species_c).await
    }
    pub async fn create_full(ctx: &Ctx, mm: &ModelManager, species_c: Species) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, species_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Species> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<SpeciesFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Species>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        species_u: SpeciesForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, species_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
