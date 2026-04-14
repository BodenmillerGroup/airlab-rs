use crate::config::web_config;
use crate::web::mw_auth::CtxW;
use crate::web::{Error, Result};
use airlab_lib::ctx::Ctx;
use airlab_lib::model::ModelManager;
use airlab_lib::model::member::{Member, MemberBmc, MemberFilter};
use airlab_lib::model::validation::{Validation, ValidationBmc};
use airlab_lib::model::validation_file::{
    ValidationFile, ValidationFileBmc, ValidationFileForCreate,
};
use axum::Router;
use axum::body::Body;
use axum::extract::{Multipart, Path, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use camino::Utf8PathBuf;
use serde_json::json;
use tokio::fs;
use tokio::fs::File;
use tokio::io::BufReader;
use tokio_util::io::ReaderStream;
#[allow(unused_imports)]
use tracing::{debug, warn};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route(
            "/api/v1/validationFiles/{file_id}/serve",
            get(api_serve_validation_handler),
        )
        .route(
            "/api/v1/validation_files/{file_id}/serve",
            get(api_serve_validation_handler),
        )
        .route(
            "/api/v1/validations/{validation_id}/validation_files",
            post(api_upload_validation_file_handler),
        )
        .with_state(mm)
}

async fn api_serve_validation_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(file_id): Path<i64>,
) -> Result<Response> {
    debug!("HANDLER - api_serve_validation_handler: {}", file_id);

    let ctx = ctx.0;
    let validation_file: ValidationFile = ValidationFileBmc::get(&ctx, &mm, file_id).await?;
    let validation: Validation =
        ValidationBmc::get(&ctx, &mm, validation_file.validation_id).await?;
    let data_path = web_config()?.DATA_PATH.clone();

    let file_path = Utf8PathBuf::from(format!(
        "{}/groups/{}/uploads/validation/{}/{}.{}",
        data_path,
        validation.group_id,
        validation.id,
        validation_file.hash,
        validation_file.extension
    ));

    if !file_path.is_file() {
        warn!("Cannot find the file: {}", file_path);
        return Ok((
            StatusCode::NOT_FOUND,
            format!("File not found: {file_path}"),
        )
            .into_response());
    }

    let file = match File::open(&file_path).await {
        Ok(file) => file,
        Err(err) => {
            return Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error reading file: {err}"),
            )
                .into_response());
        }
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static(content_type_for_extension(&validation_file.extension)),
    );
    if let Some(name) = validation_file.name.clone() {
        let disposition = format!("inline; filename=\"{}\"", name.replace('"', ""));
        if let Ok(value) = HeaderValue::from_str(&disposition) {
            headers.insert(header::CONTENT_DISPOSITION, value);
        }
    }

    let reader = BufReader::new(file);
    let stream = ReaderStream::new(reader);
    Ok((headers, Body::from_stream(stream)).into_response())
}

async fn api_upload_validation_file_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(validation_id): Path<i64>,
    mut multipart: Multipart,
) -> Result<Response> {
    let ctx = ctx.0;
    let validation = ValidationBmc::get(&ctx, &mm, validation_id).await?;
    let member_id = get_member_id(&ctx, &mm, validation.group_id, ctx.user_id()).await?;

    let mut uploaded_name: Option<String> = None;
    let mut uploaded_extension: Option<String> = None;
    let mut uploaded_bytes: Option<Vec<u8>> = None;
    let mut description: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|err| Error::BadRequest(format!("Unable to read multipart field: {err}")))?
    {
        let field_name = field.name().unwrap_or_default().to_string();
        if field_name == "description" {
            description = Some(field.text().await.map_err(|err| {
                Error::BadRequest(format!("Unable to read description field: {err}"))
            })?);
            continue;
        }

        if field_name != "file" {
            continue;
        }

        uploaded_name = field.file_name().map(ToOwned::to_owned);
        uploaded_extension = uploaded_name
            .as_deref()
            .and_then(|name| name.rsplit('.').next())
            .map(|ext| ext.trim().to_ascii_lowercase())
            .filter(|ext| !ext.is_empty());
        uploaded_bytes = Some(
            field
                .bytes()
                .await
                .map_err(|err| Error::BadRequest(format!("Unable to read uploaded file: {err}")))?
                .to_vec(),
        );
    }

    let file_bytes = uploaded_bytes
        .ok_or_else(|| Error::BadRequest("Missing multipart file field".to_string()))?;
    let file_name = uploaded_name.unwrap_or_else(|| "validation-file".to_string());
    let extension = uploaded_extension.unwrap_or_else(|| "bin".to_string());
    let hash = format!("{:x}", md5::compute(&file_bytes));

    let data_path = web_config()?.DATA_PATH.clone();
    let directory = Utf8PathBuf::from(format!(
        "{}/groups/{}/uploads/validation/{}",
        data_path, validation.group_id, validation.id
    ));
    fs::create_dir_all(directory.as_std_path()).await?;

    let file_path = directory.join(format!("{hash}.{extension}"));
    fs::write(file_path.as_std_path(), &file_bytes).await?;

    let file_id = ValidationFileBmc::create(
        &ctx,
        &mm,
        ValidationFileForCreate {
            validation_id: validation.id,
            created_by: member_id,
            hash,
            size: file_bytes.len() as i64,
            name: Some(file_name),
            extension,
            description,
            created_at: chrono::Utc::now(),
        },
    )
    .await?;

    let file = ValidationFileBmc::get(&ctx, &mm, file_id).await?;
    Ok((StatusCode::CREATED, axum::Json(json!(file))).into_response())
}

fn content_type_for_extension(ext: &str) -> &'static str {
    match ext.to_ascii_lowercase().as_str() {
        "pdf" => "application/pdf",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "txt" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

async fn get_member_id(ctx: &Ctx, mm: &ModelManager, group_id: i64, user_id: i64) -> Result<i64> {
    let filters: Vec<MemberFilter> = serde_json::from_value(json!([
        {
            "group_id": {"$eq": group_id},
            "user_id": {"$eq": user_id}
        }
    ]))?;

    let members: Vec<Member> = MemberBmc::list(ctx, mm, Some(filters), None).await?;
    members
        .into_iter()
        .next()
        .map(|member| member.id)
        .ok_or_else(|| {
            Error::BadRequest(format!(
                "No member found for group {group_id} and user {user_id}"
            ))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use airlab_lib::ctx::Ctx;
    use airlab_lib::model::validation_file::{ValidationFileBmc, ValidationFileForCreate};
    use tower::ServiceExt;

    type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    #[test]
    fn content_type_matches_common_extensions() {
        assert_eq!(content_type_for_extension("pdf"), "application/pdf");
        assert_eq!(content_type_for_extension("JPG"), "image/jpeg");
        assert_eq!(
            content_type_for_extension("unknown"),
            "application/octet-stream"
        );
    }

    #[tokio::test]
    async fn validation_file_route_returns_not_found_for_missing_blob() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let ctx = Ctx::root_ctx();
        let file_id = ValidationFileBmc::create(
            &ctx,
            &mm,
            ValidationFileForCreate {
                validation_id: 1011,
                created_by: 1303,
                hash: "missing-file".into(),
                size: 1,
                name: Some("missing.pdf".into()),
                extension: "pdf".into(),
                description: None,
                created_at: chrono::Utc::now(),
            },
        )
        .await?;
        let app = crate::web::test_support::authed_router(routes((*mm).clone()));

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri(format!("/api/v1/validation_files/{file_id}/serve"))
                    .body(axum::body::Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = crate::web::test_support::response_body_string(response).await?;
        assert!(body.contains("File not found"));

        Ok(())
    }
}
