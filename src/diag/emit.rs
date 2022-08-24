use crate::parser::ast::Span;
use ariadne::{Label, Report, ReportKind, Source, Span as _};

pub(crate) trait Emit {
    fn emit(&self, code: &str);
    fn count(&self) -> usize {
        1
    }
}

impl<T: Emit> Emit for Vec<T> {
    fn emit(&self, code: &str) {
        for emitter in self {
            emitter.emit(code);
        }
    }

    fn count(&self) -> usize {
        self.len()
    }
}

pub(crate) fn emit<N: ToString>(
    code: &str,
    span: Span,
    message: &str,
    labels: Vec<Label<Span>>,
    notes: Vec<N>,
) {
    let mut report =
        Report::build(ReportKind::Error, span.src(), span.start()).with_message(message);
    for label in labels {
        report = report.with_label(label);
    }
    for note in notes {
        report = report.with_note(note);
    }
    report
        .finish()
        .print((span.src(), Source::from(code)))
        .unwrap();
}
