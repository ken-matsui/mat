use crate::parser::ast::{compilation_unit, AST};
use chumsky::prelude::*;

pub type Span = std::ops::Range<usize>;

pub(crate) fn parse(src: String) -> Result<AST, Vec<Simple<char>>> {
    let single_line_comment = just::<_, _, Simple<char>>("//")
        .then(take_until(text::newline()))
        .ignored();

    let multi_line_comment = just::<_, _, Simple<char>>("/*")
        .then(take_until(just("*/")))
        .ignored();

    let comments = single_line_comment.or(multi_line_comment);

    compilation_unit()
        .padded_by(comments.padded().repeated())
        .parse(src)
}
