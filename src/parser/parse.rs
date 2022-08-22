use crate::parser::ast::{compilation_unit, Ast, SrcId};
use crate::parser::lib::ParserError;
use chumsky::{Parser, Span, Stream};

pub(crate) fn parse(src: SrcId, code: String) -> (Option<Ast>, Vec<ParserError>) {
    let len = code.chars().count();
    let span = |i| Span::new(src, i..i + 1);
    let eoi = span(len);

    compilation_unit().parse_recovery(Stream::from_iter(
        eoi,
        code.chars().enumerate().map(|(i, c)| (c, span(i))),
    ))
}
