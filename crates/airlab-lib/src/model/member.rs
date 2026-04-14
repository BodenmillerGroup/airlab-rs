use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
use crate::model::helpers::{bool_or, i64_or, opt_i64, opt_string};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsBool, OpValsInt64};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Clone, Fields, FromRow, Serialize, Default, Deserialize)]
pub struct Member {
    pub id: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,
    #[serde(rename = "userId")]
    pub user_id: i64,
    pub role: i64,
    #[serde(rename = "allPanels")]
    pub all_panels: bool,
    #[serde(rename = "activationKey")]
    pub activation_key: Option<String>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Fields, Deserialize, Debug)]
pub struct MemberForCreate {
    #[serde(rename = "groupId")]
    pub group_id: i64,
    #[serde(rename = "userId")]
    pub user_id: i64,
    pub role: i64,
    #[serde(rename = "activationKey")]
    pub activation_key: Option<String>,
    #[serde(rename = "allPanels")]
    pub all_panels: bool,
    #[serde(rename = "isActive")]
    pub is_active: bool,
}

impl From<Value> for MemberForCreate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        MemberForCreate {
            group_id: i64_or(&obj, "groupId", 0),
            user_id: i64_or(&obj, "userId", 0),
            role: i64_or(&obj, "role", 0),
            activation_key: opt_string(&obj, "activationKey"),
            all_panels: bool_or(&obj, "allPanels", false),
            is_active: bool_or(&obj, "isActive", false),
        }
    }
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct MemberForUpdate {
    pub role: Option<i64>,
    #[serde(rename = "activationKey")]
    pub activation_key: Option<String>,
    #[serde(rename = "allPanels")]
    pub all_panels: bool,
    #[serde(rename = "isActive")]
    pub is_active: bool,
}

impl From<Value> for MemberForUpdate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        MemberForUpdate {
            role: opt_i64(&obj, "role"),
            activation_key: opt_string(&obj, "activationKey"),
            all_panels: bool_or(&obj, "allPanels", false),
            is_active: bool_or(&obj, "isActive", false),
        }
    }
}

#[derive(FilterNodes, Deserialize, Default, Debug, Clone)]
pub struct MemberFilter {
    id: Option<OpValsInt64>,
    user_id: Option<OpValsInt64>,
    group_id: Option<OpValsInt64>,
    is_active: Option<OpValsBool>,
}

pub struct MemberBmc;

impl DbBmc for MemberBmc {
    const TABLE: &'static str = "member";
}

impl MemberBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, member_c: MemberForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, member_c).await
    }
    pub async fn create_full(ctx: &Ctx, mm: &ModelManager, member_c: Member) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, member_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Member> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<MemberFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Member>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn count(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<MemberFilter>>,
    ) -> Result<i64> {
        base::count::<Self, _>(ctx, mm, filters).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        member_u: MemberForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, member_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    use serde_json::json;

    type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    #[tokio::test]
    async fn test_member_create_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_user_id = 1000;
        let fx_group_id = 1;

        let member_c = MemberForCreate {
            group_id: fx_group_id,
            user_id: fx_user_id,
            activation_key: None,
            all_panels: false,
            is_active: false,
            role: 100,
        };
        let id = MemberBmc::create(&ctx, &mm, member_c).await?;

        let member = MemberBmc::get(&ctx, &mm, id).await?;
        assert_eq!(member.user_id, fx_user_id);

        MemberBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[allow(dead_code)]
    fn get_seed() -> Vec<(i64, i64)> {
        vec![(1, 1001)]
    }

    #[tokio::test]
    async fn test_member_list_by_filter_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_names = get_seed();
        _dev_utils::seed_members(&ctx, &mm, &fx_names).await?;

        let filters: Vec<MemberFilter> = serde_json::from_value(json!([
            {
                "group_id": 1,
                "user_id": 1001
            }
        ]))?;
        let members = MemberBmc::list(&ctx, &mm, Some(filters), None).await?;

        assert_eq!(members.len(), 1);
        assert_eq!(members[0].user_id, 1001);
        Ok(())
    }
}
