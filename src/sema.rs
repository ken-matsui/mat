mod dereference_checker;
mod diag;
pub(crate) mod entity;
mod local_resolver;
pub(crate) mod scope;
mod type_resolver;
mod type_table;
mod visitor;

use crate::diag::Emit;
use crate::hir::Hir;
use crate::sema::diag::Diagnostics;
use dereference_checker::DereferenceChecker;
use local_resolver::LocalResolver;
use matc_ast::Ast;
use type_resolver::TypeResolver;
use type_table::TypeTable;

pub(crate) fn analyze(ast: Ast, code: &str) -> Result<Hir, Box<dyn Emit>> {
    let mut hir = Hir::from(ast);
    let handle_diag = |diag: Diagnostics| -> Result<(), Box<dyn Emit>> {
        diag.warnings.emit(code);
        if diag.has_err() {
            Err(Box::new(diag.errors))
        } else {
            Ok(())
        }
    };

    handle_diag(LocalResolver::new().resolve(&mut hir))?;
    let mut type_table = TypeTable::new();
    handle_diag(TypeResolver::new(&mut type_table).resolve(&mut hir))?;
    handle_diag(type_table.semantic_check())?;
    handle_diag(DereferenceChecker::new(&type_table, &hir).check())?;

    Ok(hir)
}
