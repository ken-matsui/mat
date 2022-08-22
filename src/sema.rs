pub(crate) mod entity;
pub(crate) mod error;
pub(crate) mod local_resolver;
pub(crate) mod scope;
pub(crate) mod visitor;

use crate::parser::ast::Ast;
use error::SemanticError;
use local_resolver::LocalResolver;

pub(crate) fn analyze(ast: &Ast) -> Result<(), Vec<SemanticError>> {
    LocalResolver::new().resolve(ast)
    // .and_then()
}
