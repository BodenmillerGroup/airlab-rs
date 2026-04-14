use super::basic::{BasicGroupShadow, BasicShadowKind, build_basic_shadow};
use super::clone::{CloneGroupShadow, build_clone_group_shadow};
use airlab_lib::ctx::Ctx;
use airlab_lib::model::ModelManager;
use airlab_lib::model::group::{Group, GroupBmc};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::warn;

#[derive(Clone, Default)]
pub struct SearchShadowRegistry {
    basic_by_kind: Arc<RwLock<HashMap<BasicShadowKind, Arc<BasicGroupShadow>>>>,
    clone_by_group: Arc<RwLock<HashMap<i64, Arc<CloneGroupShadow>>>>,
}

impl SearchShadowRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_clone_shadow(&self, group_id: i64) -> Option<Arc<CloneGroupShadow>> {
        self.clone_by_group
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(&group_id)
            .cloned()
    }

    pub fn get_basic_shadow(&self, kind: BasicShadowKind) -> Option<Arc<BasicGroupShadow>> {
        self.basic_by_kind
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(&kind)
            .cloned()
    }

    pub async fn get_or_build_basic_shadow(
        &self,
        mm: &ModelManager,
        kind: BasicShadowKind,
    ) -> airlab_lib::model::Result<Arc<BasicGroupShadow>> {
        if let Some(shadow) = self.get_basic_shadow(kind) {
            warn!(
                kind = kind.label(),
                row_count = shadow.rows.len(),
                "basic shadow table cache hit"
            );
            return Ok(shadow);
        }

        warn!(
            kind = kind.label(),
            "basic shadow table missing from cache; rebuilding"
        );
        self.rebuild_basic_shadow(mm, kind).await
    }

    pub async fn rebuild_basic_shadow(
        &self,
        mm: &ModelManager,
        kind: BasicShadowKind,
    ) -> airlab_lib::model::Result<Arc<BasicGroupShadow>> {
        warn!(kind = kind.label(), "building basic shadow table");
        let shadow = Arc::new(build_basic_shadow(mm, kind).await?);
        warn!(
            kind = kind.label(),
            row_count = shadow.rows.len(),
            "basic shadow table loaded"
        );
        self.basic_by_kind
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(kind, shadow.clone());
        Ok(shadow)
    }

    pub async fn get_or_build_clone_shadow(
        &self,
        mm: &ModelManager,
        group_id: i64,
    ) -> airlab_lib::model::Result<Arc<CloneGroupShadow>> {
        if let Some(shadow) = self.get_clone_shadow(group_id) {
            warn!(
                group_id,
                row_count = shadow.rows.len(),
                "clone shadow table cache hit"
            );
            return Ok(shadow);
        }

        warn!(
            group_id,
            "clone shadow table missing from cache; rebuilding"
        );
        self.rebuild_clone_shadow(mm, group_id).await
    }

    pub async fn rebuild_clone_shadow(
        &self,
        mm: &ModelManager,
        group_id: i64,
    ) -> airlab_lib::model::Result<Arc<CloneGroupShadow>> {
        warn!(group_id, "building clone shadow table");
        let shadow = Arc::new(build_clone_group_shadow(mm, group_id).await?);
        warn!(
            group_id,
            row_count = shadow.rows.len(),
            "clone shadow table loaded"
        );
        self.clone_by_group
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(group_id, shadow.clone());
        Ok(shadow)
    }

    pub async fn rebuild_all(&self, mm: &ModelManager) -> airlab_lib::model::Result<()> {
        for kind in BasicShadowKind::ALL {
            self.rebuild_basic_shadow(mm, kind).await?;
        }

        let ctx = Ctx::root_ctx();
        let groups: Vec<Group> = GroupBmc::list(&ctx, mm, None, None).await?;
        for group in groups {
            self.rebuild_clone_shadow(mm, group.id).await?;
        }

        Ok(())
    }
}
