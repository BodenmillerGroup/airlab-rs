use crate::ctx::Ctx;
use crate::model::conjugate::{Conjugate, ConjugateBmc, ConjugateFilter};
use crate::model::lot::{Lot, LotBmc, LotFilter};
use crate::model::panel_element::{PanelElement, PanelElementBmc, PanelElementFilter};
use serde_json::json;

use crate::model::ModelManager;
use crate::model::Result;
use modql::filter::ListOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    lot: MinLot,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ViewPanelElement {
    pub id: i32,
    #[serde(rename = "panelId")]
    pub panel_id: i32,
    #[serde(rename = "conjugateId")]
    pub conjugate_id: i32,
    #[serde(rename = "dilutionType")]
    pub dilution_type: i32,
    pub concentration: Option<f32>,
    pub conjugate: MinConjugate,
}

pub struct ViewPanelElementBmc;

impl ViewPanelElementBmc {
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<ViewPanelElement> {
        let element = PanelElementBmc::get(ctx, mm, id).await?;
        let conjugate = ConjugateBmc::get(ctx, mm, element.conjugate_id).await?;
        let lot = LotBmc::get(ctx, mm, conjugate.lot_id).await?;
        let ret = ViewPanelElement {
            id: element.id,
            panel_id: element.panel_id,
            conjugate_id: element.conjugate_id,
            concentration: element.concentration,
            dilution_type: i32::from(element.dilution_type),
            conjugate: MinConjugate {
                id: conjugate.id,
                tube_number: conjugate.tube_number,
                lot: MinLot {
                    id: lot.id,
                    name: lot.name,
                },
            },
        };

        Ok(ret)
    }
    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<PanelElementFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<ViewPanelElement>> {
        let elements: Vec<PanelElement> =
            PanelElementBmc::list(ctx, mm, filters, list_options).await?;
        let conj_ids: Vec<i32> = elements.iter().map(|e| e.conjugate_id).collect();
        let mut conj_map = HashMap::new();
        let filters: Vec<ConjugateFilter> =
            serde_json::from_value(json!([{"id": {"$in": conj_ids}}])).unwrap_or(vec![]);
        let op = ListOptions {
            limit: Some(10_000),
            ..Default::default()
        };
        let conjugates: Vec<Conjugate> =
            ConjugateBmc::list(ctx, mm, Some(filters), Some(op)).await?;
        let lot_ids: Vec<i32> = conjugates.iter().map(|e| e.lot_id).collect();
        for conjugate in conjugates {
            conj_map.insert(conjugate.id, conjugate);
        }
        let mut lot_map = HashMap::new();
        let filters: Vec<LotFilter> =
            serde_json::from_value(json!([{"id": {"$in": lot_ids}}])).unwrap_or(vec![]);
        let op = ListOptions {
            limit: Some(10_000),
            ..Default::default()
        };
        let lots: Vec<Lot> = LotBmc::list(ctx, mm, Some(filters), Some(op)).await?;
        for lot in lots {
            lot_map.insert(lot.id, lot);
        }
        let mut returns = vec![];
        for element in elements {
            if let Some(conj) = conj_map.get(&element.conjugate_id) {
                if let Some(lot) = lot_map.get(&conj.lot_id) {
                    returns.push(ViewPanelElement {
                        id: element.id,
                        panel_id: element.panel_id,
                        conjugate_id: element.conjugate_id,
                        concentration: element.concentration,
                        dilution_type: i32::from(element.dilution_type),
                        conjugate: MinConjugate {
                            id: conj.id,
                            tube_number: conj.tube_number,
                            lot: MinLot {
                                id: lot.id,
                                name: lot.name.clone(),
                            },
                        },
                    });
                }
            }
        }

        Ok(returns)
    }
}
