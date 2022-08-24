mod checker;
mod dereference_checker;
pub(crate) mod entity;
mod error;
mod local_resolver;
mod resolver;
pub(crate) mod scope;
mod type_resolver;
mod type_table;
mod visitor;

use crate::hir::Hir;
use crate::parser::ast::Ast;
use crate::sema::checker::Checker;
use crate::sema::dereference_checker::DereferenceChecker;
use crate::sema::type_table::TypeTable;
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

    let mut type_table = TypeTable::new();
    let diag = TypeResolver::new(&mut type_table).resolve(&mut hir);
    diag.warnings.emit(code);
    if diag.has_err() {
        return Err(diag.errors);
    }

    let diag = type_table.semantic_check();
    diag.warnings.emit(code);
    if diag.has_err() {
        return Err(diag.errors);
    }

    let diag = DereferenceChecker::new(&type_table, &hir).check();
    diag.warnings.emit(code);
    if diag.has_err() {
        return Err(diag.errors);
    }

    Ok(hir)
}
