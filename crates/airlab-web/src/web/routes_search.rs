#![allow(unused_imports)]
use crate::search_shadow::SearchState;
use crate::search_shadow::basic::{map_basic_shadow_query, search_basic_shadow};
use crate::search_shadow::clone::{
    CloneShadowDirection, CloneShadowOrder, CloneShadowOrderField, CloneShadowQuery,
    map_filter as map_clone_shadow_filter, search_clone_shadow,
};
use crate::web::mw_auth::CtxW;
use crate::web::{Error, Result};
use ReturnType as RT;
use airlab_lib::ctx::Ctx;
use airlab_lib::model::ModelManager as MM;
use airlab_lib::model::clone::{Clone, CloneBmc, CloneFilter, CloneForUpdate, CloneId};
use airlab_lib::model::conjugate::{Conjugate, ConjugateBmc, ConjugateFilter};
use airlab_lib::model::lot::{Lot, LotBmc, LotFilter};
use airlab_lib::model::member::{Member, MemberBmc, MemberFilter};
use airlab_lib::model::panel::{Panel, PanelBmc, PanelFilter};
use airlab_lib::model::panel_element::{PanelElement, PanelElementBmc, PanelElementFilter};
use airlab_lib::model::protein::{Protein, ProteinBmc, ProteinFilter};
use airlab_lib::model::provider::{Provider, ProviderBmc, ProviderFilter};
use airlab_lib::model::species::{Species, SpeciesBmc, SpeciesFilter};
use airlab_lib::model::tag::{Tag, TagBmc, TagFilter};
use airlab_lib::model::validation::{Validation, ValidationBmc, ValidationFilter};
use axum::extract::{Json as eJson, State};
use axum::routing::post;
use axum::{Json, Router};
use modql::filter::IntoFilterNodes;
use modql::filter::{FilterNodes, OpValsInt64, OpValsString};
use modql::filter::{ListOptions as LO, OpVal::Value as MqValue, OrderBy, OrderBys};
use sea_query::Expr;
use sea_query::IntoIden;
use sea_query::QueryBuilder;
use sea_query::extension::postgres::PgExpr;
use sea_query::{
    Alias, ColumnRef, Condition, Order, PostgresQueryBuilder, Query, SqlWriter, Value as SeaValue,
    Values,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use sqlx::Row;
use std::collections::{BTreeMap, BTreeSet};
use std::collections::{HashSet, VecDeque};
use strum_macros::Display;
#[allow(unused_imports)]
use tracing::{debug, info, warn};

type Graph = BTreeMap<ReturnType, Vec<JoinEdge>>;

pub(crate) const PAGESIZE: i64 = 50;

pub fn routes(state: SearchState) -> Router {
    Router::new()
        .route("/api/v1/search", post(api_post_search_handler))
        .with_state(state)
}

async fn api_post_search_handler(
    State(state): State<SearchState>,
    ctx: CtxW,
    eJson(mut req): eJson<RpcSearchRequest>,
) -> Result<Json<Value>> {
    if req.return_type == ReturnType::Panel && !req.show_all.unwrap_or(false) {
        inject_panel_owner_filter(&ctx.0, &state.mm, &mut req).await?;
    }

    if req.return_type == ReturnType::Clone {
        let Some(query) = map_clone_shadow_query(&req) else {
            return Err(Error::UnsupportedQueryValue(
                "No clone shadow query available".to_string(),
            ));
        };
        return clone_shadow_search_handler(&state, &query).await;
    }

    if let Some(query) = map_basic_shadow_query(&req) {
        return basic_shadow_search_handler(&state, &query).await;
    }

    if crate::search_shadow::basic::BasicShadowKind::from_return_type(req.return_type).is_some() {
        return Err(Error::UnsupportedQueryValue(format!(
            "No shadow query available for {:?}",
            req.return_type
        )));
    }

    sql_search_handler(&state.mm, req).await
}

async fn clone_shadow_search_handler(
    state: &SearchState,
    query: &CloneShadowQuery,
) -> Result<Json<Value>> {
    let shadow = state
        .registry
        .get_or_build_clone_shadow(&state.mm, query.group_id)
        .await?;
    let result = search_clone_shadow(&shadow, query);

    let lo = LO {
        limit: Some(query.limit),
        offset: Some((query.page.max(1) - 1) * query.limit.max(1)),
        ..Default::default()
    };

    let mut response = PaginatedResponse::<i64>::from_lo(&lo, result.search_total);
    response.page = query.page.max(1) as u32;
    response.limit = query.limit.max(1) as u32;
    response.has_previous = query.page.max(1) > 1;
    response.has_next = (query.page.max(1) * query.limit.max(1)) < result.search_total;
    response.items = result.items;

    Ok(Json(json!(response)))
}

async fn basic_shadow_search_handler(
    state: &SearchState,
    query: &crate::search_shadow::basic::BasicShadowQuery,
) -> Result<Json<Value>> {
    let shadow = state
        .registry
        .get_or_build_basic_shadow(&state.mm, query.kind)
        .await?;
    let result = search_basic_shadow(&shadow, query);

    let lo = LO {
        limit: Some(query.limit),
        offset: Some((query.page.max(1) - 1) * query.limit.max(1)),
        ..Default::default()
    };

    let mut response = PaginatedResponse::<i64>::from_lo(&lo, result.search_total);
    response.page = query.page.max(1) as u32;
    response.limit = query.limit.max(1) as u32;
    response.has_previous = query.page.max(1) > 1;
    response.has_next = (query.page.max(1) * query.limit.max(1)) < result.search_total;
    response.items = result.items;

    Ok(Json(json!(response)))
}

async fn sql_search_handler(mm: &MM, req: RpcSearchRequest) -> Result<Json<Value>> {
    info!("HANDLER - api_post_search_handler: {:?}", req);
    let graph = get_graph();
    let allowed = allowed_filter_tables(req.return_type);
    let global_filter = req
        .global_filter
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);
    let mut filters: Vec<Filter> = req
        .filters
        .into_iter()
        .filter(|filter| {
            if allowed.contains(&filter.table) {
                true
            } else {
                warn!(
                    "Skipping filter on {:?}.{} (not allowed for return_type {:?})",
                    filter.table, filter.field, req.return_type
                );
                false
            }
        })
        .collect();

    let order = req.order.and_then(|order| {
        if allowed.contains(&order.table) {
            Some(order)
        } else {
            warn!(
                "Skipping order on {:?}.{} (not allowed for return_type {:?})",
                order.table, order.field, req.return_type
            );
            None
        }
    });

    let mut required = get_required_tables(req.return_type, &filters, order.as_ref());
    if global_filter.is_some() && req.return_type == ReturnType::Lot {
        required.extend([
            ReturnType::Lot,
            ReturnType::Clone,
            ReturnType::Protein,
            ReturnType::Provider,
            ReturnType::Collection,
            ReturnType::Group,
        ]);
    }
    debug!("required tables: {required:#?}");
    let join_plan = find_join_plan(&graph, req.return_type, &required);
    debug!("join plan: {join_plan:#?}");
    let joined_tables = get_joined_tables(req.return_type, &join_plan);

    filters.retain(|filter| {
        if joined_tables.contains(&filter.table) {
            true
        } else {
            warn!(
                "Skipping filter on {:?}.{} (not joined for return_type {:?})",
                filter.table, filter.field, req.return_type
            );
            false
        }
    });
    let offset = req
        .limit
        .zip(req.page)
        .map(|(limit, page)| (page - 1) * limit);

    let frm = Alias::new(req.return_type.table_name()).into_iden();
    let colref = ColumnRef::TableColumn(
        Alias::new(req.return_type.table_name()).into_iden(),
        Alias::new("id".to_string()).into_iden(),
    );
    let mut query = Query::select().column(colref).from(frm).to_owned();
    for jn in &join_plan {
        let to = Alias::new(jn.to_table.table_name()).into_iden();
        let frm_colref = ColumnRef::TableColumn(
            Alias::new(jn.on_left.0.to_string().to_lowercase()).into_iden(),
            Alias::new(jn.on_left.1.to_string()).into_iden(),
        );
        let to_colref = ColumnRef::TableColumn(
            Alias::new(jn.on_right.0.to_string().to_lowercase()).into_iden(),
            Alias::new(jn.on_right.1.to_string()).into_iden(),
        );
        query.inner_join(to, Expr::col(frm_colref).equals(to_colref));
    }
    for filter in filters {
        let table_name = filter.table.table_name();
        let field_name = filter.field.to_string().to_lowercase();

        // Clone.reactivity is a bigint[] column. "contains" must use array operators.
        if filter.table == ReturnType::Clone
            && field_name == "reactivity"
            && filter.op == "contains"
        {
            match filter.value {
                Value::Number(n) => {
                    if let Some(v) = n.as_i64() {
                        let sql = format!("{table_name}.{field_name} @> ARRAY[{v}]::bigint[]");
                        query.and_where(Expr::cust(sql));
                    }
                    continue;
                }
                Value::Array(arr) => {
                    let vals: Vec<i64> = arr.iter().filter_map(|v| v.as_i64()).collect();
                    if !vals.is_empty() {
                        let csv = vals
                            .iter()
                            .map(std::string::ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(",");
                        let sql = format!("{table_name}.{field_name} && ARRAY[{csv}]::bigint[]");
                        query.and_where(Expr::cust(sql));
                    }
                    continue;
                }
                _ => {
                    warn!(
                        "Skipping unsupported reactivity filter value: {:?}",
                        filter.value
                    );
                    continue;
                }
            }
        }

        let cref = ColumnRef::TableColumn(
            Alias::new(table_name).into_iden(),
            Alias::new(field_name).into_iden(),
        );
        match filter.value {
            Value::String(s) => {
                if !s.is_empty() {
                    let st = format!("%{}%", s);
                    query.and_where(Expr::col(cref).ilike(st));
                }
            }
            Value::Number(n) => {
                query.and_where(Expr::col(cref).eq(n.as_i64()));
            }
            Value::Bool(b) => {
                query.and_where(Expr::col(cref).eq(b));
            }
            Value::Array(arr) => {
                // accept Vec<i64>
                let vals: Vec<i64> = arr.iter().filter_map(|v| v.as_i64()).collect();

                if !vals.is_empty() {
                    query.and_where(Expr::col(cref).is_in(vals));
                }
            }
            _ => {
                warn!("Skipping {}", filter.value);
            }
        }
    }

    if let Some(value) = &global_filter {
        if req.return_type == ReturnType::Lot {
            let pattern = format!("%{value}%");
            query.and_where(
                Condition::any()
                    .add(
                        Expr::col(ColumnRef::TableColumn(
                            Alias::new("lot").into_iden(),
                            Alias::new("name").into_iden(),
                        ))
                        .ilike(pattern.clone()),
                    )
                    .add(
                        Expr::col(ColumnRef::TableColumn(
                            Alias::new("lot").into_iden(),
                            Alias::new("reference").into_iden(),
                        ))
                        .ilike(pattern.clone()),
                    )
                    .add(
                        Expr::col(ColumnRef::TableColumn(
                            Alias::new("lot").into_iden(),
                            Alias::new("number").into_iden(),
                        ))
                        .ilike(pattern.clone()),
                    )
                    .add(
                        Expr::col(ColumnRef::TableColumn(
                            Alias::new("clone").into_iden(),
                            Alias::new("name").into_iden(),
                        ))
                        .ilike(pattern.clone()),
                    )
                    .add(
                        Expr::col(ColumnRef::TableColumn(
                            Alias::new("protein").into_iden(),
                            Alias::new("name").into_iden(),
                        ))
                        .ilike(pattern.clone()),
                    )
                    .add(
                        Expr::col(ColumnRef::TableColumn(
                            Alias::new("provider").into_iden(),
                            Alias::new("name").into_iden(),
                        ))
                        .ilike(pattern.clone()),
                    )
                    .add(
                        Expr::col(ColumnRef::TableColumn(
                            Alias::new("collection").into_iden(),
                            Alias::new("name").into_iden(),
                        ))
                        .ilike(pattern.clone()),
                    )
                    .add(
                        Expr::col(ColumnRef::TableColumn(
                            Alias::new("group").into_iden(),
                            Alias::new("name").into_iden(),
                        ))
                        .ilike(pattern),
                    )
                    .into(),
            );
        }
    }

    if let Some(order) = order {
        debug!(
            "ORDER SET: {:?} {} {:?}",
            order.table, order.field, order.direction
        );
        let order_table = order.table.table_name();
        let order_field = order.field.to_string().to_lowercase();
        let table = Alias::new(order_table).into_iden();
        let field = Alias::new(&order_field).into_iden();
        let col_ref = ColumnRef::TableColumn(table, field);
        let ordr = match order.direction {
            Direction::Desc => Order::Desc,
            Direction::Asc => Order::Asc,
        };
        query.order_by_expr(col_ref.into(), ordr);
    }
    let (sql, values) = query.build(PostgresQueryBuilder);
    let mut q = sqlx::query(&sql);
    for v in values {
        q = match v {
            SeaValue::Bool(b) => q.bind(b),
            SeaValue::TinyInt(i) => q.bind(i),
            SeaValue::SmallInt(i) => q.bind(i),
            SeaValue::Int(i) => q.bind(i),
            SeaValue::BigInt(i) => q.bind(i),
            SeaValue::Float(f) => q.bind(f),
            SeaValue::Double(d) => q.bind(d),

            SeaValue::String(s) => {
                let s: Option<String> = s.map(|b| *b);
                q.bind(s)
            }

            other => {
                return Err(Error::UnsupportedQueryValue(format!(
                    "Unsupported SeaValue: {other:?}"
                )));
            }
        }
    }
    let rows = q
        .fetch_all(mm.db())
        .await
        .map_err(airlab_lib::model::Error::from)?;
    let mut seen_ids: HashSet<i64> = HashSet::new();
    let mut ids: Vec<i64> = Vec::new();
    for row in rows {
        let id: i64 = row.get("id");
        if seen_ids.insert(id) {
            ids.push(id);
        }
    }
    let limit = req.limit.unwrap_or(PAGESIZE) as usize;
    let page = req.page.unwrap_or(1) as usize;

    let lo = LO {
        limit: req.limit,
        offset,
        ..Default::default()
    };

    let mut ret2 = PaginatedResponse::<i64>::from_lo(&lo, ids.len() as i64);
    let start = (page.saturating_sub(1)) * limit;
    let end = (start + limit).min(ids.len());
    ret2.items = ids.get(start..end).unwrap_or(&[]).to_vec();
    Ok(Json(json!(ret2)))
}

#[derive(Serialize, Default)]
struct PaginatedResponse<T> {
    items: Vec<T>,
    search_total: i64,
    page: u32,
    limit: u32,
    has_next: bool,
    has_previous: bool,
}

impl<T> PaginatedResponse<T> {
    fn from_lo(lo: &LO, search_total: i64) -> Self {
        let offset = lo.offset.unwrap_or(PAGESIZE);
        let limit = lo.limit.unwrap_or(PAGESIZE);
        let page = (offset / limit) as u32;
        Self {
            items: vec![],
            search_total,
            page,
            limit: limit as u32,
            has_next: offset + limit < search_total,
            has_previous: page > 0,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RpcSearchRequest {
    pub return_type: RT,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    #[serde(default)]
    pub filters: Vec<Filter>,
    pub order: Option<SearchOrder>,
    pub global_filter: Option<String>,
    pub show_all: Option<bool>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord, Display, Hash, Copy)]
#[strum(serialize_all = "lowercase")]
pub enum ReturnType {
    User,
    Member,
    Group,
    Clone,
    Species,
    Protein,
    Lot,
    Tag,
    Conjugate,
    Panel,
    PanelElement,
    Validation,
    Provider,
    Storage,
    Collection,
}

impl ReturnType {
    fn table_name(self) -> &'static str {
        match self {
            ReturnType::User => "user",
            ReturnType::Member => "member",
            ReturnType::Group => "group",
            ReturnType::Clone => "clone",
            ReturnType::Species => "species",
            ReturnType::Protein => "protein",
            ReturnType::Lot => "lot",
            ReturnType::Tag => "tag",
            ReturnType::Conjugate => "conjugate",
            ReturnType::Panel => "panel",
            ReturnType::PanelElement => "panel_element",
            ReturnType::Validation => "validation",
            ReturnType::Provider => "provider",
            ReturnType::Storage => "storage",
            ReturnType::Collection => "collection",
        }
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum Direction {
    #[serde(alias = "asc")]
    Asc,
    #[serde(alias = "desc")]
    Desc,
}

#[derive(Debug, Deserialize)]
pub struct SearchOrder {
    pub table: ReturnType,
    pub field: String,
    pub direction: Direction,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Filter {
    pub table: ReturnType,
    pub field: String,
    pub op: String,
    pub value: Value,
}

#[allow(dead_code)]
fn format_value_for_sql(value: &Value, wildcard: bool) -> String {
    match value {
        Value::String(s) => match wildcard {
            false => format!("'{}'", s.replace('\'', "''")),
            true => format!("'%{}%'", s.replace('\'', "''")),
        },
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "NULL".to_string(),
        _ => {
            warn!("Unsupported value type for SQL conversion: {:?}", value);
            "NULL".to_string()
        }
    }
}

fn map_clone_shadow_query(req: &RpcSearchRequest) -> Option<CloneShadowQuery> {
    let group_id = extract_clone_group_id(&req.filters)?;
    let mut filters = Vec::new();

    for filter in &req.filters {
        let Some(mapped) = map_clone_shadow_filter(
            &filter.table.to_string(),
            filter.field.as_str(),
            &filter.value,
        ) else {
            warn!(
                table = ?filter.table,
                field = filter.field,
                "skipping unsupported clone shadow filter"
            );
            continue;
        };
        filters.push(mapped);
    }

    let order = match req.order.as_ref() {
        Some(order) => map_clone_shadow_order_field(order).map(|field| CloneShadowOrder {
            field,
            direction: match order.direction {
                Direction::Asc => CloneShadowDirection::Asc,
                Direction::Desc => CloneShadowDirection::Desc,
            },
        }),
        None => None,
    };

    Some(CloneShadowQuery {
        group_id,
        filters,
        global_filter: req.global_filter.clone(),
        order,
        page: req.page.unwrap_or(1),
        limit: req.limit.unwrap_or(PAGESIZE),
    })
}

fn extract_clone_group_id(filters: &[Filter]) -> Option<i64> {
    filters.iter().find_map(|filter| {
        if filter.table == ReturnType::Clone && filter.field == "group_id" {
            filter.value.as_i64()
        } else {
            None
        }
    })
}

fn extract_group_id_for_table(filters: &[Filter], table: ReturnType) -> Option<i64> {
    filters.iter().find_map(|filter| {
        if filter.table == table && filter.field == "group_id" {
            filter.value.as_i64()
        } else {
            None
        }
    })
}

async fn inject_panel_owner_filter(ctx: &Ctx, mm: &MM, req: &mut RpcSearchRequest) -> Result<()> {
    let Some(group_id) = extract_group_id_for_table(&req.filters, ReturnType::Panel) else {
        return Ok(());
    };

    if req
        .filters
        .iter()
        .any(|filter| filter.table == ReturnType::Panel && filter.field == "created_by")
    {
        return Ok(());
    }

    let member_id = get_member_id(ctx, mm, group_id, ctx.user_id()).await?;
    req.filters.push(Filter {
        table: ReturnType::Panel,
        field: "created_by".to_string(),
        op: "eq".to_string(),
        value: json!(member_id),
    });
    Ok(())
}

async fn get_member_id(ctx: &Ctx, mm: &MM, group_id: i64, user_id: i64) -> Result<i64> {
    let filters: Vec<MemberFilter> = serde_json::from_value(json!([
        {
            "group_id": {"$eq": group_id},
            "user_id": {"$eq": user_id}
        }
    ]))?;
    let members: Vec<Member> = MemberBmc::list(ctx, mm, Some(filters), None).await?;
    Ok(members.first().map_or(0, |m| m.id))
}

fn map_clone_shadow_order_field(order: &SearchOrder) -> Option<CloneShadowOrderField> {
    match (order.table, order.field.as_str()) {
        (ReturnType::Clone, "id") => Some(CloneShadowOrderField::Id),
        (ReturnType::Clone, "name") => Some(CloneShadowOrderField::CloneName),
        (ReturnType::Protein, "name") => Some(CloneShadowOrderField::ProteinName),
        (ReturnType::Species, "name") => Some(CloneShadowOrderField::SpeciesName),
        (ReturnType::Clone, "is_phospho") => Some(CloneShadowOrderField::IsPhospho),
        (ReturnType::Clone, "application") => Some(CloneShadowOrderField::Application),
        (ReturnType::Validation, "application") => {
            Some(CloneShadowOrderField::ValidationApplication)
        }
        (ReturnType::Validation, "status") => Some(CloneShadowOrderField::ValidationStatus),
        _ => None,
    }
}

fn get_required_tables(
    return_type: ReturnType,
    filters: &[Filter],
    order: Option<&SearchOrder>,
) -> BTreeSet<ReturnType> {
    let mut tables_used: BTreeSet<ReturnType> = BTreeSet::new();
    tables_used.insert(return_type);

    for filter in filters {
        tables_used.insert(filter.table);
    }

    if let Some(order) = order {
        tables_used.insert(order.table);
    }

    tables_used
}

pub(crate) fn allowed_filter_tables(return_type: ReturnType) -> BTreeSet<ReturnType> {
    use ReturnType as RT;
    match return_type {
        RT::User => [RT::User].into_iter().collect(),
        RT::Member => [RT::Member, RT::User, RT::Group].into_iter().collect(),
        RT::Group => [RT::Group, RT::Member, RT::User].into_iter().collect(),
        RT::Clone => [
            RT::Clone,
            RT::Protein,
            RT::Species,
            RT::Validation,
            RT::Group,
        ]
        .into_iter()
        .collect(),
        RT::Species => [RT::Species, RT::Group].into_iter().collect(),
        RT::Protein => [RT::Protein, RT::Group].into_iter().collect(),
        RT::Lot => [
            RT::Lot,
            RT::Clone,
            RT::Protein,
            RT::Species,
            RT::Provider,
            RT::Collection,
            RT::Group,
        ]
        .into_iter()
        .collect(),
        RT::Tag => [RT::Tag, RT::Group].into_iter().collect(),
        RT::Conjugate => [
            RT::Conjugate,
            RT::Lot,
            RT::Clone,
            RT::Protein,
            RT::Species,
            RT::Tag,
            RT::Collection,
            RT::Group,
        ]
        .into_iter()
        .collect(),
        RT::Panel => [RT::Panel, RT::Group, RT::User].into_iter().collect(),
        RT::PanelElement => [RT::PanelElement, RT::Panel, RT::Conjugate, RT::Tag]
            .into_iter()
            .collect(),
        RT::Validation => [
            RT::Validation,
            RT::Clone,
            RT::Lot,
            RT::Protein,
            RT::Species,
            RT::Group,
        ]
        .into_iter()
        .collect(),
        RT::Provider => [RT::Provider, RT::Group].into_iter().collect(),
        RT::Storage => [RT::Storage].into_iter().collect(),
        RT::Collection => [RT::Collection].into_iter().collect(),
    }
}

fn get_joined_tables(start: ReturnType, join_plan: &[JoinEdge]) -> BTreeSet<ReturnType> {
    let mut joined: BTreeSet<ReturnType> = BTreeSet::new();
    joined.insert(start);
    for edge in join_plan {
        joined.insert(edge.from_table);
        joined.insert(edge.to_table);
    }
    joined
}

fn get_graph() -> Graph {
    let mut graph = Graph::new();

    add_edge(
        &mut graph,
        ReturnType::Clone,
        ReturnType::Protein,
        ("clone", "protein_id"),
        ("protein", "id"),
    );
    add_edge(
        &mut graph,
        ReturnType::Clone,
        ReturnType::Species,
        ("clone", "species_id"),
        ("species", "id"),
    );
    add_edge(
        &mut graph,
        ReturnType::Clone,
        ReturnType::Group,
        ("clone", "group_id"),
        ("group", "id"),
    );
    add_edge(
        &mut graph,
        ReturnType::Protein,
        ReturnType::Group,
        ("protein", "group_id"),
        ("group", "id"),
    );
    add_edge(
        &mut graph,
        ReturnType::Species,
        ReturnType::Group,
        ("species", "group_id"),
        ("group", "id"),
    );
    add_edge(
        &mut graph,
        ReturnType::Lot,
        ReturnType::Clone,
        ("lot", "clone_id"),
        ("clone", "id"),
    );
    add_edge(
        &mut graph,
        ReturnType::Lot,
        ReturnType::Provider,
        ("lot", "provider_id"),
        ("provider", "id"),
    );
    add_edge(
        &mut graph,
        ReturnType::Lot,
        ReturnType::Collection,
        ("lot", "collection_id"),
        ("collection", "id"),
    );
    add_edge(
        &mut graph,
        ReturnType::Lot,
        ReturnType::Group,
        ("lot", "group_id"),
        ("group", "id"),
    );
    add_edge(
        &mut graph,
        ReturnType::Provider,
        ReturnType::Group,
        ("provider", "group_id"),
        ("group", "id"),
    );
    add_edge(
        &mut graph,
        ReturnType::Tag,
        ReturnType::Group,
        ("tag", "group_id"),
        ("group", "id"),
    );
    add_edge(
        &mut graph,
        ReturnType::Conjugate,
        ReturnType::Lot,
        ("conjugate", "lot_id"),
        ("lot", "id"),
    );
    add_edge(
        &mut graph,
        ReturnType::Conjugate,
        ReturnType::Tag,
        ("conjugate", "tag_id"),
        ("tag", "id"),
    );
    add_edge(
        &mut graph,
        ReturnType::Conjugate,
        ReturnType::Group,
        ("conjugate", "group_id"),
        ("group", "id"),
    );
    add_edge(
        &mut graph,
        ReturnType::Panel,
        ReturnType::Group,
        ("panel", "group_id"),
        ("group", "id"),
    );
    add_edge(
        &mut graph,
        ReturnType::Panel,
        ReturnType::User,
        ("panel", "created_by"),
        ("user", "id"),
    );
    add_edge(
        &mut graph,
        ReturnType::PanelElement,
        ReturnType::Panel,
        ("panel_element", "panel_id"),
        ("panel", "id"),
    );
    add_edge(
        &mut graph,
        ReturnType::PanelElement,
        ReturnType::Conjugate,
        ("panel_element", "conjugate_id"),
        ("conjugate", "id"),
    );
    add_edge(
        &mut graph,
        ReturnType::Validation,
        ReturnType::Clone,
        ("validation", "clone_id"),
        ("clone", "id"),
    );
    add_edge(
        &mut graph,
        ReturnType::Validation,
        ReturnType::Lot,
        ("validation", "lot_id"),
        ("lot", "id"),
    );
    add_edge(
        &mut graph,
        ReturnType::Validation,
        ReturnType::Group,
        ("validation", "group_id"),
        ("group", "id"),
    );
    add_edge(
        &mut graph,
        ReturnType::Member,
        ReturnType::Group,
        ("member", "group_id"),
        ("group", "id"),
    );
    add_edge(
        &mut graph,
        ReturnType::Member,
        ReturnType::User,
        ("member", "user_id"),
        ("user", "id"),
    );
    graph
}

fn add_edge(
    graph: &mut Graph,
    from: ReturnType,
    to: ReturnType,
    on_left: (&'static str, &'static str),
    on_right: (&'static str, &'static str),
) {
    graph.entry(from).or_default().push(JoinEdge {
        from_table: from,
        to_table: to,
        on_left,
        on_right,
    });
    graph.entry(to).or_default().push(JoinEdge {
        from_table: to,
        to_table: from,
        on_left: on_right,
        on_right: on_left,
    });
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct JoinEdge {
    from_table: ReturnType,
    to_table: ReturnType,
    on_left: (&'static str, &'static str),
    on_right: (&'static str, &'static str),
}

fn find_join_plan(
    graph: &Graph,
    start: ReturnType,
    required: &BTreeSet<ReturnType>,
) -> Vec<JoinEdge> {
    let mut visited: HashSet<ReturnType> = HashSet::new();
    let mut queue: VecDeque<ReturnType> = VecDeque::new();
    let mut prev: BTreeMap<ReturnType, (ReturnType, JoinEdge)> = BTreeMap::new();
    let mut distance: BTreeMap<ReturnType, usize> = BTreeMap::new();

    queue.push_back(start);
    visited.insert(start);
    distance.insert(start, 0);

    while let Some(current) = queue.pop_front() {
        if let Some(edges) = graph.get(&current) {
            for edge in edges {
                let next = edge.to_table;
                if next == ReturnType::Group
                    && next != start
                    && !required.contains(&ReturnType::Group)
                {
                    continue;
                }
                if visited.contains(&next) {
                    continue;
                }
                visited.insert(next);
                prev.insert(next, (current, edge.clone()));
                distance.insert(next, distance.get(&current).copied().unwrap_or(0) + 1);
                queue.push_back(next);
            }
        }
    }

    let mut joins: Vec<JoinEdge> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    for target in required.iter().copied() {
        if target == start {
            continue;
        }
        let mut current = target;
        while let Some((parent, edge)) = prev.get(&current).cloned() {
            let key = format!(
                "{:?}->{:?}:{}.{},{}.{},",
                edge.from_table,
                edge.to_table,
                edge.on_left.0,
                edge.on_left.1,
                edge.on_right.0,
                edge.on_right.1
            );
            if seen.insert(key) {
                joins.push(edge);
            }
            if parent == start {
                break;
            }
            current = parent;
        }
    }

    joins.sort_by_key(|edge| {
        distance
            .get(&edge.from_table)
            .copied()
            .unwrap_or(usize::MAX)
    });
    joins
}

#[cfg(test)]
mod tests {
    use super::*;
    use airlab_lib::ctx::Ctx;
    use airlab_lib::model::clone::{CloneBmc, CloneForCreate};
    use airlab_lib::model::provider::{ProviderBmc, ProviderForCreate};
    use axum::Router;
    use tower::ServiceExt;

    type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    async fn post_search(app: &Router, request: Value) -> TestResult<Value> {
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/search")
                    .header(axum::http::header::CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::from(request.to_string()))?,
            )
            .await?;

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let body = crate::web::test_support::response_body_string(response).await?;
        let value = serde_json::from_str(&body)?;
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

    #[test]
    fn conjugate_to_protein_uses_lot_clone_path() {
        let graph = get_graph();
        let required = BTreeSet::from([ReturnType::Conjugate, ReturnType::Protein]);

        let join_plan = find_join_plan(&graph, ReturnType::Conjugate, &required);
        let edges: Vec<(ReturnType, ReturnType)> = join_plan
            .iter()
            .map(|edge| (edge.from_table, edge.to_table))
            .collect();

        assert_eq!(
            edges,
            vec![
                (ReturnType::Conjugate, ReturnType::Lot),
                (ReturnType::Lot, ReturnType::Clone),
                (ReturnType::Clone, ReturnType::Protein),
            ]
        );
    }

    #[test]
    fn conjugate_to_group_still_joins_group_directly() {
        let graph = get_graph();
        let required = BTreeSet::from([ReturnType::Conjugate, ReturnType::Group]);

        let join_plan = find_join_plan(&graph, ReturnType::Conjugate, &required);
        let edges: Vec<(ReturnType, ReturnType)> = join_plan
            .iter()
            .map(|edge| (edge.from_table, edge.to_table))
            .collect();

        assert_eq!(edges, vec![(ReturnType::Conjugate, ReturnType::Group)]);
    }

    #[tokio::test]
    async fn search_route_returns_seeded_provider_match() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = crate::web::test_support::authed_router(routes(
            crate::search_shadow::SearchState::new((*mm).clone()),
        ));
        let request = json!({
            "return_type": "Provider",
            "filters": [
                {
                    "table": "Provider",
                    "field": "name",
                    "op": "contains",
                    "value": "seed-provider"
                }
            ],
            "page": 1,
            "limit": 10
        });

        let response = post_search(&app, request).await?;
        assert_eq!(item_ids(&response)?, vec![1003]);
        assert_eq!(response["search_total"], 1);

        Ok(())
    }

    #[tokio::test]
    async fn search_route_supports_joined_filters_for_lot_results() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = crate::web::test_support::authed_router(routes(
            crate::search_shadow::SearchState::new((*mm).clone()),
        ));
        let request = json!({
            "return_type": "Lot",
            "filters": [
                {
                    "table": "Provider",
                    "field": "name",
                    "op": "contains",
                    "value": "seed-provider"
                }
            ],
            "order": {
                "table": "Lot",
                "field": "id",
                "direction": "asc"
            },
            "page": 1,
            "limit": 10
        });

        let response = post_search(&app, request).await?;
        assert_eq!(item_ids(&response)?, vec![1007]);
        assert_eq!(response["search_total"], 1);

        Ok(())
    }

    #[tokio::test]
    async fn search_route_applies_global_filter_for_lot_sql_fallback() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = crate::web::test_support::authed_router(routes(
            crate::search_shadow::SearchState::new((*mm).clone()),
        ));
        let request = json!({
            "return_type": "Lot",
            "filters": [
                {
                    "table": "Validation",
                    "field": "status",
                    "op": "eq",
                    "value": 1
                }
            ],
            "global_filter": "seed-provider",
            "order": {
                "table": "Lot",
                "field": "id",
                "direction": "asc"
            },
            "page": 1,
            "limit": 10
        });

        let response = post_search(&app, request).await?;
        assert_eq!(item_ids(&response)?, vec![1007]);
        assert_eq!(response["search_total"], 1);

        Ok(())
    }

    #[tokio::test]
    async fn search_route_supports_joined_order_for_validation_results() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = crate::web::test_support::authed_router(routes(
            crate::search_shadow::SearchState::new((*mm).clone()),
        ));
        let request = json!({
            "return_type": "Validation",
            "filters": [
                {
                    "table": "Validation",
                    "field": "status",
                    "op": "eq",
                    "value": 1
                }
            ],
            "order": {
                "table": "Clone",
                "field": "name",
                "direction": "asc"
            },
            "page": 1,
            "limit": 10
        });

        let response = post_search(&app, request).await?;
        assert_eq!(item_ids(&response)?, vec![2221, 1011]);
        assert_eq!(response["search_total"], 2);

        Ok(())
    }

    #[tokio::test]
    async fn search_route_skips_disallowed_filters_for_provider_results() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = crate::web::test_support::authed_router(routes(
            crate::search_shadow::SearchState::new((*mm).clone()),
        ));
        let request = json!({
            "return_type": "Provider",
            "filters": [
                {
                    "table": "Tag",
                    "field": "name",
                    "op": "contains",
                    "value": "seed-tag"
                }
            ],
            "order": {
                "table": "Provider",
                "field": "id",
                "direction": "asc"
            },
            "page": 1,
            "limit": 10
        });

        let response = post_search(&app, request).await?;
        assert_eq!(item_ids(&response)?, vec![103, 1003, 1013]);
        assert_eq!(response["search_total"], 3);

        Ok(())
    }

    #[tokio::test]
    async fn search_route_skips_disallowed_order_without_failing() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = crate::web::test_support::authed_router(routes(
            crate::search_shadow::SearchState::new((*mm).clone()),
        ));
        let request = json!({
            "return_type": "Provider",
            "filters": [
                {
                    "table": "Provider",
                    "field": "name",
                    "op": "contains",
                    "value": "seed-provider"
                }
            ],
            "order": {
                "table": "Tag",
                "field": "name",
                "direction": "desc"
            },
            "page": 1,
            "limit": 10
        });

        let response = post_search(&app, request).await?;
        assert_eq!(item_ids(&response)?, vec![1003]);
        assert_eq!(response["search_total"], 1);

        Ok(())
    }

    #[tokio::test]
    async fn search_route_applies_order_and_pagination_for_provider_results() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let ctx = Ctx::root_ctx();
        let unique_prefix = "search-provider-order";
        let seeded = airlab_lib::_dev_utils::get_provider_seed(unique_prefix);
        let created = airlab_lib::_dev_utils::seed_providers(&ctx, &mm, &seeded).await?;
        let app = crate::web::test_support::authed_router(routes(
            crate::search_shadow::SearchState::new((*mm).clone()),
        ));
        let request = json!({
            "return_type": "Provider",
            "filters": [
                {
                    "table": "Provider",
                    "field": "name",
                    "op": "contains",
                    "value": unique_prefix
                }
            ],
            "order": {
                "table": "Provider",
                "field": "name",
                "direction": "asc"
            },
            "page": 2,
            "limit": 2
        });

        let response = post_search(&app, request).await?;
        assert_eq!(response["search_total"], 4);
        assert_eq!(item_ids(&response)?, vec![created[2].id, created[3].id]);
        assert_eq!(response["has_previous"], true);
        assert_eq!(response["has_next"], false);

        Ok(())
    }

    #[tokio::test]
    async fn search_route_supports_clone_reactivity_scalar_filter() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let ctx = Ctx::root_ctx();
        let clone_id = CloneBmc::create(
            &ctx,
            &mm,
            CloneForCreate {
                group_id: 1000,
                created_by: Some(1303),
                protein_id: 1002,
                species_id: Some(1004),
                name: "search-reactivity-scalar".into(),
                isotype: String::new(),
                epitope: String::new(),
                is_phospho: false,
                is_polyclonal: false,
                is_archived: None,
                reactivity: Some(vec![77001, 77002]),
                application: None,
            },
        )
        .await?;
        let app = crate::web::test_support::authed_router(routes(
            crate::search_shadow::SearchState::new((*mm).clone()),
        ));
        let request = json!({
            "return_type": "Clone",
            "filters": [
                {
                    "table": "Clone",
                    "field": "group_id",
                    "op": "eq",
                    "value": 1000
                },
                {
                    "table": "Clone",
                    "field": "reactivity",
                    "op": "contains",
                    "value": 77001
                }
            ],
            "order": {
                "table": "Clone",
                "field": "id",
                "direction": "asc"
            },
            "page": 1,
            "limit": 10
        });

        let response = post_search(&app, request).await?;
        assert_eq!(item_ids(&response)?, vec![clone_id]);
        assert_eq!(response["search_total"], 1);

        Ok(())
    }

    #[tokio::test]
    async fn search_route_supports_clone_reactivity_array_overlap_filter() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let ctx = Ctx::root_ctx();
        let matching_clone_id = CloneBmc::create(
            &ctx,
            &mm,
            CloneForCreate {
                group_id: 1000,
                created_by: Some(1303),
                protein_id: 1002,
                species_id: Some(1004),
                name: "search-reactivity-array-match".into(),
                isotype: String::new(),
                epitope: String::new(),
                is_phospho: false,
                is_polyclonal: false,
                is_archived: None,
                reactivity: Some(vec![77100, 77200]),
                application: None,
            },
        )
        .await?;
        CloneBmc::create(
            &ctx,
            &mm,
            CloneForCreate {
                group_id: 1000,
                created_by: Some(1303),
                protein_id: 1002,
                species_id: Some(1004),
                name: "search-reactivity-array-miss".into(),
                isotype: String::new(),
                epitope: String::new(),
                is_phospho: false,
                is_polyclonal: false,
                is_archived: None,
                reactivity: Some(vec![77300]),
                application: None,
            },
        )
        .await?;
        let app = crate::web::test_support::authed_router(routes(
            crate::search_shadow::SearchState::new((*mm).clone()),
        ));
        let request = json!({
            "return_type": "Clone",
            "filters": [
                {
                    "table": "Clone",
                    "field": "group_id",
                    "op": "eq",
                    "value": 1000
                },
                {
                    "table": "Clone",
                    "field": "reactivity",
                    "op": "contains",
                    "value": [77200, 9999]
                }
            ],
            "order": {
                "table": "Clone",
                "field": "id",
                "direction": "asc"
            },
            "page": 1,
            "limit": 10
        });

        let response = post_search(&app, request).await?;
        assert_eq!(item_ids(&response)?, vec![matching_clone_id]);
        assert_eq!(response["search_total"], 1);

        Ok(())
    }
}
