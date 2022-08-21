use crate::parser::ast::Span;
use std::ops::Range;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum SemanticError {
    DuplicatedDef(Span, Span),
    UnresolvedRef(Span),
}
