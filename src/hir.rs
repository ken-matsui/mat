use crate::sema::entity::Entity;
use crate::sema::scope::Scope;
use matc_ast::{Ast, Expr, Stmt, Type};
use matc_span::{Span, Spanned};
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
        assert_eq!(self.scope, None, "must not happen: Scope is set twice");
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

    pub(crate) fn defined_variables(&self) -> Vec<DefinedVariable> {
        let mut defvars = Vec::<DefinedVariable>::new();

        for stmt in &self.defs {
            if let Stmt::DefVar {
                is_mut, name, expr, ..
            } = stmt.deref()
            {
                defvars.push(DefinedVariable {
                    is_mut: *is_mut,
                    name,
                    expr,
                });
            }
        }

        defvars
    }

    pub(crate) fn defined_functions(&self) -> Vec<DefinedFunction> {
        let mut functions = Vec::<DefinedFunction>::new();

        for stmt in &self.defs {
            if let Stmt::DefFn { body, .. } = stmt.deref() {
                functions.push(DefinedFunction { body })
            }
        }

        functions
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

#[derive(Debug, Clone)]
pub(crate) struct DefinedVariable<'a> {
    pub(crate) is_mut: bool,
    pub(crate) name: &'a Spanned<String>,
    pub(crate) expr: &'a Option<Spanned<Expr>>,
}

impl<'a> DefinedVariable<'a> {
    pub(crate) fn is_constant(&self) -> Result<(), Span> {
        if self.is_mut {
            return Err(self.name.span);
        } else if let Some(expr) = self.expr {
            if !expr.value.is_constant() {
                return Err(expr.span);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DefinedFunction<'a> {
    pub(crate) body: &'a Spanned<Stmt>,
}
