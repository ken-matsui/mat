pub(crate) mod ast;
mod diag;
mod lib;

use crate::diag::Emit;
use ast::{compilation_unit, Ast, SrcId};
use chumsky::{Parser, Span, Stream};
use std::path::Path;

pub(crate) fn parse<P: AsRef<Path>>(src: P, code: &str) -> Result<Ast, Box<dyn Emit>> {
    let src = SrcId::from_path(src);
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
        Err(Box::new(errors))
    }
}
