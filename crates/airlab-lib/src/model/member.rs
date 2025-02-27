use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

impl MemberBmc {
    #[must_use]
    pub fn get_create_sql(drop_table: bool) -> String {
        let table = Self::TABLE;
        format!(
            r##"{}
create table if not exists "{table}" (
  id serial primary key,
  group_id integer NOT NULL,
  user_id integer NOT NULL,
  role smallint DEFAULT 0 NOT NULL,
  all_panels boolean DEFAULT false NOT NULL,
  activation_key character varying,
  is_active boolean DEFAULT false NOT NULL,
  created_at timestamp with time zone DEFAULT now() NOT NULL,
  updated_at timestamp with time zone DEFAULT now() NOT NULL
);
ALTER TABLE ONLY member
  ADD CONSTRAINT "UQ_member_group_id_and_user_id" UNIQUE (group_id, user_id);
CREATE INDEX "IDX_member_activation_key" ON member USING btree (activation_key);
CREATE INDEX "IDX_member_group_id" ON member USING btree (group_id);
CREATE INDEX "IDX_member_is_active" ON member USING btree (is_active);
CREATE INDEX "IDX_member_user_id" ON member USING btree (user_id);

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
pub struct Member {
    pub id: i32,

    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub role: i16,
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
    pub group_id: i32,
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub role: i16,
    #[serde(rename = "activationKey")]
    pub activation_key: Option<String>,
    #[serde(rename = "allPanels")]
    pub all_panels: bool,
    #[serde(rename = "isActive")]
    pub is_active: bool,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct MemberForUpdate {
    pub role: Option<i16>,
    #[serde(rename = "activationKey")]
    pub activation_key: Option<String>,
    #[serde(rename = "allPanels")]
    pub all_panels: bool,
    #[serde(rename = "isActive")]
    pub is_active: bool,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct MemberFilter {
    id: Option<OpValsInt64>,
    user_id: Option<OpValsInt64>,
    group_id: Option<OpValsInt64>,
}

pub struct MemberBmc;

impl DbBmc for MemberBmc {
    const TABLE: &'static str = "member";
}

impl MemberBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, member_c: MemberForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, member_c).await
    }
    pub async fn create_full(ctx: &Ctx, mm: &ModelManager, member_c: Member) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, member_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Member> {
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

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        member_u: MemberForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, member_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    use anyhow::Result;
    use serde_json::json;

    #[ignore]
    #[tokio::test]
    async fn test_member_create_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let fx_user_id = 1;
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
    fn get_seed() -> Vec<(i32, i32)> {
        vec![(1000, 1000)]
    }

    #[ignore]
    #[tokio::test]
    async fn test_member_list_by_filter_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let fx_names = get_seed();
        _dev_utils::seed_members(&ctx, &mm, &fx_names).await?;

        let filters: Vec<MemberFilter> = serde_json::from_value(json!([
            {
                "user_id": 1001
            }
        ]))?;
        let members = MemberBmc::list(&ctx, &mm, Some(filters), None).await?;
        println!("MEMBERS {members:?}");

        assert_eq!(members.len(), 3);
        assert!(members[0].id == 1);
        Ok(())
    }
}
