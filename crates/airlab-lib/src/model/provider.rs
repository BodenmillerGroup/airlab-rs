use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

impl ProviderBmc {
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
  url character varying,
  meta jsonb,
  created_at timestamp with time zone DEFAULT now() NOT NULL
);
ALTER TABLE ONLY provider
  ADD CONSTRAINT "UQ_provider_group_id_and_name" UNIQUE (group_id, name);
CREATE INDEX "IDX_provider_group_id" ON provider USING btree (group_id);
CREATE INDEX "IDX_provider_name" ON provider USING btree (name);
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
pub struct Provider {
    pub id: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct ProviderForCreate {
    pub name: String,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    pub description: Option<String>,
    pub url: Option<String>,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct ProviderForUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ProviderFilter {
    id: Option<OpValsInt64>,
    group_id: Option<OpValsInt64>,
    name: Option<OpValsString>,
}

pub struct ProviderBmc;

impl DbBmc for ProviderBmc {
    const TABLE: &'static str = "provider";
}

impl ProviderBmc {
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        provider_c: ProviderForCreate,
    ) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, provider_c).await
    }
    pub async fn create_full(ctx: &Ctx, mm: &ModelManager, provider_c: Provider) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, provider_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Provider> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<ProviderFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Provider>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        provider_u: ProviderForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, provider_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
