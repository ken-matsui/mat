use crate::parser::ast::{compilation_unit, Ast};
use chumsky::prelude::{Parser, Simple};

pub type Span = std::ops::Range<usize>;

pub(crate) fn parse(src: String) -> (Option<Ast>, Vec<Simple<char>>) {
    compilation_unit().parse_recovery(src)
}
