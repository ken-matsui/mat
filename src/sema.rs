mod entity;
mod error;
mod local_resolver;
mod resolver;
mod scope;
mod type_resolver;
mod visitor;

use crate::parser::ast::Ast;
use crate::Emit;
use error::SemanticError;
use local_resolver::LocalResolver;
use resolver::Resolver;
use type_resolver::TypeResolver;

pub(crate) fn analyze(ast: &Ast, code: &str) -> Result<(), Vec<SemanticError>> {
    let diag = LocalResolver::new().resolve(ast);
    diag.warnings.emit(code);
    if diag.is_err() {
        return Err(diag.errors);
    }

    let diag = TypeResolver::new().resolve(ast);
    diag.warnings.emit(code);
    if diag.is_err() {
        return Err(diag.errors);
    }

    Ok(())
}
