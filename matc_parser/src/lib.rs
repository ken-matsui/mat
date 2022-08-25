mod comment;
mod compilation_unit;
pub mod error;
mod expr;
mod ident;
mod integer;
pub(crate) mod prelude;
mod stmt;
mod string;
mod ty;
mod variable;

use chumsky::{Parser, Span, Stream};
use compilation_unit::compilation_unit;
use error::Error;
use matc_ast::Ast;
use matc_span::SrcId;
use std::path::Path;

pub fn parse<P: AsRef<Path>>(src: P, code: &str) -> Result<Ast, Vec<Error>> {
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
        Err(errors)
    }
}
