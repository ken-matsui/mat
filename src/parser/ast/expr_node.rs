use crate::parser::ast::{
    character, integer, string, variable, IntegerLiteralNode, StringLiteralNode, VariableNode,
};
use chumsky::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct CastNode {
    type_node: String, // TODO: TypeNode
    expr: Box<ExprNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum ExprNode {
    Integer(IntegerLiteralNode),
    String(StringLiteralNode),
    Variable(VariableNode),
    Cast(CastNode),

    /// +=
    AddAssign(Box<ExprNode>, Box<ExprNode>),
    /// -=
    SubAssign(Box<ExprNode>, Box<ExprNode>),
    /// *=
    MulAssign(Box<ExprNode>, Box<ExprNode>),
    /// /=
    DivAssign(Box<ExprNode>, Box<ExprNode>),
    /// %=
    RemAssign(Box<ExprNode>, Box<ExprNode>),
    /// &=
    BitAndAssign(Box<ExprNode>, Box<ExprNode>),
    /// |=
    BitOrAssign(Box<ExprNode>, Box<ExprNode>),
    /// ^=
    BitXorAssign(Box<ExprNode>, Box<ExprNode>),
    /// <<=
    ShlAssign(Box<ExprNode>, Box<ExprNode>),
    /// >>=
    ShrAssign(Box<ExprNode>, Box<ExprNode>),

    /// ||
    Or(Box<ExprNode>, Box<ExprNode>),

    /// &&
    And(Box<ExprNode>, Box<ExprNode>),

    /// <
    Lt(Box<ExprNode>, Box<ExprNode>),
    /// >
    Gt(Box<ExprNode>, Box<ExprNode>),
    /// <=
    Lte(Box<ExprNode>, Box<ExprNode>),
    /// >=
    Gte(Box<ExprNode>, Box<ExprNode>),
    /// ==
    Eq(Box<ExprNode>, Box<ExprNode>),
    /// !=
    Neq(Box<ExprNode>, Box<ExprNode>),

    /// |
    BitOr(Box<ExprNode>, Box<ExprNode>),

    /// ^
    BitXor(Box<ExprNode>, Box<ExprNode>),

    /// &
    BitAnd(Box<ExprNode>, Box<ExprNode>),

    /// <<
    Shl(Box<ExprNode>, Box<ExprNode>),
    /// >>
    Shr(Box<ExprNode>, Box<ExprNode>),

    /// +
    Add(Box<ExprNode>, Box<ExprNode>),
    /// -
    Sub(Box<ExprNode>, Box<ExprNode>),

    /// *
    Mul(Box<ExprNode>, Box<ExprNode>),
    /// /
    Div(Box<ExprNode>, Box<ExprNode>),
    /// %
    Rem(Box<ExprNode>, Box<ExprNode>),
}

pub(crate) fn expr8() -> impl Parser<char, ExprNode, Error = Simple<char>> + Clone {
    expr7()
        .then(
            just("&&")
                .to(ExprNode::And as fn(_, _) -> _)
                .then(expr7())
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
}

pub(crate) fn expr7() -> impl Parser<char, ExprNode, Error = Simple<char>> + Clone {
    expr6()
        .then(
            just("!=")
                .to(ExprNode::Neq as fn(_, _) -> _)
                .or(just("==").to(ExprNode::Eq as fn(_, _) -> _))
                .or(just(">=").to(ExprNode::Gte as fn(_, _) -> _))
                .or(just("<=").to(ExprNode::Lte as fn(_, _) -> _))
                .or(just('>').to(ExprNode::Gt as fn(_, _) -> _))
                .or(just('<').to(ExprNode::Lt as fn(_, _) -> _))
                .then(expr6())
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
}

pub(crate) fn expr6() -> impl Parser<char, ExprNode, Error = Simple<char>> + Clone {
    expr5()
        .then(
            just('|')
                .to(ExprNode::BitOr as fn(_, _) -> _)
                .then(expr5())
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
}

pub(crate) fn expr5() -> impl Parser<char, ExprNode, Error = Simple<char>> + Clone {
    expr4()
        .then(
            just('^')
                .to(ExprNode::BitXor as fn(_, _) -> _)
                .then(expr4())
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
}

pub(crate) fn expr4() -> impl Parser<char, ExprNode, Error = Simple<char>> + Clone {
    expr3()
        .then(
            just('&')
                .to(ExprNode::BitAnd as fn(_, _) -> _)
                .then(expr3())
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
}

pub(crate) fn expr3() -> impl Parser<char, ExprNode, Error = Simple<char>> + Clone {
    expr2()
        .then(
            just("<<")
                .to(ExprNode::Shl as fn(_, _) -> _)
                .or(just(">>").to(ExprNode::Shr as fn(_, _) -> _))
                .then(expr2())
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
}

pub(crate) fn expr2() -> impl Parser<char, ExprNode, Error = Simple<char>> + Clone {
    expr1()
        .then(
            just('+')
                .to(ExprNode::Add as fn(_, _) -> _)
                .or(just('-').to(ExprNode::Sub as fn(_, _) -> _))
                .then(expr1())
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
}

pub(crate) fn expr1() -> impl Parser<char, ExprNode, Error = Simple<char>> + Clone {
    term()
        .then(
            just('*')
                .to(ExprNode::Mul as fn(_, _) -> _)
                .or(just('/').to(ExprNode::Div as fn(_, _) -> _))
                .or(just('%').to(ExprNode::Rem as fn(_, _) -> _))
                .then(term())
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
}

pub(crate) fn term() -> impl Parser<char, ExprNode, Error = Simple<char>> + Clone {
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

pub(crate) fn suffix() -> impl Parser<char, ExprNode, Error = Simple<char>> + Clone {
    primary()
    // or(FnCall)
}

pub(crate) fn primary() -> impl Parser<char, ExprNode, Error = Simple<char>> + Clone {
    choice((
        integer().map(ExprNode::Integer),
        character().map(ExprNode::Integer),
        string().map(ExprNode::String),
        variable().map(ExprNode::Variable),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;

    #[test]
    fn expr8_test() {
        assert_eq!(
            expr8().parse("1 && 2 != 3 | 4 ^ 5 & 6 << 7 + 8*9"),
            Ok(ExprNode::And(
                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                Box::new(ExprNode::Neq(
                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(ExprNode::BitOr(
                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(ExprNode::BitXor(
                            Box::from(ExprNode::Integer(IntegerLiteralNode::I32(4))),
                            Box::from(ExprNode::BitAnd(
                                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(5))),
                                Box::from(ExprNode::Shl(
                                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(6))),
                                    Box::from(ExprNode::Add(
                                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(7))),
                                        Box::from(ExprNode::Mul(
                                            Box::from(ExprNode::Integer(IntegerLiteralNode::I32(
                                                8
                                            ))),
                                            Box::from(ExprNode::Integer(IntegerLiteralNode::I32(
                                                9
                                            )))
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
            Ok(ExprNode::Integer(IntegerLiteralNode::I32(1)))
        );
    }

    #[test]
    fn expr7_test() {
        assert_eq!(
            expr7().parse("1 != 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(ExprNode::Neq(
                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                Box::from(ExprNode::BitOr(
                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(ExprNode::BitXor(
                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(ExprNode::BitAnd(
                            Box::from(ExprNode::Integer(IntegerLiteralNode::I32(4))),
                            Box::from(ExprNode::Shl(
                                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(5))),
                                Box::from(ExprNode::Add(
                                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(6))),
                                    Box::from(ExprNode::Mul(
                                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(7))),
                                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(8)))
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
            Ok(ExprNode::Eq(
                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                Box::from(ExprNode::BitOr(
                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(ExprNode::BitXor(
                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(ExprNode::BitAnd(
                            Box::from(ExprNode::Integer(IntegerLiteralNode::I32(4))),
                            Box::from(ExprNode::Shl(
                                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(5))),
                                Box::from(ExprNode::Add(
                                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(6))),
                                    Box::from(ExprNode::Mul(
                                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(7))),
                                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(8)))
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
            Ok(ExprNode::Gte(
                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                Box::from(ExprNode::BitOr(
                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(ExprNode::BitXor(
                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(ExprNode::BitAnd(
                            Box::from(ExprNode::Integer(IntegerLiteralNode::I32(4))),
                            Box::from(ExprNode::Shl(
                                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(5))),
                                Box::from(ExprNode::Add(
                                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(6))),
                                    Box::from(ExprNode::Mul(
                                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(7))),
                                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(8)))
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
            Ok(ExprNode::Lte(
                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                Box::from(ExprNode::BitOr(
                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(ExprNode::BitXor(
                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(ExprNode::BitAnd(
                            Box::from(ExprNode::Integer(IntegerLiteralNode::I32(4))),
                            Box::from(ExprNode::Shl(
                                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(5))),
                                Box::from(ExprNode::Add(
                                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(6))),
                                    Box::from(ExprNode::Mul(
                                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(7))),
                                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(8)))
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
            Ok(ExprNode::Gt(
                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                Box::from(ExprNode::BitOr(
                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(ExprNode::BitXor(
                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(ExprNode::BitAnd(
                            Box::from(ExprNode::Integer(IntegerLiteralNode::I32(4))),
                            Box::from(ExprNode::Shl(
                                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(5))),
                                Box::from(ExprNode::Add(
                                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(6))),
                                    Box::from(ExprNode::Mul(
                                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(7))),
                                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(8)))
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
            Ok(ExprNode::Lt(
                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                Box::from(ExprNode::BitOr(
                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(ExprNode::BitXor(
                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(ExprNode::BitAnd(
                            Box::from(ExprNode::Integer(IntegerLiteralNode::I32(4))),
                            Box::from(ExprNode::Shl(
                                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(5))),
                                Box::from(ExprNode::Add(
                                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(6))),
                                    Box::from(ExprNode::Mul(
                                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(7))),
                                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(8)))
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
            Ok(ExprNode::Integer(IntegerLiteralNode::I32(1)))
        );
    }

    #[test]
    fn expr6_test() {
        assert_eq!(
            expr6().parse("1 | 2 ^ 3 & 4 << 5 + 6*7"),
            Ok(ExprNode::BitOr(
                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                Box::from(ExprNode::BitXor(
                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(ExprNode::BitAnd(
                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(ExprNode::Shl(
                            Box::from(ExprNode::Integer(IntegerLiteralNode::I32(4))),
                            Box::from(ExprNode::Add(
                                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(5))),
                                Box::from(ExprNode::Mul(
                                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(6))),
                                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(7)))
                                )),
                            )),
                        )),
                    )),
                )),
            ))
        );

        assert_eq!(
            expr6().parse("1"),
            Ok(ExprNode::Integer(IntegerLiteralNode::I32(1)))
        );
    }

    #[test]
    fn expr5_test() {
        assert_eq!(
            expr5().parse("1 ^ 2 & 3 << 4 + 5*6"),
            Ok(ExprNode::BitXor(
                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                Box::from(ExprNode::BitAnd(
                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(ExprNode::Shl(
                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(ExprNode::Add(
                            Box::from(ExprNode::Integer(IntegerLiteralNode::I32(4))),
                            Box::from(ExprNode::Mul(
                                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(5))),
                                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(6)))
                            )),
                        )),
                    )),
                )),
            ))
        );

        assert_eq!(
            expr5().parse("1"),
            Ok(ExprNode::Integer(IntegerLiteralNode::I32(1)))
        );
    }

    #[test]
    fn expr4_test() {
        assert_eq!(
            expr4().parse("1 & 2 << 3 + 4*5"),
            Ok(ExprNode::BitAnd(
                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                Box::from(ExprNode::Shl(
                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(ExprNode::Add(
                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(ExprNode::Mul(
                            Box::from(ExprNode::Integer(IntegerLiteralNode::I32(4))),
                            Box::from(ExprNode::Integer(IntegerLiteralNode::I32(5)))
                        )),
                    )),
                )),
            ))
        );

        assert_eq!(
            expr4().parse("1"),
            Ok(ExprNode::Integer(IntegerLiteralNode::I32(1)))
        );
    }

    #[test]
    fn expr3_test() {
        assert_eq!(
            expr3().parse("1 << 2 + 3*4"),
            Ok(ExprNode::Shl(
                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                Box::from(ExprNode::Add(
                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(ExprNode::Mul(
                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(4)))
                    )),
                )),
            ))
        );

        assert_eq!(
            expr3().parse("1 >> 2 + 3*4"),
            Ok(ExprNode::Shr(
                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                Box::from(ExprNode::Add(
                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(ExprNode::Mul(
                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3))),
                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(4)))
                    )),
                )),
            ))
        );

        assert_eq!(
            expr3().parse("1"),
            Ok(ExprNode::Integer(IntegerLiteralNode::I32(1)))
        );
    }

    #[test]
    fn expr2_test() {
        assert_eq!(
            expr2().parse("1 + 2*3"),
            Ok(ExprNode::Add(
                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                Box::from(ExprNode::Mul(
                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3)))
                )),
            ))
        );

        assert_eq!(
            expr2().parse("1 - 2*3"),
            Ok(ExprNode::Sub(
                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                Box::from(ExprNode::Mul(
                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2))),
                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3)))
                )),
            ))
        );

        assert_eq!(
            expr2().parse("1*2 + 3*4"),
            Ok(ExprNode::Add(
                Box::from(ExprNode::Mul(
                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2)))
                )),
                Box::from(ExprNode::Mul(
                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3))),
                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(4)))
                )),
            ))
        );

        assert_eq!(
            expr2().parse("1"),
            Ok(ExprNode::Integer(IntegerLiteralNode::I32(1)))
        );
    }

    #[test]
    fn expr1_test() {
        assert_eq!(
            expr1().parse("1*1"),
            Ok(ExprNode::Mul(
                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
            ))
        );
        assert_eq!(
            expr1().parse("1 / 1"),
            Ok(ExprNode::Div(
                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
            ))
        );
        assert_eq!(
            expr1().parse("1 %2"),
            Ok(ExprNode::Rem(
                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2))),
            ))
        );

        assert_eq!(
            expr1().parse("1 % 2 / 3 * 4"),
            Ok(ExprNode::Mul(
                Box::from(ExprNode::Div(
                    Box::from(ExprNode::Rem(
                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                        Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2)))
                    )),
                    Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3)))
                )),
                Box::from(ExprNode::Integer(IntegerLiteralNode::I32(4))),
            ))
        );

        assert_eq!(
            expr1().parse("1"),
            Ok(ExprNode::Integer(IntegerLiteralNode::I32(1)))
        );
    }

    #[test]
    fn primary_test() {
        assert_eq!(
            primary().parse("1"),
            Ok(ExprNode::Integer(IntegerLiteralNode::I32(1)))
        );
        assert_eq!(
            primary().parse("'a'"),
            Ok(ExprNode::Integer(IntegerLiteralNode::I8(97)))
        );
        assert_eq!(
            primary().parse("\"a\""),
            Ok(ExprNode::String(StringLiteralNode("a".to_string())))
        );
        assert_eq!(
            primary().parse("var"),
            Ok(ExprNode::Variable(VariableNode("var".to_string())))
        );
    }
}
