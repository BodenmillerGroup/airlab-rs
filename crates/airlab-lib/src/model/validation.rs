use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
use crate::model::helpers::{i64_or, opt_bool, opt_datetime, opt_i64, opt_string};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default)]
pub struct MinValidation {
    pub id: i64,
    #[serde(rename = "cloneId")]
    pub clone_id: i64,
    pub application: i64,
    pub status: i64,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default)]
pub struct Validation {
    pub id: i64,
    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "createdBy")]
    pub created_by: i64,
    #[serde(rename = "cloneId")]
    pub clone_id: i64,
    #[serde(rename = "lotId")]
    pub lot_id: Option<i64>,
    #[serde(rename = "conjugateId")]
    pub conjugate_id: Option<i64>,
    #[serde(rename = "speciesId")]
    pub species_id: Option<i64>,
    pub application: i64,
    #[serde(rename = "positiveControl")]
    pub positive_control: Option<String>,
    #[serde(rename = "negativeControl")]
    pub negative_control: Option<String>,
    #[serde(rename = "incubationConditions")]
    pub incubation_conditions: Option<String>,
    pub concentration: Option<String>,
    #[serde(rename = "concentrationUnit")]
    pub concentration_unit: Option<String>,
    pub tissue: Option<String>,
    pub fixation: Option<i64>,
    #[serde(rename = "fixationNotes")]
    pub fixation_notes: Option<String>,
    pub notes: Option<String>,
    pub status: i64,
    #[serde(rename = "antigenRetrievalType")]
    pub antigen_retrieval_type: Option<String>,
    #[serde(rename = "antigenRetrievalTime")]
    pub antigen_retrieval_time: Option<String>,
    #[serde(rename = "antigenRetrievalTemperature")]
    pub antigen_retrieval_temperature: Option<String>,
    pub saponin: Option<bool>,
    #[serde(rename = "saponinConcentration")]
    pub saponin_concentration: Option<String>,
    #[serde(rename = "saponinTreatment")]
    pub methanol_treatment: Option<bool>,
    #[serde(rename = "methanolTreatmentConcentration")]
    pub methanol_treatment_concentration: Option<String>,
    #[serde(rename = "methanolStaining")]
    pub surface_staining: Option<bool>,
    #[serde(rename = "surfaceStainingConcentration")]
    pub surface_staining_concentration: Option<String>,
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "fileId")]
    pub file_id: Option<i64>,
    #[serde(rename = "isArchived")]
    pub is_archived: bool,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct ValidationForCreate {
    #[serde(rename = "groupId")]
    pub group_id: i64,
    #[serde(rename = "createdBy")]
    pub created_by: Option<i64>,
    #[serde(rename = "cloneId")]
    pub clone_id: i64,
    #[serde(rename = "lotId")]
    pub lot_id: Option<i64>,
    #[serde(rename = "conjugateId")]
    pub conjugate_id: Option<i64>,
    #[serde(rename = "speciesId")]
    pub species_id: Option<i64>,
    pub application: Option<i64>,
    #[serde(rename = "positiveControl")]
    pub positive_control: Option<String>,
    #[serde(rename = "negativeControl")]
    pub negative_control: Option<String>,
    #[serde(rename = "incubationConditions")]
    pub incubation_conditions: Option<String>,
    pub concentration: Option<String>,
    #[serde(rename = "concentrationUnit")]
    pub concentration_unit: Option<String>,
    pub tissue: Option<String>,
    pub fixation: Option<i64>,
    #[serde(rename = "fixationNotes")]
    pub fixation_notes: Option<String>,
    pub notes: Option<String>,
    pub status: Option<i64>,
    #[serde(rename = "antigenRetrievalType")]
    pub antigen_retrieval_type: Option<String>,
    #[serde(rename = "antigenRetrievalTime")]
    pub antigen_retrieval_time: Option<String>,
    #[serde(rename = "antigenRetrievalTemperature")]
    pub antigen_retrieval_temperature: Option<String>,
    pub saponin: Option<bool>,
    #[serde(rename = "saponinConcentration")]
    pub saponin_concentration: Option<String>,
    #[serde(rename = "saponinTreatment")]
    pub methanol_treatment: Option<bool>,
    #[serde(rename = "methanolTreatmentConcentration")]
    pub methanol_treatment_concentration: Option<String>,
    #[serde(rename = "methanolStaining")]
    pub surface_staining: Option<bool>,
    #[serde(rename = "surfaceStainingConcentration")]
    pub surface_staining_concentration: Option<String>,
    #[serde(rename = "fileId")]
    pub file_id: Option<i64>,
    #[serde(rename = "isArchived")]
    pub is_archived: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<Value> for ValidationForCreate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        ValidationForCreate {
            group_id: i64_or(&obj, "groupId", 0),
            created_by: opt_i64(&obj, "createdBy"),
            clone_id: i64_or(&obj, "cloneId", 0),
            lot_id: opt_i64(&obj, "lotId"),
            conjugate_id: opt_i64(&obj, "conjugateId"),
            species_id: opt_i64(&obj, "speciesId"),
            application: opt_i64(&obj, "application"),
            positive_control: opt_string(&obj, "positiveControl"),
            negative_control: opt_string(&obj, "negativeControl"),
            incubation_conditions: opt_string(&obj, "incubationConditions"),
            concentration: opt_string(&obj, "concentration"),
            concentration_unit: opt_string(&obj, "concentrationUnit"),
            tissue: opt_string(&obj, "tissue"),
            fixation: opt_i64(&obj, "fixation"),
            fixation_notes: opt_string(&obj, "fixationNotes"),
            notes: opt_string(&obj, "notes"),
            status: opt_i64(&obj, "status"),
            antigen_retrieval_type: opt_string(&obj, "antigenRetrievalType"),
            antigen_retrieval_time: opt_string(&obj, "antigenRetrievalTime"),
            antigen_retrieval_temperature: opt_string(&obj, "antigenRetrievalTemperature"),
            saponin: opt_bool(&obj, "saponin"),
            saponin_concentration: opt_string(&obj, "saponinConcentration"),
            methanol_treatment: opt_bool(&obj, "saponinTreatment"),
            methanol_treatment_concentration: opt_string(&obj, "methanolTreatmentConcentration"),
            surface_staining: opt_bool(&obj, "methanolStaining"),
            surface_staining_concentration: opt_string(&obj, "surfaceStainingConcentration"),
            file_id: opt_i64(&obj, "fileId"),
            is_archived: opt_bool(&obj, "isArchived"),
            created_at: opt_datetime(&obj, "createdAt"),
            updated_at: opt_datetime(&obj, "updatedAt"),
        }
    }
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct ValidationForUpdate {
    pub application: i64,
    #[serde(rename = "positiveControl")]
    pub positive_control: Option<String>,
    #[serde(rename = "negativeControl")]
    pub negative_control: Option<String>,
    #[serde(rename = "incubationConditions")]
    pub incubation_conditions: Option<String>,
    pub concentration: Option<String>,
    #[serde(rename = "concentrationUnit")]
    pub concentration_unit: Option<String>,
    pub tissue: Option<String>,
    pub fixation: Option<i64>,
    #[serde(rename = "fixationNotes")]
    pub fixation_notes: Option<String>,
    pub notes: Option<String>,
    pub status: i64,
    #[serde(rename = "antigenRetrievalType")]
    pub antigen_retrieval_type: Option<String>,
    #[serde(rename = "antigenRetrievalTime")]
    pub antigen_retrieval_time: Option<String>,
    #[serde(rename = "antigenRetrievalTemperature")]
    pub antigen_retrieval_temperature: Option<String>,
    pub saponin: Option<bool>,
    #[serde(rename = "saponinConcentration")]
    pub saponin_concentration: Option<String>,
    #[serde(rename = "saponinTreatment")]
    pub methanol_treatment: Option<bool>,
    #[serde(rename = "methanolTreatmentConcentration")]
    pub methanol_treatment_concentration: Option<String>,
    #[serde(rename = "methanolStaining")]
    pub surface_staining: Option<bool>,
    #[serde(rename = "surfaceStainingConcentration")]
    pub surface_staining_concentration: Option<String>,
    #[serde(rename = "fileId")]
    pub file_id: Option<i64>,
    #[serde(rename = "isArchived")]
    pub is_archived: Option<bool>,
}

impl From<Value> for ValidationForUpdate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        ValidationForUpdate {
            application: i64_or(&obj, "application", 0),
            positive_control: opt_string(&obj, "positiveControl"),
            negative_control: opt_string(&obj, "negativeControl"),
            incubation_conditions: opt_string(&obj, "incubationConditions"),
            concentration: opt_string(&obj, "concentration"),
            concentration_unit: opt_string(&obj, "concentrationUnit"),
            tissue: opt_string(&obj, "tissue"),
            fixation: opt_i64(&obj, "fixation"),
            fixation_notes: opt_string(&obj, "fixationNotes"),
            notes: opt_string(&obj, "notes"),
            status: i64_or(&obj, "status", 0),
            antigen_retrieval_type: opt_string(&obj, "antigenRetrievalType"),
            antigen_retrieval_time: opt_string(&obj, "antigenRetrievalTime"),
            antigen_retrieval_temperature: opt_string(&obj, "antigenRetrievalTemperature"),
            saponin: opt_bool(&obj, "saponin"),
            saponin_concentration: opt_string(&obj, "saponinConcentration"),
            methanol_treatment: opt_bool(&obj, "saponinTreatment"),
            methanol_treatment_concentration: opt_string(&obj, "methanolTreatmentConcentration"),
            surface_staining: opt_bool(&obj, "methanolStaining"),
            surface_staining_concentration: opt_string(&obj, "surfaceStainingConcentration"),
            file_id: opt_i64(&obj, "fileId"),
            is_archived: opt_bool(&obj, "isArchived"),
        }
    }
}

#[derive(FilterNodes, Deserialize, Default, Debug, Clone)]
pub struct ValidationFilter {
    id: Option<OpValsInt64>,
    group_id: Option<OpValsInt64>,
    clone_id: Option<OpValsInt64>,
    lot_id: Option<OpValsInt64>,
    conjugate_id: Option<OpValsInt64>,
    species_id: Option<OpValsInt64>,
    application: Option<OpValsInt64>,
    status: Option<OpValsInt64>,

    tissue: Option<OpValsString>,
}

pub struct ValidationBmc;

impl DbBmc for ValidationBmc {
    const TABLE: &'static str = "validation";
}

impl ValidationBmc {
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        validation_c: ValidationForCreate,
    ) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, validation_c).await
    }
    pub async fn create_full(
        ctx: &Ctx,
        mm: &ModelManager,
        validation_c: Validation,
    ) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, validation_c).await
    }

    pub async fn count(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<ValidationFilter>>,
    ) -> Result<i64> {
        base::count::<Self, _>(ctx, mm, filters).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Validation> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<ValidationFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Validation>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn minlist(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<ValidationFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<MinValidation>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        validation_u: ValidationForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, validation_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    use crate::model::Error;
    use serde_json::json;

    type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    #[tokio::test]
    async fn test_validation_create_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let validation_c = ValidationForCreate {
            group_id: 1,
            created_by: Some(261),
            clone_id: 3124,
            lot_id: Some(5495),
            conjugate_id: Some(4291),
            species_id: Some(44),
            application: Some(1),
            positive_control: None,
            negative_control: None,
            incubation_conditions: None,
            concentration: None,
            concentration_unit: None,
            tissue: None,
            fixation: None,
            fixation_notes: None,
            notes: None,
            antigen_retrieval_type: None,
            antigen_retrieval_time: None,
            antigen_retrieval_temperature: None,
            status: Some(1),
            saponin: Some(false),
            saponin_concentration: None,
            methanol_treatment: Some(false),
            methanol_treatment_concentration: None,
            surface_staining: Some(false),
            surface_staining_concentration: None,
            file_id: Some(2909),
            is_archived: Some(false),
            created_at: Some(chrono::offset::Utc::now()),
            updated_at: Some(chrono::offset::Utc::now()),
        };
        let id = ValidationBmc::create(&ctx, &mm, validation_c).await?;

        let validation = ValidationBmc::get(&ctx, &mm, id).await?;
        assert_eq!(validation.group_id, 1);
        assert_eq!(validation.clone_id, 3124);
        assert_eq!(validation.conjugate_id, Some(4291));

        ValidationBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_validation_get_err_not_found() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = ValidationBmc::get(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "validation",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_validation_list_all_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_validation_list_all_ok";
        let seeds = _dev_utils::get_validation_seed(tname);
        _dev_utils::seed_validations(&ctx, &mm, &seeds).await?;

        let validations = ValidationBmc::list(&ctx, &mm, None, None).await?;

        let validations: Vec<Validation> = validations
            .into_iter()
            .filter(|t| {
                t.tissue
                    .as_deref()
                    .is_some_and(|tissue| tissue.starts_with(tname))
            })
            .collect();
        assert_eq!(validations.len(), 4, "number of seeded validations.");

        Ok(())
    }

    #[tokio::test]
    async fn test_validation_list_filters_by_clone_id() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_validation_list_filters_by_clone_id";
        let seeds = _dev_utils::get_validation_seed(tname);
        _dev_utils::seed_validations(&ctx, &mm, &seeds).await?;

        let filters: Vec<ValidationFilter> = serde_json::from_value(json!([
            {
                "group_id": { "$eq": 1000 },
                "clone_id": { "$eq": 1006 }
            }
        ]))?;

        let validations = ValidationBmc::list(&ctx, &mm, Some(filters.clone()), None).await?;
        let total = ValidationBmc::count(&ctx, &mm, Some(filters)).await?;

        assert!(!validations.is_empty());
        assert!(
            validations
                .iter()
                .all(|validation| validation.clone_id == 1006)
        );
        assert_eq!(total, validations.len() as i64);

        Ok(())
    }

    #[tokio::test]
    async fn test_validation_list_by_filter_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_validation_list_by_filter_ok";
        let seeds = _dev_utils::get_validation_seed(tname);
        _dev_utils::seed_validations(&ctx, &mm, &seeds).await?;

        let filters: Vec<ValidationFilter> = serde_json::from_value(json!([
            {
                "tissue": {
                    "$endsWith": ".a",
                    "$containsAny": ["01", "02"]
                }
            },
            {
                "tissue": {"$contains": "03"}
            }
        ]))?;
        let list_options = serde_json::from_value(json!({
            "order_bys": "!id"
        }))?;
        let validations = ValidationBmc::list(&ctx, &mm, Some(filters), Some(list_options)).await?;

        assert_eq!(validations.len(), 3);
        assert_eq!(
            validations[0].tissue.as_deref(),
            Some("test_validation_list_by_filter_ok-03")
        );
        assert_eq!(
            validations[1].tissue.as_deref(),
            Some("test_validation_list_by_filter_ok-02.a")
        );
        assert_eq!(
            validations[2].tissue.as_deref(),
            Some("test_validation_list_by_filter_ok-01.a")
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_validation_update_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_validation_update_ok";
        let seeds = _dev_utils::get_validation_seed(tname);
        let fx_validation = _dev_utils::seed_validations(&ctx, &mm, &seeds)
            .await?
            .remove(0);

        ValidationBmc::update(
            &ctx,
            &mm,
            fx_validation.id,
            ValidationForUpdate {
                application: 2,
                tissue: Some(tname.to_string()),
                status: 2,
                ..Default::default()
            },
        )
        .await?;

        let validation = ValidationBmc::get(&ctx, &mm, fx_validation.id).await?;
        assert_eq!(validation.application, 2);
        assert_eq!(validation.status, 2);
        assert_eq!(validation.tissue.as_deref(), Some(tname));

        Ok(())
    }

    #[tokio::test]
    async fn test_validation_delete_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = ValidationBmc::delete(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "validation",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
