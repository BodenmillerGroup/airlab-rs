use crate::web::mw_auth::CtxW;
use crate::web::Result;
use airlab_lib::model::user::{User, UserBmc, UserForCreate, UserForUpdate};
use airlab_lib::model::ModelManager;
use axum::extract::{Json as eJson, Path, State};
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use serde_json::{json, Value};
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/v1/users", post(api_create_user_handler))
        .route("/api/v1/users", get(api_list_user_handler))
        .route(
            "/api/v1/users/profile",
            patch(api_patch_user_profile_handler),
        )
        .route("/api/v1/users/profile", get(api_profile_handler))
        .with_state(mm)
}

async fn api_list_user_handler(State(mm): State<ModelManager>, ctx: CtxW) -> Result<Json<Value>> {
    debug!("HANDLER - api_list_user_handler");
    let ctx = ctx.0;

    let conjugates: Vec<User> = UserBmc::list(&ctx, &mm, None, None).await?;
    Ok(Json(json!(conjugates)))
}

async fn api_create_user_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    eJson(mut payload): eJson<UserForCreate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_patch_user_profile_handler: {:?}", payload);
    let ctx = ctx.0;
    payload.username = Some(payload.email.clone());

    let user_id = UserBmc::create(&ctx, &mm, payload).await?;

    let group: User = UserBmc::get(&ctx, &mm, user_id).await?;
    Ok(Json(json!(group)))
}

async fn api_patch_user_profile_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i32>,
    eJson(payload): eJson<UserForUpdate>,
) -> Result<Json<Value>> {
    debug!(
        "HANDLER - api_patch_user_profile: {}; {:?}",
        group_id, payload
    );
    let ctx = ctx.0;

    UserBmc::update(&ctx, &mm, group_id, payload).await?;

    let group: User = UserBmc::get(&ctx, &mm, group_id).await?;
    Ok(Json(json!(group)))
}

async fn api_profile_handler(State(mm): State<ModelManager>, ctx: CtxW) -> Result<Json<Value>> {
    debug!("HANDLER - api_profile_handler");

    let ctx = ctx.0;
    let user: i32 = ctx.user_id();

    let user: User = UserBmc::get::<User>(&ctx, &mm, user).await?;

    Ok(Json(json!(user)))
}
