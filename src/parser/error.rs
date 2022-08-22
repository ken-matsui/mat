use crate::parser::ast::Span;
use crate::Emit;
use ariadne::{Color, Fmt, Label, Report, ReportKind, Source, Span as _};
use chumsky::error::Simple;

pub(crate) type ParserError = Simple<char, Span>;

impl Emit for ParserError {
    fn emit(self, code: &str) {
        let report = Report::build(ReportKind::Error, self.span().src(), self.span().start());

        let report = match self.reason() {
            chumsky::error::SimpleReason::Unexpected => report
                .with_message(format!(
                    "{}{}, expected {}",
                    if self.found().is_some() {
                        "unexpected token"
                    } else {
                        "unexpected end of input"
                    },
                    if let Some(label) = self.label() {
                        format!(" while parsing {}", label.fg(Color::Green))
                    } else {
                        " something else".to_string()
                    },
                    if self.expected().count() == 0 {
                        "something else".to_string()
                    } else {
                        self.expected()
                            .map(|expected| match expected {
                                Some(expected) => expected.to_string(),
                                None => "end of input".to_string(),
                            })
                            .collect::<Vec<_>>()
                            .join(", ")
                    }
                ))
                .with_label(Label::new(self.span()).with_message(format!(
                    "Unexpected {}",
                    self.found()
                        .map(|c| format!("token {}", c.fg(Color::Red)))
                        .unwrap_or_else(|| "end of input".to_string())
                ))),
            chumsky::error::SimpleReason::Unclosed { span, delimiter } => report
                .with_message(format!(
                    "Unclosed delimiter {}",
                    delimiter.fg(Color::Yellow)
                ))
                .with_label(
                    Label::new(*span)
                        .with_message(format!(
                            "Unclosed delimiter {}",
                            delimiter.fg(Color::Yellow)
                        ))
                        .with_color(Color::Yellow),
                )
                .with_label(
                    Label::new(self.span())
                        .with_message(format!(
                            "Must be closed before this {}",
                            self.found()
                                .map(|found| found.to_string())
                                .unwrap_or_else(|| "end of input".to_string())
                                .fg(Color::Red)
                        ))
                        .with_color(Color::Red),
                ),
            chumsky::error::SimpleReason::Custom(msg) => report.with_message(msg).with_label(
                Label::new(self.span())
                    .with_message(format!("{}", msg.fg(Color::Red)))
                    .with_color(Color::Red),
            ),
        };

        report
            .finish()
            .print((self.span().src(), Source::from(code)))
            .unwrap();
    }
}

impl Emit for Vec<ParserError> {
    fn emit(self, code: &str) {
        for err in self {
            err.emit(code);
        }
    }
}
