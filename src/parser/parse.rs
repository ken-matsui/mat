use crate::parser::ast::{compilation_unit, Ast, SrcId};
use crate::parser::lib::ParserError;
use chumsky::{Parser, Span, Stream};

pub(crate) fn parse(src: SrcId, code: &str) -> Result<Ast, Vec<ParserError>> {
    let len = code.chars().count();
    let span = |i| Span::new(src, i..i + 1);
    let eoi = span(len);

    let (ast, errors) = compilation_unit().parse_recovery(Stream::from_iter(
        eoi,
        code.chars().enumerate().map(|(i, c)| (c, span(i))),
    ));

    if let Some(ast) = ast {
        Ok(ast)
    } else {
        Err(errors)
    }
}
