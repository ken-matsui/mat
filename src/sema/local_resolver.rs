/// Local Resolver
use crate::parser::ast::{Ast, Expr, Spanned, Stmt};
use crate::sema::entity::Entity;
use crate::sema::error::SemanticError;
use crate::sema::scope::Scope;
use crate::sema::toplevel_scope::ToplevelScope;
use std::collections::LinkedList;
use std::ops::Deref;

pub(crate) struct LocalResolver {
    scope_stack: LinkedList<Box<dyn Scope>>,
    errors: Vec<SemanticError>,
}

impl LocalResolver {
    pub(crate) fn new() -> Self {
        Self {
            scope_stack: LinkedList::new(),
            errors: Vec::new(),
        }
    }
}

impl LocalResolver {
    pub(crate) fn resolve(&mut self, ast: Ast) -> Result<(), Vec<SemanticError>> {
        // toplevel scope
        let mut toplevel = Box::new(ToplevelScope::new());

        self.define_entities(&ast, &mut toplevel);
        self.scope_stack.push_back(toplevel);
        self.resolve_gvar_initializers(&ast);
        // toplevel scope end

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn define_entities(&mut self, ast: &Ast, toplevel: &mut Box<ToplevelScope>) {
        for stmt in &ast.defs {
            // Convert DefVar & DefFn into Entities and define the entity.
            if let Ok(entity) = Entity::try_from(*stmt.value.clone()) {
                if let Err(err) = toplevel.define_entity(entity) {
                    self.errors.push(err);
                }
            }
        }
    }

    fn resolve_gvar_initializers(&mut self, ast: &Ast) {
        for stmt in &ast.defs {
            if let Stmt::DefVar { expr, .. } = stmt.deref() {
                if let Some(expr) = expr.deref() {
                    self.resolve_variable(expr);
                    self.resolve_string(expr);
                }
            }
        }
    }

    fn resolve_variable(&mut self, expr: &Spanned<Expr>) {
        match expr.deref() {
            Expr::Variable(var) => match self.current_scope().get_mut(var, expr.span) {
                Ok(entity) => {
                    entity.referred();
                    // TODO: node.setEntity(ent);
                }
                Err(err) => self.errors.push(err),
            },
            Expr::Or(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::And(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Lt(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Gt(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Lte(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Gte(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Eq(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Neq(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::BitOr(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::BitXor(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::BitAnd(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Shl(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Shr(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Add(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Sub(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Mul(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Div(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Rem(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::As(lhs, _ty) => {
                self.resolve_variable(lhs);
            }
            Expr::FnCall { name, args } => {
                self.resolve_variable(name);
                let _ = args.iter().map(|arg| self.resolve_variable(arg));
            }
            _ => (),
        }
    }

    fn resolve_string(&self, expr: &Spanned<Expr>) {
        if let Expr::String(str) = expr.deref() {}
    }

    fn current_scope(&mut self) -> &mut Box<dyn Scope> {
        self.scope_stack.back_mut().unwrap()
    }
}
