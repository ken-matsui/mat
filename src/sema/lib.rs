use crate::parser::ast::Ast;
use crate::sema::error::SemanticError;
use crate::LocalResolver;

pub(crate) fn analyze(ast: &Ast) -> Result<(), Vec<SemanticError>> {
    LocalResolver::new().resolve(&ast)
    // .and_then()
}
