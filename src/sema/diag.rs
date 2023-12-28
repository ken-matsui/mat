use crate::diag::{emit, Diagnostics as Diag, Emit};
use ariadne::{Color, Fmt, Label, Report, ReportKind, Source, Span as _};
use matc_span::Span;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Warning {
    // LocalResolver
    UnusedEntity(Span),
    // TypeResolver
}

impl Emit for Warning {
    fn emit(&self, code: &str) {
        match *self {
            Warning::UnusedEntity(span) => {
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
pub(crate) enum Error {
    // LocalResolver
    DuplicatedDef(Span, Span),
    UnresolvedRef(Span),
    // DereferenceChecker
    NotConstant(Span),
    NotCallable(Span),
}

impl Emit for Error {
    fn emit(&self, code: &str) {
        let (span, message, labels, notes) = match *self {
            Error::DuplicatedDef(pre_span, span) => (
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
            Error::UnresolvedRef(span) => (
                span,
                "Unresolved reference",
                vec![Label::new(span)
                    .with_message("undefined ident".fg(Color::Red))
                    .with_color(Color::Red)],
                vec![],
            ),
            Error::NotConstant(span) => (
                span,
                "Not a constant",
                vec![Label::new(span)
                    .with_message("this is not a constant".fg(Color::Red))
                    .with_color(Color::Red)],
                vec!["toplevel definitions should be constants".fg(Color::Blue)],
            ),
            Error::NotCallable(span) => (
                span,
                "Not callable",
                vec![Label::new(span)
                    .with_message("this is not a function".fg(Color::Red))
                    .with_color(Color::Red)],
                vec![],
            ),
        };
        emit(
            code,
            span,
            message.to_string(),
            labels,
            notes.iter().map(ToString::to_string).collect(),
        );
    }
}

pub(crate) type Diagnostics = Diag<Warning, Error>;
