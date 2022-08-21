use crate::sema::entity::Entity;
use crate::sema::error::SemanticError;
use linked_hash_map::LinkedHashMap;

pub(crate) struct ToplevelScope {
    entities: LinkedHashMap<String, Entity>,
}

impl ToplevelScope {
    pub(crate) fn new(entities: LinkedHashMap<String, Entity>) -> Self {
        Self { entities }
    }
}

impl Default for ToplevelScope {
    fn default() -> Self {
        Self::new(LinkedHashMap::new())
    }
}

impl ToplevelScope {
    pub(crate) fn define_entity(&mut self, entity: Entity) -> Result<(), SemanticError> {
        if let Some(dup) = self.entities.get(&**entity.name) {
            Err(SemanticError::DuplicatedDef {
                pre_span: dup.name.span,
                span: entity.name.span,
            })
        } else {
            self.entities.insert(*entity.clone().name.value, entity);
            Ok(())
        }
    }
}
