#![allow(clippy::module_inception)]
use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::clone::{CloneBmc, CloneFilter};
use crate::model::conjugate::{Conjugate, ConjugateBmc, ConjugateFilter};
use crate::model::lot::LotFilter;
use crate::model::member::{Member, MemberBmc, MemberFilter};
use crate::model::protein::ProteinBmc;
use crate::model::species::{Species, SpeciesBmc, SpeciesFilter};
use crate::model::user::{User, UserBmc};
use crate::model::validation::{Validation, ValidationBmc, ValidationFilter};
use crate::model::validation_file::{ValidationFileBmc, ValidationFileFilter};
use crate::model::view_clone::ViewClone;
use crate::model::view_clone::ViewCloneBmc;
use crate::model::view_lot::{ViewLot, ViewLotBmc};
use modql::filter::ListOptions;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MinUser {
    id: i32,
    name: String,
    #[serde(rename = "isAdmin")]
    is_admin: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MinSpecies {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MinValidationFile {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MinLot {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MinConjugate {
    id: i32,
    #[serde(rename = "tubeNumber")]
    tube_number: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MinProtein {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MinClone {
    id: i32,
    name: String,
    protein: MinProtein,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MinViewValidation {
    pub id: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,
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
    #[serde(rename = "antigenRetrievalTemperature")]
    pub antigen_retrieval_temperature: Option<String>,
    #[serde(rename = "antigenRetrievalTime")]
    pub antigen_retrieval_time: Option<String>,
    #[serde(rename = "antigenRetrievalType")]
    pub antigen_retrieval_type: Option<String>,
    pub saponin: Option<bool>,
    #[serde(rename = "saponinConcentration")]
    pub saponin_concentration: Option<String>,
    #[serde(rename = "methanolTreatment")]
    pub methanol_treatment: Option<bool>,
    #[serde(rename = "methanolTreatmentConcentration")]
    pub methanol_treatment_concentration: Option<String>,
    #[serde(rename = "surfaceStaining")]
    pub surface_staining: Option<bool>,
    #[serde(rename = "surfaceStainingConcentration")]
    pub surface_staining_concentration: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub clone: MinClone,
    pub lot: Option<MinLot>,
    pub user: MinUser,
    pub species: Option<MinSpecies>,
    pub conjugate: Option<MinConjugate>,
    #[serde(rename = "validationFiles")]
    pub validation_files: Vec<MinValidationFile>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ViewValidation {
    pub id: i32,
    pub tissue: Option<String>,
    pub notes: Option<String>,
    pub clone: ViewClone,
    #[serde(rename = "antigenRetrievalTemperature")]
    pub antigen_retrieval_temperature: Option<String>,
    #[serde(rename = "antigenRetrievalTime")]
    pub antigen_retrieval_time: Option<String>,
    #[serde(rename = "antigenRetrievalType")]
    pub antigen_retrieval_type: Option<String>,
    pub application: i32,
    #[serde(rename = "cloneId")]
    pub clone_id: i32,
    pub concentration: Option<String>,
    #[serde(rename = "concentrationUnit")]
    pub concentration_unit: Option<String>,
    #[serde(rename = "conjugateId")]
    pub conjugate_id: Option<i32>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "createdBy")]
    pub created_by: i32,
    pub fixation: Option<i32>,
    #[serde(rename = "fixationNotes")]
    pub fixation_notes: Option<String>,
    #[serde(rename = "incubationConditions")]
    pub incubation_conditions: Option<String>,
    #[serde(rename = "isArchived")]
    pub is_archived: bool,
    #[serde(rename = "lotId")]
    pub lot_id: Option<i32>,
    #[serde(rename = "fileId")]
    pub file_id: Option<i32>,
    #[serde(rename = "methanolTreatment")]
    pub methanol_treatment: Option<bool>,
    #[serde(rename = "methanolTreatmentConcentration")]
    pub methanol_treatment_concentration: Option<String>,
    #[serde(rename = "negativeControl")]
    pub negative_control: Option<String>,
    #[serde(rename = "positiveControl")]
    pub positive_control: Option<String>,
    pub saponin: Option<bool>,
    #[serde(rename = "saponinConcentration")]
    pub saponin_concentration: Option<String>,
    #[serde(rename = "speciesId")]
    pub species_id: Option<i32>,
    pub status: i32,
    #[serde(rename = "surfaceStaining")]
    pub surface_staining: Option<bool>,
    #[serde(rename = "surfaceStainingConcentration")]
    pub surface_staining_concentration: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub lot: Option<ViewLot>,
    pub conjugate: Option<Conjugate>,
    pub species: Option<Species>,
    pub user: Option<User>,
}

pub struct ViewValidationBmc;

impl ViewValidationBmc {
    pub async fn get_min(
        ctx: &Ctx,
        mm: &ModelManager,
        user_id: i32,
        id: i32,
    ) -> Result<MinViewValidation> {
        let item = ValidationBmc::get(ctx, mm, id).await?;
        let user: User = UserBmc::get(ctx, mm, user_id).await?;
        let clone = CloneBmc::get(ctx, mm, item.clone_id).await?;
        let protein = ProteinBmc::get(ctx, mm, clone.protein_id).await?;
        let species = match clone.species_id {
            Some(sid) => {
                let species = SpeciesBmc::get(ctx, mm, sid).await?;
                Some(MinSpecies {
                    id: species.id,
                    name: species.name,
                })
            }
            None => None,
        };
        let conjugate = match item.conjugate_id {
            Some(cid) => {
                let conj = ConjugateBmc::get(ctx, mm, cid).await?;
                Some(MinConjugate {
                    id: conj.id,
                    tube_number: conj.tube_number,
                })
            }
            None => None,
        };
        let lot = match item.lot_id {
            Some(lid) => {
                let lot = ViewLotBmc::get(ctx, mm, lid).await?;
                Some(MinLot {
                    id: lot.id,
                    name: lot.name,
                })
            }
            None => None,
        };
        let filters: Vec<ValidationFileFilter> =
            serde_json::from_value(json!([{"validation_id": {"$eq": item.id}}])).unwrap_or(vec![]);
        let op = ListOptions {
            limit: Some(10_000),
            ..Default::default()
        };
        let val_files = ValidationFileBmc::list(ctx, mm, Some(filters), Some(op)).await?;
        let validation_files = val_files
            .into_iter()
            .map(|e| MinValidationFile {
                id: e.id,
                name: e.name.unwrap_or_default(),
            })
            .collect();
        let ret = MinViewValidation {
            id: item.id,
            group_id: item.group_id,
            tissue: item.tissue,
            notes: item.notes,
            clone: MinClone {
                id: clone.id,
                name: clone.name.clone(),
                protein: MinProtein {
                    id: protein.id,
                    name: protein.name.clone(),
                },
            },
            antigen_retrieval_time: item.antigen_retrieval_time,
            antigen_retrieval_type: item.antigen_retrieval_type,
            antigen_retrieval_temperature: item.antigen_retrieval_temperature,
            application: item.application,
            concentration: item.concentration,
            concentration_unit: item.concentration_unit,
            created_at: item.created_at,
            fixation: item.fixation,
            fixation_notes: item.fixation_notes,
            incubation_conditions: item.incubation_conditions,
            methanol_treatment: item.methanol_treatment,
            methanol_treatment_concentration: item.methanol_treatment_concentration,
            negative_control: item.negative_control,
            positive_control: item.positive_control,
            saponin: item.saponin,
            saponin_concentration: item.saponin_concentration,
            status: item.status,
            surface_staining: item.surface_staining,
            surface_staining_concentration: item.surface_staining_concentration,
            user: MinUser {
                id: user.id,
                name: user.name.clone().unwrap_or_default(),
                is_admin: user.is_admin,
            },
            species,
            conjugate,
            lot,
            validation_files,
        };

        Ok(ret)
    }
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<ViewValidation> {
        let item = ValidationBmc::get(ctx, mm, id).await?;
        let clone = ViewCloneBmc::get(ctx, mm, item.clone_id).await?;
        let species = match clone.species_id {
            Some(sid) => Some(SpeciesBmc::get(ctx, mm, sid).await?),
            None => None,
        };
        let conjugate = match item.conjugate_id {
            Some(cid) => Some(ConjugateBmc::get(ctx, mm, cid).await?),
            None => None,
        };
        let lot = match item.lot_id {
            Some(lid) => Some(ViewLotBmc::get(ctx, mm, lid).await?),
            None => None,
        };
        let ret = ViewValidation {
            id: item.id,
            tissue: item.tissue,
            notes: item.notes,
            clone,
            antigen_retrieval_time: item.antigen_retrieval_time,
            antigen_retrieval_type: item.antigen_retrieval_type,
            antigen_retrieval_temperature: item.antigen_retrieval_temperature,
            application: item.application,
            clone_id: item.clone_id,
            concentration: item.concentration,
            concentration_unit: item.concentration_unit,
            conjugate_id: item.conjugate_id,
            created_at: item.created_at,
            created_by: item.created_by,
            fixation: item.fixation,
            fixation_notes: item.fixation_notes,
            incubation_conditions: item.incubation_conditions,
            is_archived: item.is_archived,
            lot_id: item.lot_id,
            file_id: item.file_id,
            methanol_treatment: item.methanol_treatment,
            methanol_treatment_concentration: item.methanol_treatment_concentration,
            negative_control: item.negative_control,
            positive_control: item.positive_control,
            saponin: item.saponin,
            saponin_concentration: item.saponin_concentration,
            species_id: item.species_id,
            status: item.status,
            surface_staining: item.surface_staining,
            surface_staining_concentration: item.surface_staining_concentration,
            updated_at: item.updated_at,
            user: None,
            species,
            conjugate,
            lot,
        };

        Ok(ret)
    }
    #[allow(clippy::too_many_lines)] // FIXME
    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        group_id: i32,
        filters: Option<Vec<ValidationFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<ViewValidation>> {
        let mut clone_map = HashMap::new();
        let clone_filters: Option<Vec<CloneFilter>> = match serde_json::from_value(json!([
            {
                "group_id": {"$eq":group_id}
            }
        ])) {
            Ok(ok) => Some(ok),
            Err(_) => None,
        };
        let op = ListOptions {
            limit: Some(10_000),
            ..Default::default()
        };
        let clones: Vec<ViewClone> =
            ViewCloneBmc::list(ctx, mm, Some(group_id), clone_filters, Some(op)).await?;
        for clone in clones {
            clone_map.insert(clone.id, clone);
        }

        let mut species_map = HashMap::new();
        let species_filters: Option<Vec<SpeciesFilter>> = match serde_json::from_value(json!([
            {
                "group_id": {"$eq":group_id}
            }
        ])) {
            Ok(ok) => Some(ok),
            Err(_) => None,
        };
        let op = ListOptions {
            limit: Some(10_000),
            ..Default::default()
        };
        let speciess: Vec<Species> = SpeciesBmc::list(ctx, mm, species_filters, Some(op)).await?;
        for species in speciess {
            species_map.insert(species.id, species);
        }

        let mut conjugate_map = HashMap::new();
        let conjugate_filters: Option<Vec<ConjugateFilter>> = match serde_json::from_value(json!([
            {
                "group_id": {"$eq":group_id}
            }
        ])) {
            Ok(ok) => Some(ok),
            Err(_) => None,
        };
        let op = ListOptions {
            limit: Some(10_000),
            ..Default::default()
        };
        let conjugates: Vec<Conjugate> =
            ConjugateBmc::list(ctx, mm, conjugate_filters, Some(op)).await?;
        for conjugate in conjugates {
            conjugate_map.insert(conjugate.id, conjugate);
        }

        let mut lot_map = HashMap::new();
        let mut user_map = HashMap::new();
        let lot_filters: Option<Vec<LotFilter>> = match serde_json::from_value(json!([
            {
                "group_id": {"$eq":group_id}
            }
        ])) {
            Ok(ok) => Some(ok),
            Err(_) => None,
        };
        let op = ListOptions {
            limit: Some(10_000),
            ..Default::default()
        };
        let lots: Vec<ViewLot> =
            ViewLotBmc::list(ctx, mm, Some(group_id), lot_filters, Some(op)).await?;
        for lot in lots {
            lot_map.insert(lot.id, lot);
        }

        let mem_op = ListOptions {
            limit: Some(10_000),
            ..Default::default()
        };
        let mem_filters: Option<Vec<MemberFilter>> =
            match serde_json::from_value(json!([{"group_id": {"$eq": group_id}}])) {
                Ok(o) => Some(o),
                Err(_) => None,
            };
        let members: Vec<Member> = MemberBmc::list(ctx, mm, mem_filters, Some(mem_op)).await?;
        let mem_map: HashMap<i32, i32> = members.iter().map(|e| (e.user_id, e.id)).collect();

        let op = ListOptions {
            limit: Some(10_000),
            ..Default::default()
        };
        let users: Vec<User> = UserBmc::list(ctx, mm, None, Some(op)).await?;
        for user in users {
            let member_id = mem_map.get(&user.id).unwrap_or(&0);
            user_map.insert(*member_id, user);
        }

        let validations: Vec<Validation> =
            ValidationBmc::list(ctx, mm, filters, list_options).await?;
        let mut returns = vec![];
        for item in validations {
            let clone = match clone_map.get(&{ item.clone_id }) {
                Some(v) => v.clone(),
                None => ViewClone::default(),
            };
            let mut species = None;
            if let Some(species_id) = &item.species_id {
                species = species_map.get(species_id).cloned();
            };

            let mut conjugate = None;
            if let Some(conjugate_id) = &item.conjugate_id {
                conjugate = conjugate_map.get(conjugate_id).cloned();
            };

            let mut lot = None;
            if let Some(lot_id) = &item.lot_id {
                lot = lot_map.get(lot_id).cloned();
            };

            let user = match user_map.get(&item.created_by) {
                Some(v) => v.clone(),
                None => User::default(),
            };

            returns.push(ViewValidation {
                id: item.id,
                tissue: item.tissue,
                notes: item.notes,
                clone,
                antigen_retrieval_time: item.antigen_retrieval_time,
                antigen_retrieval_type: item.antigen_retrieval_type,
                antigen_retrieval_temperature: item.antigen_retrieval_temperature,
                application: item.application,
                clone_id: item.clone_id,
                concentration: item.concentration,
                concentration_unit: item.concentration_unit,
                conjugate_id: item.conjugate_id,
                created_at: item.created_at,
                created_by: item.created_by,
                fixation: item.fixation,
                fixation_notes: item.fixation_notes,
                incubation_conditions: item.incubation_conditions,
                is_archived: item.is_archived,
                lot_id: item.lot_id,
                file_id: item.file_id,
                methanol_treatment: item.methanol_treatment,
                methanol_treatment_concentration: item.methanol_treatment_concentration,
                negative_control: item.negative_control,
                positive_control: item.positive_control,
                saponin: item.saponin,
                saponin_concentration: item.saponin_concentration,
                species_id: item.species_id,
                status: item.status,
                surface_staining: item.surface_staining,
                surface_staining_concentration: item.surface_staining_concentration,
                updated_at: item.updated_at,
                user: Some(user),
                species,
                conjugate,
                lot,
            });
        }

        Ok(returns)
    }
}
