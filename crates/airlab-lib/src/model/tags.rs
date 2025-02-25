use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::Result;
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default)]
pub struct Tags {
    pub id: i64,
    pub tag: String,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct TagsFilter {
    id: Option<OpValsInt64>,
    group_id: Option<OpValsInt64>,

    name: Option<OpValsString>,
}

pub struct TagsBmc;

impl DbBmc for TagsBmc {
    const TABLE: &'static str = "tags";
}

impl TagsBmc {
    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<TagsFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Tags>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }
}
