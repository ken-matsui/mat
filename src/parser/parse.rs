use crate::parser::ast::{compilation_unit, Ast, Spanned};
use chumsky::prelude::{Parser, Simple};

pub(crate) fn parse(src: String) -> (Option<Spanned<Ast>>, Vec<Simple<char>>) {
    compilation_unit().parse_recovery(src)
}
