use crate::ctx::Ctx;
use crate::model::member::{MemberBmc, MemberFilter};
use crate::model::user::{User, UserBmc};
use crate::model::ModelManager;
use crate::model::Result;
use modql::filter::ListOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct ViewMember {
    pub id: i32,
    pub group_id: i32,
    pub user_id: i32,
    pub created_at: chrono::NaiveDateTime,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "allPanels")]
    pub all_panels: bool,
    pub role: i16,
    pub updated_at: chrono::NaiveDateTime,
    pub url: Option<String>,
    pub user: User,
}

pub struct ViewMemberBmc;

impl ViewMemberBmc {
    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<MemberFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<ViewMember>> {
        let members = MemberBmc::list(ctx, mm, filters, list_options).await?;
        let mut returns = vec![];
        let mut user_map = HashMap::new();
        let mem_map: HashMap<i32, i32> = members.iter().map(|e| (e.user_id, e.id)).collect();

        let op = ListOptions {
            limit: Some(10_000),
            ..Default::default()
        };
        let users: Vec<User> = UserBmc::list(ctx, mm, None, Some(op)).await?;
        for user in users {
            let member_id = mem_map.get(&user.id).unwrap_or(&0);
            user_map.insert(*member_id, user);
        }
        for member in members {
            let user = match user_map.get(&member.id) {
                Some(v) => v.clone(),
                None => User::default(),
            };
            returns.push(ViewMember {
                id: member.id,
                group_id: member.group_id,
                created_at: member.created_at.naive_utc(),
                is_active: member.is_active,
                all_panels: member.all_panels,
                role: member.role,
                updated_at: member.updated_at.naive_utc(),
                user_id: member.user_id,
                url: None,
                user,
            });
        }

        Ok(returns)
    }
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<ViewMember> {
        let member = MemberBmc::get(ctx, mm, id).await?;
        Ok(ViewMember {
            id: member.id,
            group_id: member.group_id,
            created_at: member.created_at.naive_utc(),
            is_active: member.is_active,
            all_panels: member.all_panels,
            role: member.role,
            updated_at: member.updated_at.naive_utc(),
            user_id: member.user_id,
            url: None,
            user: User::default(),
        })
    }
}
