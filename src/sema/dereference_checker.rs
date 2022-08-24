use crate::ast::expr::Expr;
use crate::hir::{DefinedVariable, Hir};
use crate::parser::ast::{Spanned, Stmt};
use crate::sema::diag::{Diagnostics, Error};
use crate::sema::type_table::TypeTable;
use std::ops::Deref;

pub(crate) struct DereferenceChecker<'a> {
    type_table: &'a TypeTable,
    hir: &'a Hir,
    diag: Diagnostics,
}

impl<'a> DereferenceChecker<'a> {
    pub(crate) fn new(type_table: &'a TypeTable, hir: &'a Hir) -> Self {
        Self {
            type_table,
            hir,
            diag: Diagnostics::new(),
        }
    }

    pub(crate) fn check(&mut self) -> Diagnostics {
        for var in self.hir.defined_variables() {
            self.check_toplevel_variable(var);
        }
        for fun in self.hir.defined_functions() {
            if let Err(err) = self.visit_stmt(fun.body) {
                self.diag.push_err(err);
            }
        }

        self.diag.clone()
    }

    // Toplevel variables should be constants
    // TODO: test(not_constant.mat)
    fn check_toplevel_variable(&mut self, var: DefinedVariable) {
        self.check_variable(&var);
        if let Err(span) = var.is_constant() {
            self.diag.push_err(Error::NotConstant(span));
        }
    }

    fn check_variable(&mut self, var: &DefinedVariable) {
        if let Some(expr) = var.expr {
            if let Err(err) = self.visit_expr(expr) {
                self.diag.push_err(err);
            }
        }
    }

    fn visit_stmt(&mut self, stmt: &Spanned<Stmt>) -> Result<(), Error> {
        match stmt.deref() {
            Stmt::DefVar {
                is_mut, name, expr, ..
            } => {
                self.check_variable(&DefinedVariable {
                    is_mut: *is_mut,
                    name,
                    expr,
                });
                if let Some(expr) = expr {
                    self.visit_expr(expr)?;
                }
            }
            Stmt::Block(stmts) => {
                for stmt in stmts {
                    self.visit_stmt(stmt)?;
                }
            }
            // TODO: Check assignablity to lhs of `Stmt::*Assign`
            _ => {}
        }
        Ok(())
    }

    // TODO: test(not_callable.mat)
    fn visit_expr(&self, expr: &Spanned<Expr>) -> Result<(), Error> {
        match expr.deref() {
            Expr::FnCall { name, args } => {
                for arg in args {
                    self.visit_expr(arg)?;
                }
                if !self.is_callable(name) {
                    return Err(Error::NotCallable(name.span));
                }
            }
            Expr::Or(lhs, rhs)
            | Expr::And(lhs, rhs)
            | Expr::Lt(lhs, rhs)
            | Expr::Gt(lhs, rhs)
            | Expr::Lte(lhs, rhs)
            | Expr::Gte(lhs, rhs)
            | Expr::Eq(lhs, rhs)
            | Expr::Neq(lhs, rhs)
            | Expr::BitOr(lhs, rhs)
            | Expr::BitXor(lhs, rhs)
            | Expr::BitAnd(lhs, rhs)
            | Expr::Shl(lhs, rhs)
            | Expr::Shr(lhs, rhs)
            | Expr::Add(lhs, rhs)
            | Expr::Sub(lhs, rhs)
            | Expr::Mul(lhs, rhs)
            | Expr::Div(lhs, rhs)
            | Expr::Rem(lhs, rhs) => {
                self.visit_expr(rhs)?;
                self.visit_expr(lhs)?;
            }
            Expr::As(expr, _) => {
                self.visit_expr(expr)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn is_callable(&self, expr: &Spanned<Expr>) -> bool {
        if let Expr::Variable(var_name) = expr.deref() {
            for def in &self.hir.defs {
                if let Stmt::DefFn { name, .. } = def.deref() {
                    if var_name == name.deref() {
                        return true;
                    }
                }
            }
        }
        false
    }
}
