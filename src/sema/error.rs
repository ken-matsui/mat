use crate::error::Emit;
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

impl Emit for Vec<SemanticWarning> {
    fn emit(&self, code: &str) {
        for warn in self {
            warn.emit(code);
        }
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
}

impl Emit for SemanticError {
    fn emit(&self, code: &str) {
        match *self {
            SemanticError::DuplicatedDef(pre_span, span) => {
                Report::build(ReportKind::Error, span.src(), span.start())
                    .with_message("Duplicated definition")
                    .with_label(
                        Label::new(pre_span)
                            .with_message("previous definition".fg(Color::Blue))
                            .with_color(Color::Blue),
                    )
                    .with_label(
                        Label::new(span)
                            .with_message("redefined here".fg(Color::Red))
                            .with_color(Color::Red),
                    )
                    .finish()
                    .print((span.src(), Source::from(code)))
            }
            SemanticError::UnresolvedRef(span) => {
                Report::build(ReportKind::Error, span.src(), span.start())
                    .with_message("Unresolved reference")
                    .with_label(
                        Label::new(span)
                            .with_message("undefined ident".fg(Color::Red))
                            .with_color(Color::Red),
                    )
                    .finish()
                    .print((span.src(), Source::from(code)))
            }
            SemanticError::DuplicatedType(pre_span, span) => {
                // TODO: too similar to DuplicatedDef
                Report::build(ReportKind::Error, span.src(), span.start())
                    .with_message("Duplicated type")
                    .with_label(
                        Label::new(pre_span)
                            .with_message("previous definition".fg(Color::Blue))
                            .with_color(Color::Blue),
                    )
                    .with_label(
                        Label::new(span)
                            .with_message("redefined here".fg(Color::Red))
                            .with_color(Color::Red),
                    )
                    .finish()
                    .print((span.src(), Source::from(code)))
            }
            SemanticError::UnresolvedType(span) => {
                // TODO: too similar to UnresolvedRef
                Report::build(ReportKind::Error, span.src(), span.start())
                    .with_message("Unresolved type")
                    .with_label(
                        Label::new(span)
                            .with_message("undefined ident".fg(Color::Red))
                            .with_color(Color::Red),
                    )
                    .finish()
                    .print((span.src(), Source::from(code)))
            }
            SemanticError::RecursiveTypeDef(pre_span, span) => {
                // TODO: too similar to DuplicatedDef
                Report::build(ReportKind::Error, span.src(), span.start())
                    .with_message("Recursive type definition")
                    .with_label(
                        Label::new(pre_span)
                            .with_message("previous definition".fg(Color::Blue))
                            .with_color(Color::Blue),
                    )
                    .with_label(
                        Label::new(span)
                            .with_message("redefined here".fg(Color::Red))
                            .with_color(Color::Red),
                    )
                    .finish()
                    .print((span.src(), Source::from(code)))
            }
        }
        .unwrap();
    }
}

impl Emit for Vec<SemanticError> {
    fn emit(&self, code: &str) {
        for err in self {
            err.emit(code);
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Diagnostics<W: Emit, E: Emit> {
    pub(crate) warnings: Vec<W>,
    pub(crate) errors: Vec<E>,
}

impl<W, E> Emit for Diagnostics<W, E>
where
    W: Emit,
    Vec<W>: Emit,
    E: Emit,
    Vec<E>: Emit,
{
    fn emit(&self, code: &str) {
        self.warnings.emit(code);
        self.errors.emit(code);
    }
}

impl<W: Emit, E: Emit> Diagnostics<W, E> {
    pub(crate) fn new() -> Self {
        Self {
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub(crate) fn has_err(&self) -> bool {
        !self.errors.is_empty()
    }

    pub(crate) fn push_warn(&mut self, warn: W) {
        self.warnings.push(warn);
    }
    pub(crate) fn push_err(&mut self, err: E) {
        self.errors.push(err);
    }
}

pub(crate) type SemanticDiag = Diagnostics<SemanticWarning, SemanticError>;
