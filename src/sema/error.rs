use crate::parser::ast::Span;

pub(crate) enum SemanticError {
    DuplicatedDef { pre_span: Span, span: Span },
}
