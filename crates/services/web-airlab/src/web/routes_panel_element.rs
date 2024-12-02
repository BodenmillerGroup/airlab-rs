use crate::web::mw_auth::CtxW;
use crate::web::Result;
use axum::extract::{Json as eJson, Path, State};
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use lib_core::model::panel_element::{
    PanelElement, PanelElementBmc, PanelElementForCreate, PanelElementForUpdate,
};
use lib_core::model::ModelManager;
use serde_json::{json, Value};
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route(
            "/api/v1/panel_elements",
            post(api_post_panel_element_handler),
        )
        .route(
            "/api/v1/panel_elements/:panel_element_id",
            patch(api_patch_panel_element_handler),
        )
        .route(
            "/api/v1/panel_elements/:panel_element_id",
            get(api_panel_element_handler),
        )
        .route("/api/v1/panel_elements", get(api_panel_elements_handler))
        .with_state(mm)
}

async fn api_post_panel_element_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    eJson(payload): eJson<PanelElementForCreate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_post_panel_element_handler: {:?}", payload);
    let ctx = ctx.0;
    let panel_element_id = PanelElementBmc::create(&ctx, &mm, payload).await?;

    let panel_element: PanelElement = PanelElementBmc::get(&ctx, &mm, panel_element_id).await?;
    Ok(Json(json!(panel_element)))
}

async fn api_patch_panel_element_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(panel_e_id): Path<i32>,
    eJson(payload): eJson<PanelElementForUpdate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_panel_element: {}; {:?}", panel_e_id, payload);
    let ctx = ctx.0;

    PanelElementBmc::update(&ctx, &mm, panel_e_id, payload).await?;

    let panel_element: PanelElement = PanelElementBmc::get(&ctx, &mm, panel_e_id).await?;
    Ok(Json(json!(panel_element)))
}

async fn api_panel_element_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(panel_e_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_panel_element_handler: {}", panel_e_id);
    let ctx = ctx.0;

    let panel_element: PanelElement = PanelElementBmc::get(&ctx, &mm, panel_e_id).await?;
    Ok(Json(json!(panel_element)))
}

async fn api_panel_elements_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_proteins_handler");
    let ctx = ctx.0;

    let proteins: Vec<PanelElement> = PanelElementBmc::list(&ctx, &mm, None, None).await?;
    Ok(Json(json!(proteins)))
}
