use crate::parser::ast::{Ast, Spanned, Stmt, Type};
use crate::sema::entity::Entity;
use crate::sema::scope::Scope;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(crate) struct Hir {
    pub(crate) imports: Vec<Spanned<Stmt>>,
    pub(crate) defs: Vec<Spanned<Stmt>>,
    pub(crate) scope: Option<Rc<RefCell<Scope>>>,
}

impl From<Ast> for Hir {
    fn from(ast: Ast) -> Self {
        Self {
            imports: ast.imports,
            defs: ast.defs,
            scope: None,
        }
    }
}

impl Hir {
    pub(crate) fn set_scope(&mut self, scope: Rc<RefCell<Scope>>) {
        self.scope = Some(scope);
    }

    pub(crate) fn definitions(&self) -> Vec<Entity> {
        let mut entities = Vec::<Entity>::new();

        for stmt in &self.defs {
            // Convert DefVar & DefFn into Entities and define the entity.
            if let Ok(entity) = Entity::try_from(*stmt.value.clone()) {
                entities.push(entity);
            }
        }

        entities
    }

    pub(crate) fn types(&self) -> Vec<TypeDef> {
        let mut types = Vec::<TypeDef>::new();

        for stmt in &self.defs {
            match stmt.deref() {
                Stmt::TypeDef { name, ty } => {
                    types.push(TypeDef { name, ty });
                }
                // TODO: Stmt::DefStruct
                _ => {}
            }
        }

        types
    }

    #[cfg(test)]
    pub(crate) fn from_defs(defs: Vec<Spanned<Stmt>>) -> Self {
        Self {
            imports: Vec::new(),
            defs,
            scope: None,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TypeDef<'a> {
    pub(crate) name: &'a Spanned<String>,
    pub(crate) ty: &'a Spanned<Type>,
}
