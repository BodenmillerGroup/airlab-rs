use crate::ctx::Ctx;
use crate::model::conjugate::{Conjugate, ConjugateBmc, ConjugateFilter};
use crate::model::lot::LotFilter;
use crate::model::member::{Member, MemberBmc, MemberFilter};
use crate::model::tag::{Tag, TagBmc, TagFilter};
use crate::model::user::{User, UserBmc};
use crate::model::view_lot::{ViewLot, ViewLotBmc};
use crate::model::ModelManager;
use crate::model::Result;
use modql::field::Fields;
use modql::filter::ListOptions;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ViewConjugate {
    pub id: i32,
    #[serde(rename = "createdBy")]
    pub created_by: i32,
    #[serde(rename = "lotId")]
    pub lot_id: i32,
    #[serde(rename = "tagId")]
    pub tag_id: i32,
    #[serde(rename = "labeledBy")]
    pub labeled_by: Option<i32>,
    #[serde(rename = "finishedBy")]
    pub finished_by: Option<i32>,
    pub status: i16,
    #[serde(rename = "tubeNumber")]
    pub tube_number: i32,
    pub concentration: Option<f32>,
    pub description: Option<String>,
    #[serde(rename = "isArchived")]
    pub is_archived: Option<bool>,
    //pub meta: Option<String>,
    #[serde(rename = "finishedAt")]
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "customId")]
    pub custom_id: Option<String>,
    pub tag: Tag,
    pub lot: ViewLot,
    pub validations: Vec<String>,
    pub user: User,
}

#[derive(Serialize, FromRow, Fields, Deserialize, Debug, Clone, Default)]
pub struct ViewConjugateForLot {
    pub id: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "createdBy")]
    pub created_by: i32,
    #[serde(rename = "lotId")]
    pub lot_id: i32,
    #[serde(rename = "tagId")]
    pub tag_id: i32,
    #[serde(rename = "labeledBy")]
    pub labeled_by: Option<i32>,
    #[serde(rename = "finishedBy")]
    pub finished_by: Option<i32>,
    pub status: i16,
    #[serde(rename = "tubeNumber")]
    pub tube_number: i32,
    pub concentration: Option<f32>,
    pub description: Option<String>,
    #[serde(rename = "isArchived")]
    pub is_archived: Option<bool>,
    //pub meta: Option<String>,
    #[serde(rename = "finishedAt")]
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "customId")]
    pub custom_id: Option<String>,
    pub meta: Option<serde_json::Value>,
    pub tag: Option<serde_json::Value>,
    pub user: Option<serde_json::Value>,
    pub lot: Option<serde_json::Value>,
}

pub struct ViewConjugateBmc;

impl ViewConjugateBmc {
    pub async fn list_for_lot(mm: &ModelManager, lot_id: i32) -> Result<Vec<ViewConjugateForLot>> {
        let stmt = r#"SELECT
  c.id,
  c.group_id,
  c.created_by,
  c.lot_id,
  c.tag_id,
  c.labeled_by,
  c.finished_by,
  c.status,
  c.tube_number,
  c.concentration,
  c.description,
  c.is_archived,
  c.finished_at,
  c.created_at,
  c.updated_at,
  c.custom_id,
  c.meta,
  jsonb_build_object(
    'id', tag.id,
    'name', tag.name,
    'mw', tag.mw
  ) AS tag,
  jsonb_build_object(
   'id', lot.id,
   'number', lot.number,
   'clone', jsonb_build_object(
     'id', cl.id,
     'name', cl.name,
     'isPhospho', cl.is_phospho,
     'protein', jsonb_build_object(
       'id', protein.id,
       'name', protein.name
     )
   )
  ) AS lot,
  jsonb_build_object(
    'id', usr.id,
    'name', usr.name,
    'isAdmin', usr.is_admin
  ) AS user
FROM
  conjugate c
LEFT JOIN
    tag ON c.tag_id = tag.id
LEFT JOIN
    member m ON c.created_by = m.id
LEFT JOIN
    "user" usr ON m.user_id = usr.id
LEFT JOIN
    lot ON c.lot_id = lot.id
LEFT JOIN
    clone cl ON lot.clone_id = cl.id
LEFT JOIN
    protein ON cl.protein_id = protein.id
WHERE
  lot_id = $1"#;
        let ret: Vec<ViewConjugateForLot> = sqlx::query_as::<_, ViewConjugateForLot>(stmt)
            .bind(lot_id)
            .fetch_all(&mm.db)
            .await?;
        Ok(ret)
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        group_id: Option<i32>,
        filters: Option<Vec<ConjugateFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<ViewConjugate>> {
        let mut tag_map = HashMap::new();
        let mut lot_map = HashMap::new();
        let mut user_map = HashMap::new();
        if let Some(group_id) = group_id {
            let tag_filters: Option<Vec<TagFilter>> = match serde_json::from_value(json!([
                {
                    "group_id": {"$eq":group_id}
                }
            ])) {
                Ok(ok) => Some(ok),
                Err(_) => None,
            };
            let op = ListOptions {
                limit: Some(10_000),
                ..Default::default()
            };
            let tags: Vec<Tag> = TagBmc::list(ctx, mm, tag_filters, Some(op)).await?;
            for tag in tags {
                tag_map.insert(tag.id, tag);
            }

            let lot_filters: Option<Vec<LotFilter>> = match serde_json::from_value(json!([
                {
                    "group_id": {"$eq":group_id}
                }
            ])) {
                Ok(ok) => Some(ok),
                Err(_) => None,
            };
            let op = ListOptions {
                limit: Some(10_000),
                ..Default::default()
            };
            let lots: Vec<ViewLot> =
                ViewLotBmc::list(ctx, mm, Some(group_id), lot_filters, Some(op)).await?;
            for lot in lots {
                lot_map.insert(lot.id, lot);
            }

            let mem_op = ListOptions {
                limit: Some(10_000),
                ..Default::default()
            };
            let mem_filters: Option<Vec<MemberFilter>> =
                match serde_json::from_value(json!([{"group_id": {"$eq": group_id}}])) {
                    Ok(o) => Some(o),
                    Err(_) => None,
                };
            let members: Vec<Member> = MemberBmc::list(ctx, mm, mem_filters, Some(mem_op)).await?;
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
        }

        let conjugates: Vec<Conjugate> = ConjugateBmc::list(ctx, mm, filters, list_options).await?;
        let mut returns = vec![];
        for item in conjugates {
            let tag = match tag_map.get(&{ item.tag_id }) {
                Some(v) => v.clone(),
                None => Tag::default(),
            };
            let lot = match lot_map.get(&{ item.lot_id }) {
                Some(v) => v.clone(),
                None => ViewLot::default(),
            };

            let user = match user_map.get(&item.created_by) {
                Some(v) => v.clone(),
                None => User::default(),
            };

            returns.push(ViewConjugate {
                id: item.id,
                created_by: item.created_by,
                lot_id: item.lot_id,
                tag_id: item.tag_id,
                labeled_by: item.labeled_by,
                finished_by: item.finished_by,
                status: item.status,
                tube_number: item.tube_number,
                concentration: item.concentration,
                description: item.description,
                is_archived: item.is_archived,
                finished_at: item.finished_at,
                updated_at: item.updated_at,
                created_at: item.created_at,
                custom_id: item.custom_id,
                validations: vec![],
                tag,
                lot,
                user,
            });
        }

        Ok(returns)
    }
}
