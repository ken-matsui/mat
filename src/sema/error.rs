use crate::error::Emit;
use crate::parser::ast::Span;
use ariadne::{Color, Fmt, Label, Report, ReportKind, Source, Span as _};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum SemanticError {
    DuplicatedDef(Span, Span),
    UnresolvedRef(Span),
}

impl Emit for SemanticError {
    fn emit(self, code: &str) {
        match self {
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
        }
        .unwrap();
    }
}

impl Emit for Vec<SemanticError> {
    fn emit(self, code: &str) {
        for err in self {
            err.emit(code);
        }
    }
}
