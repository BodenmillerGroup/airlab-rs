use crate::web::mw_auth::CtxW;
use crate::web::Result;
use crate::web_util::get_member_id;
use axum::extract::{Json as eJson, Path, State};
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use lib_core::model::clone::CloneFilter;
use lib_core::model::protein::{Protein, ProteinBmc, ProteinForCreate, ProteinForUpdate};
use lib_core::model::view_clone::{ViewClone, ViewCloneBmc};
use lib_core::model::ModelManager;
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/v1/proteins", post(api_post_protein_handler))
        .route(
            "/api/v1/proteins/:protein_id/clones",
            get(api_protein_clones_handler),
        )
        .route(
            "/api/v1/proteins/:protein_id",
            patch(api_patch_protein_handler),
        )
        .route("/api/v1/proteins/:protein_id", get(api_protein_handler))
        .route("/api/v1/proteins", get(api_proteins_handler))
        .with_state(mm)
}

async fn api_protein_clones_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(protein_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_protein_clones_handler");
    let ctx = ctx.0;

    let filters: Vec<CloneFilter> = serde_json::from_value(json!([
        {
            "protein_id": protein_id
        }
    ]))?;

    let clones: Vec<ViewClone> = ViewCloneBmc::list(&ctx, &mm, None, Some(filters), None).await?;
    Ok(Json(json!(clones)))
}

#[derive(Deserialize, Debug)]
struct PatchProtein {
    name: Option<String>,
    #[serde(rename = "groupId")]
    group_id: Option<i32>,
    description: Option<String>,
}

async fn api_post_protein_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    eJson(payload): eJson<PatchProtein>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_post_protein_handler: {:?}", payload);
    let ctx = ctx.0;
    let name = payload.name.unwrap_or_default();
    let group_id = payload.group_id.unwrap_or_default();
    let created_by = get_member_id(&ctx, &mm, group_id, ctx.user_id()).await?;

    let protein_id = ProteinBmc::create(
        &ctx,
        &mm,
        ProteinForCreate {
            name,
            group_id,
            description: payload.description,
            created_by: created_by as i32,
        },
    )
    .await?;

    let protein: Protein = ProteinBmc::get(&ctx, &mm, protein_id).await?;
    Ok(Json(json!(protein)))
}

async fn api_patch_protein_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(protein_id): Path<i32>,
    eJson(payload): eJson<PatchProtein>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_protein: {}; {:?}", protein_id, payload);
    let ctx = ctx.0;

    ProteinBmc::update(
        &ctx,
        &mm,
        protein_id,
        ProteinForUpdate {
            name: payload.name,
            description: payload.description,
        },
    )
    .await?;

    let protein: Protein = ProteinBmc::get(&ctx, &mm, protein_id).await?;
    Ok(Json(json!(protein)))
}

async fn api_protein_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(protein_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_protein_handler: {}", protein_id);
    let ctx = ctx.0;

    let protein: Protein = ProteinBmc::get(&ctx, &mm, protein_id).await?;
    Ok(Json(json!(protein)))
}

async fn api_proteins_handler(State(mm): State<ModelManager>, ctx: CtxW) -> Result<Json<Value>> {
    debug!("HANDLER - api_proteins_handler");
    let ctx = ctx.0;

    let proteins: Vec<Protein> = ProteinBmc::list(&ctx, &mm, None, None).await?;
    Ok(Json(json!(proteins)))
}
