use crate::parser::ast::{
    character, integer, string, variable, IntegerLiteralNode, StringLiteralNode, VariableNode,
};
use chumsky::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct CastNode {
    type_node: String, // TODO: TypeNode
    expr: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Expr {
    Integer(IntegerLiteralNode),
    String(StringLiteralNode),
    Variable(VariableNode),
    Cast(CastNode),

    /// =
    Assign(Box<Expr>, Box<Expr>),
    /// +=
    AddAssign(Box<Expr>, Box<Expr>),
    /// -=
    SubAssign(Box<Expr>, Box<Expr>),
    /// *=
    MulAssign(Box<Expr>, Box<Expr>),
    /// /=
    DivAssign(Box<Expr>, Box<Expr>),
    /// %=
    RemAssign(Box<Expr>, Box<Expr>),
    /// &=
    BitAndAssign(Box<Expr>, Box<Expr>),
    /// |=
    BitOrAssign(Box<Expr>, Box<Expr>),
    /// ^=
    BitXorAssign(Box<Expr>, Box<Expr>),
    /// <<=
    ShlAssign(Box<Expr>, Box<Expr>),
    /// >>=
    ShrAssign(Box<Expr>, Box<Expr>),

    /// ||
    Or(Box<Expr>, Box<Expr>),

    /// &&
    And(Box<Expr>, Box<Expr>),

    /// <
    Lt(Box<Expr>, Box<Expr>),
    /// >
    Gt(Box<Expr>, Box<Expr>),
    /// <=
    Lte(Box<Expr>, Box<Expr>),
    /// >=
    Gte(Box<Expr>, Box<Expr>),
    /// ==
    Eq(Box<Expr>, Box<Expr>),
    /// !=
    Neq(Box<Expr>, Box<Expr>),

    /// |
    BitOr(Box<Expr>, Box<Expr>),

    /// ^
    BitXor(Box<Expr>, Box<Expr>),

    /// &
    BitAnd(Box<Expr>, Box<Expr>),

    /// <<
    Shl(Box<Expr>, Box<Expr>),
    /// >>
    Shr(Box<Expr>, Box<Expr>),

    /// +
    Add(Box<Expr>, Box<Expr>),
    /// -
    Sub(Box<Expr>, Box<Expr>),

    /// *
    Mul(Box<Expr>, Box<Expr>),
    /// /
    Div(Box<Expr>, Box<Expr>),
    /// %
    Rem(Box<Expr>, Box<Expr>),
}

pub(crate) fn expr() -> impl Parser<char, Expr, Error = Simple<char>> + Clone {
    choice((
        term()
            .then(
                just('=')
                    .to(Expr::Assign as fn(_, _) -> _)
                    .or(just("+=").to(Expr::AddAssign as fn(_, _) -> _))
                    .or(just("-=").to(Expr::SubAssign as fn(_, _) -> _))
                    .or(just("*=").to(Expr::MulAssign as fn(_, _) -> _))
                    .or(just("/=").to(Expr::DivAssign as fn(_, _) -> _))
                    .or(just("%=").to(Expr::RemAssign as fn(_, _) -> _))
                    .or(just("&=").to(Expr::BitAndAssign as fn(_, _) -> _))
                    .or(just("|=").to(Expr::BitOrAssign as fn(_, _) -> _))
                    .or(just("^=").to(Expr::BitXorAssign as fn(_, _) -> _))
                    .or(just("<<=").to(Expr::ShlAssign as fn(_, _) -> _))
                    .or(just(">>=").to(Expr::ShrAssign as fn(_, _) -> _))
                    // Here, this is not expr() because I would not allow multiple assignments like a = b = c;
                    .then(expr9()),
            )
            .map(|(lhs, (op, rhs))| op(Box::new(lhs), Box::new(rhs))),
        expr9(),
    ))
}

pub(crate) fn expr9() -> impl Parser<char, Expr, Error = Simple<char>> + Clone {
    expr8()
        .then(
            just("||")
                .to(Expr::Or as fn(_, _) -> _)
                .then(expr8())
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
}

pub(crate) fn expr8() -> impl Parser<char, Expr, Error = Simple<char>> + Clone {
    expr7()
        .then(
            just("&&")
                .to(Expr::And as fn(_, _) -> _)
                .then(expr7())
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
}

pub(crate) fn expr7() -> impl Parser<char, Expr, Error = Simple<char>> + Clone {
    expr6()
        .then(
            just("!=")
                .to(Expr::Neq as fn(_, _) -> _)
                .or(just("==").to(Expr::Eq as fn(_, _) -> _))
                .or(just(">=").to(Expr::Gte as fn(_, _) -> _))
                .or(just("<=").to(Expr::Lte as fn(_, _) -> _))
                .or(just('>').to(Expr::Gt as fn(_, _) -> _))
                .or(just('<').to(Expr::Lt as fn(_, _) -> _))
                .then(expr6())
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
}

pub(crate) fn expr6() -> impl Parser<char, Expr, Error = Simple<char>> + Clone {
    expr5()
        .then(
            just('|')
                .to(Expr::BitOr as fn(_, _) -> _)
                .then(expr5())
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
}

pub(crate) fn expr5() -> impl Parser<char, Expr, Error = Simple<char>> + Clone {
    expr4()
        .then(
            just('^')
                .to(Expr::BitXor as fn(_, _) -> _)
                .then(expr4())
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
}

pub(crate) fn expr4() -> impl Parser<char, Expr, Error = Simple<char>> + Clone {
    expr3()
        .then(
            just('&')
                .to(Expr::BitAnd as fn(_, _) -> _)
                .then(expr3())
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
}

pub(crate) fn expr3() -> impl Parser<char, Expr, Error = Simple<char>> + Clone {
    expr2()
        .then(
            just("<<")
                .to(Expr::Shl as fn(_, _) -> _)
                .or(just(">>").to(Expr::Shr as fn(_, _) -> _))
                .then(expr2())
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
}

pub(crate) fn expr2() -> impl Parser<char, Expr, Error = Simple<char>> + Clone {
    expr1()
        .then(
            just('+')
                .to(Expr::Add as fn(_, _) -> _)
                .or(just('-').to(Expr::Sub as fn(_, _) -> _))
                .then(expr1())
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
}

pub(crate) fn expr1() -> impl Parser<char, Expr, Error = Simple<char>> + Clone {
    term()
        .then(
            just('*')
                .to(Expr::Mul as fn(_, _) -> _)
                .or(just('/').to(Expr::Div as fn(_, _) -> _))
                .or(just('%').to(Expr::Rem as fn(_, _) -> _))
                .then(term())
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
}

pub(crate) fn term() -> impl Parser<char, Expr, Error = Simple<char>> + Clone {
    suffix()
    // TODO: ident -> type
    // (text::ident()
    //     .delimited_by(just("("), just(")"))
    //     .padded()
    //     .then(term())
    //     .map(|(ty, expr)| {
    //         Cast(CastNode {
    //             type_node: ty.to_string(),
    //             expr: Box::new(expr),
    //         })
    //     }))
    // .or(suffix())
}

pub(crate) fn suffix() -> impl Parser<char, Expr, Error = Simple<char>> + Clone {
    primary()
    // or(FnCall)
}

pub(crate) fn primary() -> impl Parser<char, Expr, Error = Simple<char>> + Clone {
    choice((
        integer().map(Expr::Integer),
        character().map(Expr::Integer),
        string().map(Expr::String),
        variable().map(Expr::Variable),
    ))
    .padded()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;

    fn big_expr() -> Expr {
        Expr::Or(
            Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
            Box::from(Expr::And(
                Box::from(Expr::Integer(IntegerLiteralNode::I32(2))),
                Box::from(Expr::Neq(
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(3))),
                    Box::from(Expr::BitOr(
                        Box::from(Expr::Integer(IntegerLiteralNode::I32(4))),
                        Box::from(Expr::BitXor(
                            Box::from(Expr::Integer(IntegerLiteralNode::I32(5))),
                            Box::from(Expr::BitAnd(
                                Box::from(Expr::Integer(IntegerLiteralNode::I32(6))),
                                Box::from(Expr::Shl(
                                    Box::from(Expr::Integer(IntegerLiteralNode::I32(7))),
                                    Box::from(Expr::Add(
                                        Box::from(Expr::Integer(IntegerLiteralNode::I32(8))),
                                        Box::from(Expr::Mul(
                                            Box::from(Expr::Integer(IntegerLiteralNode::I32(9))),
                                            Box::from(Expr::Integer(IntegerLiteralNode::I32(10))),
                                        )),
                                    )),
                                )),
                            )),
                        )),
                    )),
                )),
            )),
        )
    }
    #[test]
    fn expr_test1() {
        assert_eq!(
            expr().parse("var = 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10"),
            Ok(Expr::Assign(
                Box::from(Expr::Variable(VariableNode("var".to_string()))),
                Box::from(big_expr()),
            ))
        );
    }
    #[test]
    fn expr_test2() {
        assert_eq!(
            expr().parse("var += 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10"),
            Ok(Expr::AddAssign(
                Box::from(Expr::Variable(VariableNode("var".to_string()))),
                Box::from(big_expr()),
            ))
        );
    }
    #[test]
    fn expr_test3() {
        assert_eq!(
            expr().parse("var -= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10"),
            Ok(Expr::SubAssign(
                Box::from(Expr::Variable(VariableNode("var".to_string()))),
                Box::from(big_expr()),
            ))
        );
    }
    #[test]
    fn expr_test4() {
        assert_eq!(
            expr().parse("var *= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10"),
            Ok(Expr::MulAssign(
                Box::from(Expr::Variable(VariableNode("var".to_string()))),
                Box::from(big_expr()),
            ))
        );
    }
    #[test]
    fn expr_test5() {
        assert_eq!(
            expr().parse("var /= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10"),
            Ok(Expr::DivAssign(
                Box::from(Expr::Variable(VariableNode("var".to_string()))),
                Box::from(big_expr()),
            ))
        );
    }
    #[test]
    fn expr_test6() {
        assert_eq!(
            expr().parse("var %= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10"),
            Ok(Expr::RemAssign(
                Box::from(Expr::Variable(VariableNode("var".to_string()))),
                Box::from(big_expr()),
            ))
        );
    }
    #[test]
    fn expr_test7() {
        assert_eq!(
            expr().parse("var &= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10"),
            Ok(Expr::BitAndAssign(
                Box::from(Expr::Variable(VariableNode("var".to_string()))),
                Box::from(big_expr()),
            ))
        );
    }
    #[test]
    fn expr_test8() {
        assert_eq!(
            expr().parse("var |= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10"),
            Ok(Expr::BitOrAssign(
                Box::from(Expr::Variable(VariableNode("var".to_string()))),
                Box::from(big_expr()),
            ))
        );
    }
    #[test]
    fn expr_test9() {
        assert_eq!(
            expr().parse("var ^= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10"),
            Ok(Expr::BitXorAssign(
                Box::from(Expr::Variable(VariableNode("var".to_string()))),
                Box::from(big_expr()),
            ))
        );
    }
    #[test]
    fn expr_test10() {
        assert_eq!(
            expr().parse("var <<= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10"),
            Ok(Expr::ShlAssign(
                Box::from(Expr::Variable(VariableNode("var".to_string()))),
                Box::from(big_expr()),
            ))
        );
    }
    #[test]
    fn expr_test11() {
        assert_eq!(
            expr().parse("var >>= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10"),
            Ok(Expr::ShrAssign(
                Box::from(Expr::Variable(VariableNode("var".to_string()))),
                Box::from(big_expr()),
            ))
        );
    }
    #[test]
    fn expr_test12() {
        assert_eq!(
            expr().parse("1"),
            Ok(Expr::Integer(IntegerLiteralNode::I32(1)))
        );
    }

    #[test]
    fn expr9_test() {
        assert_eq!(
            expr9().parse("1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10"),
            Ok(Expr::Or(
                Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
                Box::from(Expr::And(
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(Expr::Neq(
                        Box::from(Expr::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(Expr::BitOr(
                            Box::from(Expr::Integer(IntegerLiteralNode::I32(4))),
                            Box::from(Expr::BitXor(
                                Box::from(Expr::Integer(IntegerLiteralNode::I32(5))),
                                Box::from(Expr::BitAnd(
                                    Box::from(Expr::Integer(IntegerLiteralNode::I32(6))),
                                    Box::from(Expr::Shl(
                                        Box::from(Expr::Integer(IntegerLiteralNode::I32(7))),
                                        Box::from(Expr::Add(
                                            Box::from(Expr::Integer(IntegerLiteralNode::I32(8))),
                                            Box::from(Expr::Mul(
                                                Box::from(Expr::Integer(IntegerLiteralNode::I32(
                                                    9
                                                ))),
                                                Box::from(Expr::Integer(IntegerLiteralNode::I32(
                                                    10
                                                )))
                                            )),
                                        )),
                                    )),
                                )),
                            )),
                        )),
                    )),
                )),
            ))
        );

        assert_eq!(
            expr9().parse("1"),
            Ok(Expr::Integer(IntegerLiteralNode::I32(1)))
        );
    }

    #[test]
    fn expr8_test() {
        assert_eq!(
            expr8().parse("1 && 2 != 3 | 4 ^ 5 & 6 << 7 + 8*9"),
            Ok(Expr::And(
                Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
                Box::from(Expr::Neq(
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(Expr::BitOr(
                        Box::from(Expr::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(Expr::BitXor(
                            Box::from(Expr::Integer(IntegerLiteralNode::I32(4))),
                            Box::from(Expr::BitAnd(
                                Box::from(Expr::Integer(IntegerLiteralNode::I32(5))),
                                Box::from(Expr::Shl(
                                    Box::from(Expr::Integer(IntegerLiteralNode::I32(6))),
                                    Box::from(Expr::Add(
                                        Box::from(Expr::Integer(IntegerLiteralNode::I32(7))),
                                        Box::from(Expr::Mul(
                                            Box::from(Expr::Integer(IntegerLiteralNode::I32(8))),
                                            Box::from(Expr::Integer(IntegerLiteralNode::I32(9)))
                                        )),
                                    )),
                                )),
                            )),
                        )),
                    )),
                )),
            ))
        );

        assert_eq!(
            expr8().parse("1"),
            Ok(Expr::Integer(IntegerLiteralNode::I32(1)))
        );
    }

    #[test]
    fn expr7_test() {
        assert_eq!(
            expr7().parse("1 != 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Expr::Neq(
                Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
                Box::from(Expr::BitOr(
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(Expr::BitXor(
                        Box::from(Expr::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(Expr::BitAnd(
                            Box::from(Expr::Integer(IntegerLiteralNode::I32(4))),
                            Box::from(Expr::Shl(
                                Box::from(Expr::Integer(IntegerLiteralNode::I32(5))),
                                Box::from(Expr::Add(
                                    Box::from(Expr::Integer(IntegerLiteralNode::I32(6))),
                                    Box::from(Expr::Mul(
                                        Box::from(Expr::Integer(IntegerLiteralNode::I32(7))),
                                        Box::from(Expr::Integer(IntegerLiteralNode::I32(8)))
                                    )),
                                )),
                            )),
                        )),
                    )),
                )),
            ))
        );

        assert_eq!(
            expr7().parse("1 == 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Expr::Eq(
                Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
                Box::from(Expr::BitOr(
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(Expr::BitXor(
                        Box::from(Expr::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(Expr::BitAnd(
                            Box::from(Expr::Integer(IntegerLiteralNode::I32(4))),
                            Box::from(Expr::Shl(
                                Box::from(Expr::Integer(IntegerLiteralNode::I32(5))),
                                Box::from(Expr::Add(
                                    Box::from(Expr::Integer(IntegerLiteralNode::I32(6))),
                                    Box::from(Expr::Mul(
                                        Box::from(Expr::Integer(IntegerLiteralNode::I32(7))),
                                        Box::from(Expr::Integer(IntegerLiteralNode::I32(8)))
                                    )),
                                )),
                            )),
                        )),
                    )),
                )),
            ))
        );

        assert_eq!(
            expr7().parse("1 >= 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Expr::Gte(
                Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
                Box::from(Expr::BitOr(
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(Expr::BitXor(
                        Box::from(Expr::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(Expr::BitAnd(
                            Box::from(Expr::Integer(IntegerLiteralNode::I32(4))),
                            Box::from(Expr::Shl(
                                Box::from(Expr::Integer(IntegerLiteralNode::I32(5))),
                                Box::from(Expr::Add(
                                    Box::from(Expr::Integer(IntegerLiteralNode::I32(6))),
                                    Box::from(Expr::Mul(
                                        Box::from(Expr::Integer(IntegerLiteralNode::I32(7))),
                                        Box::from(Expr::Integer(IntegerLiteralNode::I32(8)))
                                    )),
                                )),
                            )),
                        )),
                    )),
                )),
            ))
        );

        assert_eq!(
            expr7().parse("1 <= 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Expr::Lte(
                Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
                Box::from(Expr::BitOr(
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(Expr::BitXor(
                        Box::from(Expr::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(Expr::BitAnd(
                            Box::from(Expr::Integer(IntegerLiteralNode::I32(4))),
                            Box::from(Expr::Shl(
                                Box::from(Expr::Integer(IntegerLiteralNode::I32(5))),
                                Box::from(Expr::Add(
                                    Box::from(Expr::Integer(IntegerLiteralNode::I32(6))),
                                    Box::from(Expr::Mul(
                                        Box::from(Expr::Integer(IntegerLiteralNode::I32(7))),
                                        Box::from(Expr::Integer(IntegerLiteralNode::I32(8)))
                                    )),
                                )),
                            )),
                        )),
                    )),
                )),
            ))
        );

        assert_eq!(
            expr7().parse("1 > 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Expr::Gt(
                Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
                Box::from(Expr::BitOr(
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(Expr::BitXor(
                        Box::from(Expr::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(Expr::BitAnd(
                            Box::from(Expr::Integer(IntegerLiteralNode::I32(4))),
                            Box::from(Expr::Shl(
                                Box::from(Expr::Integer(IntegerLiteralNode::I32(5))),
                                Box::from(Expr::Add(
                                    Box::from(Expr::Integer(IntegerLiteralNode::I32(6))),
                                    Box::from(Expr::Mul(
                                        Box::from(Expr::Integer(IntegerLiteralNode::I32(7))),
                                        Box::from(Expr::Integer(IntegerLiteralNode::I32(8)))
                                    )),
                                )),
                            )),
                        )),
                    )),
                )),
            ))
        );

        assert_eq!(
            expr7().parse("1 < 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Expr::Lt(
                Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
                Box::from(Expr::BitOr(
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(Expr::BitXor(
                        Box::from(Expr::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(Expr::BitAnd(
                            Box::from(Expr::Integer(IntegerLiteralNode::I32(4))),
                            Box::from(Expr::Shl(
                                Box::from(Expr::Integer(IntegerLiteralNode::I32(5))),
                                Box::from(Expr::Add(
                                    Box::from(Expr::Integer(IntegerLiteralNode::I32(6))),
                                    Box::from(Expr::Mul(
                                        Box::from(Expr::Integer(IntegerLiteralNode::I32(7))),
                                        Box::from(Expr::Integer(IntegerLiteralNode::I32(8)))
                                    )),
                                )),
                            )),
                        )),
                    )),
                )),
            ))
        );

        assert_eq!(
            expr7().parse("1"),
            Ok(Expr::Integer(IntegerLiteralNode::I32(1)))
        );
    }

    #[test]
    fn expr6_test() {
        assert_eq!(
            expr6().parse("1 | 2 ^ 3 & 4 << 5 + 6*7"),
            Ok(Expr::BitOr(
                Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
                Box::from(Expr::BitXor(
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(Expr::BitAnd(
                        Box::from(Expr::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(Expr::Shl(
                            Box::from(Expr::Integer(IntegerLiteralNode::I32(4))),
                            Box::from(Expr::Add(
                                Box::from(Expr::Integer(IntegerLiteralNode::I32(5))),
                                Box::from(Expr::Mul(
                                    Box::from(Expr::Integer(IntegerLiteralNode::I32(6))),
                                    Box::from(Expr::Integer(IntegerLiteralNode::I32(7)))
                                )),
                            )),
                        )),
                    )),
                )),
            ))
        );

        assert_eq!(
            expr6().parse("1"),
            Ok(Expr::Integer(IntegerLiteralNode::I32(1)))
        );
    }

    #[test]
    fn expr5_test() {
        assert_eq!(
            expr5().parse("1 ^ 2 & 3 << 4 + 5*6"),
            Ok(Expr::BitXor(
                Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
                Box::from(Expr::BitAnd(
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(Expr::Shl(
                        Box::from(Expr::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(Expr::Add(
                            Box::from(Expr::Integer(IntegerLiteralNode::I32(4))),
                            Box::from(Expr::Mul(
                                Box::from(Expr::Integer(IntegerLiteralNode::I32(5))),
                                Box::from(Expr::Integer(IntegerLiteralNode::I32(6)))
                            )),
                        )),
                    )),
                )),
            ))
        );

        assert_eq!(
            expr5().parse("1"),
            Ok(Expr::Integer(IntegerLiteralNode::I32(1)))
        );
    }

    #[test]
    fn expr4_test() {
        assert_eq!(
            expr4().parse("1 & 2 << 3 + 4*5"),
            Ok(Expr::BitAnd(
                Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
                Box::from(Expr::Shl(
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(Expr::Add(
                        Box::from(Expr::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(Expr::Mul(
                            Box::from(Expr::Integer(IntegerLiteralNode::I32(4))),
                            Box::from(Expr::Integer(IntegerLiteralNode::I32(5)))
                        )),
                    )),
                )),
            ))
        );

        assert_eq!(
            expr4().parse("1"),
            Ok(Expr::Integer(IntegerLiteralNode::I32(1)))
        );
    }

    #[test]
    fn expr3_test() {
        assert_eq!(
            expr3().parse("1 << 2 + 3*4"),
            Ok(Expr::Shl(
                Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
                Box::from(Expr::Add(
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(Expr::Mul(
                        Box::from(Expr::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(Expr::Integer(IntegerLiteralNode::I32(4)))
                    )),
                )),
            ))
        );

        assert_eq!(
            expr3().parse("1 >> 2 + 3*4"),
            Ok(Expr::Shr(
                Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
                Box::from(Expr::Add(
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(Expr::Mul(
                        Box::from(Expr::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(Expr::Integer(IntegerLiteralNode::I32(4)))
                    )),
                )),
            ))
        );

        assert_eq!(
            expr3().parse("1"),
            Ok(Expr::Integer(IntegerLiteralNode::I32(1)))
        );
    }

    #[test]
    fn expr2_test() {
        assert_eq!(
            expr2().parse("1 + 2*3"),
            Ok(Expr::Add(
                Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
                Box::from(Expr::Mul(
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(3)))
                )),
            ))
        );

        assert_eq!(
            expr2().parse("1 - 2*3"),
            Ok(Expr::Sub(
                Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
                Box::from(Expr::Mul(
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(3)))
                )),
            ))
        );

        assert_eq!(
            expr2().parse("1*2 + 3*4"),
            Ok(Expr::Add(
                Box::from(Expr::Mul(
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(2)))
                )),
                Box::from(Expr::Mul(
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(3))),
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(4)))
                )),
            ))
        );

        assert_eq!(
            expr2().parse("1"),
            Ok(Expr::Integer(IntegerLiteralNode::I32(1)))
        );
    }

    #[test]
    fn expr1_test() {
        assert_eq!(
            expr1().parse("1*1"),
            Ok(Expr::Mul(
                Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
                Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
            ))
        );
        assert_eq!(
            expr1().parse("1 / 1"),
            Ok(Expr::Div(
                Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
                Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
            ))
        );
        assert_eq!(
            expr1().parse("1 %2"),
            Ok(Expr::Rem(
                Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
                Box::from(Expr::Integer(IntegerLiteralNode::I32(2))),
            ))
        );

        assert_eq!(
            expr1().parse("1 % 2 / 3 * 4"),
            Ok(Expr::Mul(
                Box::from(Expr::Div(
                    Box::from(Expr::Rem(
                        Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
                        Box::from(Expr::Integer(IntegerLiteralNode::I32(2)))
                    )),
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(3)))
                )),
                Box::from(Expr::Integer(IntegerLiteralNode::I32(4))),
            ))
        );

        assert_eq!(
            expr1().parse("1"),
            Ok(Expr::Integer(IntegerLiteralNode::I32(1)))
        );
    }

    #[test]
    fn primary_test() {
        assert_eq!(
            primary().parse("1"),
            Ok(Expr::Integer(IntegerLiteralNode::I32(1)))
        );
        assert_eq!(
            primary().parse("'a'"),
            Ok(Expr::Integer(IntegerLiteralNode::I8(97)))
        );
        assert_eq!(
            primary().parse("\"a\""),
            Ok(Expr::String(StringLiteralNode("a".to_string())))
        );
        assert_eq!(
            primary().parse("var"),
            Ok(Expr::Variable(VariableNode("var".to_string())))
        );
    }
}
