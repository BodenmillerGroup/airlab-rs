use crate::web::mw_auth::CtxW;
use crate::web::Result;
use crate::web_util::get_member_id;
use axum::extract::{Json as eJson, Path, State};
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use lib_core::model::clone::{Clone, CloneBmc, CloneForCreate, CloneForUpdate};
use lib_core::model::lot::LotFilter;
use lib_core::model::validation::{Validation, ValidationBmc, ValidationFilter};
use lib_core::model::view_lot::{ViewLot, ViewLotBmc};
use lib_core::model::ModelManager;
use serde_json::{json, Value};
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/v1/clones", post(api_post_clone_handler))
        .route("/api/v1/clones/:clone_id/lots", get(api_clone_lots_handler))
        .route("/api/v1/clones/:clone_id", patch(api_patch_clone_handler))
        .route("/api/v1/clones/:clone_id", get(api_clone_handler))
        .route(
            "/api/v1/clones/:clone_id/validations",
            get(api_clone_validations_handler),
        )
        .route("/api/v1/clones", get(api_clones_handler))
        .with_state(mm)
}

async fn api_clone_lots_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(clone_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_group_lots_handler");
    let ctx = ctx.0;

    let filters: Vec<LotFilter> = serde_json::from_value(json!([
        {
            "clone_id": clone_id
        }
    ]))?;

    let vlots: Vec<ViewLot> = ViewLotBmc::list(&ctx, &mm, None, Some(filters), None).await?;

    Ok(Json(json!(vlots)))
}

async fn api_post_clone_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    eJson(mut payload): eJson<CloneForCreate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_post_clone_handler: {:?}", payload);
    let ctx = ctx.0;
    let created_by = get_member_id(&ctx, &mm, payload.group_id, ctx.user_id()).await?;
    payload.created_by = Some(created_by);
    let clone_id = CloneBmc::create(&ctx, &mm, payload).await?;

    let clone: Clone = CloneBmc::get(&ctx, &mm, clone_id).await?;
    Ok(Json(json!(clone)))
}

async fn api_patch_clone_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(clone_id): Path<i32>,
    eJson(payload): eJson<CloneForUpdate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_patch_clone: {}; {:?}", clone_id, payload);
    let ctx = ctx.0;

    CloneBmc::update(&ctx, &mm, clone_id, payload).await?;

    let clone: Clone = CloneBmc::get(&ctx, &mm, clone_id).await?;
    Ok(Json(json!(clone)))
}

async fn api_clone_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(clone_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_clone_handler: {}", clone_id);
    let ctx = ctx.0;

    let clone: Clone = CloneBmc::get(&ctx, &mm, clone_id).await?;
    Ok(Json(json!(clone)))
}

async fn api_clone_validations_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(clone_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_clone_validations_handler: {}", clone_id);
    let ctx = ctx.0;

    let filters: Vec<ValidationFilter> = serde_json::from_value(json!([
        {
            "clone_id": clone_id
        }
    ]))?;

    let validations: Vec<Validation> = ValidationBmc::list(&ctx, &mm, Some(filters), None).await?;
    Ok(Json(json!(validations)))
}

async fn api_clones_handler(State(mm): State<ModelManager>, ctx: CtxW) -> Result<Json<Value>> {
    debug!("HANDLER - api_clones_handler");
    let ctx = ctx.0;

    let clones: Vec<Clone> = CloneBmc::list(&ctx, &mm, None, None).await?;
    Ok(Json(json!(clones)))
}
