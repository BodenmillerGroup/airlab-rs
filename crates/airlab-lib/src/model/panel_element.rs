use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

impl PanelElementBmc {
    #[must_use]
    pub fn get_create_sql(drop_table: bool) -> String {
        let table = Self::TABLE;
        format!(
            r##"{}
create table if not exists "{table}" (
  id serial primary key,
  panel_id integer NOT NULL,
  conjugate_id integer NOT NULL,
  dilution_type smallint DEFAULT 0 NOT NULL,
  concentration real
);
ALTER TABLE ONLY panel_element
  ADD CONSTRAINT "UQ_panel_element_panel_id_and_conjugate_id" UNIQUE (panel_id, conjugate_id);
CREATE INDEX "IDX_panel_element_conjugate_id" ON panel_element USING btree (conjugate_id);
CREATE INDEX "IDX_panel_element_panel_id" ON panel_element USING btree (panel_id);
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
pub struct PanelElement {
    pub id: i32,

    #[serde(rename = "panelId")]
    pub panel_id: i32,
    #[serde(rename = "conjugateId")]
    pub conjugate_id: i32,
    #[serde(rename = "dilutionType")]
    pub dilution_type: i16,
    pub concentration: Option<f32>,
}

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct PanelElementForCreate {
    #[serde(rename = "panelId")]
    pub panel_id: i32,
    #[serde(rename = "conjugateId")]
    pub conjugate_id: i32,
    #[serde(rename = "dilutionType")]
    pub dilution_type: i32,
    pub concentration: Option<f32>,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct PanelElementForUpdate {
    #[serde(rename = "dilutionType")]
    pub dilution_type: i32,
    pub concentration: Option<f32>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
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
    ) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, panel_element_c).await
    }
    pub async fn create_full(
        ctx: &Ctx,
        mm: &ModelManager,
        panel_element_c: PanelElement,
    ) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, panel_element_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<PanelElement> {
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

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        panel_element_u: PanelElementForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, panel_element_u).await
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
    async fn test_panel_element_create_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let _fx_name = "test_create_ok name";

        let panel_element_c = PanelElementForCreate {
            panel_id: 1815,
            conjugate_id: 4292,
            dilution_type: 1,
            concentration: None,
        };
        let id = PanelElementBmc::create(&ctx, &mm, panel_element_c).await?;

        let panel_element = PanelElementBmc::get(&ctx, &mm, id).await?;
        assert_eq!(panel_element.id, 1);

        PanelElementBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_panel_element_get_err_not_found() -> Result<()> {
        let mm = ModelManager::new().await?;
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

    #[ignore]
    #[tokio::test]
    async fn test_panel_element_list_all_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let tname = "test_panel_element_list_all_ok";
        let seeds = _dev_utils::get_panel_element_seed(tname);
        _dev_utils::seed_panel_elements(&ctx, &mm, &seeds).await?;

        let panel_elements = PanelElementBmc::list(&ctx, &mm, None, None).await?;

        let panel_elements: Vec<PanelElement> =
            panel_elements.into_iter().filter(|t| t.id == 1).collect();
        assert_eq!(panel_elements.len(), 4, "number of seeded panel_elements.");

        if false {
            for panel_element in panel_elements.iter() {
                PanelElementBmc::delete(&ctx, &mm, panel_element.id).await?;
            }
        }

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_panel_element_list_by_filter_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let tname = "test_panel_element_list_all_ok";
        let seeds = _dev_utils::get_panel_element_seed(tname);
        _dev_utils::seed_panel_elements(&ctx, &mm, &seeds).await?;

        let filters: Vec<PanelElementFilter> = serde_json::from_value(json!([
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
        let panel_elements =
            PanelElementBmc::list(&ctx, &mm, Some(filters), Some(list_options)).await?;

        assert_eq!(panel_elements.len(), 3);
        assert!(panel_elements[0].id == 3);

        if false {
            let panel_elements = PanelElementBmc::list(
                &ctx,
                &mm,
                Some(serde_json::from_value(json!([{
                    "name": {"$startsWith": "test_list_by_filter_ok"}
                }]))?),
                None,
            )
            .await?;
            assert_eq!(panel_elements.len(), 5);
            for panel_element in panel_elements.iter() {
                PanelElementBmc::delete(&ctx, &mm, panel_element.id).await?;
            }
        }

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_panel_element_update_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let tname = "test_panel_element_list_all_ok";
        let seeds = _dev_utils::get_panel_element_seed(tname);
        let fx_panel_element = _dev_utils::seed_panel_elements(&ctx, &mm, &seeds)
            .await?
            .remove(0);

        PanelElementBmc::update(
            &ctx,
            &mm,
            fx_panel_element.id,
            PanelElementForUpdate {
                ..Default::default()
            },
        )
        .await?;

        let panel_element = PanelElementBmc::get(&ctx, &mm, fx_panel_element.id).await?;
        assert_eq!(panel_element.id, 1);

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_panel_element_delete_err_not_found() -> Result<()> {
        let mm = ModelManager::new().await?;
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
