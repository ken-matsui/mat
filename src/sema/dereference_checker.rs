use crate::hir::{DefinedVariable, Hir};
use crate::parser::ast::{Expr, Spanned};
use crate::sema::checker::Checker;
use crate::sema::error::{SemanticDiag, SemanticError};
use crate::sema::type_table::TypeTable;
use std::ops::Deref;

pub(crate) struct DereferenceChecker<'a> {
    type_table: &'a TypeTable,
    diag: SemanticDiag,
}

impl<'a> DereferenceChecker<'a> {
    pub(crate) fn new(type_table: &'a TypeTable) -> Self {
        Self {
            type_table,
            diag: SemanticDiag::new(),
        }
    }
}

impl Checker for DereferenceChecker<'_> {
    fn check(&mut self, hir: &mut Hir) -> SemanticDiag {
        for var in hir.defined_variables() {
            self.check_toplevel_variable(var);
        }

        self.diag.clone()
    }
}

impl DereferenceChecker<'_> {
    // Toplevel variables should be constants
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

    fn visit_expr(&mut self, expr: &Spanned<Expr>) -> Result<(), SemanticError> {
        // match expr.deref() {
        //     Expr::FnCall { name, .. } => {}
        // };
        Ok(())
    }
}

// TODO: test(not_constant.mat)
