use crate::ctx::Ctx;
use crate::model::member::{Member, MemberBmc};
use crate::model::panel::{Panel, PanelBmc, PanelFilter};
use crate::model::panel_element::{PanelElement, PanelElementBmc};
use crate::model::user::{User, UserBmc};
use crate::model::ModelManager;
use crate::model::Result;
use modql::filter::{ListOptions, OrderBy, OrderBys};
use serde::{Deserialize, Serialize};
use std::collections::{hash_map::Entry, HashMap};
use tracing::debug;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ViewPanel {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub application: Option<i32>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "createdBy")]
    pub created_by: i32,
    #[serde(rename = "isArchived")]
    pub is_archived: bool,
    #[serde(rename = "isFluorophore")]
    pub is_fluorophore: bool,
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user: User,
    pub elements: Vec<PanelElement>,
}

pub struct ViewPanelBmc;

impl ViewPanelBmc {
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<ViewPanel> {
        let mut element_map = HashMap::new();
        let element_options = ListOptions {
            limit: Some(1_000_000),
            offset: None,
            order_bys: Some(OrderBys::new(vec![OrderBy::Desc("id".into())])),
        };
        let elements: Vec<PanelElement> =
            PanelElementBmc::list(ctx, mm, None, Some(element_options)).await?;
        for element in elements {
            let toi = match element_map.entry(element.panel_id) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => v.insert(vec![]),
            };
            toi.push(element);
        }

        let mut member_map = HashMap::new();
        let members: Vec<Member> = MemberBmc::list(ctx, mm, None, None).await?;
        for member in members {
            member_map.insert(member.id, member);
        }

        let mut user_map = HashMap::new();
        let users: Vec<User> = UserBmc::list(ctx, mm, None, None).await?;
        for user in users {
            user_map.insert(user.id, user);
        }

        let item: Panel = PanelBmc::get(ctx, mm, id).await?;
        let elements = match element_map.get(&(item.id as i32)) {
            Some(v) => v.clone(),
            None => vec![],
        };

        let member = match member_map.get(&(item.created_by as i32)) {
            Some(v) => v.clone(),
            None => Member::default(),
        };
        debug!("member: {:?}", member);

        let user = match user_map.get(&(member.user_id as i32)) {
            Some(v) => v.clone(),
            None => User::default(),
        };
        debug!("user: {:?}", user);

        let ret = ViewPanel {
            id: item.id,
            name: item.name,
            application: item.application,
            created_at: item.created_at,
            created_by: item.created_by,
            description: item.description,
            is_archived: item.is_archived,
            is_fluorophore: item.is_fluorophore,
            updated_at: item.updated_at,
            is_locked: item.is_locked,
            elements,
            user,
        };

        Ok(ret)
    }
    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<PanelFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<ViewPanel>> {
        let mut element_map = HashMap::new();
        let element_options = ListOptions {
            limit: Some(1_000_000),
            offset: None,
            order_bys: Some(OrderBys::new(vec![OrderBy::Desc("id".into())])),
        };
        let elements: Vec<PanelElement> =
            PanelElementBmc::list(ctx, mm, None, Some(element_options)).await?;
        for element in elements {
            let toi = match element_map.entry(element.panel_id) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => v.insert(vec![]),
            };
            toi.push(element);
        }

        let mut member_map = HashMap::new();
        let members: Vec<Member> = MemberBmc::list(ctx, mm, None, None).await?;
        for member in members {
            member_map.insert(member.id, member);
        }

        let mut user_map = HashMap::new();
        let users: Vec<User> = UserBmc::list(ctx, mm, None, None).await?;
        for user in users {
            user_map.insert(user.id, user);
        }

        let panels: Vec<Panel> = PanelBmc::list(ctx, mm, filters, list_options).await?;
        let mut returns = vec![];
        for item in panels {
            let elements = match element_map.get(&{ item.id }) {
                Some(v) => v.clone(),
                None => vec![],
            };

            let member = match member_map.get(&{ item.created_by }) {
                Some(v) => v.clone(),
                None => Member::default(),
            };

            let user = match user_map.get(&{ member.user_id }) {
                Some(v) => v.clone(),
                None => User::default(),
            };

            returns.push(ViewPanel {
                id: item.id,
                name: item.name,
                application: item.application,
                created_at: item.created_at,
                created_by: item.created_by,
                description: item.description,
                is_archived: item.is_archived,
                is_fluorophore: item.is_fluorophore,
                updated_at: item.updated_at,
                is_locked: item.is_locked,
                elements,
                user,
            });
        }

        Ok(returns)
    }
}
