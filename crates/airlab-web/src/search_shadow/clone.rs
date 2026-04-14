use airlab_lib::model::ModelManager;
use serde_json::Value;
use sqlx::FromRow;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tracing::warn;

#[derive(Debug)]
pub struct CloneGroupShadow {
    pub rows: Arc<Vec<CloneTableShadowRow>>,
}

#[derive(Debug, Clone)]
pub struct CloneTableShadowRow {
    pub group_id: i64,
    pub clone_id: i64,
    pub clone_name: String,
    pub protein_name: String,
    pub species_name: String,
    pub is_phospho: bool,
    pub is_polyclonal: bool,
    pub isotype: String,
    pub epitope: String,
    pub application_label: Option<String>,
    pub reactivity_id: Option<i64>,
    pub reactivity_label: Option<String>,
    pub validation_id: Option<i64>,
    pub validation_application: Option<i64>,
    pub validation_application_label: Option<String>,
    pub validation_status: Option<i64>,
    pub validation_status_label: Option<String>,
    pub fulltext: String,
}

#[derive(Debug, Clone)]
pub enum CloneShadowFilter {
    CloneIdEq(i64),
    GroupIdEq(i64),
    CloneNameContains(String),
    ProteinNameContains(String),
    SpeciesNameContains(String),
    ValidationApplicationEq(i64),
    ValidationStatusEq(i64),
    ReactivityContains(Vec<i64>),
}

#[derive(Debug, Clone, Copy)]
pub enum CloneShadowOrderField {
    Id,
    CloneName,
    ProteinName,
    SpeciesName,
    IsPhospho,
    Application,
    ValidationApplication,
    ValidationStatus,
}

#[derive(Debug, Clone, Copy)]
pub enum CloneShadowDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub struct CloneShadowOrder {
    pub field: CloneShadowOrderField,
    pub direction: CloneShadowDirection,
}

#[derive(Debug, Clone)]
pub struct CloneShadowQuery {
    pub group_id: i64,
    pub filters: Vec<CloneShadowFilter>,
    pub global_filter: Option<String>,
    pub order: Option<CloneShadowOrder>,
    pub page: i64,
    pub limit: i64,
}

#[derive(Debug, Clone)]
pub struct CloneShadowSearchResult {
    pub items: Vec<i64>,
    pub search_total: i64,
}

#[derive(Debug, FromRow)]
struct CloneShadowDbRow {
    group_id: i64,
    clone_id: i64,
    clone_name: String,
    protein_name: Option<String>,
    species_name: Option<String>,
    is_phospho: bool,
    is_polyclonal: bool,
    isotype: Option<String>,
    epitope: Option<String>,
    reactivity: Option<Vec<i64>>,
    application: Option<Value>,
    validation_id: Option<i64>,
    validation_application: Option<i64>,
    validation_status: Option<i64>,
}

#[derive(Debug, FromRow)]
struct SpeciesNameRow {
    id: i64,
    name: String,
}

pub async fn build_clone_group_shadow(
    mm: &ModelManager,
    group_id: i64,
) -> airlab_lib::model::Result<CloneGroupShadow> {
    let db_rows: Vec<CloneShadowDbRow> = sqlx::query_as(
        r#"
        SELECT
            c.group_id AS group_id,
            c.id AS clone_id,
            c.name AS clone_name,
            p.name AS protein_name,
            s.name AS species_name,
            c.is_phospho AS is_phospho,
            c.is_polyclonal AS is_polyclonal,
            c.isotype AS isotype,
            c.epitope AS epitope,
            c.reactivity AS reactivity,
            c.application AS application,
            v.id AS validation_id,
            v.application AS validation_application,
            v.status AS validation_status
        FROM public.clone c
        LEFT JOIN public.protein p ON p.id = c.protein_id
        LEFT JOIN public.species s ON s.id = c.species_id
        LEFT JOIN public.validation v ON v.clone_id = c.id
        WHERE c.group_id = $1
        "#,
    )
    .bind(group_id)
    .fetch_all(mm.db())
    .await
    .map_err(airlab_lib::model::Error::from)?;

    let reactivity_ids: Vec<i64> = db_rows
        .iter()
        .flat_map(|row| row.reactivity.clone().unwrap_or_default())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let species_lookup = if reactivity_ids.is_empty() {
        HashMap::new()
    } else {
        let rows: Vec<SpeciesNameRow> = sqlx::query_as(
            r#"
            SELECT id, name
            FROM public.species
            WHERE id = ANY($1)
            "#,
        )
        .bind(&reactivity_ids)
        .fetch_all(mm.db())
        .await
        .map_err(airlab_lib::model::Error::from)?;

        rows.into_iter().map(|row| (row.id, row.name)).collect()
    };

    let mut rows = Vec::new();
    for row in db_rows {
        let application_entries = {
            let parsed = parse_clone_application_entries(row.application.as_ref());
            if parsed.is_empty() {
                vec![(None, None)]
            } else {
                parsed
            }
        };

        let reactivity_entries = {
            let parsed = row
                .reactivity
                .clone()
                .unwrap_or_default()
                .into_iter()
                .map(|id| (Some(id), species_lookup.get(&id).cloned()))
                .collect::<Vec<_>>();
            if parsed.is_empty() {
                vec![(None, None)]
            } else {
                parsed
            }
        };

        for (_application_id, application_label) in &application_entries {
            for (reactivity_id, reactivity_label) in &reactivity_entries {
                let mut shadow = CloneTableShadowRow {
                    group_id: row.group_id,
                    clone_id: row.clone_id,
                    clone_name: row.clone_name.clone(),
                    protein_name: row.protein_name.clone().unwrap_or_default(),
                    species_name: row.species_name.clone().unwrap_or_default(),
                    is_phospho: row.is_phospho,
                    is_polyclonal: row.is_polyclonal,
                    isotype: row.isotype.clone().unwrap_or_default(),
                    epitope: row.epitope.clone().unwrap_or_default(),
                    application_label: application_label.clone(),
                    reactivity_id: *reactivity_id,
                    reactivity_label: reactivity_label.clone(),
                    validation_id: row.validation_id,
                    validation_application: row.validation_application,
                    validation_application_label: row
                        .validation_application
                        .map(validation_application_to_string),
                    validation_status: row.validation_status,
                    validation_status_label: row.validation_status.map(validation_status_to_string),
                    fulltext: String::new(),
                };
                shadow.fulltext = build_clone_fulltext(&shadow);
                rows.push(shadow);
            }
        }
    }

    Ok(CloneGroupShadow {
        rows: Arc::new(rows),
    })
}

pub fn search_clone_shadow(
    shadow: &CloneGroupShadow,
    query: &CloneShadowQuery,
) -> CloneShadowSearchResult {
    let requires_validation_row = query_requires_validation_row(query);

    warn!(
        group_id = query.group_id,
        row_count = shadow.rows.len(),
        filter_count = query.filters.len(),
        requires_validation_row,
        has_global_filter = query
            .global_filter
            .as_ref()
            .is_some_and(|value| !value.trim().is_empty()),
        page = query.page,
        limit = query.limit,
        "searching clone shadow table"
    );

    let mut matches = shadow
        .rows
        .iter()
        .filter(|row| row.group_id == query.group_id)
        .filter(|row| !requires_validation_row || row.validation_id.is_some())
        .filter(|row| {
            query
                .filters
                .iter()
                .all(|filter| row_matches_filter(row, filter))
        })
        .filter(|row| row_matches_global(row, &query.global_filter))
        .collect::<Vec<_>>();

    warn!(
        group_id = query.group_id,
        matched_row_count = matches.len(),
        "clone shadow filter pass complete"
    );

    if let Some(order) = &query.order {
        warn!(
            group_id = query.group_id,
            field = ?order.field,
            direction = ?order.direction,
            "sorting clone shadow search results"
        );
        matches.sort_by(|left, right| compare_rows(left, right, order));
    } else {
        warn!(
            group_id = query.group_id,
            "clone shadow search using insertion order"
        );
    }

    let mut unique_ids = Vec::new();
    let mut seen = HashSet::new();
    for row in matches {
        if seen.insert(row.clone_id) {
            unique_ids.push(row.clone_id);
        }
    }

    let total = unique_ids.len() as i64;
    let limit = query.limit.max(1) as usize;
    let page = query.page.max(1) as usize;
    let start = (page.saturating_sub(1)) * limit;
    let end = (start + limit).min(unique_ids.len());

    warn!(
        group_id = query.group_id,
        unique_clone_count = unique_ids.len(),
        page = page,
        limit = limit,
        page_start = start,
        page_end = end,
        "clone shadow search dedupe and pagination complete"
    );

    CloneShadowSearchResult {
        items: unique_ids.get(start..end).unwrap_or(&[]).to_vec(),
        search_total: total,
    }
}

fn row_matches_filter(row: &CloneTableShadowRow, filter: &CloneShadowFilter) -> bool {
    match filter {
        CloneShadowFilter::CloneIdEq(value) => row.clone_id == *value,
        CloneShadowFilter::GroupIdEq(value) => row.group_id == *value,
        CloneShadowFilter::CloneNameContains(value) => contains_ci(&row.clone_name, value),
        CloneShadowFilter::ProteinNameContains(value) => contains_ci(&row.protein_name, value),
        CloneShadowFilter::SpeciesNameContains(value) => contains_ci(&row.species_name, value),
        CloneShadowFilter::ValidationApplicationEq(value) => {
            row.validation_application == Some(*value)
        }
        CloneShadowFilter::ValidationStatusEq(value) => row.validation_status == Some(*value),
        CloneShadowFilter::ReactivityContains(values) => {
            values.iter().any(|value| row.reactivity_id == Some(*value))
        }
    }
}

fn row_matches_global(row: &CloneTableShadowRow, global: &Option<String>) -> bool {
    match global {
        Some(value) if !value.trim().is_empty() => {
            row.fulltext.contains(&value.trim().to_lowercase())
        }
        _ => true,
    }
}

fn query_requires_validation_row(query: &CloneShadowQuery) -> bool {
    query.filters.iter().any(|filter| {
        matches!(
            filter,
            CloneShadowFilter::ValidationApplicationEq(_)
                | CloneShadowFilter::ValidationStatusEq(_)
        )
    })
}

fn compare_rows(
    left: &CloneTableShadowRow,
    right: &CloneTableShadowRow,
    order: &CloneShadowOrder,
) -> Ordering {
    let ord = match order.field {
        CloneShadowOrderField::Id => left.clone_id.cmp(&right.clone_id),
        CloneShadowOrderField::CloneName => cmp_text(&left.clone_name, &right.clone_name),
        CloneShadowOrderField::ProteinName => cmp_text(&left.protein_name, &right.protein_name),
        CloneShadowOrderField::SpeciesName => cmp_text(&left.species_name, &right.species_name),
        CloneShadowOrderField::IsPhospho => left.is_phospho.cmp(&right.is_phospho),
        CloneShadowOrderField::Application => cmp_optional_text(
            left.application_label.as_deref(),
            right.application_label.as_deref(),
        ),
        CloneShadowOrderField::ValidationApplication => left
            .validation_application
            .cmp(&right.validation_application),
        CloneShadowOrderField::ValidationStatus => {
            left.validation_status.cmp(&right.validation_status)
        }
    };

    let ord = if ord == Ordering::Equal {
        left.clone_id.cmp(&right.clone_id)
    } else {
        ord
    };

    match order.direction {
        CloneShadowDirection::Asc => ord,
        CloneShadowDirection::Desc => ord.reverse(),
    }
}

fn build_clone_fulltext(row: &CloneTableShadowRow) -> String {
    let mut parts = vec![
        row.clone_name.clone(),
        row.protein_name.clone(),
        row.species_name.clone(),
        row.isotype.clone(),
        row.epitope.clone(),
    ];

    if row.is_phospho {
        parts.push("phospho".to_string());
    }
    if row.is_polyclonal {
        parts.push("polyclonal".to_string());
    }
    if let Some(label) = &row.application_label {
        parts.push(label.clone());
    }
    if let Some(label) = &row.validation_application_label {
        parts.push(label.clone());
    }
    if let Some(label) = &row.validation_status_label {
        parts.push(label.clone());
    }
    if let Some(label) = &row.reactivity_label {
        parts.push(label.clone());
    }

    parts
        .into_iter()
        .map(|part| part.trim().to_lowercase())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn contains_ci(haystack: &str, needle: &str) -> bool {
    haystack
        .to_lowercase()
        .contains(&needle.trim().to_lowercase())
}

fn cmp_text(left: &str, right: &str) -> Ordering {
    left.to_lowercase().cmp(&right.to_lowercase())
}

fn cmp_optional_text(left: Option<&str>, right: Option<&str>) -> Ordering {
    match (left, right) {
        (Some(left), Some(right)) => cmp_text(left, right),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
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

fn parse_clone_application_entries(value: Option<&Value>) -> Vec<(Option<i64>, Option<String>)> {
    let Some(Value::Object(map)) = value else {
        return Vec::new();
    };

    let mut entries = map
        .iter()
        .filter_map(|(key, raw_value)| match raw_value {
            Value::Bool(true) => key.parse::<i64>().ok(),
            Value::String(v) if v.eq_ignore_ascii_case("true") => key.parse::<i64>().ok(),
            _ => None,
        })
        .map(|id| (Some(id), Some(validation_application_to_string(id))))
        .collect::<Vec<_>>();

    entries.sort_by_key(|(id, _)| *id);
    entries
}

pub fn map_filter(field_table: &str, field_name: &str, value: &Value) -> Option<CloneShadowFilter> {
    match (field_table, field_name, value) {
        ("clone", "id", Value::Number(v)) => v.as_i64().map(CloneShadowFilter::CloneIdEq),
        ("clone", "group_id", Value::Number(v)) => v.as_i64().map(CloneShadowFilter::GroupIdEq),
        ("clone", "name", Value::String(v)) => {
            Some(CloneShadowFilter::CloneNameContains(v.clone()))
        }
        ("protein", "name", Value::String(v)) => {
            Some(CloneShadowFilter::ProteinNameContains(v.clone()))
        }
        ("species", "name", Value::String(v)) => {
            Some(CloneShadowFilter::SpeciesNameContains(v.clone()))
        }
        ("validation", "application", Value::Number(v)) => {
            v.as_i64().map(CloneShadowFilter::ValidationApplicationEq)
        }
        ("validation", "status", Value::Number(v)) => {
            v.as_i64().map(CloneShadowFilter::ValidationStatusEq)
        }
        ("clone", "reactivity", Value::Number(v)) => v
            .as_i64()
            .map(|id| CloneShadowFilter::ReactivityContains(vec![id])),
        ("clone", "reactivity", Value::Array(values)) => {
            let ids = values.iter().filter_map(Value::as_i64).collect::<Vec<_>>();
            if ids.is_empty() {
                None
            } else {
                Some(CloneShadowFilter::ReactivityContains(ids))
            }
        }
        _ => None,
    }
}
