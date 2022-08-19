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

    /// as
    As(Box<Self>, Type),

    FnCall {
        name: Box<Self>,
        args: Vec<Self>,
    },

    /// Atom
    String(String),
    Variable(String),
    Int(Int),
}

type RecFnCall<'a> = Recursive<'a, char, Expr, Simple<char>>;

pub(crate) fn args(
    fn_call: Option<RecFnCall>,
) -> impl Parser<char, Vec<Expr>, Error = Simple<char>> + Clone + '_ {
    expr(fn_call).separated_by(just(',')).boxed()
}

pub(crate) fn expr(
    fn_call: Option<RecFnCall>,
) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + '_ {
    expr8(fn_call.clone())
        .then(just("||").to(Expr::Or).then(expr8(fn_call)).repeated())
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
        .boxed()
}

fn expr8(fn_call: Option<RecFnCall>) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + '_ {
    expr7(fn_call.clone())
        .then(just("&&").to(Expr::And).then(expr7(fn_call)).repeated())
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
        .boxed()
}

fn expr7(fn_call: Option<RecFnCall>) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + '_ {
    expr6(fn_call.clone())
        .then(
            choice((
                just("!=").to(Expr::Neq as fn(_, _) -> _),
                just("==").to(Expr::Eq as fn(_, _) -> _),
                just(">=").to(Expr::Gte as fn(_, _) -> _),
                just("<=").to(Expr::Lte as fn(_, _) -> _),
                just('>').to(Expr::Gt as fn(_, _) -> _),
                just('<').to(Expr::Lt as fn(_, _) -> _),
            ))
            .then(expr6(fn_call))
            .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
        .boxed()
}

fn expr6(fn_call: Option<RecFnCall>) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + '_ {
    expr5(fn_call.clone())
        .then(just('|').to(Expr::BitOr).then(expr5(fn_call)).repeated())
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
        .boxed()
}

fn expr5(fn_call: Option<RecFnCall>) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + '_ {
    expr4(fn_call.clone())
        .then(just('^').to(Expr::BitXor).then(expr4(fn_call)).repeated())
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
        .boxed()
}

fn expr4(fn_call: Option<RecFnCall>) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + '_ {
    expr3(fn_call.clone())
        .then(just('&').to(Expr::BitAnd).then(expr3(fn_call)).repeated())
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
        .boxed()
}

fn expr3(fn_call: Option<RecFnCall>) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + '_ {
    expr2(fn_call.clone())
        .then(
            choice((
                just("<<").to(Expr::Shl as fn(_, _) -> _),
                just(">>").to(Expr::Shr as fn(_, _) -> _),
            ))
            .then(expr2(fn_call))
            .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
        .boxed()
}

fn expr2(fn_call: Option<RecFnCall>) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + '_ {
    expr1(fn_call.clone())
        .then(
            choice((
                just('+').to(Expr::Add as fn(_, _) -> _),
                just('-').to(Expr::Sub as fn(_, _) -> _),
            ))
            .then(expr1(fn_call))
            .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
        .boxed()
}

fn expr1(fn_call: Option<RecFnCall>) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + '_ {
    cast(fn_call.clone())
        .then(
            choice((
                just('*').to(Expr::Mul as fn(_, _) -> _),
                just('/').to(Expr::Div as fn(_, _) -> _),
                just('%').to(Expr::Rem as fn(_, _) -> _),
            ))
            .then(cast(fn_call))
            .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
        .boxed()
}

// cast expr: expr as type as type
pub(crate) fn cast(
    fn_call_rec: Option<RecFnCall>,
) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + '_ {
    let as_expr = just("as").to(Expr::As).then(typeref().padded()).repeated();

    match fn_call_rec {
        None => fn_call()
            .then(as_expr)
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), rhs))
            .boxed(),
        Some(fn_call_rec) => fn_call_rec
            .then(as_expr)
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), rhs))
            .boxed(),
    }
}

// fn(a1, a2)
fn fn_call() -> impl Parser<char, Expr, Error = Simple<char>> + Clone {
    recursive(|fn_call| {
        primary()
            .then(
                args(Some(fn_call))
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
    fn expr_test() {
        assert_eq!(
            expr(None).parse("1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10"),
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

        assert_eq!(expr(None).parse("1"), Ok(Expr::Int(Int::I32(1))));
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
    fn cast_test() {
        assert_eq!(
            cast(None).parse("var as cast"),
            Ok(Expr::As(
                Box::new(Expr::Variable("var".to_string())),
                Type::User("cast".to_string())
            ))
        );
        assert_eq!(
            cast(None).parse("127 as u8"),
            Ok(Expr::As(Box::new(Expr::Int(Int::I32(127))), Type::U8))
        );
        // TODO: This should also work
        // assert_eq!(
        //     cast(None).parse("fun(a1 as char, a2 as u64) as u8"), todo!()
        // );

        // cast should parse fn_call as well
        assert_eq!(cast(None).parse("1"), Ok(Expr::Int(Int::I32(1))));
        assert_eq!(cast(None).parse("'a'"), Ok(Expr::Int(Int::I8(97))));
        assert_eq!(cast(None).parse("\"a\""), Ok(Expr::String("a".to_string())));
        assert_eq!(
            cast(None).parse("var"),
            Ok(Expr::Variable("var".to_string()))
        );
    }

    #[test]
    fn fn_call_test() {
        assert_eq!(
            fn_call().parse("fun(1, a2, '3', \"4\")"),
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
        assert_eq!(
            fn_call().parse("fun(a1 as char, a2 as u64)"),
            Ok(Expr::FnCall {
                name: Box::new(Expr::Variable("fun".to_string())),
                args: vec![
                    Expr::As(Box::new(Expr::Variable("a1".to_string())), Type::I8),
                    Expr::As(Box::new(Expr::Variable("a2".to_string())), Type::U64),
                ]
            })
        );

        assert_eq!(fn_call().parse("1"), Ok(Expr::Int(Int::I32(1))));
        assert_eq!(fn_call().parse("'a'"), Ok(Expr::Int(Int::I8(97))));
        assert_eq!(fn_call().parse("\"a\""), Ok(Expr::String("a".to_string())));
        assert_eq!(
            fn_call().parse("var"),
            Ok(Expr::Variable("var".to_string()))
        );
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
