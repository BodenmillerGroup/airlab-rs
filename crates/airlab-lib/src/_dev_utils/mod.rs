mod test_db;

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
pub use test_db::TestDb;
pub use test_db::init_test_env;

pub async fn init_test() -> TestDb {
    test_db::new_test_db().await
}

fn seeded_names(unique_str: &str) -> [String; 4] {
    [
        format!("{unique_str}-01.a"),
        format!("{unique_str}-02.a"),
        format!("{unique_str}-03"),
        format!("{unique_str}-04"),
    ]
}

pub fn get_protein_seed(unique_str: &str) -> Vec<ProteinForCreate> {
    let names = seeded_names(unique_str);
    vec![
        ProteinForCreate {
            name: names[0].clone(),
            description: Some(names[0].clone()),
            group_id: 1000,
            created_by: 1303,
        },
        ProteinForCreate {
            name: names[1].clone(),
            description: Some(names[1].clone()),
            group_id: 1000,
            created_by: 1303,
        },
        ProteinForCreate {
            name: names[2].clone(),
            description: Some(names[2].clone()),
            group_id: 1000,
            created_by: 1303,
        },
        ProteinForCreate {
            name: names[3].clone(),
            description: Some(names[3].clone()),
            group_id: 1000,
            created_by: 1303,
        },
    ]
}

pub fn get_conjugate_seed(unique_str: &str) -> Vec<ConjugateForCreate> {
    let names = seeded_names(unique_str);
    names
        .into_iter()
        .map(|description| ConjugateForCreate {
            description: Some(description),
            group_id: 1000,
            created_by: Some(1303),
            labeled_by: None,
            finished_by: None,
            lot_id: 1007,
            status: Some(1),
            tag_id: 1005,
            storage_id: None,
            concentration: None,
            finished_at: None,
            is_archived: None,
            custom_id: None,
        })
        .collect()
}

pub fn get_species_seed(unique_str: &str) -> Vec<SpeciesForCreate> {
    let names = seeded_names(unique_str);
    vec![
        SpeciesForCreate {
            name: names[0].clone(),
            group_id: 1,
            acronym: "bb1".to_string(),
        },
        SpeciesForCreate {
            name: names[1].clone(),
            group_id: 1,
            acronym: "bb2".to_string(),
        },
        SpeciesForCreate {
            name: names[2].clone(),
            group_id: 1,
            acronym: "bb3".to_string(),
        },
        SpeciesForCreate {
            name: names[3].clone(),
            group_id: 1,
            acronym: "bb4".to_string(),
        },
    ]
}

pub fn get_provider_seed(unique_str: &str) -> Vec<ProviderForCreate> {
    let names = seeded_names(unique_str);
    vec![
        ProviderForCreate {
            name: names[0].clone(),
            group_id: 1000,
            description: None,
            url: None,
        },
        ProviderForCreate {
            name: names[1].clone(),
            group_id: 1000,
            description: None,
            url: None,
        },
        ProviderForCreate {
            name: names[2].clone(),
            group_id: 1000,
            description: None,
            url: None,
        },
        ProviderForCreate {
            name: names[3].clone(),
            group_id: 1000,
            description: None,
            url: None,
        },
    ]
}

pub fn get_clone_seed(unique_str: &str) -> Vec<CloneForCreate> {
    let names = seeded_names(unique_str);
    vec![
        CloneForCreate {
            name: names[0].clone(),
            group_id: 1000,
            created_by: Some(1303),
            epitope: String::new(),
            is_archived: None,
            is_phospho: false,
            application: None,
            reactivity: None,
            is_polyclonal: false,
            isotype: String::new(),
            protein_id: 1002,
            species_id: Some(1004),
        },
        CloneForCreate {
            name: names[1].clone(),
            group_id: 1000,
            created_by: Some(1303),
            epitope: String::new(),
            is_archived: None,
            is_phospho: false,
            application: None,
            reactivity: None,
            is_polyclonal: false,
            isotype: String::new(),
            protein_id: 1002,
            species_id: Some(1004),
        },
        CloneForCreate {
            name: names[2].clone(),
            group_id: 1000,
            created_by: Some(1303),
            epitope: String::new(),
            is_archived: None,
            is_phospho: false,
            application: None,
            reactivity: None,
            is_polyclonal: false,
            isotype: String::new(),
            protein_id: 1002,
            species_id: Some(1004),
        },
        CloneForCreate {
            name: names[3].clone(),
            group_id: 1000,
            created_by: Some(1303),
            epitope: String::new(),
            is_archived: None,
            is_phospho: false,
            application: None,
            reactivity: None,
            is_polyclonal: false,
            isotype: String::new(),
            protein_id: 1002,
            species_id: Some(1004),
        },
    ]
}

pub fn get_panel_element_seed(_unique_str: &str) -> Vec<PanelElementForCreate> {
    vec![
        PanelElementForCreate {
            panel_id: 1009,
            conjugate_id: 1008,
            dilution_type: 1,
            concentration: Some(0.1),
        },
        PanelElementForCreate {
            panel_id: 1815,
            conjugate_id: 1008,
            dilution_type: 2,
            concentration: Some(0.2),
        },
        PanelElementForCreate {
            panel_id: 1009,
            conjugate_id: 4292,
            dilution_type: 3,
            concentration: Some(0.3),
        },
        PanelElementForCreate {
            panel_id: 1815,
            conjugate_id: 4292,
            dilution_type: 4,
            concentration: Some(0.4),
        },
    ]
}

pub fn get_validation_file_seed(unique_str: &str) -> Vec<ValidationFileForCreate> {
    let names = seeded_names(unique_str);
    vec![
        ValidationFileForCreate {
            name: Some(names[0].clone()),
            created_by: 1303,
            validation_id: 1011,
            hash: String::new(),
            size: 0,
            extension: "pdf".into(),
            description: None,
            created_at: chrono::offset::Utc::now(),
        },
        ValidationFileForCreate {
            name: Some(names[1].clone()),
            created_by: 1303,
            validation_id: 1011,
            hash: String::new(),
            size: 0,
            extension: "pdf".into(),
            description: None,
            created_at: chrono::offset::Utc::now(),
        },
        ValidationFileForCreate {
            name: Some(names[2].clone()),
            created_by: 1303,
            validation_id: 1011,
            hash: String::new(),
            size: 0,
            extension: "pdf".into(),
            description: None,
            created_at: chrono::offset::Utc::now(),
        },
        ValidationFileForCreate {
            name: Some(names[3].clone()),
            created_by: 1303,
            validation_id: 1011,
            hash: String::new(),
            size: 0,
            extension: "pdf".into(),
            description: None,
            created_at: chrono::offset::Utc::now(),
        },
    ]
}

pub fn get_validation_seed(unique_str: &str) -> Vec<ValidationForCreate> {
    let tissues = seeded_names(unique_str);
    tissues
        .into_iter()
        .map(|tissue| ValidationForCreate {
            group_id: 1000,
            created_by: Some(1303),
            clone_id: 1006,
            lot_id: Some(1007),
            conjugate_id: Some(1008),
            species_id: Some(1004),
            application: Some(1),
            positive_control: None,
            negative_control: None,
            incubation_conditions: None,
            concentration: None,
            concentration_unit: None,
            tissue: Some(tissue),
            fixation: None,
            fixation_notes: None,
            notes: None,
            antigen_retrieval_type: None,
            antigen_retrieval_time: None,
            antigen_retrieval_temperature: None,
            status: Some(1),
            saponin: Some(false),
            saponin_concentration: None,
            methanol_treatment: Some(false),
            methanol_treatment_concentration: None,
            surface_staining: Some(false),
            surface_staining_concentration: None,
            file_id: Some(1),
            is_archived: Some(false),
            created_at: Some(chrono::offset::Utc::now()),
            updated_at: Some(chrono::offset::Utc::now()),
        })
        .collect()
}

pub fn get_lot_seed(unique_str: &str) -> Vec<LotForCreate> {
    let names = seeded_names(unique_str);
    names
        .into_iter()
        .map(|name| LotForCreate {
            name,
            group_id: 1000,
            created_by: Some(1303),
            clone_id: 1006,
            provider_id: Some(1003),
            storage_id: None,
            collection_id: None,
            reference: None,
            approved_by: None,
            finished_by: None,
            finished_at: None,
            requested_by: None,
            is_archived: Some(false),
            ordered_at: None,
            note: None,
            ordered_by: None,
            received_by: None,
            price: None,
            purpose: None,
            status: None,
            received_at: None,
            requested_at: None,
            url: None,
        })
        .collect()
}

pub fn get_tag_seed(unique_str: &str) -> Vec<TagForCreate> {
    let names = seeded_names(unique_str);
    vec![
        TagForCreate {
            name: names[0].clone(),
            group_id: 1000,
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
        },
        TagForCreate {
            name: names[1].clone(),
            group_id: 1000,
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
        },
        TagForCreate {
            name: names[2].clone(),
            group_id: 1000,
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
        },
        TagForCreate {
            name: names[3].clone(),
            group_id: 1000,
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
        },
    ]
}

pub fn get_panel_seed(unique_str: &str) -> Vec<PanelForCreate> {
    let names = seeded_names(unique_str);
    vec![
        PanelForCreate {
            name: Some(names[0].clone()),
            group_id: 1000,
            created_by: Some(1303),
            description: None,
            is_fluorophore: false,
            is_locked: false,
            application: None,
        },
        PanelForCreate {
            name: Some(names[1].clone()),
            group_id: 1000,
            created_by: Some(1303),
            description: None,
            is_fluorophore: false,
            is_locked: false,
            application: None,
        },
        PanelForCreate {
            name: Some(names[2].clone()),
            group_id: 1000,
            created_by: Some(1303),
            description: None,
            is_fluorophore: false,
            is_locked: false,
            application: None,
        },
        PanelForCreate {
            name: Some(names[3].clone()),
            group_id: 1000,
            created_by: Some(1303),
            description: None,
            is_fluorophore: false,
            is_locked: false,
            application: None,
        },
    ]
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
    user_ids: &[(i64, i64)],
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
