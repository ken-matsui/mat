mod entity;
mod error;
mod local_resolver;
mod resolver;
pub(crate) mod scope;
mod type_resolver;
mod visitor;

use crate::hir::lib::Hir;
use crate::parser::ast::Ast;
use crate::Emit;
use error::SemanticError;
use local_resolver::LocalResolver;
use resolver::Resolver;
use type_resolver::TypeResolver;

pub(crate) fn analyze(ast: Ast, code: &str) -> Result<Hir, Vec<SemanticError>> {
    let mut hir = Hir::from(ast);

    let diag = LocalResolver::new().resolve(&mut hir);
    diag.warnings.emit(code);
    if diag.has_err() {
        return Err(diag.errors);
    }

    let diag = TypeResolver::new().resolve(&mut hir);
    diag.warnings.emit(code);
    if diag.has_err() {
        return Err(diag.errors);
    }

    Ok(hir)
}
