use crate::web::mw_auth::CtxW;
use crate::web::Result;
use airlab_lib::model::lot::LotFilter;
use airlab_lib::model::provider::{Provider, ProviderBmc, ProviderForCreate, ProviderForUpdate};
use airlab_lib::model::view_lot::{ViewLot, ViewLotBmc};
use airlab_lib::model::ModelManager;
use axum::extract::{Json as eJson, Path, State};
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use serde_json::{json, Value};
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/v1/providers", post(api_post_provider_handler))
        .route(
            "/api/v1/providers/:provider_id/lots",
            get(api_provider_lots_handler),
        )
        .route(
            "/api/v1/providers/:provider_id",
            patch(api_patch_provider_handler),
        )
        .route("/api/v1/providers/:provider_id", get(api_provider_handler))
        .route("/api/v1/providers", get(api_providers_handler))
        .with_state(mm)
}

async fn api_provider_lots_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(provider_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_group_lots_handler");
    let ctx = ctx.0;

    let filters: Vec<LotFilter> = serde_json::from_value(json!([
        {
            "provider_id": provider_id
        }
    ]))?;

    let vlots: Vec<ViewLot> = ViewLotBmc::list(&ctx, &mm, None, Some(filters), None).await?;

    Ok(Json(json!(vlots)))
}

async fn api_post_provider_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    eJson(payload): eJson<ProviderForCreate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_post_provider_handler: {:?}", payload);
    let ctx = ctx.0;
    let provider_id = ProviderBmc::create(&ctx, &mm, payload).await?;

    let provider: Provider = ProviderBmc::get(&ctx, &mm, provider_id).await?;
    Ok(Json(json!(provider)))
}

async fn api_patch_provider_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(provider_id): Path<i32>,
    eJson(payload): eJson<ProviderForUpdate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_provider: {}; {:?}", provider_id, payload);
    let ctx = ctx.0;

    ProviderBmc::update(&ctx, &mm, provider_id, payload).await?;

    let provider: Provider = ProviderBmc::get(&ctx, &mm, provider_id).await?;
    Ok(Json(json!(provider)))
}

async fn api_provider_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(provider_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_provider_handler: {}", provider_id);
    let ctx = ctx.0;

    let provider: Provider = ProviderBmc::get(&ctx, &mm, provider_id).await?;
    Ok(Json(json!(provider)))
}

async fn api_providers_handler(State(mm): State<ModelManager>, ctx: CtxW) -> Result<Json<Value>> {
    debug!("HANDLER - api_providers_handler");
    let ctx = ctx.0;

    let providers: Vec<Provider> = ProviderBmc::list(&ctx, &mm, None, None).await?;
    Ok(Json(json!(providers)))
}
