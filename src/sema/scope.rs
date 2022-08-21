use crate::parser::ast::Span;
use crate::sema::entity::Entity;
use crate::SemanticError;
use linked_hash_map::LinkedHashMap;

#[derive(Debug, Clone)]
pub(crate) struct Scope {
    parent: Option<Box<Self>>,
    // Toplevel has DefVars & DefFns, otherwise, only DefVars will be held.
    entities: LinkedHashMap<String, Entity>, // TODO: DefinedVariable?
    children: Vec<Self>,
}

impl Scope {
    pub(crate) fn new(parent: Option<Box<Self>>) -> Self {
        Self {
            parent,
            entities: LinkedHashMap::new(),
            children: Vec::new(),
        }
    }

    fn parent(&self) -> Option<Box<Self>> {
        self.parent.clone()
    }
    fn parent_mut(&mut self) -> Option<&mut Box<Self>> {
        self.parent.as_mut()
    }

    pub(crate) fn add_child(&mut self, s: Self) {
        self.children.push(s);
    }

    pub(crate) fn refer(&mut self, name: &str, span: Span) -> Result<(), SemanticError> {
        if let Some(var) = self.entities.get_mut(name) {
            var.referred();
            Ok(())
        } else {
            // Find the variable on the upper scope until toplevel
            self.parent_mut()
                .ok_or(SemanticError::UnresolvedRef { span })
                .and_then(|parent| parent.refer(name, span))
        }
    }
}

/// Impls for Toplevel Scope
impl Scope {
    pub(crate) fn define_entity(&mut self, entity: Entity) -> Result<(), SemanticError> {
        assert!(self.parent.is_none());

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
