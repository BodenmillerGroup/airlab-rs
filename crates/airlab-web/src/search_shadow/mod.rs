pub mod basic;
pub mod clone;
pub mod registry;

use airlab_lib::model::ModelManager;

pub use self::registry::SearchShadowRegistry;

#[derive(Clone)]
pub struct SearchState {
    pub mm: ModelManager,
    pub registry: SearchShadowRegistry,
}

impl SearchState {
    pub fn new(mm: ModelManager) -> Self {
        Self {
            mm,
            registry: SearchShadowRegistry::new(),
        }
    }
}
