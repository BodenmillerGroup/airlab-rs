use crate::config::web_config;
use crate::search_shadow::SearchState;
use crate::web::Result;
use crate::web::mw_auth::CtxW;
use Operation as Op;
use ReturnType as RT;
use airlab_lib::ctx::Ctx;
use airlab_lib::model::ModelManager as MM;
use airlab_lib::model::clone::{Clone, CloneBmc, CloneFilter, CloneForCreate, CloneForUpdate};
use airlab_lib::model::collection::{
    Collection, CollectionBmc, CollectionFilter, CollectionForCreate, CollectionForUpdate,
};
use airlab_lib::model::conjugate::{
    Conjugate, ConjugateBmc, ConjugateFilter, ConjugateForCreate, ConjugateForUpdate,
};
use airlab_lib::model::group::{Group, GroupBmc, GroupFilter, GroupForCreate, GroupForUpdate};
use airlab_lib::model::lot::{Lot, LotBmc, LotFilter, LotForCreate, LotForUpdate};
use airlab_lib::model::member::{
    Member, MemberBmc, MemberFilter, MemberForCreate, MemberForUpdate,
};
use airlab_lib::model::panel::{
    ElementUpdate, Panel, PanelBmc, PanelFilter, PanelForCreate, PanelForUpdate,
    PanelPayloadForUpdate,
};
use airlab_lib::model::panel_element::{
    PanelElement, PanelElementBmc, PanelElementFilter, PanelElementForCreate, PanelElementForUpdate,
};
use airlab_lib::model::protein::{
    Protein, ProteinBmc, ProteinFilter, ProteinForCreate, ProteinForUpdate,
};
use airlab_lib::model::provider::{
    Provider, ProviderBmc, ProviderFilter, ProviderForCreate, ProviderForUpdate,
};
use airlab_lib::model::species::{
    Species, SpeciesBmc, SpeciesFilter, SpeciesForCreate, SpeciesForUpdate,
};
use airlab_lib::model::storage::{
    Storage, StorageBmc, StorageFilter, StorageForCreate, StorageForUpdate,
};
use airlab_lib::model::tag::{Tag, TagBmc, TagFilter, TagForCreate, TagForUpdate};
use airlab_lib::model::user::{User, UserBmc, UserFilter, UserForCreate, UserForUpdate};
use airlab_lib::model::validation::{
    Validation, ValidationBmc, ValidationFilter, ValidationForCreate, ValidationForUpdate,
};
use airlab_lib::model::validation_file::{
    ValidationFile, ValidationFileBmc, ValidationFileFilter, ValidationFileForCreate,
    ValidationFileForUpdate,
};
use axum::extract::{Json as eJson, State};
use axum::routing::post;
use axum::{Json, Router};
use camino::Utf8PathBuf;
use chrono::Utc;
use modql::filter::{ListOptions as LO, OrderBy, OrderBys};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use sqlx::query;
use std::collections::{HashMap, HashSet};
use tokio::fs;
#[allow(unused_imports)]
use tracing::{debug, info, warn};

pub fn routes(state: SearchState) -> Router {
    Router::new()
        .route("/api/v1/json", post(api_post_json_handler))
        .with_state(state)
}

async fn api_post_json_handler(
    State(state): State<SearchState>,
    ctx: CtxW,
    eJson(req): eJson<RpcRequest>,
) -> Result<Json<Value>> {
    let mm = state.mm.clone();
    info!("HANDLER - api_post_json_handler: {:?}", req);
    let ctx = ctx.0;
    let filter_json = filters_to_json_array(req.filters);
    let offset = req
        .limit
        .zip(req.page)
        .map(|(limit, page)| (page - 1) * limit);

    let lo = LO {
        limit: req.limit,
        offset,
        order_bys: Some(OrderBys::new(vec![OrderBy::Desc("id".into())])),
    };
    let should_rebuild_shadows = matches!(
        req.operation,
        Op::Insert | Op::Update | Op::Delete | Op::Reorder
    );

    let result = match (req.return_type, req.operation) {
        (RT::Clone, Op::Get) => get_clones(&ctx, &mm, filter_json, lo).await?,
        (RT::Clone, Op::Update) => update_clone(&ctx, &mm, req.id, req.payload).await?,
        (RT::Clone, Op::Insert) => insert_clone(&ctx, &mm, req.payload).await?,
        (RT::Clone, Op::Delete) => delete_clone(&ctx, &mm, req.id).await?,
        (RT::Species, Op::Get) => get_species(&ctx, &mm, filter_json, lo).await?,
        (RT::Species, Op::Update) => update_species(&ctx, &mm, req.id, req.payload).await?,
        (RT::Species, Op::Insert) => insert_species(&ctx, &mm, req.payload).await?,
        (RT::Species, Op::Delete) => delete_species(&ctx, &mm, req.id).await?,
        (RT::Protein, Op::Get) => get_proteins(&ctx, &mm, filter_json, lo).await?,
        (RT::Protein, Op::Update) => update_protein(&ctx, &mm, req.id, req.payload).await?,
        (RT::Protein, Op::Insert) => insert_protein(&ctx, &mm, req.payload).await?,
        (RT::Protein, Op::Delete) => delete_protein(&ctx, &mm, req.id).await?,
        (RT::Conjugate, Op::Get) => get_conjugates(&ctx, &mm, filter_json, lo).await?,
        (RT::Conjugate, Op::Update) => update_conjugate(&ctx, &mm, req.id, req.payload).await?,
        (RT::Conjugate, Op::Insert) => insert_conjugate(&ctx, &mm, req.payload).await?,
        (RT::Conjugate, Op::Delete) => delete_conjugate(&ctx, &mm, req.id).await?,
        (RT::Tag, Op::Get) => get_tags(&ctx, &mm, filter_json, lo).await?,
        (RT::Tag, Op::Update) => update_tag(&ctx, &mm, req.id, req.payload).await?,
        (RT::Tag, Op::Insert) => insert_tag(&ctx, &mm, req.payload).await?,
        (RT::Tag, Op::Delete) => delete_tag(&ctx, &mm, req.id).await?,
        (RT::Lot, Op::Get) => get_lots(&ctx, &mm, filter_json, lo).await?,
        (RT::Lot, Op::Update) => update_lot(&ctx, &mm, req.id, req.payload).await?,
        (RT::Lot, Op::Reorder) => reorder_lot(&ctx, &mm, req.id, req.payload).await?,
        (RT::Lot, Op::Insert) => insert_lot(&ctx, &mm, req.payload).await?,
        (RT::Lot, Op::Delete) => delete_lot(&ctx, &mm, req.id).await?,
        (RT::Validation, Op::Get) => get_validations(&ctx, &mm, filter_json, lo).await?,
        (RT::Validation, Op::Update) => update_validation(&ctx, &mm, req.id, req.payload).await?,
        (RT::Validation, Op::Insert) => insert_validation(&ctx, &mm, req.payload).await?,
        (RT::Validation, Op::Delete) => delete_validation(&ctx, &mm, req.id).await?,
        (RT::ValidationFile, Op::Get) => get_validation_files(&ctx, &mm, filter_json, lo).await?,
        (RT::ValidationFile, Op::Update) => {
            update_validation_file(&ctx, &mm, req.id, req.payload).await?
        }
        (RT::ValidationFile, Op::Insert) => insert_validation_file(&ctx, &mm, req.payload).await?,
        (RT::ValidationFile, Op::Delete) => delete_validation_file(&ctx, &mm, req.id).await?,
        (RT::Provider, Op::Get) => get_providers(&ctx, &mm, filter_json, lo).await?,
        (RT::Provider, Op::Update) => update_provider(&ctx, &mm, req.id, req.payload).await?,
        (RT::Provider, Op::Insert) => insert_provider(&ctx, &mm, req.payload).await?,
        (RT::Provider, Op::Delete) => delete_provider(&ctx, &mm, req.id).await?,
        (RT::Storage, Op::Get) => get_storages(&ctx, &mm, filter_json, lo).await?,
        (RT::Storage, Op::Update) => update_storage(&ctx, &mm, req.id, req.payload).await?,
        (RT::Storage, Op::Insert) => insert_storage(&ctx, &mm, req.payload).await?,
        (RT::Storage, Op::Delete) => delete_storage(&ctx, &mm, req.id).await?,
        (RT::Collection, Op::Get) => get_collections(&ctx, &mm, filter_json, lo).await?,
        (RT::Collection, Op::Update) => update_collection(&ctx, &mm, req.id, req.payload).await?,
        (RT::Collection, Op::Insert) => insert_collection(&ctx, &mm, req.payload).await?,
        (RT::Collection, Op::Delete) => delete_collection(&ctx, &mm, req.id).await?,
        (RT::Panel, Op::Get) => get_panels(&ctx, &mm, filter_json, lo).await?,
        (RT::Panel, Op::Update) => update_panel(&ctx, &mm, req.id, req.payload).await?,
        (RT::Panel, Op::Insert) => insert_panel(&ctx, &mm, req.payload).await?,
        (RT::Panel, Op::Delete) => delete_panel(&ctx, &mm, req.id).await?,
        (RT::PanelElement, Op::Get) => get_panel_elements(&ctx, &mm, filter_json, lo).await?,
        (RT::PanelElement, Op::Update) => {
            update_panel_element(&ctx, &mm, req.id, req.payload).await?
        }
        (RT::PanelElement, Op::Insert) => insert_panel_element(&ctx, &mm, req.payload).await?,
        (RT::PanelElement, Op::Delete) => delete_panel_element(&ctx, &mm, req.id).await?,
        (RT::User, Op::Get) => get_users(&ctx, &mm, filter_json, lo).await?,
        (RT::User, Op::Update) => update_user(&ctx, &mm, req.id, req.payload).await?,
        (RT::User, Op::Insert) => insert_user(&ctx, &mm, req.payload).await?,
        (RT::User, Op::Delete) => delete_user(&ctx, &mm, req.id).await?,
        (RT::Member, Op::Get) => get_members(&ctx, &mm, filter_json, lo).await?,
        (RT::Member, Op::Update) => update_member(&ctx, &mm, req.id, req.payload).await?,
        (RT::Member, Op::Insert) => insert_member(&ctx, &mm, req.payload).await?,
        (RT::Member, Op::Delete) => delete_member(&ctx, &mm, req.id).await?,
        (RT::Group, Op::Get) => get_groups(&ctx, &mm, filter_json, lo).await?,
        (RT::Group, Op::Update) => update_group(&ctx, &mm, req.id, req.payload).await?,
        (RT::Group, Op::Insert) => insert_group(&ctx, &mm, req.payload).await?,
        (RT::Group, Op::Delete) => delete_group(&ctx, &mm, req.id).await?,
        (rt, Op::Reorder) => {
            warn!("Unsupported Reorder operation for return_type: {:?}", rt);
            json!({})
        }
    };
    if should_rebuild_shadows {
        state.registry.rebuild_all(&mm).await?;
    }
    Ok(Json(result))
}

async fn delete_clone(ctx: &Ctx, mm: &MM, id: Option<i64>) -> Result<serde_json::Value> {
    if let Some(id) = id {
        CloneBmc::delete(ctx, mm, id).await?;
    }
    Ok(json!({}))
}

async fn update_clone(
    ctx: &Ctx,
    mm: &MM,
    id: Option<i64>,
    payload: Option<Value>,
) -> Result<serde_json::Value> {
    if let (Some(id), Some(payload)) = (id, payload) {
        let fu: CloneForUpdate = payload.into();
        warn!("UPDATE: {fu:?}");
        CloneBmc::update(ctx, mm, id, fu).await?;
    }
    Ok(json!({}))
}

async fn insert_clone(ctx: &Ctx, mm: &MM, payload: Option<Value>) -> Result<serde_json::Value> {
    let mut id = 0;
    if let Some(payload) = payload {
        let fc: CloneForCreate = payload.into();
        warn!("UPDATE: {fc:?}");
        id = CloneBmc::create(ctx, mm, fc).await?;
    }
    Ok(json!({"id": id}))
}

async fn get_proteins(ctx: &Ctx, mm: &MM, filter_json: Value, lo: LO) -> Result<serde_json::Value> {
    let filters: Vec<ProteinFilter> = serde_json::from_value(filter_json)?;
    let total = ProteinBmc::count(ctx, mm, Some(filters.clone())).await?;
    let mut ret = PaginatedResponse::<Protein>::from_lo(&lo, total);
    ret.items = ProteinBmc::list(ctx, mm, Some(filters), Some(lo)).await?;
    Ok(json!(ret))
}

async fn delete_protein(ctx: &Ctx, mm: &MM, id: Option<i64>) -> Result<serde_json::Value> {
    if let Some(id) = id {
        ProteinBmc::delete(ctx, mm, id).await?;
    }
    Ok(json!({}))
}

async fn update_protein(
    ctx: &Ctx,
    mm: &MM,
    id: Option<i64>,
    payload: Option<Value>,
) -> Result<serde_json::Value> {
    if let (Some(id), Some(payload)) = (id, payload) {
        let fu: ProteinForUpdate = payload.into();
        warn!("UPDATE: {fu:?}");
        ProteinBmc::update(ctx, mm, id, fu).await?;
    }
    Ok(json!({}))
}

async fn insert_protein(ctx: &Ctx, mm: &MM, payload: Option<Value>) -> Result<serde_json::Value> {
    let mut id = 0;
    if let Some(payload) = payload {
        let fc: ProteinForCreate = payload.into();
        warn!("UPDATE: {fc:?}");
        id = ProteinBmc::create(ctx, mm, fc).await?;
    }
    Ok(json!({"id": id}))
}

async fn get_validations(
    ctx: &Ctx,
    mm: &MM,
    filter_json: Value,
    lo: LO,
) -> Result<serde_json::Value> {
    let filters: Vec<ValidationFilter> = serde_json::from_value(filter_json)?;
    let total = ValidationBmc::count(ctx, mm, Some(filters.clone())).await?;
    let mut ret = PaginatedResponse::<Validation>::from_lo(&lo, total);
    ret.items = ValidationBmc::list(ctx, mm, Some(filters), Some(lo)).await?;
    Ok(json!(ret))
}

async fn delete_validation(ctx: &Ctx, mm: &MM, id: Option<i64>) -> Result<serde_json::Value> {
    if let Some(id) = id {
        ValidationBmc::delete(ctx, mm, id).await?;
    }
    Ok(json!({}))
}

async fn update_validation(
    ctx: &Ctx,
    mm: &MM,
    id: Option<i64>,
    payload: Option<Value>,
) -> Result<serde_json::Value> {
    if let (Some(id), Some(payload)) = (id, payload) {
        let fu: ValidationForUpdate = payload.into();
        warn!("UPDATE: {fu:?}");
        ValidationBmc::update(ctx, mm, id, fu).await?;
    }
    Ok(json!({}))
}

async fn insert_validation(
    ctx: &Ctx,
    mm: &MM,
    payload: Option<Value>,
) -> Result<serde_json::Value> {
    let mut id = 0;
    if let Some(payload) = payload {
        let fc: ValidationForCreate = payload.into();
        warn!("UPDATE: {fc:?}");
        id = ValidationBmc::create(ctx, mm, fc).await?;
    }
    Ok(json!({"id": id}))
}

async fn get_validation_files(
    ctx: &Ctx,
    mm: &MM,
    filter_json: Value,
    lo: LO,
) -> Result<serde_json::Value> {
    let filters_for_total: Vec<ValidationFileFilter> = serde_json::from_value(filter_json.clone())?;
    let total = ValidationFileBmc::list(ctx, mm, Some(filters_for_total), None)
        .await?
        .len() as i64;
    let filters: Vec<ValidationFileFilter> = serde_json::from_value(filter_json)?;
    let mut ret = PaginatedResponse::<ValidationFile>::from_lo(&lo, total);
    ret.items = ValidationFileBmc::list(ctx, mm, Some(filters), Some(lo)).await?;
    Ok(json!(ret))
}

async fn update_validation_file(
    ctx: &Ctx,
    mm: &MM,
    id: Option<i64>,
    payload: Option<Value>,
) -> Result<serde_json::Value> {
    if let (Some(id), Some(payload)) = (id, payload) {
        let fu: ValidationFileForUpdate = serde_json::from_value(payload)?;
        warn!("UPDATE: {fu:?}");
        ValidationFileBmc::update(ctx, mm, id, fu).await?;
    }
    Ok(json!({}))
}

async fn insert_validation_file(
    ctx: &Ctx,
    mm: &MM,
    payload: Option<Value>,
) -> Result<serde_json::Value> {
    let mut id = 0;
    if let Some(payload) = payload {
        let fc: ValidationFileForCreate = serde_json::from_value(payload)?;
        warn!("UPDATE: {fc:?}");
        id = ValidationFileBmc::create(ctx, mm, fc).await?;
    }
    Ok(json!({"id": id}))
}

async fn delete_validation_file(ctx: &Ctx, mm: &MM, id: Option<i64>) -> Result<serde_json::Value> {
    if let Some(id) = id {
        let validation_file = ValidationFileBmc::get(ctx, mm, id).await?;
        let validation = ValidationBmc::get(ctx, mm, validation_file.validation_id).await?;
        let data_path = web_config()?.DATA_PATH.clone();
        let file_path = Utf8PathBuf::from(format!(
            "{}/groups/{}/uploads/validation/{}/{}.{}",
            data_path,
            validation.group_id,
            validation.id,
            validation_file.hash,
            validation_file.extension
        ));
        if file_path.is_file() {
            fs::remove_file(file_path.as_std_path()).await?;
        }
        ValidationFileBmc::delete(ctx, mm, id).await?;
    }
    Ok(json!({}))
}

async fn get_panel_elements(
    ctx: &Ctx,
    mm: &MM,
    filter_json: Value,
    lo: LO,
) -> Result<serde_json::Value> {
    let filters: Vec<PanelElementFilter> = serde_json::from_value(filter_json)?;
    let total = PanelElementBmc::count(ctx, mm, Some(filters.clone())).await?;
    let mut ret = PaginatedResponse::<PanelElement>::from_lo(&lo, total);
    ret.items = PanelElementBmc::list(ctx, mm, Some(filters), Some(lo)).await?;
    Ok(json!(ret))
}

async fn delete_panel_element(ctx: &Ctx, mm: &MM, id: Option<i64>) -> Result<serde_json::Value> {
    if let Some(id) = id {
        PanelElementBmc::delete(ctx, mm, id).await?;
    }
    Ok(json!({}))
}

async fn update_panel_element(
    ctx: &Ctx,
    mm: &MM,
    id: Option<i64>,
    payload: Option<Value>,
) -> Result<serde_json::Value> {
    if let (Some(id), Some(payload)) = (id, payload) {
        let fu: PanelElementForUpdate = payload.into();
        warn!("UPDATE: {fu:?}");
        update_panel_element_values(ctx, mm, id, fu.dilution_type, fu.concentration).await?;
    }
    Ok(json!({}))
}

async fn insert_panel_element(
    ctx: &Ctx,
    mm: &MM,
    payload: Option<Value>,
) -> Result<serde_json::Value> {
    let mut id = 0;
    if let Some(payload) = payload {
        let fc: PanelElementForCreate = payload.into();
        warn!("UPDATE: {fc:?}");
        id = PanelElementBmc::create(ctx, mm, fc).await?;
    }
    Ok(json!({"id": id}))
}

async fn update_panel_element_values(
    _ctx: &Ctx,
    mm: &MM,
    id: i64,
    dilution_type: i64,
    concentration: Option<f32>,
) -> Result<()> {
    warn!(
        "PANEL_ELEMENT UPDATE id={} dilution_type={} concentration={:?}",
        id, dilution_type, concentration
    );
    let sql = "UPDATE panel_element SET dilution_type = $1, concentration = $2 WHERE id = $3";
    let affected = mm
        .dbx()
        .execute(query(sql).bind(dilution_type).bind(concentration).bind(id))
        .await
        .map_err(airlab_lib::model::Error::Dbx)?;

    if affected == 0 {
        return Err(airlab_lib::model::Error::EntityNotFound {
            entity: "panel_element",
            id,
        }
        .into());
    }

    Ok(())
}

async fn get_panels(ctx: &Ctx, mm: &MM, filter_json: Value, lo: LO) -> Result<serde_json::Value> {
    let filters: Vec<PanelFilter> = serde_json::from_value(filter_json)?;
    let total = PanelBmc::count(ctx, mm, Some(filters.clone())).await?;
    let mut ret = PaginatedResponse::<Panel>::from_lo(&lo, total);
    ret.items = PanelBmc::list(ctx, mm, Some(filters), Some(lo)).await?;
    Ok(json!(ret))
}
async fn delete_panel(ctx: &Ctx, mm: &MM, id: Option<i64>) -> Result<serde_json::Value> {
    if let Some(id) = id {
        PanelBmc::delete(ctx, mm, id).await?;
    }
    Ok(json!({}))
}

async fn update_panel(
    ctx: &Ctx,
    mm: &MM,
    id: Option<i64>,
    payload: Option<Value>,
) -> Result<serde_json::Value> {
    if let (Some(id), Some(payload)) = (id, payload) {
        let elements_present = payload
            .as_object()
            .and_then(|map| map.get("elements"))
            .is_some()
            || payload.is_array();

        let fu: PanelForUpdate = payload.clone().into();
        warn!("UPDATE: {fu:?}");

        mm.dbx()
            .begin_txn()
            .await
            .map_err(airlab_lib::model::Error::Dbx)?;
        let res: Result<()> = async {
            PanelBmc::update(ctx, mm, id, fu).await?;

            if elements_present {
                let elements = if payload.is_array() {
                    serde_json::from_value::<Vec<ElementUpdate>>(payload.clone())?
                } else {
                    let payload_full: PanelPayloadForUpdate =
                        serde_json::from_value(payload.clone())?;
                    payload_full.elements
                };

                let mut incoming_by_conjugate: HashMap<i64, ElementUpdate> = HashMap::new();
                for element in elements {
                    incoming_by_conjugate.insert(element.conjugate_id, element);
                }

                let filters: Vec<PanelElementFilter> = serde_json::from_value(json!([
                    {
                        "panel_id": {"$eq": id}
                    }
                ]))?;
                let existing = PanelElementBmc::list(ctx, mm, Some(filters), None).await?;
                let mut existing_by_conjugate: HashMap<i64, PanelElement> = HashMap::new();
                for element in existing {
                    existing_by_conjugate.insert(element.conjugate_id, element);
                }

                for (conjugate_id, element) in &incoming_by_conjugate {
                    if let Some(existing) = existing_by_conjugate.get(conjugate_id) {
                        update_panel_element_values(
                            ctx,
                            mm,
                            existing.id,
                            element.dilution_type,
                            element.concentration.map(|c| c as f32),
                        )
                        .await?;
                    } else {
                        let create = PanelElementForCreate {
                            panel_id: id,
                            conjugate_id: *conjugate_id,
                            dilution_type: element.dilution_type,
                            concentration: element.concentration.map(|c| c as f32),
                        };
                        PanelElementBmc::create(ctx, mm, create).await?;
                    }
                }

                let incoming_ids: HashSet<i64> = incoming_by_conjugate.keys().copied().collect();
                for (conjugate_id, element) in existing_by_conjugate {
                    if !incoming_ids.contains(&conjugate_id) {
                        PanelElementBmc::delete(ctx, mm, element.id).await?;
                    }
                }
            }

            Ok(())
        }
        .await;

        match res {
            Ok(()) => {
                mm.dbx()
                    .commit_txn()
                    .await
                    .map_err(airlab_lib::model::Error::Dbx)?;
            }
            Err(err) => {
                let _ = mm.dbx().rollback_txn().await;
                return Err(err);
            }
        }
    }
    Ok(json!({}))
}

async fn insert_panel(ctx: &Ctx, mm: &MM, payload: Option<Value>) -> Result<serde_json::Value> {
    let mut id = 0;
    if let Some(payload) = payload {
        let fc: PanelForCreate = payload.into();
        warn!("UPDATE: {fc:?}");
        id = PanelBmc::create(ctx, mm, fc).await?;
    }
    Ok(json!({"id": id}))
}

async fn get_providers(
    ctx: &Ctx,
    mm: &MM,
    filter_json: Value,
    lo: LO,
) -> Result<serde_json::Value> {
    let filters: Vec<ProviderFilter> = serde_json::from_value(filter_json)?;
    let total = ProviderBmc::count(ctx, mm, Some(filters.clone())).await?;
    let mut ret = PaginatedResponse::<Provider>::from_lo(&lo, total);
    ret.items = ProviderBmc::list(ctx, mm, Some(filters), Some(lo)).await?;
    Ok(json!(ret))
}

async fn delete_provider(ctx: &Ctx, mm: &MM, id: Option<i64>) -> Result<serde_json::Value> {
    if let Some(id) = id {
        ProviderBmc::delete(ctx, mm, id).await?;
    }
    Ok(json!({}))
}

async fn update_provider(
    ctx: &Ctx,
    mm: &MM,
    id: Option<i64>,
    payload: Option<Value>,
) -> Result<serde_json::Value> {
    if let (Some(id), Some(payload)) = (id, payload) {
        let fu: ProviderForUpdate = payload.into();
        warn!("UPDATE: {fu:?}");
        ProviderBmc::update(ctx, mm, id, fu).await?;
    }
    Ok(json!({}))
}

async fn insert_provider(ctx: &Ctx, mm: &MM, payload: Option<Value>) -> Result<serde_json::Value> {
    let mut id = 0;
    if let Some(payload) = payload {
        let fc: ProviderForCreate = payload.into();
        warn!("UPDATE: {fc:?}");
        id = ProviderBmc::create(ctx, mm, fc).await?;
    }
    Ok(json!({"id": id}))
}

async fn get_storages(ctx: &Ctx, mm: &MM, filter_json: Value, lo: LO) -> Result<serde_json::Value> {
    let filters: Vec<StorageFilter> = serde_json::from_value(filter_json)?;
    let total = StorageBmc::count(ctx, mm, Some(filters.clone())).await?;
    let mut ret = PaginatedResponse::<Storage>::from_lo(&lo, total);
    ret.items = StorageBmc::list(ctx, mm, Some(filters), Some(lo)).await?;
    Ok(json!(ret))
}

async fn delete_storage(ctx: &Ctx, mm: &MM, id: Option<i64>) -> Result<serde_json::Value> {
    if let Some(id) = id {
        StorageBmc::delete(ctx, mm, id).await?;
    }
    Ok(json!({}))
}

async fn update_storage(
    ctx: &Ctx,
    mm: &MM,
    id: Option<i64>,
    payload: Option<Value>,
) -> Result<serde_json::Value> {
    if let (Some(id), Some(payload)) = (id, payload) {
        let fu: StorageForUpdate = payload.into();
        warn!("UPDATE: {fu:?}");
        StorageBmc::update(ctx, mm, id, fu).await?;
    }
    Ok(json!({}))
}

async fn insert_storage(ctx: &Ctx, mm: &MM, payload: Option<Value>) -> Result<serde_json::Value> {
    let mut id = 0;
    if let Some(payload) = payload {
        let fc: StorageForCreate = payload.into();
        warn!("UPDATE: {fc:?}");
        id = StorageBmc::create(ctx, mm, fc).await?;
    }
    Ok(json!({"id": id}))
}

async fn get_collections(
    ctx: &Ctx,
    mm: &MM,
    filter_json: Value,
    lo: LO,
) -> Result<serde_json::Value> {
    let filters: Vec<CollectionFilter> = serde_json::from_value(filter_json)?;
    let total = CollectionBmc::count(ctx, mm, Some(filters.clone())).await?;
    let mut ret = PaginatedResponse::<Collection>::from_lo(&lo, total);
    ret.items = CollectionBmc::list(ctx, mm, Some(filters), Some(lo)).await?;
    Ok(json!(ret))
}

async fn delete_collection(ctx: &Ctx, mm: &MM, id: Option<i64>) -> Result<serde_json::Value> {
    if let Some(id) = id {
        CollectionBmc::delete(ctx, mm, id).await?;
    }
    Ok(json!({}))
}

async fn update_collection(
    ctx: &Ctx,
    mm: &MM,
    id: Option<i64>,
    payload: Option<Value>,
) -> Result<serde_json::Value> {
    if let (Some(id), Some(payload)) = (id, payload) {
        let fu: CollectionForUpdate = payload.into();
        warn!("UPDATE: {fu:?}");
        CollectionBmc::update(ctx, mm, id, fu).await?;
    }
    Ok(json!({}))
}

async fn insert_collection(
    ctx: &Ctx,
    mm: &MM,
    payload: Option<Value>,
) -> Result<serde_json::Value> {
    let mut id = 0;
    if let Some(payload) = payload {
        let fc: CollectionForCreate = payload.into();
        warn!("UPDATE: {fc:?}");
        id = CollectionBmc::create(ctx, mm, fc).await?;
    }
    Ok(json!({"id": id}))
}

async fn get_tags(ctx: &Ctx, mm: &MM, filter_json: Value, lo: LO) -> Result<serde_json::Value> {
    let filters: Vec<TagFilter> = serde_json::from_value(filter_json)?;
    let total = TagBmc::count(ctx, mm, Some(filters.clone())).await?;
    let mut ret = PaginatedResponse::<Tag>::from_lo(&lo, total);
    ret.items = TagBmc::list(ctx, mm, Some(filters), Some(lo)).await?;
    Ok(json!(ret))
}

async fn delete_tag(ctx: &Ctx, mm: &MM, id: Option<i64>) -> Result<serde_json::Value> {
    if let Some(id) = id {
        TagBmc::delete(ctx, mm, id).await?;
    }
    Ok(json!({}))
}

async fn update_tag(
    ctx: &Ctx,
    mm: &MM,
    id: Option<i64>,
    payload: Option<Value>,
) -> Result<serde_json::Value> {
    if let (Some(id), Some(payload)) = (id, payload) {
        let fu: TagForUpdate = payload.into();
        warn!("UPDATE: {fu:?}");
        TagBmc::update(ctx, mm, id, fu).await?;
    }
    Ok(json!({}))
}

async fn insert_tag(ctx: &Ctx, mm: &MM, payload: Option<Value>) -> Result<serde_json::Value> {
    let mut id = 0;
    if let Some(payload) = payload {
        let fc: TagForCreate = payload.into();
        warn!("UPDATE: {fc:?}");
        id = TagBmc::create(ctx, mm, fc).await?;
    }
    Ok(json!({"id": id}))
}

async fn get_conjugates(
    ctx: &Ctx,
    mm: &MM,
    filter_json: Value,
    lo: LO,
) -> Result<serde_json::Value> {
    let filters: Vec<ConjugateFilter> = serde_json::from_value(filter_json)?;
    let total = ConjugateBmc::count(ctx, mm, Some(filters.clone())).await?;
    let mut ret = PaginatedResponse::<Conjugate>::from_lo(&lo, total);
    ret.items = ConjugateBmc::list(ctx, mm, Some(filters), Some(lo)).await?;
    Ok(json!(ret))
}

async fn delete_conjugate(ctx: &Ctx, mm: &MM, id: Option<i64>) -> Result<serde_json::Value> {
    if let Some(id) = id {
        ConjugateBmc::delete(ctx, mm, id).await?;
    }
    Ok(json!({}))
}

async fn update_conjugate(
    ctx: &Ctx,
    mm: &MM,
    id: Option<i64>,
    payload: Option<Value>,
) -> Result<serde_json::Value> {
    if let (Some(id), Some(payload)) = (id, payload) {
        let fu: ConjugateForUpdate = payload.into();
        warn!("UPDATE: {fu:?}");
        ConjugateBmc::update(ctx, mm, id, fu).await?;
    }
    Ok(json!({}))
}

async fn insert_conjugate(ctx: &Ctx, mm: &MM, payload: Option<Value>) -> Result<serde_json::Value> {
    let mut id = 0;
    if let Some(payload) = payload {
        let fc: ConjugateForCreate = payload.into();
        warn!("UPDATE: {fc:?}");
        id = ConjugateBmc::create(ctx, mm, fc).await?;
    }
    Ok(json!({"id": id}))
}

async fn get_lots(ctx: &Ctx, mm: &MM, filter_json: Value, lo: LO) -> Result<serde_json::Value> {
    let filters: Vec<LotFilter> = serde_json::from_value(filter_json)?;
    let total = LotBmc::count(ctx, mm, Some(filters.clone())).await?;
    let mut ret = PaginatedResponse::<Lot>::from_lo(&lo, total);
    ret.items = LotBmc::list(ctx, mm, Some(filters), Some(lo)).await?;
    Ok(json!(ret))
}

async fn delete_lot(ctx: &Ctx, mm: &MM, id: Option<i64>) -> Result<serde_json::Value> {
    if let Some(id) = id {
        LotBmc::delete(ctx, mm, id).await?;
    }
    Ok(json!({}))
}

async fn update_lot(
    ctx: &Ctx,
    mm: &MM,
    id: Option<i64>,
    payload: Option<Value>,
) -> Result<serde_json::Value> {
    if let (Some(id), Some(payload)) = (id, payload) {
        let fu: LotForUpdate = payload.into();
        warn!("UPDATE: {fu:?}");
        let lot = LotBmc::get(ctx, mm, id).await?;
        let member_id = get_member_id(ctx, mm, lot.group_id, ctx.user_id()).await?;
        LotBmc::update(ctx, mm, id, member_id, fu).await?;
    }
    Ok(json!({}))
}

async fn insert_lot(ctx: &Ctx, mm: &MM, payload: Option<Value>) -> Result<serde_json::Value> {
    let mut id = 0;
    if let Some(payload) = payload {
        let fc: LotForCreate = payload.into();
        warn!("UPDATE: {fc:?}");
        id = LotBmc::create(ctx, mm, fc).await?;
    }
    Ok(json!({"id": id}))
}

#[derive(Deserialize, Debug)]
struct ReorderLotPayload {
    purpose: String,
    #[serde(rename = "requestedBy")]
    requested_by: Option<i64>,
    #[serde(rename = "requestedAt")]
    requested_at: Option<chrono::DateTime<chrono::Utc>>,
}

async fn reorder_lot(
    ctx: &Ctx,
    mm: &MM,
    id: Option<i64>,
    payload: Option<Value>,
) -> Result<serde_json::Value> {
    let Some(lot_id) = id else {
        return Ok(json!({}));
    };
    let Some(payload) = payload else {
        return Ok(json!({}));
    };

    let payload: ReorderLotPayload = serde_json::from_value(payload)?;
    let lot = LotBmc::get(ctx, mm, lot_id).await?;
    let member_id = get_member_id(ctx, mm, lot.group_id, ctx.user_id()).await?;
    let requested_by = payload.requested_by.or(Some(member_id));
    let requested_at = payload.requested_at.or(Some(Utc::now()));

    let new_lot = LotForCreate {
        approved_by: None,
        clone_id: lot.clone_id,
        created_by: Some(member_id),
        finished_at: None,
        finished_by: None,
        group_id: lot.group_id,
        is_archived: Some(false),
        name: lot.name,
        note: None,
        ordered_at: None,
        ordered_by: None,
        price: lot.price,
        provider_id: lot.provider_id,
        storage_id: lot.storage_id,
        collection_id: lot.collection_id,
        purpose: Some(payload.purpose),
        received_by: None,
        received_at: None,
        reference: lot.reference,
        requested_by,
        requested_at,
        status: Some(0),
        url: lot.url,
    };

    let new_id = LotBmc::create(ctx, mm, new_lot).await?;
    let lot = LotBmc::get(ctx, mm, new_id).await?;
    Ok(json!(lot))
}

async fn get_clones(ctx: &Ctx, mm: &MM, filter_json: Value, lo: LO) -> Result<serde_json::Value> {
    let filters: Vec<CloneFilter> = serde_json::from_value(filter_json)?;
    let total = CloneBmc::count(ctx, mm, Some(filters.clone())).await?;
    let mut ret = PaginatedResponse::<Clone>::from_lo(&lo, total);
    ret.items = CloneBmc::list(ctx, mm, Some(filters), Some(lo)).await?;
    Ok(json!(ret))
}

async fn get_groups(ctx: &Ctx, mm: &MM, filter_json: Value, lo: LO) -> Result<serde_json::Value> {
    let filters: Vec<GroupFilter> = serde_json::from_value(filter_json)?;
    let total = GroupBmc::count(ctx, mm, Some(filters.clone())).await?;
    let mut ret = PaginatedResponse::<Group>::from_lo(&lo, total);
    ret.items = GroupBmc::list(ctx, mm, Some(filters), Some(lo)).await?;
    Ok(json!(ret))
}

async fn delete_group(ctx: &Ctx, mm: &MM, id: Option<i64>) -> Result<serde_json::Value> {
    if let Some(id) = id {
        GroupBmc::delete(ctx, mm, id).await?;
    }
    Ok(json!({}))
}

async fn update_group(
    ctx: &Ctx,
    mm: &MM,
    id: Option<i64>,
    payload: Option<Value>,
) -> Result<serde_json::Value> {
    if let (Some(id), Some(payload)) = (id, payload) {
        let fu: GroupForUpdate = payload.into();
        warn!("UPDATE: {fu:?}");
        GroupBmc::update(ctx, mm, id, fu).await?;
    }
    Ok(json!({}))
}

async fn insert_group(ctx: &Ctx, mm: &MM, payload: Option<Value>) -> Result<serde_json::Value> {
    let mut id = 0;
    if let Some(payload) = payload {
        let fc: GroupForCreate = payload.into();
        warn!("UPDATE: {fc:?}");
        id = GroupBmc::create(ctx, mm, fc).await?;
    }
    Ok(json!({"id": id}))
}

async fn get_users(ctx: &Ctx, mm: &MM, filter_json: Value, lo: LO) -> Result<serde_json::Value> {
    let filters: Vec<UserFilter> = serde_json::from_value(filter_json)?;
    let total = UserBmc::count(ctx, mm, Some(filters.clone())).await?;
    let mut ret = PaginatedResponse::<User>::from_lo(&lo, total);
    ret.items = UserBmc::list(ctx, mm, Some(filters), Some(lo)).await?;
    Ok(json!(ret))
}

async fn delete_user(ctx: &Ctx, mm: &MM, id: Option<i64>) -> Result<serde_json::Value> {
    if let Some(id) = id {
        UserBmc::delete(ctx, mm, id).await?;
    }
    Ok(json!({}))
}

async fn update_user(
    ctx: &Ctx,
    mm: &MM,
    id: Option<i64>,
    payload: Option<Value>,
) -> Result<serde_json::Value> {
    if let (Some(id), Some(payload)) = (id, payload) {
        let fu: UserForUpdate = payload.into();
        warn!("UPDATE: {fu:?}");
        UserBmc::update(ctx, mm, id, fu).await?;
    }
    Ok(json!({}))
}

async fn insert_user(ctx: &Ctx, mm: &MM, payload: Option<Value>) -> Result<serde_json::Value> {
    let mut id = 0;
    if let Some(payload) = payload {
        let fc: UserForCreate = payload.into();
        warn!("UPDATE: {fc:?}");
        id = UserBmc::create(ctx, mm, fc).await?;
    }
    Ok(json!({"id": id}))
}

async fn get_members(ctx: &Ctx, mm: &MM, filter_json: Value, lo: LO) -> Result<serde_json::Value> {
    let filters: Vec<MemberFilter> = serde_json::from_value(filter_json)?;
    warn!("MemberFilers: {:#?}", filters);
    let total = MemberBmc::count(ctx, mm, Some(filters.clone())).await?;
    let mut ret = PaginatedResponse::<Member>::from_lo(&lo, total);
    ret.items = MemberBmc::list(ctx, mm, Some(filters), Some(lo)).await?;
    Ok(json!(ret))
}

async fn delete_member(ctx: &Ctx, mm: &MM, id: Option<i64>) -> Result<serde_json::Value> {
    if let Some(id) = id {
        MemberBmc::delete(ctx, mm, id).await?;
    }
    Ok(json!({}))
}

async fn update_member(
    ctx: &Ctx,
    mm: &MM,
    id: Option<i64>,
    payload: Option<Value>,
) -> Result<serde_json::Value> {
    if let (Some(id), Some(payload)) = (id, payload) {
        let fu: MemberForUpdate = payload.into();
        warn!("UPDATE: {fu:?}");
        MemberBmc::update(ctx, mm, id, fu).await?;
    }
    Ok(json!({}))
}

async fn insert_member(ctx: &Ctx, mm: &MM, payload: Option<Value>) -> Result<serde_json::Value> {
    let mut id = 0;
    if let Some(payload) = payload {
        let fc: MemberForCreate = payload.into();
        warn!("UPDATE: {fc:?}");
        id = MemberBmc::create(ctx, mm, fc).await?;
    }
    Ok(json!({"id": id}))
}

async fn get_species(ctx: &Ctx, mm: &MM, filter_json: Value, lo: LO) -> Result<serde_json::Value> {
    let filters: Vec<SpeciesFilter> = serde_json::from_value(filter_json)?;
    let total = SpeciesBmc::count(ctx, mm, Some(filters.clone())).await?;
    let mut ret = PaginatedResponse::<Species>::from_lo(&lo, total);
    ret.items = SpeciesBmc::list(ctx, mm, Some(filters), Some(lo)).await?;
    Ok(json!(ret))
}

async fn delete_species(ctx: &Ctx, mm: &MM, id: Option<i64>) -> Result<serde_json::Value> {
    if let Some(id) = id {
        SpeciesBmc::delete(ctx, mm, id).await?;
    }
    Ok(json!({}))
}

async fn update_species(
    ctx: &Ctx,
    mm: &MM,
    id: Option<i64>,
    payload: Option<Value>,
) -> Result<serde_json::Value> {
    if let (Some(id), Some(payload)) = (id, payload) {
        let fu: SpeciesForUpdate = payload.into();
        warn!("UPDATE: {fu:?}");
        SpeciesBmc::update(ctx, mm, id, fu).await?;
    }
    Ok(json!({}))
}

async fn insert_species(ctx: &Ctx, mm: &MM, payload: Option<Value>) -> Result<serde_json::Value> {
    let mut id = 0;
    if let Some(payload) = payload {
        let fc: SpeciesForCreate = payload.into();
        warn!("UPDATE: {fc:?}");
        id = SpeciesBmc::create(ctx, mm, fc).await?;
    }
    Ok(json!({"id": id}))
}

pub fn filters_to_json_array(filters: Vec<Filter>) -> Value {
    let mut field_map = Map::new();

    for filter in filters {
        let mut op_map = Map::new();
        let op_key = format!("${}", filter.op);
        op_map.insert(op_key, filter.value);

        field_map.insert(filter.field, Value::Object(op_map));
    }

    Value::Array(vec![Value::Object(field_map)])
}

async fn get_member_id(ctx: &Ctx, mm: &MM, group_id: i64, user_id: i64) -> Result<i64> {
    let filters: Vec<MemberFilter> = serde_json::from_value(json!([
        {
            "group_id": {"$eq": group_id},
            "user_id": {"$eq": user_id}
        }
    ]))?;
    let members: Vec<Member> = MemberBmc::list(ctx, mm, Some(filters), None).await?;
    Ok(members.first().map_or(0, |m| m.id))
}

#[derive(Serialize, Default)]
struct PaginatedResponse<T> {
    items: Vec<T>,
    total: i64,
    page: u32,
    limit: u32,
    has_next: bool,
    has_previous: bool,
}

impl<T> PaginatedResponse<T> {
    fn from_lo(lo: &LO, total: i64) -> Self {
        let offset = lo.offset.unwrap_or(50);
        let limit = lo.limit.unwrap_or(50);
        let page = (offset / limit) as u32;
        Self {
            items: vec![],
            total,
            page,
            limit: limit as u32,
            has_next: offset + limit < total,
            has_previous: page > 0,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RpcRequest {
    pub operation: Op,
    pub return_type: RT,
    pub id: Option<i64>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    #[serde(default)]
    pub filters: Vec<Filter>,
    pub payload: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub enum Operation {
    Get,
    Insert,
    Update,
    Reorder,
    Delete,
}

#[derive(Debug, Deserialize)]
pub enum ReturnType {
    User,
    Member,
    Group,
    Clone,
    Species,
    Protein,
    Lot,
    Tag,
    Conjugate,
    Panel,
    PanelElement,
    Validation,
    ValidationFile,
    Provider,
    Storage,
    Collection,
}

#[derive(Debug, Deserialize)]
pub struct Filter {
    pub field: String,
    pub op: String,
    pub value: Value,
}

#[cfg(test)]
mod tests {
    use super::*;
    use airlab_lib::ctx::Ctx;
    use airlab_lib::model::collection::CollectionBmc;
    use airlab_lib::model::storage::{StorageBmc, StorageForCreate};
    use tower::ServiceExt;

    type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    #[test]
    fn filters_to_json_array_groups_fields_in_one_object() {
        let value = filters_to_json_array(vec![
            Filter {
                field: "group_id".into(),
                op: "eq".into(),
                value: json!(1000),
            },
            Filter {
                field: "name".into(),
                op: "contains".into(),
                value: json!("seed"),
            },
        ]);

        assert_eq!(
            value,
            json!([
                {
                    "group_id": { "$eq": 1000 },
                    "name": { "$contains": "seed" }
                }
            ])
        );
    }

    #[tokio::test]
    async fn json_route_can_list_seeded_providers() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = crate::web::test_support::authed_router(routes(
            crate::search_shadow::SearchState::new((*mm).clone()),
        ));
        let request = json!({
            "operation": "Get",
            "return_type": "Provider",
            "filters": [
                { "field": "group_id", "op": "eq", "value": 1000 }
            ],
            "page": 1,
            "limit": 10
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/json")
                    .header(axum::http::header::CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::from(request.to_string()))?,
            )
            .await?;

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let body = crate::web::test_support::response_body_string(response).await?;
        assert!(body.contains("seed-provider"));

        Ok(())
    }

    #[tokio::test]
    async fn json_route_can_insert_collection() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = crate::web::test_support::authed_router(routes(
            crate::search_shadow::SearchState::new((*mm).clone()),
        ));
        let request = json!({
            "operation": "Insert",
            "return_type": "Collection",
            "payload": {
                "name": "json-collection",
                "description": "created through json route",
                "createdBy": 1
            }
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/json")
                    .header(axum::http::header::CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::from(request.to_string()))?,
            )
            .await?;

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let body = crate::web::test_support::response_body_string(response).await?;
        let value: serde_json::Value = serde_json::from_str(&body)?;
        let id = value["id"]
            .as_i64()
            .ok_or_else(|| std::io::Error::other("insert should return id"))?;
        let collection = CollectionBmc::get(&Ctx::root_ctx(), &mm, id).await?;
        assert_eq!(collection.name, "json-collection");

        Ok(())
    }

    #[tokio::test]
    async fn json_route_can_update_storage() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let ctx = Ctx::root_ctx();
        let id = StorageBmc::create(
            &ctx,
            &mm,
            StorageForCreate {
                name: "json-storage-before".into(),
                r#type: "fridge".into(),
                location: "Room A".into(),
                temperature_c: 4,
                active: true,
            },
        )
        .await?;
        let app = crate::web::test_support::authed_router(routes(
            crate::search_shadow::SearchState::new((*mm).clone()),
        ));
        let request = json!({
            "operation": "Update",
            "return_type": "Storage",
            "id": id,
            "payload": {
                "name": "json-storage-after",
                "type": "freezer",
                "location": "Room B",
                "temperatureC": -80,
                "active": false
            }
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/json")
                    .header(axum::http::header::CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::from(request.to_string()))?,
            )
            .await?;

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let storage = StorageBmc::get(&ctx, &mm, id).await?;
        assert_eq!(storage.name, "json-storage-after");
        assert_eq!(storage.location, "Room B");
        assert_eq!(storage.temperature_c, -80);
        assert!(!storage.active);

        Ok(())
    }
}
