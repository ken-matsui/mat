use crate::parser::ast::comment;
use chumsky::prelude::*;

pub(crate) fn ident() -> impl Parser<char, String, Error = Simple<char>> + Clone {
    text::ident()
        .padded()
        .padded_by(comment().padded().repeated())
}
