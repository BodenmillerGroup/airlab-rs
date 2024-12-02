use crate::ctx::Ctx;
use crate::model::clone::{Clone, CloneBmc, CloneFilter};
use crate::model::protein::{Protein, ProteinBmc, ProteinFilter};
use crate::model::species::{Species, SpeciesBmc, SpeciesFilter};
use crate::model::validation::{MinValidation, ValidationBmc, ValidationFilter};
use crate::model::ModelManager;
use crate::model::Result;
use modql::filter::ListOptions;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{hash_map::Entry, HashMap};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ViewClone {
    pub id: i32,
    #[serde(rename = "createdBy")]
    pub created_by: i32,
    #[serde(rename = "updatedBy")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "proteinId")]
    pub protein_id: i32,
    #[serde(rename = "speciesId")]
    pub species_id: Option<i32>,
    pub name: String,
    pub isotype: Option<String>,
    pub epitope: Option<String>,
    #[serde(rename = "isPhospho")]
    pub is_phospho: bool,
    #[serde(rename = "isPolyclonal")]
    pub is_polyclonal: bool,
    pub reactivity: Option<Vec<i32>>,
    pub application: Option<serde_json::Value>,
    pub meta: Option<serde_json::Value>,
    pub protein: Protein,
    pub species: Option<Species>,
    pub validations: Vec<MinValidation>,
}

pub struct ViewCloneBmc;

impl ViewCloneBmc {
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<ViewClone> {
        let clone = CloneBmc::get(ctx, mm, id).await?;
        let protein = ProteinBmc::get(ctx, mm, clone.protein_id).await?;
        let species = match clone.species_id {
            Some(sid) => Some(SpeciesBmc::get(ctx, mm, sid).await?),
            None => None,
        };
        let ret = ViewClone {
            id: clone.id,
            application: clone.application,
            created_by: clone.created_by,
            epitope: clone.epitope,
            is_phospho: clone.is_phospho,
            updated_at: clone.updated_at,
            protein_id: clone.protein_id,
            species_id: clone.species_id,
            name: clone.name,
            isotype: clone.isotype,
            is_polyclonal: clone.is_polyclonal,
            reactivity: clone.reactivity,
            meta: clone.meta,
            protein,
            species,
            validations: vec![],
        };
        Ok(ret)
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        group_id: Option<i32>,
        filters: Option<Vec<CloneFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<ViewClone>> {
        let mut protein_map = HashMap::new();
        let mut species_map = HashMap::new();
        if let Some(group_id) = group_id {
            let protein_filters: Option<Vec<ProteinFilter>> = match serde_json::from_value(json!([
                {
                    "group_id": {"$eq":group_id}
                }
            ])) {
                Ok(ok) => Some(ok),
                Err(_) => None,
            };
            let protein_op = ListOptions {
                limit: Some(10_000),
                ..Default::default()
            };
            let proteins: Vec<Protein> =
                ProteinBmc::list(ctx, mm, protein_filters, Some(protein_op)).await?;
            for protein in proteins {
                protein_map.insert(protein.id, protein);
            }

            let species_filters: Option<Vec<SpeciesFilter>> = match serde_json::from_value(json!([
                {
                    "group_id": {"$eq": group_id}
                }
            ])) {
                Ok(ok) => Some(ok),
                Err(_) => None,
            };
            let speciess: Vec<Species> = SpeciesBmc::list(ctx, mm, species_filters, None).await?;
            for species in speciess {
                species_map.insert(species.id, species);
            }
        }

        let clones: Vec<Clone> = CloneBmc::list(ctx, mm, filters, list_options).await?;
        let filters: Option<Vec<ValidationFilter>> =
            match serde_json::from_value(json!([{"group_id": { "$eq": group_id}}])) {
                Ok(o) => Some(o),
                Err(_) => None,
            };
        let valop = ListOptions {
            limit: Some(10_000),
            ..Default::default()
        };
        let mut val_map: HashMap<i32, Vec<MinValidation>> = HashMap::new();
        let min_validations = ValidationBmc::minlist(ctx, mm, filters, Some(valop)).await?;
        for val in min_validations {
            let o = match val_map.entry(val.clone_id) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => v.insert(vec![]),
            };
            o.push(val);
        }
        let mut returns = vec![];
        for clone in clones {
            let protein = match protein_map.get(&{ clone.protein_id }) {
                Some(v) => v.clone(),
                None => Protein {
                    id: 0,
                    group_id: 0,
                    created_at: chrono::offset::Utc::now(),
                    created_by: 0,
                    description: None,
                    ..Default::default()
                },
            };
            let mut species = None;

            if let Some(species_id) = &clone.species_id {
                species = species_map.get(species_id).cloned();
            }
            let validations = match val_map.get(&clone.id) {
                Some(s) => (*s).clone(),
                None => vec![],
            };
            returns.push(ViewClone {
                id: clone.id,
                application: clone.application,
                created_by: clone.created_by,
                epitope: clone.epitope,
                is_phospho: clone.is_phospho,
                updated_at: clone.updated_at,
                protein_id: clone.protein_id,
                species_id: clone.species_id,
                name: clone.name,
                isotype: clone.isotype,
                is_polyclonal: clone.is_polyclonal,
                reactivity: clone.reactivity,
                meta: clone.meta,
                protein,
                species,
                validations,
            });
        }

        Ok(returns)
    }
}
