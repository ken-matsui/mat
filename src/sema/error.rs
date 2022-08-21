use crate::parser::ast::Span;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum SemanticError {
    DuplicatedDef(Span, Span),
    UnresolvedRef(Span),
}
