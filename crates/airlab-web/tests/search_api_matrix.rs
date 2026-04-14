use airlab_lib::_dev_utils;
use airlab_lib::ctx::Ctx;
use airlab_web::web::mw_auth::{CtxExtError, CtxW};
use airlab_web::web::routes_search::routes;
use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::Request;
use axum::middleware::{self, Next};
use axum::response::Response;
use serde_json::{Value, json};
use std::fs;
use std::sync::Once;
use tower::ServiceExt;

type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

struct SearchCase {
    name: &'static str,
    request: Value,
    expected_ids: Vec<i64>,
    expected_total: i64,
}

fn init_web_test_env() {
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        _dev_utils::init_test_env();
    });

    let root = std::env::temp_dir().join("airlab-web-integration-test-assets");
    let web_dir = root.join("web");
    let data_dir = root.join("data");
    let _ = fs::create_dir_all(&web_dir);
    let _ = fs::create_dir_all(&data_dir);
    let _ = fs::write(web_dir.join("index.html"), "<html>airlab-test</html>");

    for (key, value) in [
        ("SERVICE_HOST_PORT", "3000"),
        ("SERVICE_HOST_ADDR", "127.0.0.1"),
        ("SERVICE_WEB_FOLDER", web_dir.to_string_lossy().as_ref()),
        ("SERVICE_EMAIL_FROM_ADDRESS", "noreply@example.test"),
        ("SERVICE_EMAIL_FROM_NAME", "Airlab"),
        ("SERVICE_EMAIL_TOKEN", "token"),
        ("SERVICE_EMAIL_ADDRESS", "smtp.example.test"),
        ("SERVICE_LOG_AGGR_URL", "https://logs.example.test"),
        ("SERVICE_RESET_PWD_URL", "https://example.test/reset"),
        ("SERVICE_DATA_PATH", data_dir.to_string_lossy().as_ref()),
        ("SUPER_USER", "admin@example.test"),
        ("SUPER_USER_PWD", "secret"),
        ("SETUP_DEMO_GROUP", "false"),
    ] {
        // SAFETY: test setup controls these process env vars.
        unsafe { std::env::set_var(key, value) };
    }
}

async fn init_test_db() -> _dev_utils::TestDb {
    init_web_test_env();
    _dev_utils::init_test().await
}

fn authed_router(router: Router) -> Router {
    router.layer(middleware::from_fn(inject_test_ctx))
}

async fn inject_test_ctx(mut req: Request<Body>, next: Next) -> Response {
    if let Ok(ctx) = Ctx::new(1) {
        req.extensions_mut()
            .insert::<core::result::Result<CtxW, CtxExtError>>(Ok(CtxW(ctx)));
    }
    next.run(req).await
}

async fn response_body_string(response: Response) -> TestResult<String> {
    let bytes = to_bytes(response.into_body(), usize::MAX).await?;
    Ok(String::from_utf8(bytes.to_vec())?)
}

async fn post_search(app: &Router, request: Value, case_name: &str) -> TestResult<Value> {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/search")
                .header(axum::http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(request.to_string()))?,
        )
        .await?;

    let body = response_body_string(response).await?;
    let value: Value = serde_json::from_str(&body)?;
    assert!(
        value.get("error").is_none(),
        "case {case_name} returned error body: {body}"
    );
    assert!(
        value.get("items").is_some(),
        "case {case_name} returned unexpected body: {body}"
    );
    Ok(value)
}

fn item_ids(response: &Value) -> TestResult<Vec<i64>> {
    response["items"]
        .as_array()
        .ok_or_else(|| std::io::Error::other("items should be an array"))?
        .iter()
        .map(|item| {
            item.as_i64()
                .ok_or_else(|| std::io::Error::other("items should contain ids").into())
        })
        .collect()
}

async fn assert_case(app: &Router, case: &SearchCase) -> TestResult {
    let response = post_search(app, case.request.clone(), case.name).await?;
    assert_eq!(
        item_ids(&response)?,
        case.expected_ids,
        "case {}",
        case.name
    );
    assert_eq!(
        response["search_total"],
        json!(case.expected_total),
        "case {}",
        case.name
    );
    Ok(())
}

#[tokio::test]
async fn search_smoke_matrix_covers_all_return_types() -> TestResult {
    let mm = init_test_db().await;
    let app = authed_router(routes(airlab_web::search_shadow::SearchState::new(
        (*mm).clone(),
    )));

    let cases = vec![
        SearchCase {
            name: "user by exact id",
            request: json!({
                "return_type": "User",
                "filters": [
                    {"table": "User", "field": "id", "op": "eq", "value": 1001}
                ],
                "order": {"table": "User", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1001],
            expected_total: 1,
        },
        SearchCase {
            name: "member via joined group and user",
            request: json!({
                "return_type": "Member",
                "filters": [
                    {"table": "Group", "field": "name", "op": "contains", "value": "seed group"},
                    {"table": "User", "field": "name", "op": "contains", "value": "Seed Owner"}
                ],
                "order": {"table": "Member", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1303],
            expected_total: 1,
        },
        SearchCase {
            name: "group by seed name",
            request: json!({
                "return_type": "Group",
                "filters": [
                    {"table": "Group", "field": "name", "op": "contains", "value": "seed group"}
                ],
                "order": {"table": "Group", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1000],
            expected_total: 1,
        },
        SearchCase {
            name: "protein by seed name",
            request: json!({
                "return_type": "Protein",
                "filters": [
                    {"table": "Protein", "field": "name", "op": "contains", "value": "seed-protein"}
                ],
                "order": {"table": "Protein", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1002],
            expected_total: 1,
        },
        SearchCase {
            name: "provider by seed name",
            request: json!({
                "return_type": "Provider",
                "filters": [
                    {"table": "Provider", "field": "name", "op": "contains", "value": "seed-provider"}
                ],
                "order": {"table": "Provider", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1003],
            expected_total: 1,
        },
        SearchCase {
            name: "species by exact name",
            request: json!({
                "return_type": "Species",
                "filters": [
                    {"table": "Species", "field": "name", "op": "contains", "value": "Mouse"}
                ],
                "order": {"table": "Species", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1004],
            expected_total: 1,
        },
        SearchCase {
            name: "tag by seed name",
            request: json!({
                "return_type": "Tag",
                "filters": [
                    {"table": "Tag", "field": "name", "op": "contains", "value": "seed-tag"}
                ],
                "order": {"table": "Tag", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1005],
            expected_total: 1,
        },
        SearchCase {
            name: "clone by seed name",
            request: json!({
                "return_type": "Clone",
                "filters": [
                    {"table": "Clone", "field": "group_id", "op": "eq", "value": 1000},
                    {"table": "Clone", "field": "name", "op": "contains", "value": "seed-clone"}
                ],
                "order": {"table": "Clone", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1006],
            expected_total: 1,
        },
        SearchCase {
            name: "lot via provider join",
            request: json!({
                "return_type": "Lot",
                "filters": [
                    {"table": "Provider", "field": "name", "op": "contains", "value": "seed-provider"}
                ],
                "order": {"table": "Lot", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1007],
            expected_total: 1,
        },
        SearchCase {
            name: "conjugate via tag join",
            request: json!({
                "return_type": "Conjugate",
                "filters": [
                    {"table": "Tag", "field": "name", "op": "contains", "value": "seed-tag"}
                ],
                "order": {"table": "Conjugate", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1008],
            expected_total: 1,
        },
        SearchCase {
            name: "panel by seed name",
            request: json!({
                "return_type": "Panel",
                "filters": [
                    {"table": "Panel", "field": "name", "op": "contains", "value": "seed-panel"}
                ],
                "order": {"table": "Panel", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1009],
            expected_total: 1,
        },
        SearchCase {
            name: "panel element via panel and conjugate joins",
            request: json!({
                "return_type": "PanelElement",
                "filters": [
                    {"table": "Panel", "field": "name", "op": "contains", "value": "backup-panel"},
                    {"table": "Conjugate", "field": "description", "op": "contains", "value": "seed-conjugate"}
                ],
                "order": {"table": "PanelElement", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1021],
            expected_total: 1,
        },
        SearchCase {
            name: "validation by seed tissue",
            request: json!({
                "return_type": "Validation",
                "filters": [
                    {"table": "Validation", "field": "tissue", "op": "contains", "value": "seed-tissue"}
                ],
                "order": {"table": "Validation", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1011],
            expected_total: 1,
        },
        SearchCase {
            name: "storage by exact seed name",
            request: json!({
                "return_type": "Storage",
                "filters": [
                    {"table": "Storage", "field": "name", "op": "contains", "value": "seed-storage-a"}
                ],
                "order": {"table": "Storage", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1201],
            expected_total: 1,
        },
        SearchCase {
            name: "collection by exact seed name",
            request: json!({
                "return_type": "Collection",
                "filters": [
                    {"table": "Collection", "field": "name", "op": "contains", "value": "seed-collection-a"}
                ],
                "order": {"table": "Collection", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1101],
            expected_total: 1,
        },
    ];

    for case in &cases {
        assert_case(&app, case).await?;
    }

    Ok(())
}

#[tokio::test]
async fn search_sorting_matrix_covers_all_return_types() -> TestResult {
    let mm = init_test_db().await;
    let app = authed_router(routes(airlab_web::search_shadow::SearchState::new(
        (*mm).clone(),
    )));

    let asc_cases = vec![
        SearchCase {
            name: "user asc",
            request: json!({
                "return_type": "User",
                "filters": [
                    {"table": "User", "field": "id", "op": "eq", "value": [1002, 1001]}
                ],
                "order": {"table": "User", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1001, 1002],
            expected_total: 2,
        },
        SearchCase {
            name: "member asc by joined user name",
            request: json!({
                "return_type": "Member",
                "filters": [
                    {"table": "Group", "field": "name", "op": "contains", "value": "seed group"}
                ],
                "order": {"table": "User", "field": "name", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1000, 1304, 1303],
            expected_total: 3,
        },
        SearchCase {
            name: "group asc",
            request: json!({
                "return_type": "Group",
                "filters": [
                    {"table": "Group", "field": "name", "op": "contains", "value": "group"}
                ],
                "order": {"table": "Group", "field": "name", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1, 1000],
            expected_total: 2,
        },
        SearchCase {
            name: "protein asc",
            request: json!({
                "return_type": "Protein",
                "filters": [
                    {"table": "Group", "field": "name", "op": "contains", "value": "seed group"}
                ],
                "order": {"table": "Protein", "field": "name", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1012, 1002],
            expected_total: 2,
        },
        SearchCase {
            name: "provider asc",
            request: json!({
                "return_type": "Provider",
                "filters": [
                    {"table": "Group", "field": "name", "op": "contains", "value": "seed group"}
                ],
                "order": {"table": "Provider", "field": "name", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1013, 1003],
            expected_total: 2,
        },
        SearchCase {
            name: "species asc",
            request: json!({
                "return_type": "Species",
                "filters": [
                    {"table": "Group", "field": "name", "op": "contains", "value": "seed group"}
                ],
                "order": {"table": "Species", "field": "name", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1004, 1014],
            expected_total: 2,
        },
        SearchCase {
            name: "tag asc",
            request: json!({
                "return_type": "Tag",
                "filters": [
                    {"table": "Group", "field": "name", "op": "contains", "value": "seed group"}
                ],
                "order": {"table": "Tag", "field": "name", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1015, 1005],
            expected_total: 2,
        },
        SearchCase {
            name: "clone asc",
            request: json!({
                "return_type": "Clone",
                "filters": [
                    {"table": "Clone", "field": "group_id", "op": "eq", "value": 1000},
                    {"table": "Group", "field": "name", "op": "contains", "value": "seed group"}
                ],
                "order": {"table": "Clone", "field": "name", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1016, 1006],
            expected_total: 2,
        },
        SearchCase {
            name: "lot asc",
            request: json!({
                "return_type": "Lot",
                "filters": [
                    {"table": "Group", "field": "name", "op": "contains", "value": "seed group"}
                ],
                "order": {"table": "Lot", "field": "name", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1018, 1007],
            expected_total: 2,
        },
        SearchCase {
            name: "conjugate asc",
            request: json!({
                "return_type": "Conjugate",
                "filters": [
                    {"table": "Group", "field": "name", "op": "contains", "value": "seed group"}
                ],
                "order": {"table": "Conjugate", "field": "description", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1019, 1008],
            expected_total: 2,
        },
        SearchCase {
            name: "panel asc",
            request: json!({
                "return_type": "Panel",
                "filters": [
                    {"table": "Panel", "field": "group_id", "op": "eq", "value": 1000}
                ],
                "order": {"table": "Panel", "field": "name", "direction": "asc"},
                "show_all": true,
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1020, 1009],
            expected_total: 2,
        },
        SearchCase {
            name: "panel element asc by joined conjugate description",
            request: json!({
                "return_type": "PanelElement",
                "filters": [
                    {"table": "Panel", "field": "name", "op": "contains", "value": "backup-panel"}
                ],
                "order": {"table": "Conjugate", "field": "description", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1023, 1021],
            expected_total: 2,
        },
        SearchCase {
            name: "validation asc",
            request: json!({
                "return_type": "Validation",
                "filters": [
                    {"table": "Group", "field": "name", "op": "contains", "value": "seed group"}
                ],
                "order": {"table": "Validation", "field": "tissue", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1024, 1011],
            expected_total: 2,
        },
        SearchCase {
            name: "storage asc",
            request: json!({
                "return_type": "Storage",
                "filters": [
                    {"table": "Storage", "field": "name", "op": "contains", "value": "storage"}
                ],
                "order": {"table": "Storage", "field": "temperature_c", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1202, 1201],
            expected_total: 2,
        },
        SearchCase {
            name: "collection asc",
            request: json!({
                "return_type": "Collection",
                "filters": [
                    {"table": "Collection", "field": "name", "op": "contains", "value": "seed-collection"}
                ],
                "order": {"table": "Collection", "field": "name", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1101, 1102],
            expected_total: 2,
        },
    ];

    for case in &asc_cases {
        assert_case(&app, case).await?;
    }

    let desc_cases = vec![
        SearchCase {
            name: "user desc",
            request: json!({
                "return_type": "User",
                "filters": [
                    {"table": "User", "field": "id", "op": "eq", "value": [1002, 1001]}
                ],
                "order": {"table": "User", "field": "id", "direction": "desc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1002, 1001],
            expected_total: 2,
        },
        SearchCase {
            name: "member desc by joined user name",
            request: json!({
                "return_type": "Member",
                "filters": [
                    {"table": "Group", "field": "name", "op": "contains", "value": "seed group"}
                ],
                "order": {"table": "User", "field": "name", "direction": "desc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1303, 1304, 1000],
            expected_total: 3,
        },
        SearchCase {
            name: "provider desc",
            request: json!({
                "return_type": "Provider",
                "filters": [
                    {"table": "Group", "field": "name", "op": "contains", "value": "seed group"}
                ],
                "order": {"table": "Provider", "field": "name", "direction": "desc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1003, 1013],
            expected_total: 2,
        },
        SearchCase {
            name: "panel element desc by joined conjugate description",
            request: json!({
                "return_type": "PanelElement",
                "filters": [
                    {"table": "Panel", "field": "name", "op": "contains", "value": "backup-panel"}
                ],
                "order": {"table": "Conjugate", "field": "description", "direction": "desc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1021, 1023],
            expected_total: 2,
        },
        SearchCase {
            name: "storage desc",
            request: json!({
                "return_type": "Storage",
                "filters": [
                    {"table": "Storage", "field": "name", "op": "contains", "value": "storage"}
                ],
                "order": {"table": "Storage", "field": "temperature_c", "direction": "desc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1201, 1202],
            expected_total: 2,
        },
    ];

    for case in &desc_cases {
        assert_case(&app, case).await?;
    }

    Ok(())
}

#[tokio::test]
async fn search_filter_matrix_covers_joined_and_value_variants() -> TestResult {
    let mm = init_test_db().await;
    let app = authed_router(routes(airlab_web::search_shadow::SearchState::new(
        (*mm).clone(),
    )));

    let cases = vec![
        SearchCase {
            name: "provider generic array id filter",
            request: json!({
                "return_type": "Provider",
                "filters": [
                    {"table": "Provider", "field": "id", "op": "eq", "value": [1013, 1003]}
                ],
                "order": {"table": "Provider", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1003, 1013],
            expected_total: 2,
        },
        SearchCase {
            name: "storage bool filter",
            request: json!({
                "return_type": "Storage",
                "filters": [
                    {"table": "Storage", "field": "active", "op": "eq", "value": true}
                ],
                "order": {"table": "Storage", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1201],
            expected_total: 1,
        },
        SearchCase {
            name: "validation numeric filter",
            request: json!({
                "return_type": "Validation",
                "filters": [
                    {"table": "Validation", "field": "status", "op": "eq", "value": 2}
                ],
                "order": {"table": "Validation", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1024],
            expected_total: 1,
        },
        SearchCase {
            name: "lot joined collection filter",
            request: json!({
                "return_type": "Lot",
                "filters": [
                    {"table": "Collection", "field": "name", "op": "contains", "value": "seed-collection-a"}
                ],
                "order": {"table": "Lot", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1007],
            expected_total: 1,
        },
        SearchCase {
            name: "conjugate joined collection filter through lot",
            request: json!({
                "return_type": "Conjugate",
                "filters": [
                    {"table": "Collection", "field": "name", "op": "contains", "value": "seed-collection-a"}
                ],
                "order": {"table": "Conjugate", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1008],
            expected_total: 1,
        },
        SearchCase {
            name: "panel element joined tag filter",
            request: json!({
                "return_type": "PanelElement",
                "filters": [
                    {"table": "Panel", "field": "name", "op": "contains", "value": "backup-panel"},
                    {"table": "Tag", "field": "name", "op": "contains", "value": "seed-tag"}
                ],
                "order": {"table": "PanelElement", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1021],
            expected_total: 1,
        },
        SearchCase {
            name: "validation joined clone filter",
            request: json!({
                "return_type": "Validation",
                "filters": [
                    {"table": "Clone", "field": "name", "op": "contains", "value": "backup-clone"}
                ],
                "order": {"table": "Validation", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1024],
            expected_total: 1,
        },
        SearchCase {
            name: "clone reactivity scalar contains filter",
            request: json!({
                "return_type": "Clone",
                "filters": [
                    {"table": "Clone", "field": "group_id", "op": "eq", "value": 1000},
                    {"table": "Clone", "field": "reactivity", "op": "contains", "value": 7001}
                ],
                "order": {"table": "Clone", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1006],
            expected_total: 1,
        },
        SearchCase {
            name: "clone reactivity overlap filter",
            request: json!({
                "return_type": "Clone",
                "filters": [
                    {"table": "Clone", "field": "group_id", "op": "eq", "value": 1000},
                    {"table": "Clone", "field": "reactivity", "op": "contains", "value": [7200, 9999]}
                ],
                "order": {"table": "Clone", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1016],
            expected_total: 1,
        },
        SearchCase {
            name: "disallowed provider filter is skipped",
            request: json!({
                "return_type": "Provider",
                "filters": [
                    {"table": "Tag", "field": "name", "op": "contains", "value": "seed-tag"}
                ],
                "order": {"table": "Provider", "field": "id", "direction": "asc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![103, 1003, 1013],
            expected_total: 3,
        },
        SearchCase {
            name: "disallowed provider order is skipped",
            request: json!({
                "return_type": "Provider",
                "filters": [
                    {"table": "Group", "field": "name", "op": "contains", "value": "seed group"}
                ],
                "order": {"table": "Tag", "field": "name", "direction": "desc"},
                "page": 1,
                "limit": 10
            }),
            expected_ids: vec![1003, 1013],
            expected_total: 2,
        },
    ];

    for case in &cases {
        assert_case(&app, case).await?;
    }

    let paged = post_search(
        &app,
        json!({
            "return_type": "Provider",
            "filters": [
                {"table": "Group", "field": "name", "op": "contains", "value": "seed group"}
            ],
            "order": {"table": "Provider", "field": "name", "direction": "asc"},
            "page": 2,
            "limit": 1
        }),
        "provider pagination",
    )
    .await?;

    assert_eq!(item_ids(&paged)?, vec![1003]);
    assert_eq!(paged["search_total"], 2);
    assert_eq!(paged["has_previous"], true);
    assert_eq!(paged["has_next"], false);

    Ok(())
}
