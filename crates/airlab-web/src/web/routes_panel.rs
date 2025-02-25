use crate::web::mw_auth::CtxW;
use crate::web::Result;
use crate::web_util::get_member_id;
use airlab_lib::model::panel::{
    Panel, PanelBmc, PanelForCreate, PanelForUpdate, PanelPayloadForUpdate,
};
use airlab_lib::model::panel_element::{
    PanelElement, PanelElementBmc, PanelElementFilter, PanelElementForCreate, PanelElementForUpdate,
};
use airlab_lib::model::view_panel::{ViewPanel, ViewPanelBmc};
use airlab_lib::model::view_panel_element::{ViewPanelElement, ViewPanelElementBmc};
use airlab_lib::model::ModelManager;
use axum::extract::{Json as eJson, Path, State};
use axum::routing::{get, patch, post, put};
use axum::{Json, Router};
use modql::filter::ListOptions;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
#[allow(unused_imports)]
use tracing::{debug, warn};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/v1/panels", post(api_post_panel_handler))
        .route(
            "/api/v1/panels/:panel_id/elements",
            get(api_panel_elements_handler),
        )
        .route("/api/v1/panels/:panel_id", patch(api_patch_panel_handler))
        .route("/api/v1/panels/:panel_id", get(api_panel_handler))
        .route("/api/v1/panels/:panel_id", put(api_put_panel_handler))
        .route(
            "/api/v1/panels/:panel_id/archive",
            patch(api_patch_archive_panel_handler),
        )
        .route("/api/v1/panels", get(api_panels_handler))
        .with_state(mm)
}

async fn api_panel_elements_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(panel_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_panel_elements_handler");
    let ctx = ctx.0;
    let filters: Vec<PanelElementFilter> = serde_json::from_value(json!([
        {
            "panel_id": panel_id
        }
    ]))?;

    let panel_elements: Vec<ViewPanelElement> =
        ViewPanelElementBmc::list(&ctx, &mm, Some(filters), None).await?;
    Ok(Json(json!(panel_elements)))
}

#[derive(Deserialize, Debug)]
struct ElementPayload {
    #[serde(rename = "conjugateId")]
    conjugate_id: i32,
    #[serde(rename = "dilutionType")]
    dilution_type: i32,
}

#[derive(Deserialize, Debug)]
struct PanelForCreatePayload {
    name: String,
    #[serde(rename = "groupId")]
    group_id: i32,
    description: Option<String>,
    #[serde(rename = "isFluorophore")]
    is_fluorophore: bool,
    #[serde(rename = "isLocked")]
    is_locked: bool,
    application: Option<i32>,
    elements: Vec<ElementPayload>,
}

async fn api_post_panel_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    eJson(payload): eJson<PanelForCreatePayload>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_post_panel_handler: {:?}", payload);
    let ctx = ctx.0;
    let created_by = get_member_id(&ctx, &mm, payload.group_id, ctx.user_id()).await?;
    let mut forc = PanelForCreate {
        created_by: Some(created_by),
        name: Some(payload.name),
        application: payload.application,
        description: payload.description,
        group_id: payload.group_id,
        is_fluorophore: payload.is_fluorophore,
        is_locked: payload.is_locked,
    };
    forc.created_by = Some(created_by);
    let panel_id = PanelBmc::create(&ctx, &mm, forc).await?;

    for element in payload.elements {
        let element_c = PanelElementForCreate {
            concentration: None,
            conjugate_id: element.conjugate_id,
            dilution_type: element.dilution_type,
            panel_id,
        };
        PanelElementBmc::create(&ctx, &mm, element_c).await?;
    }

    let panel: Panel = PanelBmc::get(&ctx, &mm, panel_id).await?;
    Ok(Json(json!(panel)))
}

#[derive(Deserialize, Debug)]
struct PanelArchive {
    state: bool,
}

async fn api_patch_archive_panel_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(panel_id): Path<i32>,
    eJson(payload): eJson<PanelArchive>,
) -> Result<Json<Value>> {
    debug!(
        "HANDLER - api_patch_archive_panel: {}; {:?} {}",
        panel_id, payload, payload.state
    );
    let ctx = ctx.0;
    let panel_update = PanelForUpdate {
        is_archived: Some(true),
        ..Default::default()
    };

    PanelBmc::update(&ctx, &mm, panel_id, panel_update).await?;
    let panel: Panel = PanelBmc::get(&ctx, &mm, panel_id).await?;
    Ok(Json(json!(panel)))
}

#[derive(Deserialize, Debug)]
struct PanelDuplicate {
    #[serde(rename = "groupId")]
    group_id: i32,
    name: String,
}

async fn api_put_panel_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(panel_id): Path<i32>,
    eJson(payload): eJson<PanelDuplicate>,
) -> Result<Json<Value>> {
    debug!(
        "HANDLER - api_panel: {}; {:?} {} {}",
        panel_id, payload, payload.group_id, payload.name
    );
    let ctx = ctx.0;
    let panel = PanelBmc::get(&ctx, &mm, panel_id).await?;
    let created_by = get_member_id(&ctx, &mm, panel.group_id, ctx.user_id()).await?;
    let panel_c = PanelForCreate {
        name: Some(payload.name),
        description: Some(format!(
            "Duplicate of {} (id: {})",
            panel.name.unwrap_or(String::new()),
            panel.id
        )),
        created_by: Some(created_by),
        group_id: payload.group_id,
        application: panel.application,
        is_fluorophore: panel.is_fluorophore,
        is_locked: false,
    };
    let new_panel_id = PanelBmc::create(&ctx, &mm, panel_c).await?;
    let filters: Vec<PanelElementFilter> = serde_json::from_value(json!([
        {
            "panel_id": panel_id
        }
    ]))?;

    let panel_elements: Vec<PanelElement> =
        PanelElementBmc::list(&ctx, &mm, Some(filters), None).await?;
    for element in panel_elements {
        let element_c = PanelElementForCreate {
            concentration: element.concentration,
            conjugate_id: element.conjugate_id,
            dilution_type: i32::from(element.dilution_type),
            panel_id: new_panel_id,
        };
        PanelElementBmc::create(&ctx, &mm, element_c).await?;
    }

    let panel: Panel = PanelBmc::get(&ctx, &mm, new_panel_id).await?;
    Ok(Json(json!(panel)))
}

async fn api_patch_panel_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(panel_id): Path<i32>,
    eJson(payload): eJson<PanelPayloadForUpdate>,
) -> Result<Json<Value>> {
    debug!(
        "HANDLER - api_patch_panel_handler: {}; {:?}",
        panel_id, payload
    );
    let ctx = ctx.0;
    let filters: Vec<PanelElementFilter> =
        serde_json::from_value(json!([{"panel_id": {"$eq": panel_id}}]))?;
    let op = ListOptions {
        limit: Some(10_000),
        ..Default::default()
    };
    let current_elements = PanelElementBmc::list(&ctx, &mm, Some(filters), Some(op)).await?;
    let mut present_ids: HashSet<i32> = current_elements.iter().map(|e| e.id).collect();
    let conjugates_present: HashMap<i32, i32> = current_elements
        .into_iter()
        .map(|e| (e.conjugate_id, e.id))
        .collect();
    for element in &payload.elements {
        if let Some(id) = conjugates_present.get(&element.conjugate_id) {
            present_ids.remove(&id);
            let element_u = PanelElementForUpdate {
                concentration: element.concentration,
                dilution_type: element.dilution_type,
            };
            PanelElementBmc::update(&ctx, &mm, *id, element_u).await?;
        } else {
            let for_create = PanelElementForCreate {
                panel_id,
                concentration: element.concentration,
                dilution_type: element.dilution_type,
                conjugate_id: element.conjugate_id,
            };
            PanelElementBmc::create(&ctx, &mm, for_create).await?;
        }
    }
    for id_to_remove in present_ids {
        println!("Warning: removing panel_element id: {id_to_remove}");
        warn!("Warning: removing panel_element id: {id_to_remove}");
        PanelElementBmc::delete(&ctx, &mm, id_to_remove).await?;
    }
    let pu = PanelForUpdate {
        name: payload.name.clone(),
        is_locked: payload.is_locked,
        is_fluorophore: payload.is_fluorophore,
        description: payload.description.clone(),
        application: payload.application,
        ..Default::default()
    };

    if pu.name.is_none()
        && pu.is_locked.is_none()
        && pu.is_fluorophore.is_none()
        && pu.description.is_none()
        && pu.application.is_none()
    {
    } else {
        PanelBmc::update(&ctx, &mm, panel_id, pu).await?;
    }

    let panel: Panel = PanelBmc::get(&ctx, &mm, panel_id).await?;
    Ok(Json(json!(panel)))
}

async fn api_panel_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(panel_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_panel_handler: {}", panel_id);
    let ctx = ctx.0;

    let panel: ViewPanel = ViewPanelBmc::get(&ctx, &mm, panel_id).await?;
    Ok(Json(json!(panel)))
}

async fn api_panels_handler(State(mm): State<ModelManager>, ctx: CtxW) -> Result<Json<Value>> {
    debug!("HANDLER - api_panels_handler");
    let ctx = ctx.0;

    let panels: Vec<Panel> = PanelBmc::list(&ctx, &mm, None, None).await?;
    Ok(Json(json!(panels)))
}
