use crate::Type;
use matc_span::Spanned;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
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
    I32(i32),
    String(String),
    Variable(String),
}

impl Expr {
    pub fn is_constant(&self) -> bool {
        matches!(self, Expr::I8(_) | Expr::I32(_) | Expr::String(_))
    }
}
