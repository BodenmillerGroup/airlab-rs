use crate::web::Result;
use crate::web::mw_auth::CtxW;
use airlab_lib::model::ModelManager;
use airlab_lib::model::clone::CloneFilter;
use airlab_lib::model::conjugate::ConjugateFilter;
use airlab_lib::model::group::{Group, GroupBmc, GroupForCreate, GroupForUpdate};
use airlab_lib::model::lot::LotFilter;
use airlab_lib::model::member::{Member, MemberBmc, MemberFilter, MemberForCreate};
use airlab_lib::model::panel::PanelFilter;
use airlab_lib::model::panel_element::{PanelElement, PanelElementBmc};
use airlab_lib::model::protein::{Protein, ProteinBmc, ProteinFilter};
use airlab_lib::model::provider::{Provider, ProviderBmc, ProviderFilter};
use airlab_lib::model::species::{Species, SpeciesBmc, SpeciesFilter};
use airlab_lib::model::tag::{Tag, TagBmc, TagFilter};
use airlab_lib::model::validation::ValidationFilter;
use airlab_lib::model::validation_file::{ValidationFile, ValidationFileBmc};
use airlab_lib::model::view_clone::{ViewClone, ViewCloneBmc};
use airlab_lib::model::view_conjugate::{ViewConjugate, ViewConjugateBmc};
use airlab_lib::model::view_group::ViewGroup;
use airlab_lib::model::view_lot::{ViewLot, ViewLotBmc};
use airlab_lib::model::view_member::{ViewMember, ViewMemberBmc};
use airlab_lib::model::view_panel::{ViewPanel, ViewPanelBmc};
use airlab_lib::model::view_validation::{ViewValidation, ViewValidationBmc};
use axum::extract::{Json as eJson, Path, Query, State};
use axum::http::Uri;
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use modql::filter::{ListOptions, OrderBy, OrderBys};
use serde_json::{Value, json};
use std::collections::{HashMap, hash_map::Entry};
use tracing::{debug, warn};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/*all", get(api_handler))
        .route("/api/v1/groups/", post(api_post_group_handler))
        .route("/api/v1/groups/:group_id", patch(api_patch_group_handler))
        .route("/api/v1/groups/:group_id", get(api_group_handler))
        .route(
            "/api/v1/groups/:group_id/members/me",
            get(api_group_me_handler),
        )
        .route(
            "/api/v1/groups/:group_id/members",
            get(api_group_members_handler),
        )
        .route("/api/v1/groups", get(api_groups_handler))
        .route(
            "/api/v1/groups/:group_id/proteins",
            get(api_group_proteins_handler),
        )
        .route("/api/v1/groups/:group_id/tags", get(api_group_tags_handler))
        .route(
            "/api/v1/groups/:group_id/validation_files",
            get(api_group_validation_files_handler),
        )
        .route(
            "/api/v1/groups/:group_id/panel_elements",
            get(api_group_panel_elements_handler),
        )
        .route(
            "/api/v1/groups/:group_id/validations",
            get(api_group_validations_handler),
        )
        .route(
            "/api/v1/groups/:group_id/panels",
            get(api_group_panels_handler),
        )
        .route("/api/v1/groups/:group_id/lots", get(api_group_lots_handler))
        .route(
            "/api/v1/groups/:group_id/conjugates",
            get(api_group_conjugates_handler),
        )
        .route(
            "/api/v1/groups/:group_id/clones",
            get(api_group_clones_handler),
        )
        .route(
            "/api/v1/groups/:group_id/species",
            get(api_group_species_handler),
        )
        .route(
            "/api/v1/groups/:group_id/providers",
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
    Path(group_id): Path<i32>,
    eJson(payload): eJson<GroupForUpdate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_group_handler: {}; {:?}", group_id, payload);
    let ctx = ctx.0;

    GroupBmc::update(&ctx, &mm, group_id, payload).await?;

    let group: Group = GroupBmc::get(&ctx, &mm, group_id).await?;
    Ok(Json(json!(group)))
}

async fn api_group_proteins_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i32>,
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
    Path(group_id): Path<i32>,
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
    Path(group_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_group_validations_handler");
    let ctx = ctx.0;

    let filters: Vec<ValidationFilter> = serde_json::from_value(json!([
        {
            "group_id": {"$eq": group_id }
        }
    ]))?;

    let op = ListOptions {
        order_bys: Some(OrderBys::new(vec![OrderBy::Desc("id".into())])),
        ..Default::default()
    };
    let validations: Vec<ViewValidation> =
        ViewValidationBmc::list(&ctx, &mm, group_id, Some(filters), Some(op)).await?;
    Ok(Json(json!(validations)))
}

async fn api_group_panels_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i32>,
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

    let panels: Vec<ViewPanel> =
        ViewPanelBmc::list(&ctx, &mm, Some(filters), Some(list_options)).await?;
    Ok(Json(json!(panels)))
}

async fn api_group_conjugates_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i32>,
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
    let conjugates: Vec<ViewConjugate> =
        ViewConjugateBmc::list(&ctx, &mm, Some(group_id), Some(filters), Some(op)).await?;
    Ok(Json(json!(conjugates)))
}

async fn api_group_clones_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_group_clones_handler");
    let ctx = ctx.0;

    let filters: Vec<CloneFilter> = serde_json::from_value(json!([
        {
            "group_id": {"$eq":group_id}
        }
    ]))?;
    let op = ListOptions {
        order_bys: Some(OrderBys::new(vec![OrderBy::Desc("id".into())])),
        ..Default::default()
    };

    let clones: Vec<ViewClone> =
        ViewCloneBmc::list(&ctx, &mm, Some(group_id), Some(filters), Some(op)).await?;
    Ok(Json(json!(clones)))
}

async fn api_group_lots_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i32>,
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

    let vlots: Vec<ViewLot> =
        ViewLotBmc::list(&ctx, &mm, Some(group_id), Some(filters), Some(options)).await?;

    Ok(Json(json!(vlots)))
}

async fn api_group_species_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i32>,
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
    Path(group_id): Path<i32>,
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
    Path(group_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_group_handler {}", group_id);
    let ctx = ctx.0;
    let group: Group = GroupBmc::get(&ctx, &mm, group_id).await?;
    Ok(Json(json!(group)))
}

async fn api_group_me_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i32>,
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
    let member = match member.first() {
        Some(m) => m,
        None => panic!(
            "The group {} does not have a member with id {}",
            group_id,
            ctx.user_id()
        ),
    };
    Ok(Json(json!(member)))
}

async fn api_group_members_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_group_member_handler");
    let ctx = ctx.0;

    let filters: Vec<MemberFilter> = serde_json::from_value(json!([
        {
            "group_id": {"$eq":group_id}
        }
    ]))?;

    let members: Vec<ViewMember> = ViewMemberBmc::list(&ctx, &mm, Some(filters), None).await?;
    Ok(Json(json!(members)))
}

async fn api_groups_handler(State(mm): State<ModelManager>, ctx: CtxW) -> Result<Json<Value>> {
    debug!("HANDLER - api_groups_handler");
    let ctx = ctx.0;

    let groups: Vec<Group> = GroupBmc::list(&ctx, &mm, None, None).await?;
    let members: Vec<Member> = MemberBmc::list(&ctx, &mm, None, None).await?;
    let mut memhash = HashMap::new();
    for member in members {
        let grpid = match memhash.entry(member.group_id) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(vec![]),
        };
        grpid.push(member);
    }
    let mut grpmems = vec![];
    for group in groups {
        let members = memhash
            .get(&{ group.id })
            .map_or_else(std::vec::Vec::new, std::clone::Clone::clone);
        let grpmem = ViewGroup {
            id: group.id,
            name: group.name,
            url: group.url.unwrap_or(String::new()),
            is_open: group.is_open,
            institution: group.institution,
            created_at: group.created_at,
            members,
        };
        grpmems.push(grpmem);
    }

    Ok(Json(json!(grpmems)))
}
