use crate::web::Result;
use crate::web::mw_auth::CtxW;
use crate::web_config;
use crate::web_util::get_member_id;
use airlab_lib::model::ModelManager;
use airlab_lib::model::validation::{
    Validation, ValidationBmc, ValidationForCreate, ValidationForUpdate,
};
use airlab_lib::model::validation_file::{ValidationFileBmc, ValidationFileForCreate};
use airlab_lib::model::view_validation::{MinViewValidation, ViewValidationBmc};
use axum::extract::{Json as eJson, Multipart, Path, State};
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use camino::Utf8PathBuf;
use hex::encode;
use md5;
use serde_json::{Value, json};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
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
        .route(
            "/api/v1/validations/:validation_id/upload",
            post(api_post_validation_upload_handler),
        )
        .route("/api/v1/validations", get(api_validations_handler))
        .with_state(mm)
}

async fn ensure_parent_dirs(path: &Utf8PathBuf) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    Ok(())
}

async fn api_post_validation_upload_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(validation_id): Path<i32>,
    mut payload: Multipart,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_post_validation_handler: {:?}", payload);
    let mut group_id = String::new();
    let ctx = ctx.0;

    while let Some(field) = payload.next_field().await.unwrap() {
        let name = field.name().unwrap_or_default().to_string();

        if name == "groupId" {
            group_id = field.text().await.unwrap_or_default();
        } else if name == "file" {
            let file_name = field
                .file_name()
                .map(|s| s.to_string())
                .unwrap_or("upload.pdf".into());
            let content_type = field.content_type().unwrap_or("application/octet-stream");

            if content_type != "application/pdf" {
                return Ok(Json(json!("{}"))); // FIXME: return an error
            }
            let extension = "pdf";

            let data = field.bytes().await.unwrap();
            let data_len = data.len();
            let digest = md5::compute(&data);
            let md5_checksum = encode(*digest);
            let file_path = Utf8PathBuf::from(format!(
                "{}/groups/{}/uploads/validation/{}/{}.{}",
                &web_config().DATA_PATH,
                group_id,
                validation_id,
                md5_checksum,
                extension
            ));

            ensure_parent_dirs(&file_path).await?;
            debug!(
                "uploading file: {} -- {} -- {} -- {}",
                data_len, md5_checksum, file_name, file_path
            );
            let mut file = File::create(&file_path).await.unwrap();
            file.write(&data).await.unwrap();

            let group_id: i32 = group_id.parse().expect("Cannot");
            let created_by = get_member_id(&ctx, &mm, group_id, ctx.user_id()).await?;
            let upd = ValidationFileForCreate {
                validation_id,
                created_by,
                hash: md5_checksum,
                size: data_len as i32,
                description: None,
                extension: extension.into(),
                created_at: chrono::offset::Utc::now(),
                name: Some(file_name),
            };
            ValidationFileBmc::create(&ctx, &mm, upd).await?;
        }
    }

    Ok(Json(json!("{}")))
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
