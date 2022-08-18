mod comment;
mod expr;
mod ident;
mod integer;
mod stmt;
mod string;
mod ty;
mod variable;

pub(crate) use comment::*;
pub(crate) use expr::*;
pub(crate) use ident::*;
pub(crate) use integer::*;
pub(crate) use stmt::*;
pub(crate) use string::*;
pub(crate) use ty::*;
pub(crate) use variable::*;

use chumsky::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Ast {
    imports: Vec<Stmt>,
    defs: Vec<Stmt>,
}

pub(crate) fn compilation_unit() -> impl Parser<char, Ast, Error = Simple<char>> + Clone {
    import_stmt()
        .repeated()
        .then(top_defs())
        .then_ignore(end())
        .map(|(imports, defs)| Ast { imports, defs })
}
