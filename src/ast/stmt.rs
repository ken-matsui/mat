use crate::ast::{Expr, Param, Type};
use matc_span::Spanned;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Stmt {
    Empty,

    Import(String),

    DefFn {
        name: Spanned<String>,
        args: Vec<Param>,
        ret_ty: Spanned<Type>,
        body: Spanned<Self>,
    },

    DefVar {
        is_mut: bool,
        name: Spanned<String>,
        ty: Spanned<Type>,
        expr: Option<Spanned<Expr>>,
    },

    TypeDef {
        name: Spanned<String>,
        ty: Spanned<Type>,
    },

    Block(Vec<Spanned<Self>>),

    If {
        cond: Spanned<Expr>,
        then: Spanned<Self>,
        els: Option<Spanned<Self>>,
    },

    Return(Option<Spanned<Expr>>),

    /// =
    Assign(Spanned<Expr>, Spanned<Expr>),
    /// +=
    AddAssign(Spanned<Expr>, Spanned<Expr>),
    /// -=
    SubAssign(Spanned<Expr>, Spanned<Expr>),
    /// *=
    MulAssign(Spanned<Expr>, Spanned<Expr>),
    /// /=
    DivAssign(Spanned<Expr>, Spanned<Expr>),
    /// %=
    RemAssign(Spanned<Expr>, Spanned<Expr>),
    /// &=
    BitAndAssign(Spanned<Expr>, Spanned<Expr>),
    /// |=
    BitOrAssign(Spanned<Expr>, Spanned<Expr>),
    /// ^=
    BitXorAssign(Spanned<Expr>, Spanned<Expr>),
    /// <<=
    ShlAssign(Spanned<Expr>, Spanned<Expr>),
    /// >>=
    ShrAssign(Spanned<Expr>, Spanned<Expr>),

    Expr(Spanned<Expr>),
}
