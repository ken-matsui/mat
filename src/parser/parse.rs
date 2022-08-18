use crate::parser::ast::{compilation_unit, AST};
use chumsky::prelude::{Parser, Simple};

pub type Span = std::ops::Range<usize>;

pub(crate) fn parse(src: String) -> Result<AST, Vec<Simple<char>>> {
    compilation_unit().parse(src)
}
