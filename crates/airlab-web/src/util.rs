use crate::Result;
use airlab_lib::ctx::Ctx;
use airlab_lib::model::ModelManager;
use airlab_lib::model::clone::{CloneBmc, CloneForCreate};
use airlab_lib::model::conjugate::{ConjugateBmc, ConjugateForCreate};
use airlab_lib::model::group::{GroupBmc, GroupForCreate};
use airlab_lib::model::lot::{LotBmc, LotForCreate};
use airlab_lib::model::member::{MemberBmc, MemberForCreate};
use airlab_lib::model::panel::{PanelBmc, PanelForCreate};
use airlab_lib::model::panel_element::{PanelElementBmc, PanelElementForCreate};
use airlab_lib::model::protein::{ProteinBmc, ProteinForCreate};
use airlab_lib::model::species::{SpeciesBmc, SpeciesForCreate};
use airlab_lib::model::tag::{TagBmc, TagForCreate};
use airlab_lib::model::validation::{ValidationBmc, ValidationForCreate};
use airlab_lib::model::validation_file::{ValidationFileBmc, ValidationFileForCreate};

struct Entity {
    group_id: i32,
    user_id: i32,
    protein_id: i32,
    species_id: i32,
    clone_id: i32,
    lot_id: i32,
    tag_id: i32,
    tag_index: usize,
    conjugate_id: i32,
    panel_id: i32,
    panel_element_id: i32,
    validation_id: i32,
    validation_file_id: i32,
    protein_name: String,
    description: String,
    epitope: String,
    isotype: String,
    clone_name: String,
    lot_name: String,
    dilution_type: i32,
    concentration: f32,
}

fn get_entities() -> Vec<Entity> {
    vec![
        Entity {
            group_id: 0,
            user_id: 0,
            protein_id: 0,
            species_id: 0,
            clone_id: 0,
            lot_id: 0,
            tag_id: 0,
            tag_index: 0,
            conjugate_id: 0,
            panel_id: 0,
            panel_element_id: 0,
            validation_id: 0,
            validation_file_id: 0,
            protein_name: "CD4".into(),
            description: "T-cell surface glycoprotein CD4".into(),
            epitope: "synthetic peptide corresponding to the residues of human CD4 protein".into(),
            isotype: "IgG".into(),
            clone_name: "EP204".into(),
            lot_name: "CD4".into(),
            dilution_type: 0,
            concentration: 2.0,
        },
        Entity {
            group_id: 0,
            user_id: 0,
            protein_id: 0,
            species_id: 0,
            clone_id: 0,
            lot_id: 0,
            tag_id: 0,
            tag_index: 1,
            conjugate_id: 0,
            panel_id: 0,
            panel_element_id: 0,
            validation_id: 0,
            validation_file_id: 0,
            protein_name: "CDX2".into(),
            description: "Homeobox protein CDX-2".into(),
            epitope: "".into(),
            isotype: "IgG".into(),
            clone_name: "EPR2764Y".into(),
            lot_name: "CDX2".into(),
            dilution_type: 0,
            concentration: 3.0,
        },
    ]
}

pub async fn setup_demo_group(ctx: &Ctx, mm: &ModelManager, user_id: i32) -> Result<()> {
    let group_id = create_group(ctx, mm, user_id).await?;
    let species_id = create_species(ctx, mm, group_id).await?;
    let mut tag_ids = vec![];
    tag_ids.push(create_tag(ctx, mm, group_id, "Yb", 171).await?);
    tag_ids.push(create_tag(ctx, mm, group_id, "Pd", 105).await?);
    let panel_id = create_panel(ctx, mm, group_id, user_id).await?;
    for mut entity in get_entities() {
        entity.group_id = group_id;
        entity.user_id = user_id;
        entity.species_id = species_id;
        entity.panel_id = panel_id;
        entity.tag_id = tag_ids[entity.tag_index];
        entity.protein_id = create_protein(ctx, mm, &entity).await?;
        entity.clone_id = create_clone(ctx, mm, &entity).await?;
        entity.lot_id = create_lot(ctx, mm, &entity).await?;
        entity.conjugate_id = create_conjugate(ctx, mm, &entity).await?;
        entity.panel_element_id = create_panel_element(ctx, mm, &entity).await?;
        entity.validation_id = create_validation(ctx, mm, &entity).await?;
        entity.validation_file_id = create_validation_file(ctx, mm, &entity).await?;
    }
    Ok(())
}

async fn create_protein(ctx: &Ctx, mm: &ModelManager, entity: &Entity) -> Result<i32> {
    let fc = ProteinForCreate {
        name: entity.protein_name.clone(),
        description: Some(entity.description.clone()),
        created_by: entity.user_id,
        group_id: entity.group_id,
    };
    let id = ProteinBmc::create(ctx, mm, fc).await?;

    Ok(id)
}

async fn create_group(ctx: &Ctx, mm: &ModelManager, user_id: i32) -> Result<i32> {
    let group_c = GroupForCreate {
        name: "Demo group".into(),
        url: Some("https://example.com".into()),
        is_open: false,
        institution: "Demo institution".into(),
        tags: None,
    };
    let group_id = GroupBmc::create(ctx, mm, group_c).await?;
    let member_c = MemberForCreate {
        user_id,
        group_id,
        role: 111,
        activation_key: None,
        all_panels: true,
        is_active: true,
    };
    let _ = MemberBmc::create(ctx, mm, member_c).await?;

    println!("Demo");
    Ok(group_id)
}

async fn create_tag(
    ctx: &Ctx,
    mm: &ModelManager,
    group_id: i32,
    name: &str,
    mw: i16,
) -> Result<i32> {
    let fc = TagForCreate {
        name: name.into(),
        group_id,
        description: None,
        is_metal: true,
        is_fluorophore: false,
        is_enzyme: false,
        is_biotin: false,
        is_other: false,
        mw: Some(mw),
        emission: None,
        excitation: None,
        status: None,
    };
    let id = TagBmc::create(ctx, mm, fc).await?;

    Ok(id)
}

async fn create_species(ctx: &Ctx, mm: &ModelManager, group_id: i32) -> Result<i32> {
    let fc = SpeciesForCreate {
        name: "Rabbit".into(),
        acronym: "Rb".into(),
        group_id,
    };
    let id = SpeciesBmc::create(ctx, mm, fc).await?;

    Ok(id)
}

async fn create_clone(ctx: &Ctx, mm: &ModelManager, entity: &Entity) -> Result<i32> {
    let fc = CloneForCreate {
        name: entity.clone_name.clone(),
        group_id: entity.group_id,
        created_by: Some(entity.user_id),
        protein_id: entity.protein_id,
        species_id: Some(entity.species_id),
        epitope: entity.epitope.clone(),
        is_phospho: false,
        is_polyclonal: false,
        isotype: entity.isotype.clone(),
        is_archived: Some(false),
        reactivity: None,
        application: None,
    };
    let id = CloneBmc::create(ctx, mm, fc).await?;

    Ok(id)
}

async fn create_lot(ctx: &Ctx, mm: &ModelManager, entity: &Entity) -> Result<i32> {
    let fc = LotForCreate {
        name: entity.lot_name.clone(),
        group_id: entity.group_id,
        clone_id: entity.clone_id,
        created_by: Some(entity.user_id),
        provider_id: None,
        reference: None,
        requested_by: None,
        approved_by: None,
        ordered_by: None,
        received_by: None,
        finished_by: None,
        status: None,
        purpose: None,
        url: None,
        price: None,
        note: None,
        requested_at: None,
        ordered_at: None,
        received_at: None,
        finished_at: None,
        is_archived: Some(false),
    };
    let id = LotBmc::create(ctx, mm, fc).await?;

    Ok(id)
}

async fn create_conjugate(ctx: &Ctx, mm: &ModelManager, entity: &Entity) -> Result<i32> {
    let fc = ConjugateForCreate {
        group_id: entity.group_id,
        created_by: Some(entity.user_id),
        labeled_by: None,
        finished_by: None,
        lot_id: entity.lot_id,
        tag_id: entity.tag_id,
        status: None,
        concentration: None,
        description: None,
        finished_at: None,
        is_archived: Some(false),
        custom_id: None,
        meta: None,
    };
    let id = ConjugateBmc::create(ctx, mm, fc).await?;

    Ok(id)
}

async fn create_panel_element(ctx: &Ctx, mm: &ModelManager, entity: &Entity) -> Result<i32> {
    let fc = PanelElementForCreate {
        panel_id: entity.panel_id,
        conjugate_id: entity.conjugate_id,
        dilution_type: entity.dilution_type,
        concentration: Some(entity.concentration),
    };
    let id = PanelElementBmc::create(ctx, mm, fc).await?;

    Ok(id)
}

async fn create_panel(ctx: &Ctx, mm: &ModelManager, group_id: i32, user_id: i32) -> Result<i32> {
    let fc = PanelForCreate {
        name: Some("Antibody validation".into()),
        group_id,
        created_by: Some(user_id),
        description: None,
        is_fluorophore: false,
        is_locked: false,
        application: None,
    };
    let id = PanelBmc::create(ctx, mm, fc).await?;
    Ok(id)
}

async fn create_validation(ctx: &Ctx, mm: &ModelManager, entity: &Entity) -> Result<i32> {
    let fc = ValidationForCreate {
        group_id: entity.group_id,
        created_by: Some(entity.user_id),
        clone_id: entity.clone_id,
        lot_id: None,
        conjugate_id: None,
        species_id: None,
        application: Some(1),
        positive_control: None,
        negative_control: None,
        incubation_conditions: None,
        concentration: None,
        concentration_unit: None,
        tissue: None,
        fixation: None,
        fixation_notes: None,
        notes: None,
        status: None,
        antigen_retrieval_type: None,
        antigen_retrieval_time: None,
        antigen_retrieval_temperature: None,
        saponin: None,
        saponin_concentration: None,
        methanol_treatment: None,
        methanol_treatment_concentration: None,
        surface_staining: None,
        surface_staining_concentration: None,
        file_id: None,
        is_archived: None,
        created_at: None,
        updated_at: None,
    };
    let id = ValidationBmc::create(ctx, mm, fc).await?;

    Ok(id)
}

async fn create_validation_file(ctx: &Ctx, mm: &ModelManager, entity: &Entity) -> Result<i32> {
    let fc = ValidationFileForCreate {
        validation_id: entity.validation_id,
        created_by: entity.user_id,
        hash: String::new(),
        size: 0,
        name: None,
        extension: String::new(),
        description: None,
        created_at: chrono::Utc::now(),
    };
    let id = ValidationFileBmc::create(ctx, mm, fc).await?;
    Ok(id)
}
