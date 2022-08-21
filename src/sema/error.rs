use crate::parser::ast::Span;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum SemanticError {
    DuplicatedDef { pre_span: Span, span: Span },
    UnresolvedRef { span: Span },
}
