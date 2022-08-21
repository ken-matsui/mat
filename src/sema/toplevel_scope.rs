use crate::parser::ast::Span;
use crate::sema::entity::Entity;
use crate::sema::error::SemanticError;
use crate::sema::local_scope::LocalScope;
use crate::sema::scope::Scope;
use linked_hash_map::LinkedHashMap;

#[derive(Debug, Clone)]
pub(crate) struct ToplevelScope {
    entities: LinkedHashMap<String, Entity>,
    children: Vec<LocalScope>,
}

impl ToplevelScope {
    pub(crate) fn new() -> Self {
        Self {
            entities: LinkedHashMap::new(),
            children: Vec::new(),
        }
    }
}

impl Scope for ToplevelScope {
    fn is_toplevel() -> bool {
        true
    }
    fn toplevel(&self) -> ToplevelScope {
        self.clone()
    }
    fn parent(&self) -> Option<Box<dyn Scope>> {
        None
    }

    fn add_child(&mut self, s: LocalScope) {
        self.children.push(s);
    }

    fn get_mut(&mut self, name: &str, span: Span) -> Result<&mut Entity, SemanticError> {
        self.entities
            .get_mut(name)
            .ok_or(SemanticError::UnresolvedRef { span })
    }
}

impl ToplevelScope {
    pub(crate) fn define_entity(&mut self, entity: Entity) -> Result<(), SemanticError> {
        if let Some(dup) = self
            .entities
            .insert(*entity.clone().name.value, entity.clone())
        {
            Err(SemanticError::DuplicatedDef {
                pre_span: dup.name.span,
                span: entity.name.span,
            })
        } else {
            Ok(())
        }
    }
}
