mod dev_db;

use crate::ctx::Ctx;
use crate::model::clone::{Clone, CloneBmc, CloneForCreate};
use crate::model::conjugate::{Conjugate, ConjugateBmc, ConjugateForCreate};
use crate::model::group::{Group, GroupBmc, GroupForCreate};
use crate::model::lot::{Lot, LotBmc, LotForCreate};
use crate::model::member::{Member, MemberBmc, MemberForCreate};
use crate::model::panel::{Panel, PanelBmc, PanelForCreate};
use crate::model::panel_element::{PanelElement, PanelElementBmc, PanelElementForCreate};
use crate::model::protein::{Protein, ProteinBmc, ProteinForCreate};
use crate::model::provider::{Provider, ProviderBmc, ProviderForCreate};
use crate::model::species::{Species, SpeciesBmc, SpeciesForCreate};
use crate::model::tag::{Tag, TagBmc, TagForCreate};
use crate::model::validation::{Validation, ValidationBmc, ValidationForCreate};
use crate::model::validation_file::{ValidationFile, ValidationFileBmc, ValidationFileForCreate};
use crate::model::{self, ModelManager};
use tokio::sync::OnceCell;
use tracing::info;

#[allow(clippy::missing_panics_doc)]
pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    #[allow(clippy::unwrap_used)]
    INIT.get_or_init(|| async {
        info!("DEV - init_dev_all()");

        dev_db::init_dev_db().await.unwrap();
    })
    .await;
}

#[allow(clippy::missing_panics_doc)]
pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();

    #[allow(clippy::unwrap_used)]
    let mm = INIT
        .get_or_init(|| async {
            init_dev().await;
            ModelManager::new().await.unwrap()
        })
        .await;

    mm.clone()
}

pub fn get_group_seed(unique_str: &str) -> Vec<GroupForCreate> {
    vec![
        GroupForCreate {
            name: format!("{unique_str}_01"),
            institution: "inst 01".to_string(),
            url: Some("url 01".to_string()),
            is_open: false,
            tags: None,
        },
        GroupForCreate {
            name: format!("{unique_str}_02"),
            institution: "inst 02".to_string(),
            url: Some("url 02".to_string()),
            is_open: false,
            tags: None,
        },
    ]
}

pub async fn seed_groups(
    ctx: &Ctx,
    mm: &ModelManager,
    items_c: &[GroupForCreate],
) -> model::Result<Vec<Group>> {
    let mut items = Vec::new();

    for item in items_c {
        let id = GroupBmc::create(ctx, mm, item.clone()).await?;
        let task = GroupBmc::get(ctx, mm, id).await?;

        items.push(task);
    }

    Ok(items)
}

pub async fn seed_proteins(
    ctx: &Ctx,
    mm: &ModelManager,
    items_c: &[ProteinForCreate],
) -> model::Result<Vec<Protein>> {
    let mut items = Vec::new();

    for item in items_c {
        let id = ProteinBmc::create(ctx, mm, item.clone()).await?;
        let task = ProteinBmc::get(ctx, mm, id).await?;

        items.push(task);
    }

    Ok(items)
}

pub async fn seed_panel_elements(
    ctx: &Ctx,
    mm: &ModelManager,
    items_c: &[PanelElementForCreate],
) -> model::Result<Vec<PanelElement>> {
    let mut items = Vec::new();

    for item in items_c {
        let id = PanelElementBmc::create(ctx, mm, item.clone()).await?;
        let task = PanelElementBmc::get(ctx, mm, id).await?;

        items.push(task);
    }

    Ok(items)
}

pub async fn seed_validations(
    ctx: &Ctx,
    mm: &ModelManager,
    items_c: &[ValidationForCreate],
) -> model::Result<Vec<Validation>> {
    let mut items = Vec::new();

    for item in items_c {
        let id = ValidationBmc::create(ctx, mm, item.clone()).await?;
        let task = ValidationBmc::get(ctx, mm, id).await?;

        items.push(task);
    }

    Ok(items)
}

pub async fn seed_validation_files(
    ctx: &Ctx,
    mm: &ModelManager,
    items_c: &[ValidationFileForCreate],
) -> model::Result<Vec<ValidationFile>> {
    let mut items = Vec::new();

    for item in items_c {
        let id = ValidationFileBmc::create(ctx, mm, item.clone()).await?;
        let task = ValidationFileBmc::get(ctx, mm, id).await?;

        items.push(task);
    }

    Ok(items)
}

pub async fn seed_panels(
    ctx: &Ctx,
    mm: &ModelManager,
    items_c: &[PanelForCreate],
) -> model::Result<Vec<Panel>> {
    let mut items = Vec::new();

    for item in items_c {
        let id = PanelBmc::create(ctx, mm, item.clone()).await?;
        let task = PanelBmc::get(ctx, mm, id).await?;

        items.push(task);
    }

    Ok(items)
}

pub async fn seed_conjugates(
    ctx: &Ctx,
    mm: &ModelManager,
    items_c: &[ConjugateForCreate],
) -> model::Result<Vec<Conjugate>> {
    let mut items = Vec::new();

    for item in items_c {
        let id = ConjugateBmc::create(ctx, mm, item.clone()).await?;
        let task = ConjugateBmc::get(ctx, mm, id).await?;

        items.push(task);
    }

    Ok(items)
}

pub async fn seed_clones(
    ctx: &Ctx,
    mm: &ModelManager,
    items_c: &[CloneForCreate],
) -> model::Result<Vec<Clone>> {
    let mut items = Vec::new();

    for item in items_c {
        let id = CloneBmc::create(ctx, mm, item.clone()).await?;
        let task = CloneBmc::get(ctx, mm, id).await?;

        items.push(task);
    }

    Ok(items)
}

pub async fn seed_lots(
    ctx: &Ctx,
    mm: &ModelManager,
    items_c: &[LotForCreate],
) -> model::Result<Vec<Lot>> {
    let mut items = Vec::new();

    for item in items_c {
        let id = LotBmc::create(ctx, mm, item.clone()).await?;
        let task = LotBmc::get(ctx, mm, id).await?;

        items.push(task);
    }

    Ok(items)
}

pub async fn seed_species(
    ctx: &Ctx,
    mm: &ModelManager,
    items_c: &[SpeciesForCreate],
) -> model::Result<Vec<Species>> {
    let mut items = Vec::new();

    for item in items_c {
        let id = SpeciesBmc::create(ctx, mm, item.clone()).await?;
        let task = SpeciesBmc::get(ctx, mm, id).await?;

        items.push(task);
    }

    Ok(items)
}

pub async fn seed_providers(
    ctx: &Ctx,
    mm: &ModelManager,
    items_c: &[ProviderForCreate],
) -> model::Result<Vec<Provider>> {
    let mut items = Vec::new();

    for item in items_c {
        let id = ProviderBmc::create(ctx, mm, item.clone()).await?;
        let task = ProviderBmc::get(ctx, mm, id).await?;

        items.push(task);
    }

    Ok(items)
}

pub async fn seed_tags(
    ctx: &Ctx,
    mm: &ModelManager,
    items_c: &[TagForCreate],
) -> model::Result<Vec<Tag>> {
    let mut items = Vec::new();

    for item in items_c {
        let id = TagBmc::create(ctx, mm, item.clone()).await?;
        let task = TagBmc::get(ctx, mm, id).await?;

        items.push(task);
    }

    Ok(items)
}

pub async fn seed_members(
    ctx: &Ctx,
    mm: &ModelManager,
    user_ids: &[(i32, i32)],
) -> model::Result<Vec<Member>> {
    let mut items = Vec::new();

    for (group_id, user_id) in user_ids {
        let id = MemberBmc::create(
            ctx,
            mm,
            MemberForCreate {
                group_id: *group_id,
                user_id: *user_id,
                role: 0,
                activation_key: None,
                all_panels: false,
                is_active: false,
            },
        )
        .await?;
        let task = MemberBmc::get(ctx, mm, id).await?;

        items.push(task);
    }

    Ok(items)
}
