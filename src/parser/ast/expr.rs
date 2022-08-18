use crate::parser::ast::{character, integer, string, typeref, variable, Int, Type};
use chumsky::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Expr {
    /// ||
    Or(Box<Self>, Box<Self>),

    /// &&
    And(Box<Self>, Box<Self>),

    /// <
    Lt(Box<Self>, Box<Self>),
    /// >
    Gt(Box<Self>, Box<Self>),
    /// <=
    Lte(Box<Self>, Box<Self>),
    /// >=
    Gte(Box<Self>, Box<Self>),
    /// ==
    Eq(Box<Self>, Box<Self>),
    /// !=
    Neq(Box<Self>, Box<Self>),

    /// |
    BitOr(Box<Self>, Box<Self>),

    /// ^
    BitXor(Box<Self>, Box<Self>),

    /// &
    BitAnd(Box<Self>, Box<Self>),

    /// <<
    Shl(Box<Self>, Box<Self>),
    /// >>
    Shr(Box<Self>, Box<Self>),

    /// +
    Add(Box<Self>, Box<Self>),
    /// -
    Sub(Box<Self>, Box<Self>),

    /// *
    Mul(Box<Self>, Box<Self>),
    /// /
    Div(Box<Self>, Box<Self>),
    /// %
    Rem(Box<Self>, Box<Self>),

    FnCall {
        name: Box<Self>,
        args: Vec<Self>,
    },

    Cast {
        cast: Type,
        expr: Box<Self>,
    },

    /// Atom
    String(String),
    Variable(String),
    Int(Int),
}

pub(crate) fn args() -> impl Parser<char, Vec<Expr>, Error = Simple<char>> + Clone {
    expr9().separated_by(just(',')).boxed()
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
        .boxed()
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
        .boxed()
}

pub(crate) fn expr7() -> impl Parser<char, Expr, Error = Simple<char>> + Clone {
    expr6()
        .then(
            choice((
                just("!=").to(Expr::Neq as fn(_, _) -> _),
                just("==").to(Expr::Eq as fn(_, _) -> _),
                just(">=").to(Expr::Gte as fn(_, _) -> _),
                just("<=").to(Expr::Lte as fn(_, _) -> _),
                just('>').to(Expr::Gt as fn(_, _) -> _),
                just('<').to(Expr::Lt as fn(_, _) -> _),
            ))
            .then(expr6())
            .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
        .boxed()
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
        .boxed()
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
        .boxed()
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
        .boxed()
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
        .boxed()
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
        .boxed()
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
        .boxed()
}

// TODO: Cast can be applied only once. (Do we really do double cast?)
// (cast)suffix
pub(crate) fn term() -> impl Parser<char, Expr, Error = Simple<char>> + Clone {
    typeref()
        .delimited_by(just("("), just(")"))
        .or_not()
        .then(suffix())
        .map(|(cast, expr)| match cast {
            Some(cast) => Expr::Cast {
                cast,
                expr: Box::new(expr),
            },
            None => expr,
        })
        .boxed()
}

// TODO: That function arguments can be passed only once will prevent to call curried function.
// fn(a1, a2)
pub(crate) fn suffix() -> impl Parser<char, Expr, Error = Simple<char>> + Clone {
    // TODO: Stack overflow on args_test and other tests
    // recursive(|_| {
    //     primary()
    //         .then(args().delimited_by(just('('), just(')')).repeated())
    //         .foldl(|lhs, args| FnCall {
    //             name: Box::new(lhs),
    //             args,
    //         })
    // })
    primary().boxed()
}

pub(crate) fn primary() -> impl Parser<char, Expr, Error = Simple<char>> + Clone {
    choice((
        integer().map(Expr::Int),
        character().map(Expr::Int),
        string().map(Expr::String),
        variable().map(Expr::Variable),
    ))
    .padded()
    .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;

    #[test]
    fn args_test() {
        assert_eq!(
            args().parse("1, var, 1 +1"),
            Ok(vec![
                Expr::Int(Int::I32(1)),
                Expr::Variable("var".to_string()),
                Expr::Add(
                    Box::new(Expr::Int(Int::I32(1))),
                    Box::new(Expr::Int(Int::I32(1))),
                )
            ])
        );
        assert_eq!(args().parse("1"), Ok(vec![Expr::Int(Int::I32(1))]));
    }

    #[test]
    fn expr9_test() {
        assert_eq!(
            expr9().parse("1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10"),
            Ok(Expr::Or(
                Box::new(Expr::Int(Int::I32(1))),
                Box::new(Expr::And(
                    Box::new(Expr::Int(Int::I32(2))),
                    Box::new(Expr::Neq(
                        Box::new(Expr::Int(Int::I32(3))),
                        Box::new(Expr::BitOr(
                            Box::new(Expr::Int(Int::I32(4))),
                            Box::new(Expr::BitXor(
                                Box::new(Expr::Int(Int::I32(5))),
                                Box::new(Expr::BitAnd(
                                    Box::new(Expr::Int(Int::I32(6))),
                                    Box::new(Expr::Shl(
                                        Box::new(Expr::Int(Int::I32(7))),
                                        Box::new(Expr::Add(
                                            Box::new(Expr::Int(Int::I32(8))),
                                            Box::new(Expr::Mul(
                                                Box::new(Expr::Int(Int::I32(9))),
                                                Box::new(Expr::Int(Int::I32(10)))
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

        assert_eq!(expr9().parse("1"), Ok(Expr::Int(Int::I32(1))));
    }

    #[test]
    fn expr8_test() {
        assert_eq!(
            expr8().parse("1 && 2 != 3 | 4 ^ 5 & 6 << 7 + 8*9"),
            Ok(Expr::And(
                Box::new(Expr::Int(Int::I32(1))),
                Box::new(Expr::Neq(
                    Box::new(Expr::Int(Int::I32(2))),
                    Box::new(Expr::BitOr(
                        Box::new(Expr::Int(Int::I32(3))),
                        Box::new(Expr::BitXor(
                            Box::new(Expr::Int(Int::I32(4))),
                            Box::new(Expr::BitAnd(
                                Box::new(Expr::Int(Int::I32(5))),
                                Box::new(Expr::Shl(
                                    Box::new(Expr::Int(Int::I32(6))),
                                    Box::new(Expr::Add(
                                        Box::new(Expr::Int(Int::I32(7))),
                                        Box::new(Expr::Mul(
                                            Box::new(Expr::Int(Int::I32(8))),
                                            Box::new(Expr::Int(Int::I32(9)))
                                        )),
                                    )),
                                )),
                            )),
                        )),
                    )),
                )),
            ))
        );

        assert_eq!(expr8().parse("1"), Ok(Expr::Int(Int::I32(1))));
    }

    #[test]
    fn expr7_test() {
        let expr = Box::new(Expr::BitOr(
            Box::new(Expr::Int(Int::I32(2))),
            Box::new(Expr::BitXor(
                Box::new(Expr::Int(Int::I32(3))),
                Box::new(Expr::BitAnd(
                    Box::new(Expr::Int(Int::I32(4))),
                    Box::new(Expr::Shl(
                        Box::new(Expr::Int(Int::I32(5))),
                        Box::new(Expr::Add(
                            Box::new(Expr::Int(Int::I32(6))),
                            Box::new(Expr::Mul(
                                Box::new(Expr::Int(Int::I32(7))),
                                Box::new(Expr::Int(Int::I32(8))),
                            )),
                        )),
                    )),
                )),
            )),
        ));

        assert_eq!(
            expr7().parse("1 != 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Expr::Neq(Box::new(Expr::Int(Int::I32(1))), expr.clone()))
        );

        assert_eq!(
            expr7().parse("1 == 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Expr::Eq(Box::new(Expr::Int(Int::I32(1))), expr.clone()))
        );

        assert_eq!(
            expr7().parse("1 >= 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Expr::Gte(Box::new(Expr::Int(Int::I32(1))), expr.clone()))
        );

        assert_eq!(
            expr7().parse("1 <= 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Expr::Lte(Box::new(Expr::Int(Int::I32(1))), expr.clone()))
        );

        assert_eq!(
            expr7().parse("1 > 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Expr::Gt(Box::new(Expr::Int(Int::I32(1))), expr.clone()))
        );

        assert_eq!(
            expr7().parse("1 < 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Expr::Lt(Box::new(Expr::Int(Int::I32(1))), expr))
        );

        assert_eq!(expr7().parse("1"), Ok(Expr::Int(Int::I32(1))));
    }

    #[test]
    fn expr6_test() {
        assert_eq!(
            expr6().parse("1 | 2 ^ 3 & 4 << 5 + 6*7"),
            Ok(Expr::BitOr(
                Box::new(Expr::Int(Int::I32(1))),
                Box::new(Expr::BitXor(
                    Box::new(Expr::Int(Int::I32(2))),
                    Box::new(Expr::BitAnd(
                        Box::new(Expr::Int(Int::I32(3))),
                        Box::new(Expr::Shl(
                            Box::new(Expr::Int(Int::I32(4))),
                            Box::new(Expr::Add(
                                Box::new(Expr::Int(Int::I32(5))),
                                Box::new(Expr::Mul(
                                    Box::new(Expr::Int(Int::I32(6))),
                                    Box::new(Expr::Int(Int::I32(7)))
                                )),
                            )),
                        )),
                    )),
                )),
            ))
        );

        assert_eq!(expr6().parse("1"), Ok(Expr::Int(Int::I32(1))));
    }

    #[test]
    fn expr5_test() {
        assert_eq!(
            expr5().parse("1 ^ 2 & 3 << 4 + 5*6"),
            Ok(Expr::BitXor(
                Box::new(Expr::Int(Int::I32(1))),
                Box::new(Expr::BitAnd(
                    Box::new(Expr::Int(Int::I32(2))),
                    Box::new(Expr::Shl(
                        Box::new(Expr::Int(Int::I32(3))),
                        Box::new(Expr::Add(
                            Box::new(Expr::Int(Int::I32(4))),
                            Box::new(Expr::Mul(
                                Box::new(Expr::Int(Int::I32(5))),
                                Box::new(Expr::Int(Int::I32(6)))
                            )),
                        )),
                    )),
                )),
            ))
        );

        assert_eq!(expr5().parse("1"), Ok(Expr::Int(Int::I32(1))));
    }

    #[test]
    fn expr4_test() {
        assert_eq!(
            expr4().parse("1 & 2 << 3 + 4*5"),
            Ok(Expr::BitAnd(
                Box::new(Expr::Int(Int::I32(1))),
                Box::new(Expr::Shl(
                    Box::new(Expr::Int(Int::I32(2))),
                    Box::new(Expr::Add(
                        Box::new(Expr::Int(Int::I32(3))),
                        Box::new(Expr::Mul(
                            Box::new(Expr::Int(Int::I32(4))),
                            Box::new(Expr::Int(Int::I32(5)))
                        )),
                    )),
                )),
            ))
        );

        assert_eq!(expr4().parse("1"), Ok(Expr::Int(Int::I32(1))));
    }

    #[test]
    fn expr3_test() {
        assert_eq!(
            expr3().parse("1 << 2 + 3*4"),
            Ok(Expr::Shl(
                Box::new(Expr::Int(Int::I32(1))),
                Box::new(Expr::Add(
                    Box::new(Expr::Int(Int::I32(2))),
                    Box::new(Expr::Mul(
                        Box::new(Expr::Int(Int::I32(3))),
                        Box::new(Expr::Int(Int::I32(4)))
                    )),
                )),
            ))
        );

        assert_eq!(
            expr3().parse("1 >> 2 + 3*4"),
            Ok(Expr::Shr(
                Box::new(Expr::Int(Int::I32(1))),
                Box::new(Expr::Add(
                    Box::new(Expr::Int(Int::I32(2))),
                    Box::new(Expr::Mul(
                        Box::new(Expr::Int(Int::I32(3))),
                        Box::new(Expr::Int(Int::I32(4)))
                    )),
                )),
            ))
        );

        assert_eq!(expr3().parse("1"), Ok(Expr::Int(Int::I32(1))));
    }

    #[test]
    fn expr2_test() {
        assert_eq!(
            expr2().parse("1 + 2*3"),
            Ok(Expr::Add(
                Box::new(Expr::Int(Int::I32(1))),
                Box::new(Expr::Mul(
                    Box::new(Expr::Int(Int::I32(2))),
                    Box::new(Expr::Int(Int::I32(3)))
                )),
            ))
        );

        assert_eq!(
            expr2().parse("1 - 2*3"),
            Ok(Expr::Sub(
                Box::new(Expr::Int(Int::I32(1))),
                Box::new(Expr::Mul(
                    Box::new(Expr::Int(Int::I32(2))),
                    Box::new(Expr::Int(Int::I32(3)))
                )),
            ))
        );

        assert_eq!(
            expr2().parse("1*2 + 3*4"),
            Ok(Expr::Add(
                Box::new(Expr::Mul(
                    Box::new(Expr::Int(Int::I32(1))),
                    Box::new(Expr::Int(Int::I32(2)))
                )),
                Box::new(Expr::Mul(
                    Box::new(Expr::Int(Int::I32(3))),
                    Box::new(Expr::Int(Int::I32(4)))
                )),
            ))
        );

        assert_eq!(expr2().parse("1"), Ok(Expr::Int(Int::I32(1))));
    }

    #[test]
    fn expr1_test() {
        assert_eq!(
            expr1().parse("1*1"),
            Ok(Expr::Mul(
                Box::new(Expr::Int(Int::I32(1))),
                Box::new(Expr::Int(Int::I32(1))),
            ))
        );
        assert_eq!(
            expr1().parse("1 / 1"),
            Ok(Expr::Div(
                Box::new(Expr::Int(Int::I32(1))),
                Box::new(Expr::Int(Int::I32(1))),
            ))
        );
        assert_eq!(
            expr1().parse("1 %2"),
            Ok(Expr::Rem(
                Box::new(Expr::Int(Int::I32(1))),
                Box::new(Expr::Int(Int::I32(2))),
            ))
        );

        assert_eq!(
            expr1().parse("1 % 2 / 3 * 4"),
            Ok(Expr::Mul(
                Box::new(Expr::Div(
                    Box::new(Expr::Rem(
                        Box::new(Expr::Int(Int::I32(1))),
                        Box::new(Expr::Int(Int::I32(2)))
                    )),
                    Box::new(Expr::Int(Int::I32(3)))
                )),
                Box::new(Expr::Int(Int::I32(4))),
            ))
        );

        assert_eq!(expr1().parse("1"), Ok(Expr::Int(Int::I32(1))));
    }

    #[test]
    fn term_test() {
        assert_eq!(
            term().parse("(cast)var"),
            Ok(Expr::Cast {
                cast: Type::User("cast".to_string()),
                expr: Box::new(Expr::Variable("var".to_string())),
            })
        );
        assert_eq!(
            term().parse("(u8)127"),
            Ok(Expr::Cast {
                cast: Type::U8,
                expr: Box::new(Expr::Int(Int::I32(127))),
            })
        );

        // term should parse parse as well
        assert_eq!(term().parse("1"), Ok(Expr::Int(Int::I32(1))));
        assert_eq!(term().parse("'a'"), Ok(Expr::Int(Int::I8(97))));
        assert_eq!(term().parse("\"a\""), Ok(Expr::String("a".to_string())));
        assert_eq!(term().parse("var"), Ok(Expr::Variable("var".to_string())));
    }

    #[test]
    fn primary_test() {
        assert_eq!(primary().parse("1"), Ok(Expr::Int(Int::I32(1))));
        assert_eq!(primary().parse("'a'"), Ok(Expr::Int(Int::I8(97))));
        assert_eq!(primary().parse("\"a\""), Ok(Expr::String("a".to_string())));
        assert_eq!(
            primary().parse("var"),
            Ok(Expr::Variable("var".to_string()))
        );
    }
}
