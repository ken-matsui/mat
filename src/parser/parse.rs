use crate::parser::ast::{compilation_unit, Ast};
use crate::parser::lib::ParserError;
use chumsky::prelude::Parser;

pub(crate) fn parse(src: String) -> (Option<Ast>, Vec<ParserError>) {
    compilation_unit().parse_recovery(src)
}
