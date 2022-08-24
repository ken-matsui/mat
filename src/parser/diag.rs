use crate::diag::{emit, Emit};
use crate::parser::ast::Span;
use ariadne::{Color, Fmt, Label};
use chumsky::error::Simple;

pub(crate) type Error = Simple<char, Span>;

impl Emit for Error {
    fn emit(&self, code: &str) {
        let span = self.span();
        let (message, labels) = match self.reason() {
            chumsky::error::SimpleReason::Unexpected => (
                format!(
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
                ),
                vec![Label::new(self.span()).with_message(format!(
                    "Unexpected {}",
                    self.found()
                        .map(|c| format!("token {}", c.fg(Color::Red)))
                        .unwrap_or_else(|| "end of input".to_string())
                ))],
            ),
            chumsky::error::SimpleReason::Unclosed { span, delimiter } => (
                format!("Unclosed delimiter {}", delimiter.fg(Color::Yellow)),
                vec![
                    Label::new(*span)
                        .with_message(format!(
                            "Unclosed delimiter {}",
                            delimiter.fg(Color::Yellow)
                        ))
                        .with_color(Color::Yellow),
                    Label::new(self.span())
                        .with_message(format!(
                            "Must be closed before this {}",
                            self.found()
                                .map(|found| found.to_string())
                                .unwrap_or_else(|| "end of input".to_string())
                                .fg(Color::Red)
                        ))
                        .with_color(Color::Red),
                ],
            ),
            chumsky::error::SimpleReason::Custom(msg) => (
                msg.clone(),
                vec![Label::new(self.span())
                    .with_message(format!("{}", msg.fg(Color::Red)))
                    .with_color(Color::Red)],
            ),
        };
        emit(code, span, message, labels, vec![]);
    }
}
