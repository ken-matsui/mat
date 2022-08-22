pub(crate) mod entity;
pub(crate) mod error;
pub(crate) mod local_resolver;
pub(crate) mod scope;
pub(crate) mod visitor;

use crate::parser::ast::Ast;
use crate::Emit;
use error::SemanticError;
use local_resolver::LocalResolver;

pub(crate) fn analyze(ast: &Ast, code: &str) -> Result<(), Vec<SemanticError>> {
    let diag = LocalResolver::new().resolve(ast);
    diag.warnings.emit(code);
    if diag.is_err() {
        return Err(diag.errors);
    }
    // .and_then()

    Ok(())
}
