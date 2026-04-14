use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
use crate::model::helpers::{bool_or, i64_or, opt_bool, opt_i64, opt_string};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsBool, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize)]
pub struct Panel {
    pub id: i64,
    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "createdBy")]
    pub created_by: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "isFluorophore")]
    pub is_fluorophore: bool,
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    pub application: Option<i64>,
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "isArchived")]
    pub is_archived: bool,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct PanelForCreate {
    pub name: Option<String>,
    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "createdBy")]
    pub created_by: Option<i64>,
    pub description: Option<String>,
    #[serde(rename = "isFluorophore")]
    pub is_fluorophore: bool,
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    pub application: Option<i64>,
}

impl From<Value> for PanelForCreate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        PanelForCreate {
            name: opt_string(&obj, "name"),
            group_id: i64_or(&obj, "groupId", 0),
            created_by: opt_i64(&obj, "createdBy"),
            description: opt_string(&obj, "description"),
            is_fluorophore: bool_or(&obj, "isFluorophore", false),
            is_locked: bool_or(&obj, "isLocked", false),
            application: opt_i64(&obj, "application"),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ElementUpdate {
    pub concentration: Option<f64>,
    #[serde(rename = "conjugateId")]
    pub conjugate_id: i64,
    #[serde(rename = "dilutionType")]
    pub dilution_type: i64,
}

#[derive(Default, Deserialize, Debug)]
pub struct PanelPayloadForUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "isFluorophore")]
    pub is_fluorophore: Option<bool>,
    #[serde(rename = "isLocked")]
    pub is_locked: Option<bool>,
    pub application: Option<i64>,
    pub elements: Vec<ElementUpdate>,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct PanelForUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "isFluorophore")]
    pub is_fluorophore: Option<bool>,
    #[serde(rename = "isLocked")]
    pub is_locked: Option<bool>,
    pub is_archived: Option<bool>,
    pub application: Option<i64>,
}

impl From<Value> for PanelForUpdate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        PanelForUpdate {
            name: opt_string(&obj, "name"),
            description: opt_string(&obj, "description"),
            is_fluorophore: opt_bool(&obj, "isFluorophore"),
            is_locked: opt_bool(&obj, "isLocked"),
            is_archived: opt_bool(&obj, "is_archived"),
            application: opt_i64(&obj, "application"),
        }
    }
}

#[derive(FilterNodes, Deserialize, Default, Debug, Clone)]
pub struct PanelFilter {
    id: Option<OpValsInt64>,
    group_id: Option<OpValsInt64>,
    name: Option<OpValsString>,
    is_archived: Option<OpValsBool>,
}

pub struct PanelBmc;

impl DbBmc for PanelBmc {
    const TABLE: &'static str = "panel";
}

impl PanelBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, panel_c: PanelForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, panel_c).await
    }
    pub async fn create_full(ctx: &Ctx, mm: &ModelManager, panel_c: Panel) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, panel_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Panel> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<PanelFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Panel>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn count(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<PanelFilter>>,
    ) -> Result<i64> {
        base::count::<Self, _>(ctx, mm, filters).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        panel_u: PanelForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, panel_u).await
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
    async fn test_panel_create_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_name = "test_create_ok name";

        let panel_c = PanelForCreate {
            name: Some(fx_name.to_string()),
            group_id: 1,
            created_by: Some(261),
            description: None,
            is_fluorophore: false,
            is_locked: false,
            application: None,
        };
        let id = PanelBmc::create(&ctx, &mm, panel_c).await?;

        let panel = PanelBmc::get(&ctx, &mm, id).await?;
        assert_eq!(panel.name.as_deref(), Some(fx_name));

        PanelBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_panel_get_err_not_found() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = PanelBmc::get(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "panel",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_panel_list_all_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_panel_list_all_ok";
        let seeds = _dev_utils::get_panel_seed(tname);
        _dev_utils::seed_panels(&ctx, &mm, &seeds).await?;

        let panels = PanelBmc::list(&ctx, &mm, None, None).await?;

        let panels: Vec<Panel> = panels
            .into_iter()
            .filter(|t| {
                t.name
                    .as_deref()
                    .is_some_and(|name| name.starts_with(tname))
            })
            .collect();
        assert_eq!(panels.len(), 4, "number of seeded panels.");

        if false {
            for panel in panels.iter() {
                PanelBmc::delete(&ctx, &mm, panel.id).await?;
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_panel_list_by_filter_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_panel_list_by_filter_ok";
        let seeds = _dev_utils::get_panel_seed(tname);
        _dev_utils::seed_panels(&ctx, &mm, &seeds).await?;

        let filters: Vec<PanelFilter> = serde_json::from_value(json!([
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
        let panels = PanelBmc::list(&ctx, &mm, Some(filters), Some(list_options)).await?;

        assert_eq!(panels.len(), 3);

        if false {
            let panels = PanelBmc::list(
                &ctx,
                &mm,
                Some(serde_json::from_value(json!([{
                    "name": {"$startsWith": "test_list_by_filter_ok"}
                }]))?),
                None,
            )
            .await?;
            assert_eq!(panels.len(), 5);
            for panel in panels.iter() {
                PanelBmc::delete(&ctx, &mm, panel.id).await?;
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_panel_update_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_panel_list_by_filter_ok";
        let seeds = _dev_utils::get_panel_seed(tname);
        let fx_panel = _dev_utils::seed_panels(&ctx, &mm, &seeds).await?.remove(0);

        PanelBmc::update(
            &ctx,
            &mm,
            fx_panel.id,
            PanelForUpdate {
                name: Some(tname.to_string()),
                ..Default::default()
            },
        )
        .await?;

        let panel = PanelBmc::get(&ctx, &mm, fx_panel.id).await?;
        assert_eq!(panel.name.as_deref(), Some(tname));

        Ok(())
    }

    #[tokio::test]
    async fn test_panel_delete_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = PanelBmc::delete(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "panel",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
