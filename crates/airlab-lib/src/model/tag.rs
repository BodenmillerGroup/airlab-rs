use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
use crate::model::helpers::{bool_or, i64_or, opt_i64, opt_string, string_or};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize, Default)]
pub struct Tag {
    pub id: i64,
    #[serde(rename = "groupId")]
    pub group_id: i64,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "isMetal")]
    pub is_metal: bool,
    #[serde(rename = "isFluorophore")]
    pub is_fluorophore: bool,
    #[serde(rename = "isEnzyme")]
    pub is_enzyme: bool,
    #[serde(rename = "isBiotin")]
    pub is_biotin: bool,
    #[serde(rename = "isOther")]
    pub is_other: bool,
    pub mw: Option<i64>,
    pub emission: Option<i64>,
    pub excitation: Option<i64>,
    pub status: Option<i64>,
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Fields, Deserialize, Clone, Debug)]
pub struct TagForCreate {
    pub name: String,
    #[serde(rename = "groupId")]
    pub group_id: i64,
    pub description: Option<String>,
    #[serde(rename = "isMetal")]
    pub is_metal: bool,
    #[serde(rename = "isFluorophore")]
    pub is_fluorophore: bool,
    #[serde(rename = "isEnzyme")]
    pub is_enzyme: bool,
    #[serde(rename = "isBiotin")]
    pub is_biotin: bool,
    #[serde(rename = "isOther")]
    pub is_other: bool,
    pub mw: Option<i64>,
    pub emission: Option<i64>,
    pub excitation: Option<i64>,
    pub status: Option<i64>,
}

impl From<Value> for TagForCreate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        TagForCreate {
            name: string_or(&obj, "name"),
            group_id: i64_or(&obj, "groupId", 0),
            description: opt_string(&obj, "description"),
            is_metal: bool_or(&obj, "isMetal", false),
            is_fluorophore: bool_or(&obj, "isFluorophore", false),
            is_enzyme: bool_or(&obj, "isEnzyme", false),
            is_biotin: bool_or(&obj, "isBiotin", false),
            is_other: bool_or(&obj, "isOther", false),
            mw: opt_i64(&obj, "mw"),
            emission: opt_i64(&obj, "emission"),
            excitation: opt_i64(&obj, "excitation"),
            status: opt_i64(&obj, "status"),
        }
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Fields, Default, Deserialize, Debug)]
pub struct TagForUpdate {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "isMetal")]
    pub is_metal: bool,
    #[serde(rename = "isFluorophore")]
    pub is_fluorophore: bool,
    #[serde(rename = "isEnzyme")]
    pub is_enzyme: bool,
    #[serde(rename = "isBiotin")]
    pub is_biotin: bool,
    #[serde(rename = "isOther")]
    pub is_other: bool,
    pub mw: Option<i64>,
    pub emission: Option<i64>,
    pub excitation: Option<i64>,
    pub status: Option<i64>,
}
impl From<Value> for TagForUpdate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        TagForUpdate {
            name: string_or(&obj, "name"),
            description: opt_string(&obj, "description"),
            is_metal: bool_or(&obj, "isMetal", false),
            is_fluorophore: bool_or(&obj, "isFluorophore", false),
            is_enzyme: bool_or(&obj, "isEnzyme", false),
            is_biotin: bool_or(&obj, "isBiotin", false),
            is_other: bool_or(&obj, "isOther", false),
            mw: opt_i64(&obj, "mw"),
            emission: opt_i64(&obj, "emission"),
            excitation: opt_i64(&obj, "excitation"),
            status: opt_i64(&obj, "status"),
        }
    }
}

#[derive(FilterNodes, Deserialize, Default, Debug, Clone)]
pub struct TagFilter {
    id: Option<OpValsInt64>,
    group_id: Option<OpValsInt64>,

    name: Option<OpValsString>,
}

pub struct TagBmc;

impl DbBmc for TagBmc {
    const TABLE: &'static str = "tag";
}

impl TagBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, tag_c: TagForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, tag_c).await
    }
    pub async fn create_full(ctx: &Ctx, mm: &ModelManager, tag_c: Tag) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, tag_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Tag> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<TagFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Tag>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn count(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<TagFilter>>,
    ) -> Result<i64> {
        base::count::<Self, _>(ctx, mm, filters).await
    }

    pub async fn update(ctx: &Ctx, mm: &ModelManager, id: i64, tag_u: TagForUpdate) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, tag_u).await
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
    async fn test_tag_create_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_name = "test_create_ok name";

        let tag_c = TagForCreate {
            name: fx_name.to_string(),
            group_id: 1,
            description: None,
            is_metal: false,
            is_fluorophore: false,
            is_enzyme: false,
            is_biotin: false,
            is_other: false,
            mw: None,
            emission: None,
            excitation: None,
            status: Some(0),
        };
        let id = TagBmc::create(&ctx, &mm, tag_c).await?;

        let tag = TagBmc::get(&ctx, &mm, id).await?;
        assert_eq!(tag.name, fx_name);

        TagBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_tag_get_err_not_found() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = TagBmc::get(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "tag",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_tag_list_all_ok() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_tag_list_all_ok";
        let seeds = _dev_utils::get_tag_seed(tname);
        _dev_utils::seed_tags(&ctx, &mm, &seeds).await?;

        let tags = TagBmc::list(&ctx, &mm, None, None).await?;

        let tags: Vec<Tag> = tags
            .into_iter()
            .filter(|t| t.name.starts_with(tname))
            .collect();
        assert_eq!(tags.len(), 4, "number of seeded tags.");

        if false {
            for tag in tags.iter() {
                TagBmc::delete(&ctx, &mm, tag.id).await?;
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_tag_list_by_filter_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_tag_list_by_filter_ok";
        let seeds = _dev_utils::get_tag_seed(tname);
        _dev_utils::seed_tags(&ctx, &mm, &seeds).await?;

        let filters: Vec<TagFilter> = serde_json::from_value(json!([
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
        let tags = TagBmc::list(&ctx, &mm, Some(filters), Some(list_options)).await?;

        assert_eq!(tags.len(), 3);
        assert!(tags[0].name.ends_with("03"));
        assert!(tags[1].name.ends_with("02.a"));
        assert!(tags[2].name.ends_with("01.a"));

        if false {
            let tags = TagBmc::list(
                &ctx,
                &mm,
                Some(serde_json::from_value(json!([{
                    "name": {"$startsWith": "test_list_by_filter_ok"}
                }]))?),
                None,
            )
            .await?;
            assert_eq!(tags.len(), 5);
            for tag in tags.iter() {
                TagBmc::delete(&ctx, &mm, tag.id).await?;
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_tag_update_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tname = "test_tag_update_ok";
        let seeds = _dev_utils::get_tag_seed(tname);
        let fx_tag = _dev_utils::seed_tags(&ctx, &mm, &seeds).await?.remove(0);

        TagBmc::update(
            &ctx,
            &mm,
            fx_tag.id,
            TagForUpdate {
                name: tname.to_string(),
                ..Default::default()
            },
        )
        .await?;

        let tag = TagBmc::get(&ctx, &mm, fx_tag.id).await?;
        assert_eq!(tag.name, tname);

        Ok(())
    }

    #[tokio::test]
    async fn test_tag_delete_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = TagBmc::delete(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "tag",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
