use crate::parser::ast::Span;
use crate::sema::entity::Entity;
use crate::sema::error::{SemanticDiag, SemanticError, SemanticWarning};
use linked_hash_map::LinkedHashMap;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(crate) struct Scope {
    parent: Option<Rc<RefCell<Self>>>,
    // Toplevel has DefVars & DefFns, otherwise, only DefVars will be held.
    entities: LinkedHashMap<String, Entity>, // TODO: DefinedVariable?
    children: Vec<Rc<RefCell<Self>>>,
}

/// Impls for All Scope
impl Scope {
    pub(crate) fn new(parent: Option<Rc<RefCell<Self>>>) -> Rc<RefCell<Self>> {
        if let Some(parent) = parent {
            let this = Rc::new(RefCell::new(Self {
                parent: Some(parent.clone()),
                entities: LinkedHashMap::new(),
                children: Vec::new(),
            }));
            parent.borrow_mut().add_child(this.clone());
            this
        } else {
            Rc::new(RefCell::new(Self {
                parent,
                entities: LinkedHashMap::new(),
                children: Vec::new(),
            }))
        }
    }

    fn parent(&self) -> Option<Rc<RefCell<Self>>> {
        self.parent.clone()
    }

    fn add_child(&mut self, s: Rc<RefCell<Self>>) {
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

    pub(crate) fn check_references(&self, diag: &mut SemanticDiag) {
        for ent in self.entities.values() {
            if !ent.is_referred() {
                diag.push_warn(SemanticWarning::UnusedEntity(ent.name.span));
            }
        }

        // do not check parameters
        for func_scope in &self.children {
            for scope in &func_scope.borrow().children {
                scope.borrow().check_references(diag);
            }
        }
    }
}
