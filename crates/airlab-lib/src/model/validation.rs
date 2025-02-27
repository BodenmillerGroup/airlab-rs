use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
//use chrono::prelude::*;
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

impl ValidationBmc {
    #[must_use]
    pub fn get_create_sql(drop_table: bool) -> String {
        let table = Self::TABLE;
        format!(
            r##"{}
create table if not exists "{table}" (
  id serial primary key,
  group_id integer NOT NULL,
  created_by integer NOT NULL,
  clone_id integer NOT NULL,
  lot_id integer,
  conjugate_id integer,
  species_id integer,
  application integer NOT NULL,
  positive_control character varying,
  negative_control character varying,
  incubation_conditions character varying,
  concentration character varying,
  concentration_unit character varying,
  tissue character varying,
  fixation integer,
  fixation_notes character varying,
  notes character varying,
  status integer DEFAULT 3 NOT NULL,
  antigen_retrieval_type character varying,
  antigen_retrieval_time character varying,
  antigen_retrieval_temperature character varying,
  saponin boolean,
  saponin_concentration character varying,
  methanol_treatment boolean,
  methanol_treatment_concentration character varying,
  surface_staining boolean,
  surface_staining_concentration character varying,
  meta jsonb,
  file_id integer,
  is_archived boolean DEFAULT false NOT NULL,
  created_at timestamp with time zone DEFAULT now() NOT NULL,
  updated_at timestamp with time zone DEFAULT now() NOT NULL
);
CREATE INDEX "IDX_validation_application" ON validation USING btree (application);
CREATE INDEX "IDX_validation_clone_id" ON validation USING btree (clone_id);
CREATE INDEX "IDX_validation_conjugate_id" ON validation USING btree (conjugate_id);
CREATE INDEX "IDX_validation_created_by" ON validation USING btree (created_by);
CREATE INDEX "IDX_validation_group_id" ON validation USING btree (group_id);
CREATE INDEX "IDX_validation_lot_id" ON validation USING btree (lot_id);
CREATE INDEX "IDX_validation_species_id" ON validation USING btree (species_id);
CREATE INDEX "IDX_validation_status" ON validation USING btree (status);
        "##,
            if drop_table {
                format!("drop table if exists {table};")
            } else {
                String::new()
            }
        )
    }
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default)]
pub struct MinValidation {
    pub id: i32,
    #[serde(rename = "cloneId")]
    pub clone_id: i32,
    pub application: i32,
    pub status: i32,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default)]
pub struct Validation {
    pub id: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,

    #[serde(rename = "createdBy")]
    pub created_by: i32,
    #[serde(rename = "cloneId")]
    pub clone_id: i32,
    #[serde(rename = "lotId")]
    pub lot_id: Option<i32>,
    #[serde(rename = "conjugateId")]
    pub conjugate_id: Option<i32>,
    #[serde(rename = "speciesId")]
    pub species_id: Option<i32>,
    pub application: i32,
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
    pub fixation: Option<i32>,
    #[serde(rename = "fixationNotes")]
    pub fixation_notes: Option<String>,
    pub notes: Option<String>,
    pub status: i32,
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
    pub file_id: Option<i32>,
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
    pub group_id: i32,
    #[serde(rename = "createdBy")]
    pub created_by: Option<i32>,
    #[serde(rename = "cloneId")]
    pub clone_id: i32,
    #[serde(rename = "lotId")]
    pub lot_id: Option<i32>,
    #[serde(rename = "conjugateId")]
    pub conjugate_id: Option<i32>,
    #[serde(rename = "speciesId")]
    pub species_id: Option<i32>,
    pub application: Option<i32>,
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
    pub fixation: Option<i32>,
    #[serde(rename = "fixationNotes")]
    pub fixation_notes: Option<String>,
    pub notes: Option<String>,
    pub status: Option<i32>,
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
    //pub meta: Option<String>,
    #[serde(rename = "fileId")]
    pub file_id: Option<i32>,
    #[serde(rename = "isArchived")]
    pub is_archived: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct ValidationForUpdate {
    pub application: i32,
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
    pub fixation: Option<i32>,
    #[serde(rename = "fixationNotes")]
    pub fixation_notes: Option<String>,
    pub notes: Option<String>,
    pub status: i32,
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
    //pub meta: Option<String>,
    #[serde(rename = "fileId")]
    pub file_id: Option<i32>,
    #[serde(rename = "isArchived")]
    pub is_archived: Option<bool>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ValidationFilter {
    id: Option<OpValsInt64>,
    group_id: Option<OpValsInt64>,

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
    ) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, validation_c).await
    }
    pub async fn create_full(
        ctx: &Ctx,
        mm: &ModelManager,
        validation_c: Validation,
    ) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, validation_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Validation> {
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
        id: i32,
        validation_u: ValidationForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, validation_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    use crate::model::Error;
    use anyhow::Result;
    use serde_json::json;

    #[ignore]
    #[tokio::test]
    async fn test_validation_create_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
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
        assert_eq!(validation.id, 3);

        ValidationBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_validation_get_err_not_found() -> Result<()> {
        let mm = ModelManager::new().await?;
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

    #[ignore]
    #[tokio::test]
    async fn test_validation_list_all_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let tname = "test_validation_list_all_ok";
        let seeds = _dev_utils::get_validation_seed(tname);
        _dev_utils::seed_validations(&ctx, &mm, &seeds).await?;

        let validations = ValidationBmc::list(&ctx, &mm, None, None).await?;

        let validations: Vec<Validation> = validations.into_iter().filter(|t| t.id == 1).collect();
        assert_eq!(validations.len(), 4, "number of seeded validations.");

        for validation in validations.iter() {
            ValidationBmc::delete(&ctx, &mm, validation.id).await?;
        }

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_validation_list_by_filter_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let tname = "test_validation_list_by_filter_ok";
        let seeds = _dev_utils::get_validation_seed(tname);
        _dev_utils::seed_validations(&ctx, &mm, &seeds).await?;

        let filters: Vec<ValidationFilter> = serde_json::from_value(json!([
            {
                "name": {
                    "$endsWith": ".a",
                    "$containsAny": ["01", "02"]
                }
            },
            {
                "name": {"$contains": "03"}
            }
        ]))?;
        let list_options = serde_json::from_value(json!({
            "order_bys": "!id"
        }))?;
        let validations = ValidationBmc::list(&ctx, &mm, Some(filters), Some(list_options)).await?;

        assert_eq!(validations.len(), 3);
        assert!(validations[0].id == 1);

        let validations = ValidationBmc::list(
            &ctx,
            &mm,
            Some(serde_json::from_value(json!([{
                "name": {"$startsWith": "test_list_by_filter_ok"}
            }]))?),
            None,
        )
        .await?;
        assert_eq!(validations.len(), 5);
        for validation in validations.iter() {
            ValidationBmc::delete(&ctx, &mm, validation.id).await?;
        }
        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_validation_update_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
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
                ..Default::default()
            },
        )
        .await?;

        let validation = ValidationBmc::get(&ctx, &mm, fx_validation.id).await?;
        assert_eq!(validation.id, 1);

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_validation_delete_err_not_found() -> Result<()> {
        let mm = ModelManager::new().await?;
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
