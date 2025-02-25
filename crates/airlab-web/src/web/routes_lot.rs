use crate::web::mw_auth::CtxW;
use crate::web::Result;
use crate::web_util::get_member_id;
use airlab_lib::model::lot::{Lot, LotBmc, LotForCreate, LotForUpdate};
use airlab_lib::model::validation::{Validation, ValidationBmc, ValidationFilter};
use airlab_lib::model::view_conjugate::{ViewConjugateBmc, ViewConjugateForLot};
use airlab_lib::model::view_lot::{ViewLotBmc, ViewLotDetails};
use airlab_lib::model::ModelManager;
use axum::extract::{Json as eJson, Path, State};
use axum::routing::{get, patch, post, put};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/v1/lots", post(api_post_lot_handler))
        .route(
            "/api/v1/lots/:lot_id/conjugates",
            get(api_lot_conjugates_handler),
        )
        .route(
            "/api/v1/lots/:lot_id/status",
            patch(api_patch_lot_status_handler),
        )
        .route(
            "/api/v1/lots/:lot_id/reorder",
            put(api_put_reorder_lot_handler),
        )
        .route("/api/v1/lots/:lot_id", patch(api_patch_lot_handler))
        .route("/api/v1/lots/:lot_id", get(api_lot_handler))
        .route(
            "/api/v1/lots/:lot_id/validations",
            get(api_lot_validations_handler),
        )
        .route("/api/v1/lots", get(api_lots_handler))
        .with_state(mm)
}

async fn api_lot_conjugates_handler(
    State(mm): State<ModelManager>,
    _ctx: CtxW,
    Path(lot_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_lot_conjugates_handler");
    let entries: Vec<ViewConjugateForLot> = ViewConjugateBmc::list_for_lot(&mm, lot_id).await?;

    Ok(Json(json!(entries)))
}

async fn api_post_lot_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    eJson(mut payload): eJson<LotForCreate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_post_lot_handler: {:?}", payload);
    let ctx = ctx.0;
    let created_by = get_member_id(&ctx, &mm, payload.group_id, ctx.user_id()).await?;
    payload.created_by = Some(created_by);
    let lot_id = LotBmc::create(&ctx, &mm, payload).await?;

    let lot: Lot = LotBmc::get(&ctx, &mm, lot_id).await?;
    Ok(Json(json!(lot)))
}

#[derive(Deserialize, Debug)]
struct LotStatusUpdate {
    #[serde(rename = "lotNumber")]
    lot_number: Option<String>,
    status: Option<i16>,
}

async fn api_patch_lot_status_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(lot_id): Path<i32>,
    eJson(payload): eJson<LotStatusUpdate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_patch_lot: {}; {:?}", lot_id, payload);
    let ctx = ctx.0;
    let lot = LotBmc::get(&ctx, &mm, lot_id).await?;
    let member_id = get_member_id(&ctx, &mm, lot.group_id, ctx.user_id()).await?;

    let upd = LotForUpdate {
        number: payload.lot_number,
        status: payload.status,
        ..Default::default()
    };

    LotBmc::update(&ctx, &mm, lot_id, member_id, upd).await?;

    let lot: Lot = LotBmc::get(&ctx, &mm, lot_id).await?;
    Ok(Json(json!(lot)))
}

async fn api_patch_lot_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(lot_id): Path<i32>,
    eJson(payload): eJson<LotForUpdate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_patch_lot_handler: {}; {:?}", lot_id, payload);
    let ctx = ctx.0;
    let lot = LotBmc::get(&ctx, &mm, lot_id).await?;
    let member_id = get_member_id(&ctx, &mm, lot.group_id, ctx.user_id()).await?;

    LotBmc::update(&ctx, &mm, lot_id, member_id, payload).await?;

    let lot: Lot = LotBmc::get(&ctx, &mm, lot_id).await?;
    Ok(Json(json!(lot)))
}

#[derive(Deserialize, Debug)]
struct ReorderLot {
    purpose: String,
}

async fn api_put_reorder_lot_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(lot_id): Path<i32>,
    eJson(payload): eJson<ReorderLot>,
) -> Result<Json<Value>> {
    debug!(
        "HANDLER - api_put_reorder_lot: {}; {:?} {}",
        lot_id, payload, payload.purpose
    );
    let ctx = ctx.0;
    let lot = LotBmc::get(&ctx, &mm, lot_id).await?;
    let member_id = get_member_id(&ctx, &mm, lot.group_id, ctx.user_id()).await?;

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
        purpose: Some(payload.purpose),
        received_by: None,
        received_at: None,
        reference: lot.reference,
        requested_by: Some(member_id),
        requested_at: None,
        status: Some(0),
        url: lot.url,
    };

    let new_id = LotBmc::create(&ctx, &mm, new_lot).await?;

    let lot: Lot = LotBmc::get(&ctx, &mm, new_id).await?;
    Ok(Json(json!(lot)))
}

async fn api_lot_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(lot_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_lot_handler: {}", lot_id);
    let ctx = ctx.0;

    let lot: ViewLotDetails = ViewLotBmc::get_details(&ctx, &mm, lot_id).await?;
    Ok(Json(json!(lot)))
}

async fn api_lot_validations_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(lot_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_lot_handler: {}", lot_id);
    let ctx = ctx.0;

    let validation_filters: Vec<ValidationFilter> = serde_json::from_value(json!([
        {
            "lot_id": lot_id
        }
    ]))?;

    let validations: Vec<Validation> =
        ValidationBmc::list(&ctx, &mm, Some(validation_filters), None).await?;
    Ok(Json(json!(validations)))
}

async fn api_lots_handler(State(mm): State<ModelManager>, ctx: CtxW) -> Result<Json<Value>> {
    debug!("HANDLER - api_lots_handler");
    let ctx = ctx.0;

    let lots: Vec<Lot> = LotBmc::list(&ctx, &mm, None, None).await?;
    Ok(Json(json!(lots)))
}
