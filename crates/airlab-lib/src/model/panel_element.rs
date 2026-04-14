use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
use crate::model::helpers::{i64_or, opt_f32};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default)]
pub struct PanelElement {
    pub id: i64,

    #[serde(rename = "panelId")]
    pub panel_id: i64,
    #[serde(rename = "conjugateId")]
    pub conjugate_id: i64,
    #[serde(rename = "dilutionType")]
    pub dilution_type: i64,
    pub concentration: Option<f32>,
}

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct PanelElementForCreate {
    #[serde(rename = "panelId")]
    pub panel_id: i64,
    #[serde(rename = "conjugateId")]
    pub conjugate_id: i64,
    #[serde(rename = "dilutionType")]
    pub dilution_type: i64,
    pub concentration: Option<f32>,
}

impl From<Value> for PanelElementForCreate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        PanelElementForCreate {
            panel_id: i64_or(&obj, "panelId", 0),
            conjugate_id: i64_or(&obj, "conjugateId", 0),
            dilution_type: i64_or(&obj, "dilutionType", 0),
            concentration: opt_f32(&obj, "concentration"),
        }
    }
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct PanelElementForUpdate {
    #[serde(rename = "dilutionType")]
    pub dilution_type: i64,
    pub concentration: Option<f32>,
}

impl From<Value> for PanelElementForUpdate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        PanelElementForUpdate {
            dilution_type: i64_or(&obj, "dilutionType", 0),
            concentration: opt_f32(&obj, "concentration"),
        }
    }
}

#[derive(FilterNodes, Deserialize, Default, Debug, Clone)]
pub struct PanelElementFilter {
    id: Option<OpValsInt64>,

    panel_id: Option<OpValsInt64>,
    conjugate_id: Option<OpValsInt64>,
}

pub struct PanelElementBmc;

impl DbBmc for PanelElementBmc {
    const TABLE: &'static str = "panel_element";
}

impl PanelElementBmc {
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        panel_element_c: PanelElementForCreate,
    ) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, panel_element_c).await
    }
    pub async fn create_full(
        ctx: &Ctx,
        mm: &ModelManager,
        panel_element_c: PanelElement,
    ) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, panel_element_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<PanelElement> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<PanelElementFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<PanelElement>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn count(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<PanelElementFilter>>,
    ) -> Result<i64> {
        base::count::<Self, _>(ctx, mm, filters).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        panel_element_u: PanelElementForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, panel_element_u).await
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
    async fn test_panel_element_create_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let panel_element_c = PanelElementForCreate {
            panel_id: 1815,
            conjugate_id: 4292,
            dilution_type: 1,
            concentration: None,
        };
        let id = PanelElementBmc::create(&ctx, &mm, panel_element_c).await?;

        let panel_element = PanelElementBmc::get(&ctx, &mm, id).await?;
        assert_eq!(panel_element.panel_id, 1815);
        assert_eq!(panel_element.conjugate_id, 4292);

        PanelElementBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_panel_element_get_err_not_found() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = PanelElementBmc::get(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "panel_element",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_panel_element_list_all_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_panel_element_list_all_ok";
        let seeds = _dev_utils::get_panel_element_seed(tname);
        _dev_utils::seed_panel_elements(&ctx, &mm, &seeds).await?;

        let panel_elements = PanelElementBmc::list(&ctx, &mm, None, None).await?;

        let panel_elements: Vec<PanelElement> = panel_elements
            .into_iter()
            .filter(|t| {
                matches!(
                    (t.panel_id, t.conjugate_id),
                    (1009, 1008) | (1815, 1008) | (1009, 4292) | (1815, 4292)
                )
            })
            .collect();
        assert_eq!(panel_elements.len(), 4, "number of seeded panel_elements.");

        Ok(())
    }

    #[tokio::test]
    async fn test_panel_element_list_by_filter_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_panel_element_list_by_filter_ok";
        let seeds = _dev_utils::get_panel_element_seed(tname);
        _dev_utils::seed_panel_elements(&ctx, &mm, &seeds).await?;

        let filters: Vec<PanelElementFilter> = serde_json::from_value(json!([
            {
                "panel_id": 1009
            }
        ]))?;
        let list_options = serde_json::from_value(json!({
            "order_bys": "id"
        }))?;
        let panel_elements =
            PanelElementBmc::list(&ctx, &mm, Some(filters), Some(list_options)).await?;

        assert_eq!(panel_elements.len(), 3);
        assert_eq!(panel_elements[0].dilution_type, 2);
        assert_eq!(panel_elements[1].dilution_type, 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_panel_element_update_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_panel_element_update_ok";
        let seeds = _dev_utils::get_panel_element_seed(tname);
        let fx_panel_element = _dev_utils::seed_panel_elements(&ctx, &mm, &seeds)
            .await?
            .remove(0);

        PanelElementBmc::update(
            &ctx,
            &mm,
            fx_panel_element.id,
            PanelElementForUpdate {
                dilution_type: 9,
                concentration: Some(0.9),
            },
        )
        .await?;

        let panel_element = PanelElementBmc::get(&ctx, &mm, fx_panel_element.id).await?;
        assert_eq!(panel_element.dilution_type, 9);
        assert_eq!(panel_element.concentration, Some(0.9));

        Ok(())
    }

    #[tokio::test]
    async fn test_panel_element_delete_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = PanelElementBmc::delete(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "panel_element",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
