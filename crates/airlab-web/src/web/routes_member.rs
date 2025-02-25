use crate::web::mw_auth::CtxW;
use crate::web::Result;
use airlab_lib::model::member::{Member, MemberBmc, MemberForCreate, MemberForUpdate};
use airlab_lib::model::ModelManager;
use axum::extract::{Json as eJson, Path, State};
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use serde_json::{json, Value};
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/v1/members", post(api_post_member_handler))
        .route(
            "/api/v1/members/:member_id",
            patch(api_patch_member_handler),
        )
        .route("/api/v1/members/:member_id", get(api_member_handler))
        .route("/api/v1/members", get(api_members_handler))
        .with_state(mm)
}

async fn api_post_member_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    eJson(payload): eJson<MemberForCreate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_post_member_handler: {:?}", payload);
    let ctx = ctx.0;
    let member_id = MemberBmc::create(&ctx, &mm, payload).await?;

    let member: Member = MemberBmc::get(&ctx, &mm, member_id).await?;
    Ok(Json(json!(member)))
}

async fn api_patch_member_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(member_id): Path<i32>,
    eJson(payload): eJson<MemberForUpdate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_member_handler: {}; {:?}", member_id, payload);
    let ctx = ctx.0;

    MemberBmc::update(&ctx, &mm, member_id, payload).await?;

    let member: Member = MemberBmc::get(&ctx, &mm, member_id).await?;
    Ok(Json(json!(member)))
}

async fn api_member_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(member_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_member_handler: {}", member_id);
    let ctx = ctx.0;

    let member: Member = MemberBmc::get(&ctx, &mm, member_id).await?;
    Ok(Json(json!(member)))
}

async fn api_members_handler(State(mm): State<ModelManager>, ctx: CtxW) -> Result<Json<Value>> {
    debug!("HANDLER - api_members_handler");
    let ctx = ctx.0;

    let members: Vec<Member> = MemberBmc::list(&ctx, &mm, None, None).await?;
    Ok(Json(json!(members)))
}
