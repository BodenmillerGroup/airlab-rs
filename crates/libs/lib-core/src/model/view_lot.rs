use crate::ctx::Ctx;
use crate::model::clone::CloneFilter;
use crate::model::lot::{Lot, LotBmc, LotFilter};
use crate::model::provider::{Provider, ProviderBmc, ProviderFilter};
use crate::model::validation::Validation;
use crate::model::validation::{ValidationBmc, ValidationFilter};
use crate::model::view_clone::{ViewClone, ViewCloneBmc};
use crate::model::ModelManager;
use crate::model::Result;
use modql::field::Fields;
use modql::filter::ListOptions;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;
use std::collections::{hash_map::Entry, HashMap};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ViewLot {
    pub id: i32,
    #[serde(rename = "createdBy")]
    pub created_by: Option<i32>,
    pub storage: String,
    #[serde(rename = "cloneId")]
    pub clone_id: i32,
    #[serde(rename = "providerId")]
    pub provider_id: Option<i32>,
    pub name: String,
    pub reference: Option<String>,
    #[serde(rename = "requestedBy")]
    pub requested_by: Option<i32>,
    #[serde(rename = "approvedBy")]
    pub approved_by: Option<i32>,
    #[serde(rename = "orderedBy")]
    pub ordered_by: Option<i32>,
    #[serde(rename = "receivedBy")]
    pub received_by: Option<i32>,
    #[serde(rename = "finishedBy")]
    pub finished_by: Option<i32>,
    pub status: Option<i16>,
    pub purpose: Option<String>,
    pub number: Option<String>,
    pub url: Option<String>,
    pub price: Option<String>,
    pub note: Option<String>,
    #[serde(rename = "requestedAt")]
    pub requested_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "orderedAt")]
    pub ordered_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "receivedAt")]
    pub received_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "finishedAt")]
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    pub clone: ViewClone,
    pub validations: Vec<Validation>,
    pub provider: Option<Provider>,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default)]
pub struct ViewLotDetails {
    pub id: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "createdBy")]
    pub created_by: i32,
    #[serde(rename = "cloneId")]
    pub clone_id: i32,
    #[serde(rename = "providerId")]
    pub provider_id: i32,
    pub name: Option<String>,
    pub reference: Option<String>,
    #[serde(rename = "requestedBy")]
    pub requested_by: Option<i32>,
    #[serde(rename = "approvedBy")]
    pub approved_by: Option<i32>,
    #[serde(rename = "orderedBy")]
    pub ordered_by: Option<i32>,
    #[serde(rename = "receivedBy")]
    pub received_by: Option<i32>,
    #[serde(rename = "finishedBy")]
    pub finished_by: Option<i32>,
    pub number: Option<String>,
    pub status: Option<i16>,
    pub purpose: Option<String>,
    pub url: Option<String>,
    pub price: Option<String>,
    pub note: Option<String>,
    #[serde(rename = "requestedAt")]
    pub requested_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "orderedAt")]
    pub ordered_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "receivedAt")]
    pub received_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "finishedAt")]
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    pub meta: Option<serde_json::Value>,
    pub clone: Option<serde_json::Value>,
    pub provider: Option<serde_json::Value>,
    #[serde(rename = "requestedByUser")]
    pub requested_by_user: Option<serde_json::Value>,
    #[serde(rename = "approvedByUser")]
    pub approved_by_user: Option<serde_json::Value>,
    #[serde(rename = "orderedByUser")]
    pub ordered_by_user: Option<serde_json::Value>,
    #[serde(rename = "receivedByUser")]
    pub received_by_user: Option<serde_json::Value>,
    #[serde(rename = "finishedByUser")]
    pub finished_by_user: Option<serde_json::Value>,
}

pub struct ViewLotBmc;

impl ViewLotBmc {
    pub async fn get_details(_ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<ViewLotDetails> {
        let stmt = r#"SELECT
  lot.id,
  lot.group_id,
  lot.created_by,
  lot.clone_id,
  lot.provider_id,
  lot.name,
  lot.reference,
  lot.requested_by,
  lot.approved_by,
  lot.ordered_by,
  lot.received_by,
  lot.finished_by,
  lot.number,
  lot.status,
  lot.purpose,
  lot.url,
  lot.price,
  lot.note,
  lot.requested_at,
  lot.ordered_at,
  lot.received_at,
  lot.finished_at,
  lot.meta,
  jsonb_build_object(
    'id', reu.id,
    'name', reu.name,
    'isAdmin', reu.is_admin
  ) AS requested_by_user,
  jsonb_build_object(
    'id', apu.id,
    'name', apu.name,
    'isAdmin', apu.is_admin
  ) AS approved_by_user,
  jsonb_build_object(
    'id', oru.id,
    'name', oru.name,
    'isAdmin', oru.is_admin
  ) AS ordered_by_user,
  jsonb_build_object(
    'id', rcu.id,
    'name', rcu.name,
    'isAdmin', rcu.is_admin
  ) AS received_by_user,
  jsonb_build_object(
    'id', fiu.id,
    'name', fiu.name,
    'isAdmin', fiu.is_admin
  ) AS finished_by_user,
  jsonb_build_object(
    'id', cl.id,
    'name', cl.name
  ) AS clone,
  jsonb_build_object(
    'id', pv.id,
    'name', pv.name
  ) AS provider
FROM
  lot
LEFT JOIN 
    member rem ON lot.requested_by = rem.id
LEFT JOIN 
    "user" reu ON rem.user_id = reu.id
LEFT JOIN 
    member apm ON lot.approved_by = apm.id
LEFT JOIN 
    "user" apu ON apm.user_id = apu.id
LEFT JOIN 
    member orm ON lot.ordered_by = orm.id
LEFT JOIN 
    "user" oru ON orm.user_id = oru.id
LEFT JOIN 
    member rcm ON lot.received_by = rcm.id
LEFT JOIN 
    "user" rcu ON rcm.user_id = rcu.id
LEFT JOIN 
    member fim ON lot.finished_by = fim.id
LEFT JOIN 
    "user" fiu ON fim.user_id = fiu.id
LEFT JOIN 
    clone cl ON lot.clone_id = cl.id
LEFT JOIN 
    provider pv ON lot.provider_id = pv.id
WHERE
  lot.id = $1"#;
        let item: ViewLotDetails = sqlx::query_as::<_, ViewLotDetails>(stmt)
            .bind(id)
            .fetch_one(&mm.db)
            .await?;
        Ok(item)
    }
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<ViewLot> {
        let item = LotBmc::get(ctx, mm, id).await?;
        let clone = ViewCloneBmc::get(ctx, mm, item.clone_id).await?;
        let provider = match item.provider_id {
            Some(pid) => Some(ProviderBmc::get(ctx, mm, pid).await?),
            None => None,
        };
        let ret = ViewLot {
            id: item.id,
            created_by: item.created_by,
            storage: "storage_const".into(),
            clone_id: item.clone_id,
            provider_id: item.provider_id,
            name: item.name,
            reference: item.reference,
            requested_by: item.requested_by,
            approved_by: item.approved_by,
            ordered_by: item.ordered_by,
            received_by: item.received_by,
            finished_by: item.finished_by,
            status: item.status,
            purpose: item.purpose,
            number: item.number,
            url: item.url,
            price: item.price,
            note: item.note,
            requested_at: item.requested_at,
            ordered_at: item.ordered_at,
            received_at: item.received_at,
            finished_at: item.finished_at,
            clone,
            validations: vec![],
            provider,
        };
        Ok(ret)
    }
    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        group_id: Option<i32>,
        filters: Option<Vec<LotFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<ViewLot>> {
        let mut clone_map = HashMap::new();
        let mut provider_map = HashMap::new();
        let mut validation_map = HashMap::new();
        if let Some(group_id) = group_id {
            let clone_filters: Option<Vec<CloneFilter>> = match serde_json::from_value(json!([
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
            let clones: Vec<ViewClone> =
                ViewCloneBmc::list(ctx, mm, Some(group_id), clone_filters, Some(op)).await?;
            for clone in clones {
                clone_map.insert(clone.id, clone);
            }

            let provider_filters: Option<Vec<ProviderFilter>> =
                match serde_json::from_value(json!([
                    {
                        "group_id": {"$eq":group_id}
                    }
                ])) {
                    Ok(ok) => Some(ok),
                    Err(_) => None,
                };
            let providers: Vec<Provider> =
                ProviderBmc::list(ctx, mm, provider_filters, None).await?;
            for provider in providers {
                provider_map.insert(provider.id, provider);
            }
            let validation_op = ListOptions {
                limit: Some(100_000),
                ..Default::default()
            };

            let validation_filters: Option<Vec<ValidationFilter>> =
                match serde_json::from_value(json!([
                    {
                        "group_id": {"$eq":group_id}
                    }
                ])) {
                    Ok(ok) => Some(ok),
                    Err(_) => None,
                };
            let validations: Vec<Validation> =
                ValidationBmc::list(ctx, mm, validation_filters, Some(validation_op)).await?;
            for validation in validations {
                let h = match validation_map.entry(validation.lot_id.unwrap_or(0)) {
                    Entry::Occupied(o) => o.into_mut(),
                    Entry::Vacant(v) => v.insert(vec![]),
                };
                h.push(validation);
            }
        }

        let lots: Vec<Lot> = LotBmc::list(ctx, mm, filters, list_options).await?;
        let mut returns = vec![];
        for item in lots {
            let clone = match clone_map.get(&{ item.clone_id }) {
                Some(v) => v.clone(),
                None => ViewClone::default(),
            };
            let mut provider = None;

            if let Some(provider_id) = &item.provider_id {
                provider = provider_map.get(provider_id).cloned();
            }

            returns.push(ViewLot {
                id: item.id,
                storage: "storage_const".into(),
                created_by: item.created_by,
                clone_id: item.clone_id,
                provider_id: item.provider_id,
                name: item.name,
                reference: item.reference,
                requested_by: item.requested_by,
                approved_by: item.approved_by,
                ordered_by: item.ordered_by,
                received_by: item.received_by,
                finished_by: item.finished_by,
                status: item.status,
                purpose: item.purpose,
                number: item.number,
                url: item.url,
                price: item.price,
                note: item.note,
                requested_at: item.requested_at,
                ordered_at: item.ordered_at,
                received_at: item.received_at,
                finished_at: item.finished_at,
                clone,
                validations: validation_map.get(&item.id).unwrap_or(&vec![]).clone(),
                provider,
            });
        }

        Ok(returns)
    }
}
