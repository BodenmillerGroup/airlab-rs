use crate::web::Result;
use crate::web::mw_auth::CtxW;
use crate::web_util::get_member_id;
use airlab_lib::model::ModelManager;
use airlab_lib::model::clone::CloneFilter;
use airlab_lib::model::conjugate::{
    Conjugate, ConjugateBmc, ConjugateForCreate, ConjugateForUpdate,
};
use airlab_lib::model::lot::LotBmc;
use airlab_lib::model::panel::PanelFilter;
use airlab_lib::model::panel_element::{PanelElement, PanelElementBmc, PanelElementFilter};
use airlab_lib::model::validation::{Validation, ValidationBmc, ValidationFilter};
use airlab_lib::model::view_clone::{ViewClone, ViewCloneBmc};
use airlab_lib::model::view_panel::{ViewPanel, ViewPanelBmc};
use axum::extract::{Json as eJson, Path, State};
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use modql::filter::{ListOptions, OrderBy, OrderBys};
use serde_json::{Value, json};
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/v1/conjugates/", post(api_post_conjugate_handler))
        .route("/api/v1/conjugates", post(api_post_conjugate_handler))
        .route(
            "/api/v1/conjugates/:conjugate_id/clones",
            get(api_conjugate_clones_handler),
        )
        .route(
            "/api/v1/conjugates/:conjugate_id/panels",
            get(api_conjugate_panels_handler),
        )
        .route(
            "/api/v1/conjugates/:conjugate_id/status",
            patch(api_patch_conjugate_handler),
        )
        .route(
            "/api/v1/conjugates/:conjugate_id",
            patch(api_patch_conjugate_handler),
        )
        .route(
            "/api/v1/conjugates/:conjugate_id",
            get(api_conjugate_handler),
        )
        .route(
            "/api/v1/conjugates/:conjugate_id/validations",
            get(api_conjugate_validations_handler),
        )
        .route("/api/v1/conjugates", get(api_conjugates_handler))
        .with_state(mm)
}

async fn api_conjugate_panels_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(conjugate_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_conjugate_panels_handler");
    let ctx = ctx.0;

    let filters: Vec<PanelElementFilter> = serde_json::from_value(json!([
        {
            "conjugate_id": {"$eq": conjugate_id }
        }
    ]))?;

    let elements: Vec<PanelElement> = PanelElementBmc::list(&ctx, &mm, Some(filters), None).await?;
    let panel_ids: Vec<i32> = elements.into_iter().map(|e| e.panel_id).collect();

    let filters: Vec<PanelFilter> = serde_json::from_value(json!([
        {
            "id": {"$in": panel_ids }
        }
    ]))?;

    let list_options = ListOptions {
        limit: Some(10000),
        offset: None,
        order_bys: Some(OrderBys::new(vec![OrderBy::Desc("id".into())])),
    };

    let panels: Vec<ViewPanel> =
        ViewPanelBmc::list(&ctx, &mm, Some(filters), Some(list_options)).await?;
    Ok(Json(json!(panels)))
}

async fn api_conjugate_clones_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(conjugate_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_conjugate_clones_handler");
    let ctx = ctx.0;
    let conjugate = ConjugateBmc::get(&ctx, &mm, conjugate_id).await?;
    let lot = LotBmc::get(&ctx, &mm, conjugate.lot_id).await?;

    let filters: Vec<CloneFilter> = serde_json::from_value(json!([
        {
            "id": {"$eq": lot.clone_id }
        }
    ]))?;

    let clones: Vec<ViewClone> = ViewCloneBmc::list(&ctx, &mm, None, Some(filters), None).await?;
    Ok(Json(json!(clones)))
}

async fn api_post_conjugate_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    eJson(mut payload): eJson<ConjugateForCreate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_post_conjugate_handler: {:?}", payload);
    let ctx = ctx.0;
    let created_by = get_member_id(&ctx, &mm, payload.group_id, ctx.user_id()).await?;
    payload.created_by = Some(created_by);
    let conjugate_id = ConjugateBmc::create(&ctx, &mm, payload).await?;

    let conjugate: Conjugate = ConjugateBmc::get(&ctx, &mm, conjugate_id).await?;
    Ok(Json(json!(conjugate)))
}

async fn api_patch_conjugate_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(conjugate_id): Path<i32>,
    eJson(payload): eJson<ConjugateForUpdate>,
) -> Result<Json<Value>> {
    debug!(
        "HANDLER - api_patch_conjugate: {}; {:?}",
        conjugate_id, payload
    );
    let ctx = ctx.0;

    ConjugateBmc::update(&ctx, &mm, conjugate_id, payload).await?;

    let conjugate: Conjugate = ConjugateBmc::get(&ctx, &mm, conjugate_id).await?;
    Ok(Json(json!(conjugate)))
}

async fn api_conjugate_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(conjugate_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_conjugate_handler: {}", conjugate_id);
    let ctx = ctx.0;

    let conjugate: Conjugate = ConjugateBmc::get(&ctx, &mm, conjugate_id).await?;
    Ok(Json(json!(conjugate)))
}

async fn api_conjugate_validations_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(conjugate_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!(
        "HANDLER - api_conjugate_validations_handler: {}",
        conjugate_id
    );
    let ctx = ctx.0;

    let filters: Vec<ValidationFilter> = serde_json::from_value(json!([
        {
            "conjugate_id": {"$eq":conjugate_id}
        }
    ]))?;

    let validations: Vec<Validation> = ValidationBmc::list(&ctx, &mm, Some(filters), None).await?;
    Ok(Json(json!(validations)))
}

async fn api_conjugates_handler(State(mm): State<ModelManager>, ctx: CtxW) -> Result<Json<Value>> {
    debug!("HANDLER - api_conjugates_handler");
    let ctx = ctx.0;

    let conjugates: Vec<Conjugate> = ConjugateBmc::list(&ctx, &mm, None, None).await?;
    Ok(Json(json!(conjugates)))
}
