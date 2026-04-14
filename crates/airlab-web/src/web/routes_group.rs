use crate::web::Result;
use crate::web::mw_auth::CtxW;
use airlab_lib::model;
use airlab_lib::model::ModelManager;
use airlab_lib::model::clone::{Clone, CloneBmc, CloneFilter};
use airlab_lib::model::conjugate::{Conjugate, ConjugateBmc, ConjugateFilter};
use airlab_lib::model::group::{Group, GroupBmc, GroupForCreate, GroupForUpdate};
use airlab_lib::model::lot::{Lot, LotBmc, LotFilter};
use airlab_lib::model::member::{Member, MemberBmc, MemberFilter, MemberForCreate};
use airlab_lib::model::panel::{Panel, PanelBmc, PanelFilter};
use airlab_lib::model::panel_element::{PanelElement, PanelElementBmc};
use airlab_lib::model::protein::{Protein, ProteinBmc, ProteinFilter};
use airlab_lib::model::provider::{Provider, ProviderBmc, ProviderFilter};
use airlab_lib::model::species::{Species, SpeciesBmc, SpeciesFilter};
use airlab_lib::model::tag::{Tag, TagBmc, TagFilter};
use airlab_lib::model::validation::{Validation, ValidationBmc, ValidationFilter};
use airlab_lib::model::validation_file::{ValidationFile, ValidationFileBmc};
use axum::extract::{Json as eJson, Path, Query, State};
use axum::http::Uri;
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use modql::filter::{ListOptions, OrderBy, OrderBys};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
#[allow(unused_imports)]
use tracing::{debug, info, warn};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/{*all}", get(api_handler))
        .route("/api/v1/groups/", post(api_post_group_handler))
        .route("/api/v1/groups/{group_id}", patch(api_patch_group_handler))
        .route("/api/v1/groups/{group_id}", get(api_group_handler))
        .route(
            "/api/v1/groups/{group_id}/members/me",
            get(api_group_me_handler),
        )
        .route(
            "/api/v1/groups/{group_id}/members",
            get(api_group_members_handler),
        )
        .route("/api/v1/groups", get(api_groups_handler))
        .route(
            "/api/v1/groups/{group_id}/proteins",
            get(api_group_proteins_handler),
        )
        .route(
            "/api/v1/groups/{group_id}/oldproteins",
            get(api_group_oldproteins_handler),
        )
        .route(
            "/api/v1/groups/{group_id}/tags",
            get(api_group_tags_handler),
        )
        .route(
            "/api/v1/groups/{group_id}/validation_files",
            get(api_group_validation_files_handler),
        )
        .route(
            "/api/v1/groups/{group_id}/panel_elements",
            get(api_group_panel_elements_handler),
        )
        .route(
            "/api/v1/groups/{group_id}/validations",
            get(api_group_validations_handler),
        )
        .route(
            "/api/v1/groups/{group_id}/panels",
            get(api_group_panels_handler),
        )
        .route(
            "/api/v1/groups/{group_id}/lots",
            get(api_group_lots_handler),
        )
        .route(
            "/api/v1/groups/{group_id}/conjugates",
            get(api_group_conjugates_handler),
        )
        .route(
            "/api/v1/groups/{group_id}/clones",
            get(api_group_clones_handler),
        )
        .route(
            "/api/v1/groups/{group_id}/species",
            get(api_group_species_handler),
        )
        .route(
            "/api/v1/groups/{group_id}/providers",
            get(api_group_providers_handler),
        )
        .with_state(mm)
}

async fn api_post_group_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    eJson(payload): eJson<GroupForCreate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_post_group_handler: {:?}", payload);
    let ctx = ctx.0;
    let user_id = ctx.user_id();
    let group_id = GroupBmc::create(&ctx, &mm, payload).await?;
    let member_c = MemberForCreate {
        group_id,
        user_id,
        is_active: true,
        all_panels: true,
        role: 111,
        activation_key: None,
    };
    let _ = MemberBmc::create(&ctx, &mm, member_c).await?;

    let group: Group = GroupBmc::get(&ctx, &mm, group_id).await?;
    Ok(Json(json!(group)))
}

async fn api_handler(State(_mm): State<ModelManager>, _ctx: CtxW, uri: Uri) -> Result<Json<Value>> {
    warn!("MISSING - route missing: {:?}", uri);
    Ok(Json(json!("{}")))
}

async fn api_patch_group_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i64>,
    eJson(payload): eJson<GroupForUpdate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_group_handler: {}; {:?}", group_id, payload);
    let ctx = ctx.0;

    GroupBmc::update(&ctx, &mm, group_id, payload).await?;

    let group: Group = GroupBmc::get(&ctx, &mm, group_id).await?;
    Ok(Json(json!(group)))
}

#[derive(Deserialize)]
struct PaginationParams {
    page: Option<u32>,
    limit: Option<u32>,
    search: Option<String>,
}

#[derive(Serialize)]
struct PaginatedResponse<T> {
    items: Vec<T>,
    total: i64,
    page: u32,
    limit: u32,
    has_next: bool,
    has_previous: bool,
}

async fn api_group_proteins_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i64>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Protein>>> {
    let limit = params.limit.unwrap_or(5);
    let page = params.page.unwrap_or(1);
    let offset: usize = (page as usize - 1) * limit as usize;
    debug!("HANDLER - api_group_proteins_handler");
    let ctx = ctx.0;

    let filters: Vec<ProteinFilter> = match &params.search {
        Some(search) => serde_json::from_value(json!([
            {
                "group_id": {"$eq":group_id },
                "description": {"$contains":search},

            }
        ]))?,
        None => serde_json::from_value(json!([
            {
                "group_id": {"$eq":group_id }
            }
        ]))?,
    };

    let op = ListOptions {
        order_bys: Some(OrderBys::new(vec![OrderBy::Desc("id".into())])),
        limit: Some(limit as i64),
        offset: Some(offset as i64),
    };
    let items: Vec<Protein> = ProteinBmc::list(&ctx, &mm, Some(filters.clone()), Some(op)).await?;

    let total = ProteinBmc::count(&ctx, &mm, Some(filters.clone())).await?;
    let has_next = offset + items.len() < total as usize;
    debug!(
        "api_group_proteins_handler returned {} items of total {} with filters {:?}",
        items.len(),
        total,
        filters
    );

    Ok(Json(PaginatedResponse {
        items,
        total,
        page,
        limit,
        has_next,
        has_previous: page > 1,
    }))
}

async fn api_group_oldproteins_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i64>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_group_proteins_handler");
    let ctx = ctx.0;

    let filters: Vec<ProteinFilter> = serde_json::from_value(json!([
        {
            "group_id": {"$eq":group_id }
        }
    ]))?;

    let op = ListOptions {
        order_bys: Some(OrderBys::new(vec![OrderBy::Desc("id".into())])),
        ..Default::default()
    };
    let proteins: Vec<Protein> = ProteinBmc::list(&ctx, &mm, Some(filters), Some(op)).await?;
    Ok(Json(json!(proteins)))
}

async fn api_group_panel_elements_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_group_panel_elements_handler");
    let ctx = ctx.0;

    let panel_elements: Vec<PanelElement> = PanelElementBmc::list(&ctx, &mm, None, None).await?;
    Ok(Json(json!(panel_elements)))
}

async fn api_group_tags_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i64>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_group_tags_handler");
    let ctx = ctx.0;
    let filters: Vec<TagFilter> = serde_json::from_value(json!([{"group_id": {"$eq":group_id}}]))?;

    let op = ListOptions {
        limit: Some(10_000),
        ..Default::default()
    };
    let tags: Vec<Tag> = TagBmc::list(&ctx, &mm, Some(filters), Some(op)).await?;
    Ok(Json(json!(tags)))
}

async fn api_group_validation_files_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_group_validation_files_handler");
    let ctx = ctx.0;

    let validation_files: Vec<ValidationFile> =
        ValidationFileBmc::list(&ctx, &mm, None, None).await?;
    Ok(Json(json!(validation_files)))
}

async fn api_group_validations_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i64>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Validation>>> {
    let limit = params.limit.unwrap_or(5);
    let page = params.page.unwrap_or(1);
    let offset: usize = (page as usize - 1) * limit as usize;
    debug!("HANDLER - api_group_species_handler");
    let ctx = ctx.0;

    let filters: Vec<ValidationFilter> = match &params.search {
        Some(search) => serde_json::from_value(json!([
            {
                "group_id": {"$eq":group_id },
                "name": {"$contains":search},

            }
        ]))?,
        None => serde_json::from_value(json!([
            {
                "group_id": {"$eq":group_id }
            }
        ]))?,
    };
    info!("Validation filters: {:?}", filters);

    let op = ListOptions {
        order_bys: Some(OrderBys::new(vec![OrderBy::Desc("id".into())])),
        limit: Some(limit as i64),
        offset: Some(offset as i64),
    };

    let items: Vec<Validation> =
        ValidationBmc::list(&ctx, &mm, Some(filters.clone()), Some(op)).await?;

    let total = ValidationBmc::count(&ctx, &mm, Some(filters)).await?;
    let has_next = offset + items.len() < total as usize;

    Ok(Json(PaginatedResponse {
        items,
        total,
        page,
        limit,
        has_next,
        has_previous: page > 1,
    }))
}

async fn api_group_panels_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i64>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_group_panels_handler");
    let ctx = ctx.0;

    let filters: Vec<PanelFilter> = serde_json::from_value(json!([
        {
            "group_id": {"$eq": group_id },
            "is_archived": {"$eq": false }
        }
    ]))?;

    let list_options = ListOptions {
        limit: Some(10000),
        offset: None,
        order_bys: Some(OrderBys::new(vec![OrderBy::Desc("id".into())])),
    };

    let panels: Vec<Panel> = PanelBmc::list(&ctx, &mm, Some(filters), Some(list_options)).await?;
    Ok(Json(json!(panels)))
}

async fn api_group_conjugates_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i64>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_group_conjugates_handler");
    let ctx = ctx.0;

    let filters: Vec<ConjugateFilter> = serde_json::from_value(json!([
        {
            "group_id": {"$eq": group_id }
        }
    ]))?;

    let op = ListOptions {
        order_bys: Some(OrderBys::new(vec![OrderBy::Desc("id".into())])),
        ..Default::default()
    };
    let conjugates: Vec<Conjugate> = ConjugateBmc::list(&ctx, &mm, Some(filters), Some(op)).await?;
    Ok(Json(json!(conjugates)))
}

async fn api_group_clones_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i64>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Clone>>> {
    let limit = params.limit.unwrap_or(5);
    let page = params.page.unwrap_or(1);
    let offset: usize = (page as usize - 1) * limit as usize;
    debug!("HANDLER - api_group_clones_handler");
    let ctx = ctx.0;

    let filters: Vec<CloneFilter> = match &params.search {
        Some(search) => serde_json::from_value(json!([
            {
                "group_id": {"$eq":group_id },
                "name": {"$contains":search},

            }
        ]))?,
        None => serde_json::from_value(json!([
            {
                "group_id": {"$eq":group_id }
            }
        ]))?,
    };

    let op = ListOptions {
        order_bys: Some(OrderBys::new(vec![OrderBy::Desc("id".into())])),
        limit: Some(limit as i64),
        offset: Some(offset as i64),
    };

    let items: Vec<Clone> = CloneBmc::list(&ctx, &mm, Some(filters.clone()), Some(op)).await?;

    let total = CloneBmc::count(&ctx, &mm, Some(filters)).await?;
    let has_next = offset + items.len() < total as usize;

    Ok(Json(PaginatedResponse {
        items,
        total,
        page,
        limit,
        has_next,
        has_previous: page > 1,
    }))
}

async fn api_group_lots_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i64>,
    Query(query_params): Query<HashMap<String, i64>>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_group_lots_handler");
    let ctx = ctx.0;
    let limit = query_params.get("limit").copied();
    let options = ListOptions {
        limit,
        offset: None,
        order_bys: Some(OrderBys::new(vec![OrderBy::Desc("id".into())])),
    };
    let status = query_params.get("status");

    let filters: Vec<LotFilter> = match status {
        None => serde_json::from_value(json!([
            {
                "group_id": {"$eq":group_id},
            }
        ]))?,
        Some(status) => serde_json::from_value(json!([
            {
                "group_id": {"$eq":group_id},
                "status": {"$eq":status},
            }
        ]))?,
    };

    let vlots: Vec<Lot> = LotBmc::list(&ctx, &mm, Some(filters), Some(options)).await?;

    Ok(Json(json!(vlots)))
}

async fn api_group_species_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i64>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Species>>> {
    let limit = params.limit.unwrap_or(5);
    let page = params.page.unwrap_or(1);
    let offset: usize = (page as usize - 1) * limit as usize;
    debug!("HANDLER - api_group_species_handler");
    let ctx = ctx.0;

    let filters: Vec<SpeciesFilter> = match &params.search {
        Some(search) => serde_json::from_value(json!([
            {
                "group_id": {"$eq":group_id },
                "name": {"$contains":search},

            }
        ]))?,
        None => serde_json::from_value(json!([
            {
                "group_id": {"$eq":group_id }
            }
        ]))?,
    };
    info!("SpeciesFilters: {:?}", filters);

    let op = ListOptions {
        order_bys: Some(OrderBys::new(vec![OrderBy::Desc("id".into())])),
        limit: Some(limit as i64),
        offset: Some(offset as i64),
    };

    let items: Vec<Species> = SpeciesBmc::list(&ctx, &mm, Some(filters.clone()), Some(op)).await?;

    let total = SpeciesBmc::count(&ctx, &mm, Some(filters)).await?;
    let has_next = offset + items.len() < total as usize;

    Ok(Json(PaginatedResponse {
        items,
        total,
        page,
        limit,
        has_next,
        has_previous: page > 1,
    }))
}

#[allow(dead_code)]
async fn old_api_group_species_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i64>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_group_species_handler");
    let ctx = ctx.0;

    let filters: Vec<SpeciesFilter> = serde_json::from_value(json!([
        {
            "group_id": {"$eq":group_id}
        }
    ]))?;

    let species: Vec<Species> = SpeciesBmc::list(&ctx, &mm, Some(filters), None).await?;
    Ok(Json(json!(species)))
}

async fn api_group_providers_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i64>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_group_providers_handler");
    let ctx = ctx.0;

    let filters: Vec<ProviderFilter> = serde_json::from_value(json!([
        {
            "group_id": {"$eq":group_id}
        }
    ]))?;

    let providers: Vec<Provider> = ProviderBmc::list(&ctx, &mm, Some(filters), None).await?;
    Ok(Json(json!(providers)))
}

async fn api_group_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i64>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_group_handler {}", group_id);
    let ctx = ctx.0;
    let group: Group = GroupBmc::get(&ctx, &mm, group_id).await?;
    Ok(Json(json!(group)))
}

async fn api_group_me_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i64>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_group_me_handler");
    let ctx = ctx.0;

    let filters: Vec<MemberFilter> = serde_json::from_value(json!([
        {
            "user_id": {"$eq":ctx.user_id()},
            "group_id": {"$eq":group_id}
        }
    ]))?;

    let member: Vec<Member> = MemberBmc::list(&ctx, &mm, Some(filters), None).await?;
    let member = member.first().ok_or(model::Error::EntityNotFound {
        entity: "member",
        id: ctx.user_id(),
    })?;
    Ok(Json(json!(member)))
}

async fn api_group_members_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i64>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_group_member_handler");
    let ctx = ctx.0;

    let filters: Vec<MemberFilter> = serde_json::from_value(json!([
        {
            "group_id": {"$eq":group_id}
        }
    ]))?;

    let members: Vec<Member> = MemberBmc::list(&ctx, &mm, Some(filters), None).await?;
    Ok(Json(json!(members)))
}

async fn api_groups_handler(State(mm): State<ModelManager>, ctx: CtxW) -> Result<Json<Value>> {
    debug!("HANDLER - api_groups_handler");
    let ctx = ctx.0;

    let groups: Vec<Group> = GroupBmc::list(&ctx, &mm, None, None).await?;
    Ok(Json(json!(groups)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower::ServiceExt;

    type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    #[tokio::test]
    async fn group_providers_route_returns_seeded_provider() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = crate::web::test_support::authed_router(routes((*mm).clone()));

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/v1/groups/1000/providers")
                    .body(axum::body::Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let body = crate::web::test_support::response_body_string(response).await?;
        assert!(body.contains("seed-provider"));

        Ok(())
    }

    #[tokio::test]
    async fn group_me_route_returns_authenticated_membership() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = crate::web::test_support::authed_router(routes((*mm).clone()));

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/v1/groups/1/members/me")
                    .body(axum::body::Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let body = crate::web::test_support::response_body_string(response).await?;
        assert!(body.contains("\"groupId\":1"));
        assert!(body.contains("\"userId\":1"));

        Ok(())
    }

    #[tokio::test]
    async fn groups_route_lists_seeded_groups() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = crate::web::test_support::authed_router(routes((*mm).clone()));

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/v1/groups")
                    .body(axum::body::Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let body = crate::web::test_support::response_body_string(response).await?;
        assert!(body.contains("primary test group"));
        assert!(body.contains("seed group"));

        Ok(())
    }

    #[tokio::test]
    async fn post_group_route_creates_group_and_membership() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = crate::web::test_support::authed_router(routes((*mm).clone()));
        let payload = json!({
            "name": "route-created-group",
            "institution": "Airlab",
            "url": "https://example.test/group",
            "isOpen": false,
            "tags": ["route"]
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/groups/")
                    .header(axum::http::header::CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::from(payload.to_string()))?,
            )
            .await?;

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let body = crate::web::test_support::response_body_string(response).await?;
        assert!(body.contains("route-created-group"));

        Ok(())
    }
}
