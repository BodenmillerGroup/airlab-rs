use crate::web::Result;
use crate::web::mw_auth::CtxW;
use crate::web_config;
use airlab_lib::model::ModelManager;
use airlab_lib::model::validation::{Validation, ValidationBmc};
use airlab_lib::model::validation_file::{
    ValidationFile, ValidationFileBmc, ValidationFileForCreate, ValidationFileForUpdate,
};
use axum::body::Body;
use axum::extract::{Json as eJson, Path, State};
use axum::routing::{delete, get, patch, post};
use axum::{Json, Router};
use camino::Utf8PathBuf;
use serde_json::{Value, json};
use tokio::{fs::File, io::BufReader, io::ErrorKind};
use tokio_util::io::ReaderStream;
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route(
            "/api/v1/validation_files",
            post(api_post_validation_file_handler),
        )
        .route(
            "/api/v1/validation_files/:validation_file_id",
            patch(api_patch_validation_file_handler),
        )
        .route(
            "/api/v1/validation_files/:validation_file_id",
            get(api_validation_file_handler),
        )
        .route(
            "/api/v1/validation_files",
            get(api_validation_files_handler),
        )
        .route(
            "/api/v1/validationFiles/:validation_file_id",
            delete(api_delete_validation_file_handler),
        )
        .route(
            "/api/v1/validationFiles/:file_id/serve",
            get(api_serve_validation_handler),
        )
        .with_state(mm)
}

async fn api_post_validation_file_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    eJson(payload): eJson<ValidationFileForCreate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_post_validation_file_handler: {:?}", payload);
    let ctx = ctx.0;
    let v_file_id = ValidationFileBmc::create(&ctx, &mm, payload).await?;

    let validation_file: ValidationFile = ValidationFileBmc::get(&ctx, &mm, v_file_id).await?;
    Ok(Json(json!(validation_file)))
}

async fn api_delete_validation_file_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(v_file_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!(
        "HANDLER - api_delete_validation_file_handler: {:?}",
        v_file_id
    );
    let ctx = ctx.0;
    ValidationFileBmc::delete(&ctx, &mm, v_file_id).await?;
    // not deleting the file

    Ok(Json(json!(v_file_id)))
}

async fn api_patch_validation_file_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(v_file_id): Path<i32>,
    eJson(payload): eJson<ValidationFileForUpdate>,
) -> Result<Json<Value>> {
    debug!(
        "HANDLER - api_validation_file: {}; {:?}",
        v_file_id, payload
    );
    let ctx = ctx.0;

    ValidationFileBmc::update(&ctx, &mm, v_file_id, payload).await?;

    let validation_file: ValidationFile = ValidationFileBmc::get(&ctx, &mm, v_file_id).await?;
    Ok(Json(json!(validation_file)))
}

async fn api_validation_file_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(v_file_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_validation_file: {}", v_file_id);
    let ctx = ctx.0;

    let validation_file: ValidationFile = ValidationFileBmc::get(&ctx, &mm, v_file_id).await?;
    Ok(Json(json!(validation_file)))
}

async fn api_serve_validation_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(file_id): Path<i32>,
) -> Result<Body> {
    debug!("HANDLER - api_serve_validation_handler: {}", file_id);

    let ctx = ctx.0;
    let validation_file: ValidationFile = ValidationFileBmc::get(&ctx, &mm, file_id).await?;
    let validation: Validation =
        ValidationBmc::get(&ctx, &mm, validation_file.validation_id).await?;

    let file_path = Utf8PathBuf::from(format!(
        "{}/groups/{}/uploads/validation/{}/{}.{}",
        &web_config().DATA_PATH,
        validation.group_id,
        validation.id,
        validation_file.hash,
        validation_file.extension
    ));
    if !file_path.is_file() {
        println!("Cannot find the file: {file_path}");
    }
    match File::open(&file_path).await {
        Ok(file) => {
            let reader = BufReader::new(file);
            let stream = ReaderStream::new(reader);
            Ok(Body::from_stream(stream))
        }
        Err(e) if e.kind() == ErrorKind::NotFound => {
            let error_message = format!("File not found: ({file_path})");
            Ok(Body::from(error_message))
        }
        Err(e) => {
            let error_message = format!("error reading file: {e}");
            Ok(Body::from(error_message))
        }
    }
}

async fn api_validation_files_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_validation_files_handler");
    let ctx = ctx.0;

    let validation_files: Vec<ValidationFile> =
        ValidationFileBmc::list(&ctx, &mm, None, None).await?;
    Ok(Json(json!(validation_files)))
}
