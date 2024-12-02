use crate::web::mw_auth::CtxW;
use crate::web::Result;
use axum::extract::{Json as eJson, Path, State};
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use lib_core::model::conjugate::ConjugateFilter;
use lib_core::model::tag::{Tag, TagBmc, TagForCreate, TagForUpdate};
use lib_core::model::view_conjugate::{ViewConjugate, ViewConjugateBmc};
use lib_core::model::ModelManager;
use serde_json::{json, Value};
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/v1/tags", post(api_post_tag_handler))
        .route("/api/v1/tags/:tag_id", patch(api_patch_tag_handler))
        .route("/api/v1/tags/:tag_id", get(api_tag_handler))
        .route("/api/v1/tags", get(api_tags_handler))
        .route(
            "/api/v1/tags/:tag_id/conjugates",
            get(api_tag_conjugates_handler),
        )
        .with_state(mm)
}

async fn api_tag_conjugates_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(tag_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_tag_conjugates_handler");
    let ctx = ctx.0;

    let filters: Vec<ConjugateFilter> = serde_json::from_value(json!([
        {
            "tag_id": tag_id
        }
    ]))?;

    let entries: Vec<ViewConjugate> =
        ViewConjugateBmc::list(&ctx, &mm, None, Some(filters), None).await?;

    Ok(Json(json!(entries)))
}

async fn api_post_tag_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    eJson(payload): eJson<TagForCreate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_post_tag_handler: {:?}", payload);
    let ctx = ctx.0;

    let tag_id = TagBmc::create(&ctx, &mm, payload).await?;

    let tag: Tag = TagBmc::get(&ctx, &mm, tag_id).await?;
    Ok(Json(json!(tag)))
}

async fn api_patch_tag_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(tag_id): Path<i32>,
    eJson(payload): eJson<TagForUpdate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_tag_handler: {}; {:?}", tag_id, payload);
    let ctx = ctx.0;

    TagBmc::update(&ctx, &mm, tag_id, payload).await?;

    let tag: Tag = TagBmc::get(&ctx, &mm, tag_id).await?;
    Ok(Json(json!(tag)))
}

async fn api_tag_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(tag_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_tag_handler: {}", tag_id);
    let ctx = ctx.0;

    let tag: Tag = TagBmc::get(&ctx, &mm, tag_id).await?;
    Ok(Json(json!(tag)))
}

async fn api_tags_handler(State(mm): State<ModelManager>, ctx: CtxW) -> Result<Json<Value>> {
    debug!("HANDLER - api_tags_handler");
    let ctx = ctx.0;

    let tags: Vec<Tag> = TagBmc::list(&ctx, &mm, None, None).await?;
    Ok(Json(json!(tags)))
}
