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

type RecSuffix<'a> = Recursive<'a, char, Expr, Simple<char>>;

pub(crate) fn args(
    suffix: Option<RecSuffix>,
) -> impl Parser<char, Vec<Expr>, Error = Simple<char>> + Clone + '_ {
    expr9(suffix).separated_by(just(',')).boxed()
}

pub(crate) fn expr9(
    suffix: Option<RecSuffix>,
) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + '_ {
    expr8(suffix.clone())
        .then(just("||").to(Expr::Or).then(expr8(suffix)).repeated())
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
        .boxed()
}

fn expr8(suffix: Option<RecSuffix>) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + '_ {
    expr7(suffix.clone())
        .then(just("&&").to(Expr::And).then(expr7(suffix)).repeated())
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
        .boxed()
}

fn expr7(suffix: Option<RecSuffix>) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + '_ {
    expr6(suffix.clone())
        .then(
            choice((
                just("!=").to(Expr::Neq as fn(_, _) -> _),
                just("==").to(Expr::Eq as fn(_, _) -> _),
                just(">=").to(Expr::Gte as fn(_, _) -> _),
                just("<=").to(Expr::Lte as fn(_, _) -> _),
                just('>').to(Expr::Gt as fn(_, _) -> _),
                just('<').to(Expr::Lt as fn(_, _) -> _),
            ))
            .then(expr6(suffix))
            .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
        .boxed()
}

fn expr6(suffix: Option<RecSuffix>) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + '_ {
    expr5(suffix.clone())
        .then(just('|').to(Expr::BitOr).then(expr5(suffix)).repeated())
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
        .boxed()
}

fn expr5(suffix: Option<RecSuffix>) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + '_ {
    expr4(suffix.clone())
        .then(just('^').to(Expr::BitXor).then(expr4(suffix)).repeated())
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
        .boxed()
}

fn expr4(suffix: Option<RecSuffix>) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + '_ {
    expr3(suffix.clone())
        .then(just('&').to(Expr::BitAnd).then(expr3(suffix)).repeated())
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
        .boxed()
}

fn expr3(suffix: Option<RecSuffix>) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + '_ {
    expr2(suffix.clone())
        .then(
            choice((
                just("<<").to(Expr::Shl as fn(_, _) -> _),
                just(">>").to(Expr::Shr as fn(_, _) -> _),
            ))
            .then(expr2(suffix))
            .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
        .boxed()
}

fn expr2(suffix: Option<RecSuffix>) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + '_ {
    expr1(suffix.clone())
        .then(
            choice((
                just('+').to(Expr::Add as fn(_, _) -> _),
                just('-').to(Expr::Sub as fn(_, _) -> _),
            ))
            .then(expr1(suffix))
            .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
        .boxed()
}

fn expr1(suffix: Option<RecSuffix>) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + '_ {
    term(suffix.clone())
        .then(
            choice((
                just('*').to(Expr::Mul as fn(_, _) -> _),
                just('/').to(Expr::Div as fn(_, _) -> _),
                just('%').to(Expr::Rem as fn(_, _) -> _),
            ))
            .then(term(suffix))
            .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
        .boxed()
}

// (cast)suffix
pub(crate) fn term(
    suffix_rec: Option<RecSuffix>,
) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + '_ {
    let cast = typeref().delimited_by(just("("), just(")")).or_not();

    match suffix_rec {
        None => cast
            .then(suffix())
            .map(|(cast, expr)| match cast {
                Some(cast) => Expr::Cast {
                    cast,
                    expr: Box::new(expr),
                },
                None => expr,
            })
            .boxed(),
        Some(suffix_rec) => cast
            .then(suffix_rec)
            .map(|(cast, expr)| match cast {
                Some(cast) => Expr::Cast {
                    cast,
                    expr: Box::new(expr),
                },
                None => expr,
            })
            .boxed(),
    }
}

// fn(a1, a2)
fn suffix() -> impl Parser<char, Expr, Error = Simple<char>> + Clone {
    recursive(|suffix| {
        primary()
            .then(
                args(Some(suffix))
                    .delimited_by(just('('), just(')'))
                    .repeated(),
            )
            .foldl(|lhs, args| Expr::FnCall {
                name: Box::new(lhs),
                args,
            })
    })
    .boxed()
}

fn primary() -> impl Parser<char, Expr, Error = Simple<char>> + Clone {
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
            args(None).parse("1, var, 1 +1"),
            Ok(vec![
                Expr::Int(Int::I32(1)),
                Expr::Variable("var".to_string()),
                Expr::Add(
                    Box::new(Expr::Int(Int::I32(1))),
                    Box::new(Expr::Int(Int::I32(1))),
                )
            ])
        );
        assert_eq!(args(None).parse("1"), Ok(vec![Expr::Int(Int::I32(1))]));
    }

    #[test]
    fn expr9_test() {
        assert_eq!(
            expr9(None).parse("1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10"),
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

        assert_eq!(expr9(None).parse("1"), Ok(Expr::Int(Int::I32(1))));
    }

    #[test]
    fn expr8_test() {
        assert_eq!(
            expr8(None).parse("1 && 2 != 3 | 4 ^ 5 & 6 << 7 + 8*9"),
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

        assert_eq!(expr8(None).parse("1"), Ok(Expr::Int(Int::I32(1))));
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
            expr7(None).parse("1 != 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Expr::Neq(Box::new(Expr::Int(Int::I32(1))), expr.clone()))
        );

        assert_eq!(
            expr7(None).parse("1 == 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Expr::Eq(Box::new(Expr::Int(Int::I32(1))), expr.clone()))
        );

        assert_eq!(
            expr7(None).parse("1 >= 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Expr::Gte(Box::new(Expr::Int(Int::I32(1))), expr.clone()))
        );

        assert_eq!(
            expr7(None).parse("1 <= 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Expr::Lte(Box::new(Expr::Int(Int::I32(1))), expr.clone()))
        );

        assert_eq!(
            expr7(None).parse("1 > 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Expr::Gt(Box::new(Expr::Int(Int::I32(1))), expr.clone()))
        );

        assert_eq!(
            expr7(None).parse("1 < 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Expr::Lt(Box::new(Expr::Int(Int::I32(1))), expr))
        );

        assert_eq!(expr7(None).parse("1"), Ok(Expr::Int(Int::I32(1))));
    }

    #[test]
    fn expr6_test() {
        assert_eq!(
            expr6(None).parse("1 | 2 ^ 3 & 4 << 5 + 6*7"),
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

        assert_eq!(expr6(None).parse("1"), Ok(Expr::Int(Int::I32(1))));
    }

    #[test]
    fn expr5_test() {
        assert_eq!(
            expr5(None).parse("1 ^ 2 & 3 << 4 + 5*6"),
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

        assert_eq!(expr5(None).parse("1"), Ok(Expr::Int(Int::I32(1))));
    }

    #[test]
    fn expr4_test() {
        assert_eq!(
            expr4(None).parse("1 & 2 << 3 + 4*5"),
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

        assert_eq!(expr4(None).parse("1"), Ok(Expr::Int(Int::I32(1))));
    }

    #[test]
    fn expr3_test() {
        assert_eq!(
            expr3(None).parse("1 << 2 + 3*4"),
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
            expr3(None).parse("1 >> 2 + 3*4"),
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

        assert_eq!(expr3(None).parse("1"), Ok(Expr::Int(Int::I32(1))));
    }

    #[test]
    fn expr2_test() {
        assert_eq!(
            expr2(None).parse("1 + 2*3"),
            Ok(Expr::Add(
                Box::new(Expr::Int(Int::I32(1))),
                Box::new(Expr::Mul(
                    Box::new(Expr::Int(Int::I32(2))),
                    Box::new(Expr::Int(Int::I32(3)))
                )),
            ))
        );

        assert_eq!(
            expr2(None).parse("1 - 2*3"),
            Ok(Expr::Sub(
                Box::new(Expr::Int(Int::I32(1))),
                Box::new(Expr::Mul(
                    Box::new(Expr::Int(Int::I32(2))),
                    Box::new(Expr::Int(Int::I32(3)))
                )),
            ))
        );

        assert_eq!(
            expr2(None).parse("1*2 + 3*4"),
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

        assert_eq!(expr2(None).parse("1"), Ok(Expr::Int(Int::I32(1))));
    }

    #[test]
    fn expr1_test() {
        assert_eq!(
            expr1(None).parse("1*1"),
            Ok(Expr::Mul(
                Box::new(Expr::Int(Int::I32(1))),
                Box::new(Expr::Int(Int::I32(1))),
            ))
        );
        assert_eq!(
            expr1(None).parse("1 / 1"),
            Ok(Expr::Div(
                Box::new(Expr::Int(Int::I32(1))),
                Box::new(Expr::Int(Int::I32(1))),
            ))
        );
        assert_eq!(
            expr1(None).parse("1 %2"),
            Ok(Expr::Rem(
                Box::new(Expr::Int(Int::I32(1))),
                Box::new(Expr::Int(Int::I32(2))),
            ))
        );

        assert_eq!(
            expr1(None).parse("1 % 2 / 3 * 4"),
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

        assert_eq!(expr1(None).parse("1"), Ok(Expr::Int(Int::I32(1))));
    }

    #[test]
    fn term_test() {
        assert_eq!(
            term(None).parse("(cast)var"),
            Ok(Expr::Cast {
                cast: Type::User("cast".to_string()),
                expr: Box::new(Expr::Variable("var".to_string())),
            })
        );
        assert_eq!(
            term(None).parse("(u8)127"),
            Ok(Expr::Cast {
                cast: Type::U8,
                expr: Box::new(Expr::Int(Int::I32(127))),
            })
        );

        // term should parse parse as well
        assert_eq!(term(None).parse("1"), Ok(Expr::Int(Int::I32(1))));
        assert_eq!(term(None).parse("'a'"), Ok(Expr::Int(Int::I8(97))));
        assert_eq!(term(None).parse("\"a\""), Ok(Expr::String("a".to_string())));
        assert_eq!(
            term(None).parse("var"),
            Ok(Expr::Variable("var".to_string()))
        );
    }

    #[test]
    fn suffix_test() {
        assert_eq!(
            suffix().parse("fun(1, a2, '3', \"4\")"),
            Ok(Expr::FnCall {
                name: Box::new(Expr::Variable("fun".to_string())),
                args: vec![
                    Expr::Int(Int::I32(1)),
                    Expr::Variable("a2".to_string()),
                    Expr::Int(Int::I8(51)),
                    Expr::String("4".to_string()),
                ]
            })
        );

        assert_eq!(suffix().parse("1"), Ok(Expr::Int(Int::I32(1))));
        assert_eq!(suffix().parse("'a'"), Ok(Expr::Int(Int::I8(97))));
        assert_eq!(suffix().parse("\"a\""), Ok(Expr::String("a".to_string())));
        assert_eq!(suffix().parse("var"), Ok(Expr::Variable("var".to_string())));
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
