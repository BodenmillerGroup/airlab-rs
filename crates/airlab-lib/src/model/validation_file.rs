use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize)]
pub struct ValidationFile {
    pub id: i64,

    #[serde(rename = "validationId")]
    pub validation_id: i64,
    #[serde(rename = "createdBy")]
    pub created_by: i64,
    pub hash: String,
    pub size: i64,
    pub name: Option<String>,
    pub extension: String,
    pub description: Option<String>,
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Fields, Deserialize, Clone, Debug)]
pub struct ValidationFileForCreate {
    #[serde(rename = "validationId")]
    pub validation_id: i64,
    #[serde(rename = "createdBy")]
    pub created_by: i64,
    pub hash: String,
    pub size: i64,
    pub name: Option<String>,
    pub extension: String,
    pub description: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct ValidationFileForUpdate {
    pub hash: String,
    pub size: i64,
    pub name: Option<String>,
    pub extension: Option<String>,
    pub description: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ValidationFileFilter {
    id: Option<OpValsInt64>,
    validation_id: Option<OpValsInt64>,

    name: Option<OpValsString>,
}

pub struct ValidationFileBmc;

impl DbBmc for ValidationFileBmc {
    const TABLE: &'static str = "validation_file";
}

impl ValidationFileBmc {
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        validation_file_c: ValidationFileForCreate,
    ) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, validation_file_c).await
    }
    pub async fn create_full(
        ctx: &Ctx,
        mm: &ModelManager,
        validation_file_c: ValidationFile,
    ) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, validation_file_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<ValidationFile> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<ValidationFileFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<ValidationFile>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        validation_file_u: ValidationFileForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, validation_file_u).await
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
    async fn test_validation_file_create_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_name = "test_create_ok name";

        let validation_file_c = ValidationFileForCreate {
            name: Some(fx_name.to_string()),
            created_by: 261,
            validation_id: 2221,
            hash: String::new(),
            size: 0,
            extension: "pdf".into(),
            description: None,
            created_at: chrono::offset::Utc::now(),
        };
        let id = ValidationFileBmc::create(&ctx, &mm, validation_file_c).await?;

        let validation_file = ValidationFileBmc::get(&ctx, &mm, id).await?;
        assert_eq!(validation_file.name, Some(fx_name.into()));

        ValidationFileBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_validation_file_get_err_not_found() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = ValidationFileBmc::get(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "validation_file",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_validation_file_list_all_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_validation_file_list_all_ok";
        let seeds = _dev_utils::get_validation_file_seed(tname);
        _dev_utils::seed_validation_files(&ctx, &mm, &seeds).await?;

        let validation_files = ValidationFileBmc::list(&ctx, &mm, None, None).await?;

        let validation_files: Vec<ValidationFile> = validation_files
            .into_iter()
            .filter(|t| {
                t.name
                    .as_deref()
                    .is_some_and(|name| name.starts_with(tname))
            })
            .collect();
        assert_eq!(
            validation_files.len(),
            4,
            "number of seeded validation_files."
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_validation_file_list_by_filter_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_validation_file_list_by_filter_ok";
        let seeds = _dev_utils::get_validation_file_seed(tname);
        _dev_utils::seed_validation_files(&ctx, &mm, &seeds).await?;

        let filters: Vec<ValidationFileFilter> = serde_json::from_value(json!([
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
        let validation_files =
            ValidationFileBmc::list(&ctx, &mm, Some(filters), Some(list_options)).await?;

        assert_eq!(validation_files.len(), 3);
        assert_eq!(
            validation_files[0].name.as_deref(),
            Some("test_validation_file_list_by_filter_ok-03")
        );
        assert_eq!(
            validation_files[1].name.as_deref(),
            Some("test_validation_file_list_by_filter_ok-02.a")
        );
        assert_eq!(
            validation_files[2].name.as_deref(),
            Some("test_validation_file_list_by_filter_ok-01.a")
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_validation_file_update_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_validation_file_update_ok";
        let seeds = _dev_utils::get_validation_file_seed(tname);
        let fx_name_new = "test_update_ok - validation_file 01 - new";
        let fx_validation_file = _dev_utils::seed_validation_files(&ctx, &mm, &seeds)
            .await?
            .remove(0);

        ValidationFileBmc::update(
            &ctx,
            &mm,
            fx_validation_file.id,
            ValidationFileForUpdate {
                name: Some(fx_name_new.to_string()),
                ..Default::default()
            },
        )
        .await?;

        let validation_file = ValidationFileBmc::get(&ctx, &mm, fx_validation_file.id).await?;
        assert_eq!(validation_file.name, Some(fx_name_new.into()));

        Ok(())
    }

    #[tokio::test]
    async fn test_validation_file_delete_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = ValidationFileBmc::delete(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "validation_file",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
