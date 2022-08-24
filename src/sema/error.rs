use crate::diagnostics::{emit, Diagnostics, Emit};
use crate::parser::ast::Span;
use ariadne::{Color, Fmt, Label, Report, ReportKind, Source, Span as _};
use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum SemanticWarning {
    // LocalResolver
    UnusedEntity(Span),
    // TypeResolver
}

impl Emit for SemanticWarning {
    fn emit(&self, code: &str) {
        match *self {
            SemanticWarning::UnusedEntity(span) => {
                Report::build(ReportKind::Warning, span.src(), span.start())
                    .with_message("Unused entity")
                    .with_label(Label::new(span).with_color(Color::Yellow))
                    .finish()
                    .print((span.src(), Source::from(code)))
            }
        }
        .unwrap();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum SemanticError {
    // LocalResolver
    DuplicatedDef(Span, Span),
    UnresolvedRef(Span),
    // TypeResolver
    DuplicatedType(Span, Span),
    UnresolvedType(Span),
    // type_table.semantic_check
    RecursiveTypeDef(Span, Span),
    // DereferenceChecker
    NotConstant(Span),
    NotCallable(Span),
}

impl Emit for SemanticError {
    fn emit(&self, code: &str) {
        let (span, message, labels, notes) = match *self {
            SemanticError::DuplicatedDef(pre_span, span) => (
                span,
                "Duplicated definition",
                vec![
                    Label::new(pre_span)
                        .with_message("previous definition".fg(Color::Blue))
                        .with_color(Color::Blue),
                    Label::new(span)
                        .with_message("redefined here".fg(Color::Red))
                        .with_color(Color::Red),
                ],
                vec![],
            ),
            SemanticError::UnresolvedRef(span) => (
                span,
                "Unresolved reference",
                vec![Label::new(span)
                    .with_message("undefined ident".fg(Color::Red))
                    .with_color(Color::Red)],
                vec![],
            ),
            SemanticError::DuplicatedType(pre_span, span) => (
                span,
                "Duplicated type",
                vec![
                    Label::new(pre_span)
                        .with_message("previous definition".fg(Color::Blue))
                        .with_color(Color::Blue),
                    Label::new(span)
                        .with_message("redefined here".fg(Color::Red))
                        .with_color(Color::Red),
                ],
                vec![],
            ),
            SemanticError::UnresolvedType(span) => (
                span,
                "Unresolved type",
                vec![Label::new(span)
                    .with_message("undefined ident".fg(Color::Red))
                    .with_color(Color::Red)],
                vec![],
            ),
            SemanticError::RecursiveTypeDef(pre_span, span) => (
                span,
                "Recursive type definition",
                vec![
                    Label::new(pre_span)
                        .with_message("previous definition".fg(Color::Blue))
                        .with_color(Color::Blue),
                    Label::new(span)
                        .with_message("redefined here".fg(Color::Red))
                        .with_color(Color::Red),
                ],
                vec![],
            ),
            SemanticError::NotConstant(span) => (
                span,
                "Not a constant",
                vec![Label::new(span)
                    .with_message("this is not a constant".fg(Color::Red))
                    .with_color(Color::Red)],
                vec!["toplevel definitions should be constants".fg(Color::Blue)],
            ),
            SemanticError::NotCallable(span) => (
                span,
                "Not callable",
                vec![Label::new(span)
                    .with_message("this is not a function".fg(Color::Red))
                    .with_color(Color::Red)],
                vec![],
            ),
        };
        emit(code, span, message, labels, notes);
    }
}

pub(crate) type SemanticDiag = Diagnostics<SemanticWarning, SemanticError>;
