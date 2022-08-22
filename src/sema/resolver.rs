use crate::parser::ast::Ast;
use crate::sema::error::SemanticDiag;

pub(crate) trait Resolver {
    fn resolve(&mut self, ast: &Ast) -> SemanticDiag;
}
