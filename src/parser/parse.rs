use crate::parser::ast::{compilation_unit, Ast, Spanned};
use crate::parser::lib::ParserError;
use chumsky::prelude::Parser;

pub(crate) fn parse(src: String) -> (Option<Spanned<Ast>>, Vec<ParserError>) {
    compilation_unit().parse_recovery(src)
}
