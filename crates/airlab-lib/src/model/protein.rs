use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

impl ProteinBmc {
    #[must_use]
    pub fn get_create_sql(drop_table: bool) -> String {
        let table = Self::TABLE;
        format!(
            r##"{}
create table if not exists "{table}" (
  id serial primary key,
  group_id integer NOT NULL,
  created_by integer NOT NULL,
  name character varying NOT NULL,
  description character varying,
  meta jsonb,
  created_at timestamp with time zone DEFAULT now() NOT NULL
);
CREATE INDEX "IDX_protein_created_by" ON protein USING btree (created_by);
CREATE INDEX "IDX_protein_group_id" ON protein USING btree (group_id);
CREATE INDEX "IDX_protein_name" ON protein USING btree (name);
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
pub struct Protein {
    pub id: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,

    #[serde(rename = "createdBy")]
    pub created_by: i32,
    pub name: String,
    pub description: Option<String>,
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>, //String
}

#[derive(Fields, Deserialize, Clone)]
pub struct ProteinForCreate {
    pub name: String,
    pub description: Option<String>,
    pub group_id: i32,
    pub created_by: i32,
}

#[derive(Fields, Default, Deserialize)]
pub struct ProteinForUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ProteinFilter {
    id: Option<OpValsInt64>,
    group_id: Option<OpValsInt64>,
    name: Option<OpValsString>,
    description: Option<OpValsString>,
}

pub struct ProteinBmc;

impl DbBmc for ProteinBmc {
    const TABLE: &'static str = "protein";
}

impl ProteinBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, protein_c: ProteinForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, protein_c).await
    }
    pub async fn create_full(ctx: &Ctx, mm: &ModelManager, protein_c: Protein) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, protein_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<Protein> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<ProteinFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Protein>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        protein_u: ProteinForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, protein_u).await
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
    async fn test_protein_create_ok() -> Result<()> {
        let ctx = Ctx::root_ctx();
        let mm = ModelManager::new().await?;
        let fx_name = "test_create_ok name";

        let protein_c = ProteinForCreate {
            name: fx_name.to_string(),
            description: Some(fx_name.to_string()),
            group_id: 1,
            created_by: 261,
        };
        let id = ProteinBmc::create(&ctx, &mm, protein_c).await?;

        let protein = ProteinBmc::get(&ctx, &mm, id).await?;
        assert_eq!(protein.name, fx_name);

        ProteinBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_protein_get_err_not_found() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = ProteinBmc::get(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "protein",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_protein_list_all_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let tname = "test_protein_list_all_ok";
        let seeds = _dev_utils::get_protein_seed(tname);
        _dev_utils::seed_proteins(&ctx, &mm, &seeds).await?;

        let proteins = ProteinBmc::list(&ctx, &mm, None, None).await?;

        let proteins: Vec<Protein> = proteins
            .into_iter()
            .filter(|t| t.name.starts_with("test_list_all_ok-protein"))
            .collect();
        assert_eq!(proteins.len(), 4, "number of seeded proteins.");

        if false {
            for protein in proteins.iter() {
                ProteinBmc::delete(&ctx, &mm, protein.id).await?;
            }
        }

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_protein_list_by_filter_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let tname = "test_protein_list_by_filter_ok";
        let seeds = _dev_utils::get_protein_seed(tname);
        _dev_utils::seed_proteins(&ctx, &mm, &seeds).await?;

        let filters: Vec<ProteinFilter> = serde_json::from_value(json!([
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
        let proteins = ProteinBmc::list(&ctx, &mm, Some(filters), Some(list_options)).await?;

        assert_eq!(proteins.len(), 3);
        assert!(proteins[0].name.ends_with("03"));
        assert!(proteins[1].name.ends_with("02.a"));
        assert!(proteins[2].name.ends_with("01.a"));

        if false {
            let proteins = ProteinBmc::list(
                &ctx,
                &mm,
                Some(serde_json::from_value(json!([{
                    "name": {"$startsWith": "test_list_by_filter_ok"}
                }]))?),
                None,
            )
            .await?;
            assert_eq!(proteins.len(), 5);
            for protein in proteins.iter() {
                ProteinBmc::delete(&ctx, &mm, protein.id).await?;
            }
        }

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_protein_update_ok() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let tname = "test_protein_list_by_filter_ok";
        let seeds = _dev_utils::get_protein_seed(tname);
        let fx_protein = _dev_utils::seed_proteins(&ctx, &mm, &seeds)
            .await?
            .remove(0);

        ProteinBmc::update(
            &ctx,
            &mm,
            fx_protein.id,
            ProteinForUpdate {
                name: Some(tname.to_string()),
                ..Default::default()
            },
        )
        .await?;

        let protein = ProteinBmc::get(&ctx, &mm, fx_protein.id).await?;
        assert_eq!(protein.name, tname);

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_protein_delete_err_not_found() -> Result<()> {
        let mm = ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = ProteinBmc::delete(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "protein",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
