use crate::web::mw_auth::CtxW;
use crate::web::Result;
use airlab_lib::model::clone::CloneFilter;
use airlab_lib::model::species::{Species, SpeciesBmc, SpeciesForCreate, SpeciesForUpdate};
use airlab_lib::model::view_clone::{ViewClone, ViewCloneBmc};
use airlab_lib::model::ModelManager;
use axum::extract::{Json as eJson, Path, State};
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use serde_json::{json, Value};
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/v1/species", post(api_post_species_handler))
        .route(
            "/api/v1/species/:provider_id/clones",
            get(api_species_clones_handler),
        )
        .route(
            "/api/v1/species/:species_id",
            patch(api_patch_species_handler),
        )
        .route("/api/v1/species/:species_id", get(api_specie_handler))
        .route("/api/v1/species", get(api_species_handler))
        .with_state(mm)
}

async fn api_species_clones_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(species_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_species_clones_handler");
    let ctx = ctx.0;

    let filters: Vec<CloneFilter> = serde_json::from_value(json!([
        {
            "species_id": species_id
        }
    ]))?;

    let clones: Vec<ViewClone> = ViewCloneBmc::list(&ctx, &mm, None, Some(filters), None).await?;
    Ok(Json(json!(clones)))
}

async fn api_post_species_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    eJson(payload): eJson<SpeciesForCreate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_post_species_handler: {:?}", payload);
    let ctx = ctx.0;
    let species_id = SpeciesBmc::create(&ctx, &mm, payload).await?;

    let species: Species = SpeciesBmc::get(&ctx, &mm, species_id).await?;
    Ok(Json(json!(species)))
}

async fn api_patch_species_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(species_id): Path<i32>,
    eJson(payload): eJson<SpeciesForUpdate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_species: {}; {:?}", species_id, payload);
    let ctx = ctx.0;

    SpeciesBmc::update(&ctx, &mm, species_id, payload).await?;

    let species: Species = SpeciesBmc::get(&ctx, &mm, species_id).await?;
    Ok(Json(json!(species)))
}

async fn api_specie_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(species_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_species_handler: {}", species_id);
    let ctx = ctx.0;

    let species: Species = SpeciesBmc::get(&ctx, &mm, species_id).await?;
    Ok(Json(json!(species)))
}

async fn api_species_handler(State(mm): State<ModelManager>, ctx: CtxW) -> Result<Json<Value>> {
    debug!("HANDLER - api_species_handler");
    let ctx = ctx.0;

    let species: Vec<Species> = SpeciesBmc::list(&ctx, &mm, None, None).await?;
    Ok(Json(json!(species)))
}
