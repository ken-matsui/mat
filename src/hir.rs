use crate::parser::ast::{Ast, Spanned, Stmt};
use crate::sema::scope::Scope;
use std::cell::RefCell;
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

    #[cfg(test)]
    pub(crate) fn from_defs(defs: Vec<Spanned<Stmt>>) -> Self {
        Self {
            imports: Vec::new(),
            defs,
            scope: None,
        }
    }
}
