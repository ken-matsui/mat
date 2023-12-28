mod dereference_checker;
mod diag;
pub(crate) mod entity;
mod local_resolver;
pub(crate) mod scope;
mod visitor;

use crate::diag::Emit;
use crate::hir::Hir;
use crate::sema::diag::Diagnostics;
use dereference_checker::DereferenceChecker;
use local_resolver::LocalResolver;
use matc_ast::Ast;

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
    handle_diag(DereferenceChecker::new(&hir).check())?;

    Ok(hir)
}
