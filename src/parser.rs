mod comment;
pub(crate) mod compilation_unit;
mod diag;
mod expr;
mod ident;
mod integer;
mod lib;
mod stmt;
mod string;
mod ty;
mod variable;

pub(crate) use crate::parser::comment::*;
pub(crate) use crate::parser::expr::*;
pub(crate) use crate::parser::ident::*;
pub(crate) use crate::parser::integer::*;
pub(crate) use crate::parser::stmt::*;
pub(crate) use crate::parser::string::*;
pub(crate) use crate::parser::ty::*;
pub(crate) use crate::parser::variable::*;

pub(crate) use diag::Error;

use crate::diag::Emit;
use chumsky::{Parser, Span, Stream};
use compilation_unit::compilation_unit;
use matc_ast::Ast;
use matc_span::SrcId;
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
