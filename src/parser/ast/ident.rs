use crate::parser::ast::comment;
use crate::parser::lib::*;

pub(crate) fn ident() -> impl Parser<String> {
    text::ident()
        .padded()
        .padded_by(comment().padded().repeated())
        .boxed()
}
