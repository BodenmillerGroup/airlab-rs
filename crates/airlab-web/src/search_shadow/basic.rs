use crate::web::routes_search::{Direction, ReturnType, SearchOrder, allowed_filter_tables};
use airlab_lib::model::ModelManager;
use serde_json::Value;
use sqlx::FromRow;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tracing::warn;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BasicShadowKind {
    User,
    Member,
    Group,
    Protein,
    Provider,
    Species,
    Tag,
    Lot,
    Conjugate,
    Panel,
    PanelElement,
    Validation,
    Storage,
    Collection,
}

impl BasicShadowKind {
    pub const ALL: [Self; 14] = [
        Self::User,
        Self::Member,
        Self::Group,
        Self::Protein,
        Self::Provider,
        Self::Species,
        Self::Tag,
        Self::Lot,
        Self::Conjugate,
        Self::Panel,
        Self::PanelElement,
        Self::Validation,
        Self::Storage,
        Self::Collection,
    ];

    pub fn from_return_type(value: ReturnType) -> Option<Self> {
        match value {
            ReturnType::User => Some(Self::User),
            ReturnType::Member => Some(Self::Member),
            ReturnType::Group => Some(Self::Group),
            ReturnType::Protein => Some(Self::Protein),
            ReturnType::Provider => Some(Self::Provider),
            ReturnType::Species => Some(Self::Species),
            ReturnType::Tag => Some(Self::Tag),
            ReturnType::Lot => Some(Self::Lot),
            ReturnType::Conjugate => Some(Self::Conjugate),
            ReturnType::Panel => Some(Self::Panel),
            ReturnType::PanelElement => Some(Self::PanelElement),
            ReturnType::Validation => Some(Self::Validation),
            ReturnType::Storage => Some(Self::Storage),
            ReturnType::Collection => Some(Self::Collection),
            _ => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Member => "member",
            Self::Group => "group",
            Self::Protein => "protein",
            Self::Provider => "provider",
            Self::Species => "species",
            Self::Tag => "tag",
            Self::Lot => "lot",
            Self::Conjugate => "conjugate",
            Self::Panel => "panel",
            Self::PanelElement => "panel_element",
            Self::Validation => "validation",
            Self::Storage => "storage",
            Self::Collection => "collection",
        }
    }
}

#[derive(Debug)]
pub struct BasicGroupShadow {
    pub kind: BasicShadowKind,
    pub rows: Arc<Vec<BasicShadowRow>>,
}

#[derive(Debug, Clone)]
pub struct BasicShadowRow {
    pub id: i64,
    pub fulltext: String,
    values: HashMap<String, BasicShadowValue>,
}

#[derive(Debug, Clone, PartialEq)]
enum BasicShadowValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    Text(String),
}

impl BasicShadowValue {
    fn to_fulltext(&self) -> String {
        match self {
            Self::Int(value) => value.to_string(),
            Self::Float(value) => value.to_string(),
            Self::Bool(value) => value.to_string(),
            Self::Text(value) => value.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BasicShadowFilter {
    pub key: String,
    pub value: Value,
}

#[derive(Debug, Clone)]
pub struct BasicShadowQuery {
    pub kind: BasicShadowKind,
    pub filters: Vec<BasicShadowFilter>,
    pub global_filter: Option<String>,
    pub order_key: Option<String>,
    pub direction: Direction,
    pub page: i64,
    pub limit: i64,
}

#[derive(Debug, Clone)]
pub struct BasicShadowSearchResult {
    pub items: Vec<i64>,
    pub search_total: i64,
}

#[derive(Debug, FromRow)]
struct UserShadowDbRow {
    id: i64,
    email: Option<String>,
    name: Option<String>,
}

#[derive(Debug, FromRow)]
struct MemberShadowDbRow {
    id: i64,
    group_id: i64,
    user_id: i64,
    role: i64,
    all_panels: bool,
    is_active: bool,
    user_name: Option<String>,
    group_name: Option<String>,
}

#[derive(Debug, FromRow)]
struct GroupShadowDbRow {
    id: i64,
    name: Option<String>,
    institution: Option<String>,
    location: Option<String>,
    description: Option<String>,
}

#[derive(Debug, FromRow)]
struct GroupNamedDbRow {
    id: i64,
    group_id: i64,
    name: Option<String>,
    description: Option<String>,
    group_name: Option<String>,
}

#[derive(Debug, FromRow)]
struct SpeciesShadowDbRow {
    id: i64,
    group_id: i64,
    name: Option<String>,
    acronym: Option<String>,
    group_name: Option<String>,
}

#[derive(Debug, FromRow)]
struct TagShadowDbRow {
    id: i64,
    group_id: i64,
    name: Option<String>,
    description: Option<String>,
    group_name: Option<String>,
    is_metal: bool,
    is_fluorophore: bool,
    is_enzyme: bool,
    is_biotin: bool,
    is_other: bool,
    mw: Option<i64>,
    emission: Option<i64>,
    excitation: Option<i64>,
    status: i64,
}

#[derive(Debug, FromRow)]
struct StorageShadowDbRow {
    id: i64,
    name: Option<String>,
    r#type: Option<String>,
    location: Option<String>,
    temperature_c: i64,
    active: bool,
}

#[derive(Debug, FromRow)]
struct CollectionShadowDbRow {
    id: i64,
    name: Option<String>,
    description: Option<String>,
}

#[derive(Debug, FromRow)]
struct PanelShadowDbRow {
    id: i64,
    group_id: i64,
    created_by: i64,
    name: Option<String>,
    description: Option<String>,
    is_fluorophore: bool,
    is_locked: bool,
    application: Option<i64>,
    is_archived: bool,
    updated_at_epoch: i64,
}

#[derive(Debug, FromRow)]
struct LotShadowDbRow {
    id: i64,
    group_id: i64,
    created_by: i64,
    clone_id: i64,
    provider_id: Option<i64>,
    collection_id: Option<i64>,
    name: Option<String>,
    reference: Option<String>,
    number: Option<String>,
    status: i64,
    is_archived: bool,
    clone_name: Option<String>,
    protein_name: Option<String>,
    provider_name: Option<String>,
    collection_name: Option<String>,
    group_name: Option<String>,
    validation_application: Option<i64>,
    validation_status: Option<i64>,
}

fn validation_application_to_string(value: i64) -> String {
    match value {
        0 => "SMC".to_string(),
        1 => "IMC".to_string(),
        2 => "FC".to_string(),
        3 => "IF".to_string(),
        4 => "IHC".to_string(),
        5 => "IHC-F".to_string(),
        6 => "WB".to_string(),
        _ => value.to_string(),
    }
}

fn validation_status_to_string(value: i64) -> String {
    match value {
        0 => "Yes".to_string(),
        1 => "So-So".to_string(),
        2 => "No".to_string(),
        3 => "Undefined".to_string(),
        _ => value.to_string(),
    }
}

#[derive(Debug, FromRow)]
struct ConjugateShadowDbRow {
    id: i64,
    group_id: i64,
    created_by: i64,
    labeled_by: Option<i64>,
    finished_by: Option<i64>,
    lot_id: i64,
    tag_id: i64,
    storage_id: Option<i64>,
    status: i64,
    tube_number: i64,
    concentration: Option<f64>,
    description: Option<String>,
    is_archived: bool,
    custom_id: Option<String>,
    lot_name: Option<String>,
    clone_name: Option<String>,
    protein_name: Option<String>,
    species_name: Option<String>,
    tag_name: Option<String>,
    tag_mw: Option<i64>,
    collection_id: Option<i64>,
    collection_name: Option<String>,
    group_name: Option<String>,
}

#[derive(Debug, FromRow)]
struct PanelElementShadowDbRow {
    id: i64,
    panel_id: i64,
    conjugate_id: i64,
    dilution_type: i64,
    concentration: Option<f32>,
    panel_name: Option<String>,
    conjugate_description: Option<String>,
    tag_name: Option<String>,
}

#[derive(Debug, FromRow)]
struct ValidationShadowDbRow {
    id: i64,
    group_id: i64,
    created_by: i64,
    clone_id: i64,
    lot_id: Option<i64>,
    conjugate_id: Option<i64>,
    species_id: Option<i64>,
    application: i64,
    tissue: Option<String>,
    status: i64,
    antigen_retrieval_type: Option<String>,
    is_archived: bool,
    clone_name: Option<String>,
    protein_name: Option<String>,
    lot_number: Option<String>,
    tube_number: Option<i64>,
    species_name: Option<String>,
    user_name: Option<String>,
    group_name: Option<String>,
}

pub async fn build_basic_shadow(
    mm: &ModelManager,
    kind: BasicShadowKind,
) -> airlab_lib::model::Result<BasicGroupShadow> {
    let rows = match kind {
        BasicShadowKind::User => build_user_rows(mm).await?,
        BasicShadowKind::Member => build_member_rows(mm).await?,
        BasicShadowKind::Group => build_group_rows(mm).await?,
        BasicShadowKind::Protein => {
            build_named_group_rows(
                mm,
                kind,
                r#"
            SELECT
                p.id AS id,
                p.group_id AS group_id,
                p.name AS name,
                p.description AS description,
                g.name AS group_name
            FROM public.protein p
            LEFT JOIN public."group" g ON g.id = p.group_id
            "#,
            )
            .await?
        }
        BasicShadowKind::Provider => {
            build_named_group_rows(
                mm,
                kind,
                r#"
            SELECT
                p.id AS id,
                p.group_id AS group_id,
                p.name AS name,
                p.description AS description,
                g.name AS group_name
            FROM public.provider p
            LEFT JOIN public."group" g ON g.id = p.group_id
            "#,
            )
            .await?
        }
        BasicShadowKind::Species => build_species_rows(mm).await?,
        BasicShadowKind::Tag => build_tag_rows(mm).await?,
        BasicShadowKind::Lot => build_lot_rows(mm).await?,
        BasicShadowKind::Conjugate => build_conjugate_rows(mm).await?,
        BasicShadowKind::Panel => build_panel_rows(mm).await?,
        BasicShadowKind::PanelElement => build_panel_element_rows(mm).await?,
        BasicShadowKind::Validation => build_validation_rows(mm).await?,
        BasicShadowKind::Storage => build_storage_rows(mm).await?,
        BasicShadowKind::Collection => build_collection_rows(mm).await?,
    };

    Ok(BasicGroupShadow {
        kind,
        rows: Arc::new(rows),
    })
}

pub fn search_basic_shadow(
    shadow: &BasicGroupShadow,
    query: &BasicShadowQuery,
) -> BasicShadowSearchResult {
    warn!(
        kind = shadow.kind.label(),
        row_count = shadow.rows.len(),
        filter_count = query.filters.len(),
        has_global_filter = query
            .global_filter
            .as_ref()
            .is_some_and(|value| !value.trim().is_empty()),
        order_key = query.order_key.as_deref().unwrap_or("id"),
        direction = ?query.direction,
        page = query.page,
        limit = query.limit,
        "searching basic shadow table"
    );

    let mut matches = shadow
        .rows
        .iter()
        .filter(|row| {
            query
                .filters
                .iter()
                .all(|filter| row_matches_filter(row, filter))
        })
        .filter(|row| row_matches_global(row, &query.global_filter))
        .collect::<Vec<_>>();

    let order_key = query.order_key.as_deref().unwrap_or("id");
    matches.sort_by(|left, right| compare_rows(left, right, order_key, &query.direction));

    let mut seen_ids = HashSet::new();
    let deduped = matches
        .into_iter()
        .filter(|row| seen_ids.insert(row.id))
        .collect::<Vec<_>>();

    let total = deduped.len() as i64;
    let limit = query.limit.max(1) as usize;
    let page = query.page.max(1) as usize;
    let start = (page.saturating_sub(1)) * limit;
    let end = (start + limit).min(deduped.len());

    BasicShadowSearchResult {
        items: deduped
            .get(start..end)
            .unwrap_or(&[])
            .iter()
            .map(|row| row.id)
            .collect(),
        search_total: total,
    }
}

pub fn map_basic_shadow_query(
    req: &crate::web::routes_search::RpcSearchRequest,
) -> Option<BasicShadowQuery> {
    let kind = BasicShadowKind::from_return_type(req.return_type)?;
    let allowed = allowed_filter_tables(req.return_type);
    let filters = req
        .filters
        .iter()
        .filter(|filter| allowed.contains(&filter.table))
        .filter_map(|filter| {
            let Some(key) = field_key(kind, filter.table, &filter.field) else {
                warn!(
                    kind = kind.label(),
                    table = ?filter.table,
                    field = filter.field,
                    "skipping unsupported shadow filter"
                );
                return None;
            };

            Some(BasicShadowFilter {
                key,
                value: filter.value.clone(),
            })
        })
        .collect::<Vec<_>>();

    let (order_key, direction) = match req.order.as_ref() {
        Some(order) if allowed.contains(&order.table) => match map_order_key(kind, order) {
            Some(key) => (Some(key), order.direction),
            None => {
                warn!(
                    kind = kind.label(),
                    table = ?order.table,
                    field = order.field,
                    "skipping unsupported shadow order"
                );
                (None, Direction::Asc)
            }
        },
        None => (None, Direction::Asc),
        Some(_) => (None, Direction::Asc),
    };

    Some(BasicShadowQuery {
        kind,
        filters,
        global_filter: req.global_filter.clone(),
        order_key,
        direction,
        page: req.page.unwrap_or(1),
        limit: req.limit.unwrap_or(crate::web::routes_search::PAGESIZE),
    })
}

fn field_key(kind: BasicShadowKind, table: ReturnType, field: &str) -> Option<String> {
    if table == ReturnType::Group && field == "name" {
        return match kind {
            BasicShadowKind::Group => Some("name".to_string()),
            BasicShadowKind::Member => Some("group_name".to_string()),
            BasicShadowKind::Protein
            | BasicShadowKind::Provider
            | BasicShadowKind::Species
            | BasicShadowKind::Tag
            | BasicShadowKind::Lot
            | BasicShadowKind::Conjugate
            | BasicShadowKind::Validation => Some("group_name".to_string()),
            _ => None,
        };
    }

    if kind == BasicShadowKind::Member && table == ReturnType::User && field == "name" {
        return Some("user_name".to_string());
    }

    if kind == BasicShadowKind::Validation {
        if let Some(key) = match (table, field) {
            (ReturnType::Clone, "name") => Some("clone_name".to_string()),
            (ReturnType::Protein, "name") => Some("protein_name".to_string()),
            (ReturnType::Lot, "number") => Some("lot_number".to_string()),
            (ReturnType::Conjugate, "tube_number") => Some("tube_number".to_string()),
            (ReturnType::Species, "name") => Some("species_name".to_string()),
            (ReturnType::User, "name") => Some("user_name".to_string()),
            _ => None,
        } {
            return Some(key);
        }
    }

    if kind == BasicShadowKind::Lot {
        if let Some(key) = match (table, field) {
            (ReturnType::Clone, "name") => Some("clone_name".to_string()),
            (ReturnType::Protein, "name") => Some("protein_name".to_string()),
            (ReturnType::Provider, "name") => Some("provider_name".to_string()),
            (ReturnType::Collection, "id") => Some("collection_id".to_string()),
            (ReturnType::Collection, "name") => Some("collection_name".to_string()),
            (ReturnType::Validation, "application") => Some("validation_application".to_string()),
            (ReturnType::Validation, "status") => Some("validation_status".to_string()),
            _ => None,
        } {
            return Some(key);
        }
    }

    if kind == BasicShadowKind::Conjugate {
        if let Some(key) = match (table, field) {
            (ReturnType::Lot, "name") => Some("lot_name".to_string()),
            (ReturnType::Clone, "name") => Some("clone_name".to_string()),
            (ReturnType::Protein, "name") => Some("protein_name".to_string()),
            (ReturnType::Species, "name") => Some("species_name".to_string()),
            (ReturnType::Tag, "name") => Some("tag_name".to_string()),
            (ReturnType::Tag, "mw") => Some("tag_mw".to_string()),
            (ReturnType::Collection, "id") => Some("collection_id".to_string()),
            (ReturnType::Collection, "name") => Some("collection_name".to_string()),
            _ => None,
        } {
            return Some(key);
        }
    }

    if kind == BasicShadowKind::PanelElement {
        if let Some(key) = match (table, field) {
            (ReturnType::Panel, "name") => Some("panel_name".to_string()),
            (ReturnType::Conjugate, "description") => Some("conjugate_description".to_string()),
            (ReturnType::Tag, "name") => Some("tag_name".to_string()),
            _ => None,
        } {
            return Some(key);
        }
    }

    match (table, field) {
        (ReturnType::User, "id") => Some("id".to_string()),
        (ReturnType::User, "name") => Some("name".to_string()),
        (ReturnType::User, "email") => Some("email".to_string()),

        (ReturnType::Member, "id") => Some("id".to_string()),
        (ReturnType::Member, "group_id") => Some("group_id".to_string()),
        (ReturnType::Member, "user_id") => Some("user_id".to_string()),
        (ReturnType::Member, "role") => Some("role".to_string()),
        (ReturnType::Member, "all_panels") => Some("all_panels".to_string()),
        (ReturnType::Member, "is_active") => Some("is_active".to_string()),

        (ReturnType::Group, "id") => Some("id".to_string()),
        (ReturnType::Group, "name") => Some("name".to_string()),
        (ReturnType::Group, "institution") => Some("institution".to_string()),
        (ReturnType::Group, "city") | (ReturnType::Group, "location") => {
            Some("location".to_string())
        }
        (ReturnType::Group, "description") => Some("description".to_string()),

        (ReturnType::Protein, "id") => Some("id".to_string()),
        (ReturnType::Protein, "group_id") => Some("group_id".to_string()),
        (ReturnType::Protein, "name") => Some("name".to_string()),
        (ReturnType::Protein, "description") => Some("description".to_string()),

        (ReturnType::Provider, "id") => Some("id".to_string()),
        (ReturnType::Provider, "group_id") => Some("group_id".to_string()),
        (ReturnType::Provider, "name") => Some("name".to_string()),
        (ReturnType::Provider, "description") => Some("description".to_string()),

        (ReturnType::Species, "id") => Some("id".to_string()),
        (ReturnType::Species, "group_id") => Some("group_id".to_string()),
        (ReturnType::Species, "name") => Some("name".to_string()),
        (ReturnType::Species, "acronym") => Some("acronym".to_string()),

        (ReturnType::Tag, "id") => Some("id".to_string()),
        (ReturnType::Tag, "group_id") => Some("group_id".to_string()),
        (ReturnType::Tag, "name") => Some("name".to_string()),
        (ReturnType::Tag, "description") => Some("description".to_string()),
        (ReturnType::Tag, "is_metal") => Some("is_metal".to_string()),
        (ReturnType::Tag, "is_fluorophore") => Some("is_fluorophore".to_string()),
        (ReturnType::Tag, "is_enzyme") => Some("is_enzyme".to_string()),
        (ReturnType::Tag, "is_biotin") => Some("is_biotin".to_string()),
        (ReturnType::Tag, "is_other") => Some("is_other".to_string()),
        (ReturnType::Tag, "mw") => Some("mw".to_string()),
        (ReturnType::Tag, "emission") => Some("emission".to_string()),
        (ReturnType::Tag, "excitation") => Some("excitation".to_string()),
        (ReturnType::Tag, "status") => Some("status".to_string()),

        (ReturnType::Lot, "id") => Some("id".to_string()),
        (ReturnType::Lot, "group_id") => Some("group_id".to_string()),
        (ReturnType::Lot, "created_by") => Some("created_by".to_string()),
        (ReturnType::Lot, "clone_id") => Some("clone_id".to_string()),
        (ReturnType::Lot, "provider_id") => Some("provider_id".to_string()),
        (ReturnType::Lot, "collection_id") => Some("collection_id".to_string()),
        (ReturnType::Lot, "name") => Some("name".to_string()),
        (ReturnType::Lot, "reference") => Some("reference".to_string()),
        (ReturnType::Lot, "number") => Some("number".to_string()),
        (ReturnType::Lot, "status") => Some("status".to_string()),
        (ReturnType::Lot, "is_archived") => Some("is_archived".to_string()),

        (ReturnType::Conjugate, "id") => Some("id".to_string()),
        (ReturnType::Conjugate, "group_id") => Some("group_id".to_string()),
        (ReturnType::Conjugate, "created_by") => Some("created_by".to_string()),
        (ReturnType::Conjugate, "labeled_by") => Some("labeled_by".to_string()),
        (ReturnType::Conjugate, "finished_by") => Some("finished_by".to_string()),
        (ReturnType::Conjugate, "lot_id") => Some("lot_id".to_string()),
        (ReturnType::Conjugate, "tag_id") => Some("tag_id".to_string()),
        (ReturnType::Conjugate, "storage_id") => Some("storage_id".to_string()),
        (ReturnType::Conjugate, "status") => Some("status".to_string()),
        (ReturnType::Conjugate, "tube_number") => Some("tube_number".to_string()),
        (ReturnType::Conjugate, "concentration") => Some("concentration".to_string()),
        (ReturnType::Conjugate, "description") => Some("description".to_string()),
        (ReturnType::Conjugate, "custom_id") => Some("custom_id".to_string()),
        (ReturnType::Conjugate, "is_archived") => Some("is_archived".to_string()),

        (ReturnType::Panel, "id") => Some("id".to_string()),
        (ReturnType::Panel, "group_id") => Some("group_id".to_string()),
        (ReturnType::Panel, "created_by") => Some("created_by".to_string()),
        (ReturnType::Panel, "name") => Some("name".to_string()),
        (ReturnType::Panel, "description") => Some("description".to_string()),
        (ReturnType::Panel, "is_fluorophore") => Some("is_fluorophore".to_string()),
        (ReturnType::Panel, "is_locked") => Some("is_locked".to_string()),
        (ReturnType::Panel, "application") => Some("application".to_string()),
        (ReturnType::Panel, "is_archived") => Some("is_archived".to_string()),
        (ReturnType::Panel, "updated_at") => Some("updated_at".to_string()),

        (ReturnType::PanelElement, "id") => Some("id".to_string()),
        (ReturnType::PanelElement, "panel_id") => Some("panel_id".to_string()),
        (ReturnType::PanelElement, "conjugate_id") => Some("conjugate_id".to_string()),
        (ReturnType::PanelElement, "dilution_type") => Some("dilution_type".to_string()),
        (ReturnType::PanelElement, "concentration") => Some("concentration".to_string()),

        (ReturnType::Validation, "id") => Some("id".to_string()),
        (ReturnType::Validation, "group_id") => Some("group_id".to_string()),
        (ReturnType::Validation, "created_by") => Some("created_by".to_string()),
        (ReturnType::Validation, "clone_id") => Some("clone_id".to_string()),
        (ReturnType::Validation, "lot_id") => Some("lot_id".to_string()),
        (ReturnType::Validation, "conjugate_id") => Some("conjugate_id".to_string()),
        (ReturnType::Validation, "species_id") => Some("species_id".to_string()),
        (ReturnType::Validation, "application") => Some("application".to_string()),
        (ReturnType::Validation, "tissue") => Some("tissue".to_string()),
        (ReturnType::Validation, "status") => Some("status".to_string()),
        (ReturnType::Validation, "antigen_retrieval_type") => {
            Some("antigen_retrieval_type".to_string())
        }
        (ReturnType::Validation, "is_archived") => Some("is_archived".to_string()),

        (ReturnType::Storage, "id") => Some("id".to_string()),
        (ReturnType::Storage, "name") => Some("name".to_string()),
        (ReturnType::Storage, "type") => Some("type".to_string()),
        (ReturnType::Storage, "location") => Some("location".to_string()),
        (ReturnType::Storage, "temperature_c") => Some("temperature_c".to_string()),
        (ReturnType::Storage, "active") => Some("active".to_string()),

        (ReturnType::Collection, "id") => Some("id".to_string()),
        (ReturnType::Collection, "name") => Some("name".to_string()),
        (ReturnType::Collection, "description") => Some("description".to_string()),
        _ => None,
    }
}

fn map_order_key(kind: BasicShadowKind, order: &SearchOrder) -> Option<String> {
    let key = field_key(kind, order.table, &order.field)?;

    match kind {
        BasicShadowKind::User
        | BasicShadowKind::Member
        | BasicShadowKind::Group
        | BasicShadowKind::Protein
        | BasicShadowKind::Provider
        | BasicShadowKind::Species
        | BasicShadowKind::Tag
        | BasicShadowKind::Lot
        | BasicShadowKind::Conjugate
        | BasicShadowKind::Panel
        | BasicShadowKind::PanelElement
        | BasicShadowKind::Validation
        | BasicShadowKind::Storage
        | BasicShadowKind::Collection => Some(key),
    }
}

fn row_matches_filter(row: &BasicShadowRow, filter: &BasicShadowFilter) -> bool {
    let Some(existing) = row.values.get(&filter.key) else {
        return false;
    };

    match (&filter.value, existing) {
        (Value::String(needle), BasicShadowValue::Text(text)) => contains_ci(text, needle),
        (Value::Number(number), BasicShadowValue::Int(value)) => number.as_i64() == Some(*value),
        (Value::Number(number), BasicShadowValue::Float(value)) => number.as_f64() == Some(*value),
        (Value::Bool(expected), BasicShadowValue::Bool(value)) => *expected == *value,
        (Value::Array(values), BasicShadowValue::Int(value)) => values
            .iter()
            .filter_map(Value::as_i64)
            .any(|item| item == *value),
        (Value::Array(values), BasicShadowValue::Text(text)) => values
            .iter()
            .filter_map(Value::as_str)
            .any(|item| item.eq_ignore_ascii_case(text)),
        _ => false,
    }
}

fn row_matches_global(row: &BasicShadowRow, global: &Option<String>) -> bool {
    match global {
        Some(value) if !value.trim().is_empty() => {
            row.fulltext.contains(&value.trim().to_lowercase())
        }
        _ => true,
    }
}

fn compare_rows(
    left: &BasicShadowRow,
    right: &BasicShadowRow,
    order_key: &str,
    direction: &Direction,
) -> Ordering {
    let ord = compare_values(left.values.get(order_key), right.values.get(order_key))
        .then_with(|| left.id.cmp(&right.id));
    match direction {
        Direction::Asc => ord,
        Direction::Desc => ord.reverse(),
    }
}

fn compare_values(left: Option<&BasicShadowValue>, right: Option<&BasicShadowValue>) -> Ordering {
    match (left, right) {
        (Some(BasicShadowValue::Int(left)), Some(BasicShadowValue::Int(right))) => left.cmp(right),
        (Some(BasicShadowValue::Float(left)), Some(BasicShadowValue::Float(right))) => {
            left.partial_cmp(right).unwrap_or(Ordering::Equal)
        }
        (Some(BasicShadowValue::Bool(left)), Some(BasicShadowValue::Bool(right))) => {
            left.cmp(right)
        }
        (Some(BasicShadowValue::Text(left)), Some(BasicShadowValue::Text(right))) => {
            left.to_lowercase().cmp(&right.to_lowercase())
        }
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        _ => Ordering::Equal,
    }
}

fn contains_ci(haystack: &str, needle: &str) -> bool {
    haystack
        .to_lowercase()
        .contains(&needle.trim().to_lowercase())
}

fn row_from_pairs(id: i64, pairs: Vec<(&'static str, BasicShadowValue)>) -> BasicShadowRow {
    let values = pairs
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect::<HashMap<_, _>>();
    let fulltext = values
        .values()
        .map(BasicShadowValue::to_fulltext)
        .map(|value| value.trim().to_lowercase())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>()
        .join(" ");

    BasicShadowRow {
        id,
        fulltext,
        values,
    }
}

async fn build_user_rows(mm: &ModelManager) -> airlab_lib::model::Result<Vec<BasicShadowRow>> {
    let db_rows: Vec<UserShadowDbRow> = sqlx::query_as(
        r#"
        SELECT id, email, name
        FROM public."user"
        "#,
    )
    .fetch_all(mm.db())
    .await
    .map_err(airlab_lib::model::Error::from)?;

    Ok(db_rows
        .into_iter()
        .map(|row| {
            row_from_pairs(
                row.id,
                vec![
                    ("id", BasicShadowValue::Int(row.id)),
                    (
                        "email",
                        BasicShadowValue::Text(row.email.unwrap_or_default()),
                    ),
                    ("name", BasicShadowValue::Text(row.name.unwrap_or_default())),
                ],
            )
        })
        .collect())
}

async fn build_member_rows(mm: &ModelManager) -> airlab_lib::model::Result<Vec<BasicShadowRow>> {
    let db_rows: Vec<MemberShadowDbRow> = sqlx::query_as(
        r#"
        SELECT
            m.id AS id,
            m.group_id AS group_id,
            m.user_id AS user_id,
            m.role AS role,
            m.all_panels AS all_panels,
            m.is_active AS is_active,
            u.name AS user_name,
            g.name AS group_name
        FROM public.member m
        LEFT JOIN public."user" u ON u.id = m.user_id
        LEFT JOIN public."group" g ON g.id = m.group_id
        "#,
    )
    .fetch_all(mm.db())
    .await
    .map_err(airlab_lib::model::Error::from)?;

    Ok(db_rows
        .into_iter()
        .map(|row| {
            row_from_pairs(
                row.id,
                vec![
                    ("id", BasicShadowValue::Int(row.id)),
                    ("group_id", BasicShadowValue::Int(row.group_id)),
                    ("user_id", BasicShadowValue::Int(row.user_id)),
                    ("role", BasicShadowValue::Int(row.role)),
                    ("all_panels", BasicShadowValue::Bool(row.all_panels)),
                    ("is_active", BasicShadowValue::Bool(row.is_active)),
                    (
                        "user_name",
                        BasicShadowValue::Text(row.user_name.unwrap_or_default()),
                    ),
                    (
                        "group_name",
                        BasicShadowValue::Text(row.group_name.unwrap_or_default()),
                    ),
                ],
            )
        })
        .collect())
}

async fn build_group_rows(mm: &ModelManager) -> airlab_lib::model::Result<Vec<BasicShadowRow>> {
    let db_rows: Vec<GroupShadowDbRow> = sqlx::query_as(
        r#"
        SELECT id, name, institution, location, description
        FROM public."group"
        "#,
    )
    .fetch_all(mm.db())
    .await
    .map_err(airlab_lib::model::Error::from)?;

    Ok(db_rows
        .into_iter()
        .map(|row| {
            row_from_pairs(
                row.id,
                vec![
                    ("id", BasicShadowValue::Int(row.id)),
                    ("name", BasicShadowValue::Text(row.name.unwrap_or_default())),
                    (
                        "institution",
                        BasicShadowValue::Text(row.institution.unwrap_or_default()),
                    ),
                    (
                        "location",
                        BasicShadowValue::Text(row.location.unwrap_or_default()),
                    ),
                    (
                        "description",
                        BasicShadowValue::Text(row.description.unwrap_or_default()),
                    ),
                ],
            )
        })
        .collect())
}

async fn build_named_group_rows(
    mm: &ModelManager,
    _kind: BasicShadowKind,
    sql: &str,
) -> airlab_lib::model::Result<Vec<BasicShadowRow>> {
    let db_rows: Vec<GroupNamedDbRow> = sqlx::query_as(sql)
        .fetch_all(mm.db())
        .await
        .map_err(airlab_lib::model::Error::from)?;

    Ok(db_rows
        .into_iter()
        .map(|row| {
            row_from_pairs(
                row.id,
                vec![
                    ("id", BasicShadowValue::Int(row.id)),
                    ("group_id", BasicShadowValue::Int(row.group_id)),
                    ("name", BasicShadowValue::Text(row.name.unwrap_or_default())),
                    (
                        "description",
                        BasicShadowValue::Text(row.description.unwrap_or_default()),
                    ),
                    (
                        "group_name",
                        BasicShadowValue::Text(row.group_name.unwrap_or_default()),
                    ),
                ],
            )
        })
        .collect())
}

async fn build_tag_rows(mm: &ModelManager) -> airlab_lib::model::Result<Vec<BasicShadowRow>> {
    let db_rows: Vec<TagShadowDbRow> = sqlx::query_as(
        r#"
        SELECT
            t.id AS id,
            t.group_id AS group_id,
            t.name AS name,
            t.description AS description,
            g.name AS group_name,
            t.is_metal AS is_metal,
            t.is_fluorophore AS is_fluorophore,
            t.is_enzyme AS is_enzyme,
            t.is_biotin AS is_biotin,
            t.is_other AS is_other,
            t.mw AS mw,
            t.emission AS emission,
            t.excitation AS excitation,
            t.status AS status
        FROM public.tag t
        LEFT JOIN public."group" g ON g.id = t.group_id
        "#,
    )
    .fetch_all(mm.db())
    .await
    .map_err(airlab_lib::model::Error::from)?;

    Ok(db_rows
        .into_iter()
        .map(|row| {
            row_from_pairs(
                row.id,
                vec![
                    ("id", BasicShadowValue::Int(row.id)),
                    ("group_id", BasicShadowValue::Int(row.group_id)),
                    ("name", BasicShadowValue::Text(row.name.unwrap_or_default())),
                    (
                        "description",
                        BasicShadowValue::Text(row.description.unwrap_or_default()),
                    ),
                    (
                        "group_name",
                        BasicShadowValue::Text(row.group_name.unwrap_or_default()),
                    ),
                    ("is_metal", BasicShadowValue::Bool(row.is_metal)),
                    ("is_fluorophore", BasicShadowValue::Bool(row.is_fluorophore)),
                    ("is_enzyme", BasicShadowValue::Bool(row.is_enzyme)),
                    ("is_biotin", BasicShadowValue::Bool(row.is_biotin)),
                    ("is_other", BasicShadowValue::Bool(row.is_other)),
                    ("mw", BasicShadowValue::Int(row.mw.unwrap_or_default())),
                    (
                        "emission",
                        BasicShadowValue::Int(row.emission.unwrap_or_default()),
                    ),
                    (
                        "excitation",
                        BasicShadowValue::Int(row.excitation.unwrap_or_default()),
                    ),
                    ("status", BasicShadowValue::Int(row.status)),
                ],
            )
        })
        .collect())
}

async fn build_species_rows(mm: &ModelManager) -> airlab_lib::model::Result<Vec<BasicShadowRow>> {
    let db_rows: Vec<SpeciesShadowDbRow> = sqlx::query_as(
        r#"
        SELECT
            s.id AS id,
            s.group_id AS group_id,
            s.name AS name,
            s.acronym AS acronym,
            g.name AS group_name
        FROM public.species s
        LEFT JOIN public."group" g ON g.id = s.group_id
        "#,
    )
    .fetch_all(mm.db())
    .await
    .map_err(airlab_lib::model::Error::from)?;

    Ok(db_rows
        .into_iter()
        .map(|row| {
            row_from_pairs(
                row.id,
                vec![
                    ("id", BasicShadowValue::Int(row.id)),
                    ("group_id", BasicShadowValue::Int(row.group_id)),
                    ("name", BasicShadowValue::Text(row.name.unwrap_or_default())),
                    (
                        "acronym",
                        BasicShadowValue::Text(row.acronym.unwrap_or_default()),
                    ),
                    (
                        "group_name",
                        BasicShadowValue::Text(row.group_name.unwrap_or_default()),
                    ),
                ],
            )
        })
        .collect())
}

async fn build_panel_rows(mm: &ModelManager) -> airlab_lib::model::Result<Vec<BasicShadowRow>> {
    let db_rows: Vec<PanelShadowDbRow> = sqlx::query_as(
        r#"
        SELECT
            p.id AS id,
            p.group_id AS group_id,
            p.created_by AS created_by,
            p.name AS name,
            p.description AS description,
            p.is_fluorophore AS is_fluorophore,
            p.is_locked AS is_locked,
            p.application AS application,
            p.is_archived AS is_archived,
            EXTRACT(EPOCH FROM p.updated_at)::bigint AS updated_at_epoch
        FROM public.panel p
        "#,
    )
    .fetch_all(mm.db())
    .await
    .map_err(airlab_lib::model::Error::from)?;

    Ok(db_rows
        .into_iter()
        .map(|row| {
            row_from_pairs(
                row.id,
                vec![
                    ("id", BasicShadowValue::Int(row.id)),
                    ("group_id", BasicShadowValue::Int(row.group_id)),
                    ("created_by", BasicShadowValue::Int(row.created_by)),
                    ("name", BasicShadowValue::Text(row.name.unwrap_or_default())),
                    (
                        "description",
                        BasicShadowValue::Text(row.description.unwrap_or_default()),
                    ),
                    ("is_fluorophore", BasicShadowValue::Bool(row.is_fluorophore)),
                    ("is_locked", BasicShadowValue::Bool(row.is_locked)),
                    (
                        "application",
                        BasicShadowValue::Int(row.application.unwrap_or_default()),
                    ),
                    ("is_archived", BasicShadowValue::Bool(row.is_archived)),
                    ("updated_at", BasicShadowValue::Int(row.updated_at_epoch)),
                ],
            )
        })
        .collect())
}

async fn build_lot_rows(mm: &ModelManager) -> airlab_lib::model::Result<Vec<BasicShadowRow>> {
    let db_rows: Vec<LotShadowDbRow> = sqlx::query_as(
        r#"
        SELECT
            l.id AS id,
            l.group_id AS group_id,
            l.created_by AS created_by,
            l.clone_id AS clone_id,
            l.provider_id AS provider_id,
            l.collection_id AS collection_id,
            l.name AS name,
            l.reference AS reference,
            l.number AS number,
            l.status AS status,
            l.is_archived AS is_archived,
            c.name AS clone_name,
            p.name AS protein_name,
            pr.name AS provider_name,
            co.name AS collection_name,
            g.name AS group_name,
            v.application AS validation_application,
            v.status AS validation_status
        FROM public.lot l
        LEFT JOIN public.clone c ON c.id = l.clone_id
        LEFT JOIN public.protein p ON p.id = c.protein_id
        LEFT JOIN public.provider pr ON pr.id = l.provider_id
        LEFT JOIN public.collection co ON co.id = l.collection_id
        LEFT JOIN public."group" g ON g.id = l.group_id
        LEFT JOIN public.validation v ON v.lot_id = l.id
        "#,
    )
    .fetch_all(mm.db())
    .await
    .map_err(airlab_lib::model::Error::from)?;

    Ok(db_rows
        .into_iter()
        .map(|row| {
            let mut pairs = vec![
                ("id", BasicShadowValue::Int(row.id)),
                ("group_id", BasicShadowValue::Int(row.group_id)),
                ("created_by", BasicShadowValue::Int(row.created_by)),
                ("clone_id", BasicShadowValue::Int(row.clone_id)),
                (
                    "provider_id",
                    BasicShadowValue::Int(row.provider_id.unwrap_or_default()),
                ),
                (
                    "collection_id",
                    BasicShadowValue::Int(row.collection_id.unwrap_or_default()),
                ),
                ("name", BasicShadowValue::Text(row.name.unwrap_or_default())),
                (
                    "reference",
                    BasicShadowValue::Text(row.reference.unwrap_or_default()),
                ),
                (
                    "number",
                    BasicShadowValue::Text(row.number.unwrap_or_default()),
                ),
                ("status", BasicShadowValue::Int(row.status)),
                ("is_archived", BasicShadowValue::Bool(row.is_archived)),
                (
                    "clone_name",
                    BasicShadowValue::Text(row.clone_name.unwrap_or_default()),
                ),
                (
                    "protein_name",
                    BasicShadowValue::Text(row.protein_name.unwrap_or_default()),
                ),
                (
                    "provider_name",
                    BasicShadowValue::Text(row.provider_name.unwrap_or_default()),
                ),
                (
                    "collection_name",
                    BasicShadowValue::Text(row.collection_name.unwrap_or_default()),
                ),
                (
                    "group_name",
                    BasicShadowValue::Text(row.group_name.unwrap_or_default()),
                ),
            ];

            if let Some(value) = row.validation_application {
                pairs.push(("validation_application", BasicShadowValue::Int(value)));
                pairs.push((
                    "validation_application_label",
                    BasicShadowValue::Text(validation_application_to_string(value)),
                ));
            }

            if let Some(value) = row.validation_status {
                pairs.push(("validation_status", BasicShadowValue::Int(value)));
                pairs.push((
                    "validation_status_label",
                    BasicShadowValue::Text(validation_status_to_string(value)),
                ));
            }

            row_from_pairs(row.id, pairs)
        })
        .collect())
}

async fn build_conjugate_rows(mm: &ModelManager) -> airlab_lib::model::Result<Vec<BasicShadowRow>> {
    let db_rows: Vec<ConjugateShadowDbRow> = sqlx::query_as(
        r#"
        SELECT
            c.id AS id,
            c.group_id AS group_id,
            c.created_by AS created_by,
            c.labeled_by AS labeled_by,
            c.finished_by AS finished_by,
            c.lot_id AS lot_id,
            c.tag_id AS tag_id,
            c.storage_id AS storage_id,
            c.status AS status,
            c.tube_number AS tube_number,
            c.concentration AS concentration,
            c.description AS description,
            c.is_archived AS is_archived,
            c.custom_id AS custom_id,
            l.name AS lot_name,
            cl.name AS clone_name,
            p.name AS protein_name,
            s.name AS species_name,
            t.name AS tag_name,
            t.mw AS tag_mw,
            l.collection_id AS collection_id,
            co.name AS collection_name,
            g.name AS group_name
        FROM public.conjugate c
        LEFT JOIN public.lot l ON l.id = c.lot_id
        LEFT JOIN public.clone cl ON cl.id = l.clone_id
        LEFT JOIN public.protein p ON p.id = cl.protein_id
        LEFT JOIN public.species s ON s.id = cl.species_id
        LEFT JOIN public.tag t ON t.id = c.tag_id
        LEFT JOIN public.collection co ON co.id = l.collection_id
        LEFT JOIN public."group" g ON g.id = c.group_id
        "#,
    )
    .fetch_all(mm.db())
    .await
    .map_err(airlab_lib::model::Error::from)?;

    Ok(db_rows
        .into_iter()
        .map(|row| {
            let mut pairs = vec![
                ("id", BasicShadowValue::Int(row.id)),
                ("group_id", BasicShadowValue::Int(row.group_id)),
                ("created_by", BasicShadowValue::Int(row.created_by)),
                (
                    "labeled_by",
                    BasicShadowValue::Int(row.labeled_by.unwrap_or_default()),
                ),
                (
                    "finished_by",
                    BasicShadowValue::Int(row.finished_by.unwrap_or_default()),
                ),
                ("lot_id", BasicShadowValue::Int(row.lot_id)),
                ("tag_id", BasicShadowValue::Int(row.tag_id)),
                (
                    "storage_id",
                    BasicShadowValue::Int(row.storage_id.unwrap_or_default()),
                ),
                ("status", BasicShadowValue::Int(row.status)),
                ("tube_number", BasicShadowValue::Int(row.tube_number)),
                (
                    "description",
                    BasicShadowValue::Text(row.description.unwrap_or_default()),
                ),
                (
                    "custom_id",
                    BasicShadowValue::Text(row.custom_id.unwrap_or_default()),
                ),
                ("is_archived", BasicShadowValue::Bool(row.is_archived)),
                (
                    "lot_name",
                    BasicShadowValue::Text(row.lot_name.unwrap_or_default()),
                ),
                (
                    "clone_name",
                    BasicShadowValue::Text(row.clone_name.unwrap_or_default()),
                ),
                (
                    "protein_name",
                    BasicShadowValue::Text(row.protein_name.unwrap_or_default()),
                ),
                (
                    "species_name",
                    BasicShadowValue::Text(row.species_name.unwrap_or_default()),
                ),
                (
                    "tag_name",
                    BasicShadowValue::Text(row.tag_name.unwrap_or_default()),
                ),
                (
                    "tag_mw",
                    BasicShadowValue::Int(row.tag_mw.unwrap_or_default()),
                ),
                (
                    "collection_id",
                    BasicShadowValue::Int(row.collection_id.unwrap_or_default()),
                ),
                (
                    "collection_name",
                    BasicShadowValue::Text(row.collection_name.unwrap_or_default()),
                ),
                (
                    "group_name",
                    BasicShadowValue::Text(row.group_name.unwrap_or_default()),
                ),
            ];
            if let Some(value) = row.concentration {
                pairs.push(("concentration", BasicShadowValue::Float(value as f64)));
            }
            row_from_pairs(row.id, pairs)
        })
        .collect())
}

async fn build_panel_element_rows(
    mm: &ModelManager,
) -> airlab_lib::model::Result<Vec<BasicShadowRow>> {
    let db_rows: Vec<PanelElementShadowDbRow> = sqlx::query_as(
        r#"
        SELECT
            pe.id AS id,
            pe.panel_id AS panel_id,
            pe.conjugate_id AS conjugate_id,
            pe.dilution_type AS dilution_type,
            pe.concentration AS concentration,
            p.name AS panel_name,
            c.description AS conjugate_description,
            t.name AS tag_name
        FROM public.panel_element pe
        LEFT JOIN public.panel p ON p.id = pe.panel_id
        LEFT JOIN public.conjugate c ON c.id = pe.conjugate_id
        LEFT JOIN public.tag t ON t.id = c.tag_id
        "#,
    )
    .fetch_all(mm.db())
    .await
    .map_err(airlab_lib::model::Error::from)?;

    Ok(db_rows
        .into_iter()
        .map(|row| {
            let mut pairs = vec![
                ("id", BasicShadowValue::Int(row.id)),
                ("panel_id", BasicShadowValue::Int(row.panel_id)),
                ("conjugate_id", BasicShadowValue::Int(row.conjugate_id)),
                ("dilution_type", BasicShadowValue::Int(row.dilution_type)),
                (
                    "panel_name",
                    BasicShadowValue::Text(row.panel_name.unwrap_or_default()),
                ),
                (
                    "conjugate_description",
                    BasicShadowValue::Text(row.conjugate_description.unwrap_or_default()),
                ),
                (
                    "tag_name",
                    BasicShadowValue::Text(row.tag_name.unwrap_or_default()),
                ),
            ];
            if let Some(value) = row.concentration {
                pairs.push(("concentration", BasicShadowValue::Float(value as f64)));
            }
            row_from_pairs(row.id, pairs)
        })
        .collect())
}

async fn build_validation_rows(
    mm: &ModelManager,
) -> airlab_lib::model::Result<Vec<BasicShadowRow>> {
    let db_rows: Vec<ValidationShadowDbRow> = sqlx::query_as(
        r#"
        SELECT
            v.id AS id,
            v.group_id AS group_id,
            v.created_by AS created_by,
            v.clone_id AS clone_id,
            v.lot_id AS lot_id,
            v.conjugate_id AS conjugate_id,
            v.species_id AS species_id,
            v.application AS application,
            v.tissue AS tissue,
            v.status AS status,
            v.antigen_retrieval_type AS antigen_retrieval_type,
            v.is_archived AS is_archived,
            c.name AS clone_name,
            p.name AS protein_name,
            l.number AS lot_number,
            cg.tube_number AS tube_number,
            s.name AS species_name,
            u.name AS user_name,
            g.name AS group_name
        FROM public.validation v
        LEFT JOIN public.clone c ON c.id = v.clone_id
        LEFT JOIN public.protein p ON p.id = c.protein_id
        LEFT JOIN public.lot l ON l.id = v.lot_id
        LEFT JOIN public.conjugate cg ON cg.id = v.conjugate_id
        LEFT JOIN public.species s ON s.id = v.species_id
        LEFT JOIN public.member m ON m.id = v.created_by
        LEFT JOIN public."user" u ON u.id = m.user_id
        LEFT JOIN public."group" g ON g.id = v.group_id
        "#,
    )
    .fetch_all(mm.db())
    .await
    .map_err(airlab_lib::model::Error::from)?;

    Ok(db_rows
        .into_iter()
        .map(|row| {
            row_from_pairs(
                row.id,
                vec![
                    ("id", BasicShadowValue::Int(row.id)),
                    ("group_id", BasicShadowValue::Int(row.group_id)),
                    ("created_by", BasicShadowValue::Int(row.created_by)),
                    ("clone_id", BasicShadowValue::Int(row.clone_id)),
                    (
                        "lot_id",
                        BasicShadowValue::Int(row.lot_id.unwrap_or_default()),
                    ),
                    (
                        "conjugate_id",
                        BasicShadowValue::Int(row.conjugate_id.unwrap_or_default()),
                    ),
                    (
                        "species_id",
                        BasicShadowValue::Int(row.species_id.unwrap_or_default()),
                    ),
                    ("application", BasicShadowValue::Int(row.application)),
                    (
                        "tissue",
                        BasicShadowValue::Text(row.tissue.unwrap_or_default()),
                    ),
                    ("status", BasicShadowValue::Int(row.status)),
                    (
                        "antigen_retrieval_type",
                        BasicShadowValue::Text(row.antigen_retrieval_type.unwrap_or_default()),
                    ),
                    ("is_archived", BasicShadowValue::Bool(row.is_archived)),
                    (
                        "clone_name",
                        BasicShadowValue::Text(row.clone_name.unwrap_or_default()),
                    ),
                    (
                        "protein_name",
                        BasicShadowValue::Text(row.protein_name.unwrap_or_default()),
                    ),
                    (
                        "lot_number",
                        BasicShadowValue::Text(row.lot_number.unwrap_or_default()),
                    ),
                    (
                        "tube_number",
                        BasicShadowValue::Int(row.tube_number.unwrap_or_default()),
                    ),
                    (
                        "species_name",
                        BasicShadowValue::Text(row.species_name.unwrap_or_default()),
                    ),
                    (
                        "user_name",
                        BasicShadowValue::Text(row.user_name.unwrap_or_default()),
                    ),
                    (
                        "group_name",
                        BasicShadowValue::Text(row.group_name.unwrap_or_default()),
                    ),
                ],
            )
        })
        .collect())
}

async fn build_storage_rows(mm: &ModelManager) -> airlab_lib::model::Result<Vec<BasicShadowRow>> {
    let db_rows: Vec<StorageShadowDbRow> = sqlx::query_as(
        r#"
        SELECT id, name, "type", location, temperature_c, active
        FROM public.storage
        "#,
    )
    .fetch_all(mm.db())
    .await
    .map_err(airlab_lib::model::Error::from)?;

    Ok(db_rows
        .into_iter()
        .map(|row| {
            row_from_pairs(
                row.id,
                vec![
                    ("id", BasicShadowValue::Int(row.id)),
                    ("name", BasicShadowValue::Text(row.name.unwrap_or_default())),
                    (
                        "type",
                        BasicShadowValue::Text(row.r#type.unwrap_or_default()),
                    ),
                    (
                        "location",
                        BasicShadowValue::Text(row.location.unwrap_or_default()),
                    ),
                    ("temperature_c", BasicShadowValue::Int(row.temperature_c)),
                    ("active", BasicShadowValue::Bool(row.active)),
                ],
            )
        })
        .collect())
}

async fn build_collection_rows(
    mm: &ModelManager,
) -> airlab_lib::model::Result<Vec<BasicShadowRow>> {
    let db_rows: Vec<CollectionShadowDbRow> = sqlx::query_as(
        r#"
        SELECT id, name, description
        FROM public.collection
        "#,
    )
    .fetch_all(mm.db())
    .await
    .map_err(airlab_lib::model::Error::from)?;

    Ok(db_rows
        .into_iter()
        .map(|row| {
            row_from_pairs(
                row.id,
                vec![
                    ("id", BasicShadowValue::Int(row.id)),
                    ("name", BasicShadowValue::Text(row.name.unwrap_or_default())),
                    (
                        "description",
                        BasicShadowValue::Text(row.description.unwrap_or_default()),
                    ),
                ],
            )
        })
        .collect())
}
