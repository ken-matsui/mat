use crate::parser::ast::Span;
use crate::sema::entity::Entity;
use crate::sema::error::SemanticError;
use linked_hash_map::LinkedHashMap;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(crate) struct Scope {
    parent: Option<Rc<RefCell<Self>>>,
    // Toplevel has DefVars & DefFns, otherwise, only DefVars will be held.
    entities: LinkedHashMap<String, Entity>, // TODO: DefinedVariable?
    children: Vec<Self>,
}

/// Impls for All Scope
impl Scope {
    pub(crate) fn new(parent: Option<Rc<RefCell<Self>>>) -> Self {
        Self {
            parent,
            entities: LinkedHashMap::new(),
            children: Vec::new(),
        }
    }

    fn parent(&self) -> Option<Rc<RefCell<Self>>> {
        self.parent.clone()
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
            self.parent()
                .ok_or(SemanticError::UnresolvedRef(span))
                .and_then(|parent| parent.borrow_mut().refer(name, span))
        }
    }

    pub(crate) fn define_entity(&mut self, entity: Entity) -> Result<(), SemanticError> {
        if let Some(dup) = self
            .entities
            .insert(*entity.clone().name.value, entity.clone())
        {
            Err(SemanticError::DuplicatedDef(
                dup.name.span,
                entity.name.span,
            ))
        } else {
            Ok(())
        }
    }
}

/// Impls for Local Scope
impl Scope {}
