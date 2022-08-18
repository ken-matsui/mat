mod expr;
mod integer;
mod stmt;
mod string;
mod ty;
mod variable;

pub(crate) use expr::*;
pub(crate) use integer::*;
pub(crate) use stmt::*;
pub(crate) use string::*;
pub(crate) use ty::*;
pub(crate) use variable::*;

use chumsky::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct AST {
    imports: Vec<Stmt>,
    defs: Stmt,
}

pub(crate) fn compilation_unit() -> impl Parser<char, AST, Error = Simple<char>> + Clone {
    import_stmt()
        .repeated()
        .then(top_defs())
        .then_ignore(end())
        .map(|(imports, defs)| AST { imports, defs })
}
