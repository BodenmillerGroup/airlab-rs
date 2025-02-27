use crate::web::Result;
use crate::web::mw_auth::CtxW;
use crate::web_util::get_member_id;
use airlab_lib::model::ModelManager;
use airlab_lib::model::validation::{
    Validation, ValidationBmc, ValidationForCreate, ValidationForUpdate,
};
use airlab_lib::model::view_validation::{MinViewValidation, ViewValidationBmc};
use axum::extract::{Json as eJson, Path, State};
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use serde_json::{Value, json};
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/v1/validations", post(api_post_validation_handler))
        .route(
            "/api/v1/validations/:validation_id",
            patch(api_patch_validation_handler),
        )
        .route(
            "/api/v1/validations/:validation_id",
            get(api_view_validation_handler),
        )
        .route("/api/v1/validations", get(api_validations_handler))
        .with_state(mm)
}

async fn api_post_validation_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    eJson(mut payload): eJson<ValidationForCreate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_post_validation_handler: {:?}", payload);
    let ctx = ctx.0;
    let created_by = get_member_id(&ctx, &mm, payload.group_id, ctx.user_id()).await?;
    payload.created_by = Some(created_by);
    let validation_id = ValidationBmc::create(&ctx, &mm, payload).await?;

    let validation: Validation = ValidationBmc::get(&ctx, &mm, validation_id).await?;
    Ok(Json(json!(validation)))
}

async fn api_patch_validation_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(validation_id): Path<i32>,
    eJson(payload): eJson<ValidationForUpdate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_validation: {}; {:?}", validation_id, payload);
    let ctx = ctx.0;

    ValidationBmc::update(&ctx, &mm, validation_id, payload).await?;

    let validation: Validation = ValidationBmc::get(&ctx, &mm, validation_id).await?;
    Ok(Json(json!(validation)))
}

async fn api_view_validation_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(validation_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_view_validation_handler: {}", validation_id);
    let ctx = ctx.0;

    let validation: MinViewValidation =
        ViewValidationBmc::get_min(&ctx, &mm, ctx.user_id(), validation_id).await?;
    Ok(Json(json!(validation)))
}

#[allow(dead_code)]
async fn api_validation_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(validation_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_validation_handler: {}", validation_id);
    let ctx = ctx.0;

    let validation: Validation = ValidationBmc::get(&ctx, &mm, validation_id).await?;
    Ok(Json(json!(validation)))
}

async fn api_validations_handler(State(mm): State<ModelManager>, ctx: CtxW) -> Result<Json<Value>> {
    debug!("HANDLER - api_validations_handler");
    let ctx = ctx.0;

    let validations: Vec<Validation> = ValidationBmc::list(&ctx, &mm, None, None).await?;
    Ok(Json(json!(validations)))
}
