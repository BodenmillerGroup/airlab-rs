use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::Result;
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

impl GroupBmc {
    #[must_use]
    pub fn get_create_sql(drop_table: bool) -> String {
        let table = Self::TABLE;
        format!(
            r##"{}
create table if not exists "{table}" (
  id serial primary key,
  name character varying NOT NULL,
  institution character varying,
  description character varying,
  location character varying,
  tags character varying(64)[],
  url character varying,
  is_open boolean DEFAULT false NOT NULL,
  meta jsonb,
  created_at timestamp with time zone DEFAULT now() NOT NULL
);
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
pub struct AirLabGroup {
    pub id: i32,

    pub name: String,
    pub institution: String,
    pub url: Option<String>,
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "isOpen")]
    pub is_open: bool,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize)]
pub struct Group {
    pub id: i32,

    pub name: String,
    pub institution: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub url: Option<String>,
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "isOpen")]
    pub is_open: bool,
    pub tags: Option<Vec<String>>,
}

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct GroupForCreate {
    pub name: String,
    pub institution: String,
    pub url: String,
    #[serde(rename = "isOpen")]
    pub is_open: bool,
    pub tags: Vec<String>,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct GroupForUpdate {
    pub name: String,
    pub institution: String,
    pub url: String,
    //pub meta: Option<String>,
    #[serde(rename = "isOpen")]
    pub is_open: bool,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct GroupFilter {
    id: Option<OpValsInt64>,

    name: Option<OpValsString>,
}

pub struct GroupBmc;

impl DbBmc for GroupBmc {
    const TABLE: &'static str = "group";
}

impl GroupBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, group_c: GroupForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, group_c).await
    }
    pub async fn create_full(ctx: &Ctx, mm: &ModelManager, group_c: Group) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, group_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Group> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<GroupFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Group>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn airlab_list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<GroupFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<AirLabGroup>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        group_u: GroupForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, group_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
