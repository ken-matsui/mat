use crate::parser::ast::{Spanned, Type};

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Expr {
    /// ||
    Or(Spanned<Self>, Spanned<Self>),

    /// &&
    And(Spanned<Self>, Spanned<Self>),

    /// <
    Lt(Spanned<Self>, Spanned<Self>),
    /// >
    Gt(Spanned<Self>, Spanned<Self>),
    /// <=
    Lte(Spanned<Self>, Spanned<Self>),
    /// >=
    Gte(Spanned<Self>, Spanned<Self>),
    /// ==
    Eq(Spanned<Self>, Spanned<Self>),
    /// !=
    Neq(Spanned<Self>, Spanned<Self>),

    /// |
    BitOr(Spanned<Self>, Spanned<Self>),

    /// ^
    BitXor(Spanned<Self>, Spanned<Self>),

    /// &
    BitAnd(Spanned<Self>, Spanned<Self>),

    /// <<
    Shl(Spanned<Self>, Spanned<Self>),
    /// >>
    Shr(Spanned<Self>, Spanned<Self>),

    /// +
    Add(Spanned<Self>, Spanned<Self>),
    /// -
    Sub(Spanned<Self>, Spanned<Self>),

    /// *
    Mul(Spanned<Self>, Spanned<Self>),
    /// /
    Div(Spanned<Self>, Spanned<Self>),
    /// %
    Rem(Spanned<Self>, Spanned<Self>),

    /// as
    As(Spanned<Self>, Spanned<Type>),

    FnCall {
        name: Spanned<Self>,
        args: Vec<Spanned<Self>>,
    },

    /// Atom
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    String(String),
    Variable(String),
}
