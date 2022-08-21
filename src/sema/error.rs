use crate::parser::ast::Span;

#[derive(Debug, Clone)]
pub(crate) enum SemanticError {
    DuplicatedDef { pre_span: Span, span: Span },
}
