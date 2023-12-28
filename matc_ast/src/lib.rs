mod expr;
mod param;
mod stmt;
mod ty;

pub use expr::Expr;
pub use param::Param;
pub use stmt::Stmt;
pub use ty::Type;

use matc_span::Spanned;

#[derive(Debug, PartialEq, Clone)]
pub struct Ast {
    pub defs: Vec<Spanned<Stmt>>,
}
