mod comment;
mod compilation_unit;
pub mod error;
mod expr;
mod ident;
mod integer;
mod stmt;
mod string;
mod ty;
mod variable;

pub(crate) use chumsky::prelude::*;
pub(crate) use chumsky::Parser as _;

use chumsky::{Span, Stream};
use compilation_unit::compilation_unit;
use error::Error;
use matc_ast::Ast;
use matc_span::SrcId;
use std::path::Path;

// trait alias under stable version
pub(crate) trait Parser<T>: chumsky::Parser<char, T, Error = Error> + Clone {
    #[cfg(test)]
    fn parse_test(&self, stream: &str) -> Result<T, Vec<Self::Error>> {
        use matc_span::Span;

        let len = stream.chars().count();
        let span = |i| Span::new(SrcId::any(), i..i + 1);

        self.parse(chumsky::Stream::from_iter(
            span(len),
            stream.chars().enumerate().map(|(i, c)| (c, span(i))),
        ))
    }
}
impl<S, T> Parser<T> for S where S: chumsky::Parser<char, T, Error = Error> + Clone {}

pub(crate) type Rec<'a, T> = Recursive<'a, char, T, Error>;

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
