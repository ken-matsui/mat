use crate::hir::{DefinedVariable, Hir};
use crate::parser::ast::{Expr, Spanned, Stmt};
use crate::sema::checker::Checker;
use crate::sema::error::{SemanticDiag, SemanticError};
use crate::sema::type_table::TypeTable;
use std::ops::Deref;

pub(crate) struct DereferenceChecker<'a> {
    type_table: &'a TypeTable,
    hir: &'a Hir,
    diag: SemanticDiag,
}

impl<'a> DereferenceChecker<'a> {
    pub(crate) fn new(type_table: &'a TypeTable, hir: &'a Hir) -> Self {
        Self {
            type_table,
            hir,
            diag: SemanticDiag::new(),
        }
    }
}

impl Checker for DereferenceChecker<'_> {
    fn check(&mut self) -> SemanticDiag {
        for var in self.hir.defined_variables() {
            self.check_toplevel_variable(var);
        }

        self.diag.clone()
    }
}

impl DereferenceChecker<'_> {
    // Toplevel variables should be constants
    // TODO: test(not_constant.mat)
    fn check_toplevel_variable(&mut self, var: DefinedVariable) {
        self.check_variable(&var);
        if let Err(span) = var.is_constant() {
            self.diag.push_err(SemanticError::NotConstant(span));
        }
    }

    fn check_variable(&mut self, var: &DefinedVariable) {
        if let Some(expr) = var.expr {
            if let Err(err) = self.visit_expr(expr) {
                self.diag.push_err(err);
            }
        }
    }

    fn visit_expr(&self, expr: &Spanned<Expr>) -> Result<(), SemanticError> {
        match expr.deref() {
            Expr::FnCall { name, .. } => {
                if !self.is_callable(name) {
                    // TODO: test(not_callable.mat)
                    Err(SemanticError::NotCallable(name.span))
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
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
