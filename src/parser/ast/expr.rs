use crate::parser::ast::{character, integer, string, typeref, variable, Spanned, Type};
use crate::parser::lib::*;

#[derive(Debug, PartialEq, Clone)]
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

pub(crate) fn args(fn_call: Option<Rec<Spanned<Expr>>>) -> impl Parser<Vec<Spanned<Expr>>> + '_ {
    expr(fn_call).separated_by(just(',')).boxed()
}

pub(crate) fn expr(fn_call: Option<Rec<Spanned<Expr>>>) -> impl Parser<Spanned<Expr>> + '_ {
    expr8(fn_call.clone())
        .then(just("||").to(Expr::Or).then(expr8(fn_call)).repeated())
        .foldl(|lhs, (op, rhs)| {
            let span = lhs.span.union(rhs.span);
            Spanned::new(op(lhs, rhs), span)
        })
        .boxed()
}

fn expr8(fn_call: Option<Rec<Spanned<Expr>>>) -> impl Parser<Spanned<Expr>> + '_ {
    expr7(fn_call.clone())
        .then(just("&&").to(Expr::And).then(expr7(fn_call)).repeated())
        .foldl(|lhs, (op, rhs)| {
            let span = lhs.span.union(rhs.span);
            Spanned::new(op(lhs, rhs), span)
        })
        .boxed()
}

fn expr7(fn_call: Option<Rec<Spanned<Expr>>>) -> impl Parser<Spanned<Expr>> + '_ {
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
        .foldl(|lhs, (op, rhs)| {
            let span = lhs.span.union(rhs.span);
            Spanned::new(op(lhs, rhs), span)
        })
        .boxed()
}

fn expr6(fn_call: Option<Rec<Spanned<Expr>>>) -> impl Parser<Spanned<Expr>> + '_ {
    expr5(fn_call.clone())
        .then(just('|').to(Expr::BitOr).then(expr5(fn_call)).repeated())
        .foldl(|lhs, (op, rhs)| {
            let span = lhs.span.union(rhs.span);
            Spanned::new(op(lhs, rhs), span)
        })
        .boxed()
}

fn expr5(fn_call: Option<Rec<Spanned<Expr>>>) -> impl Parser<Spanned<Expr>> + '_ {
    expr4(fn_call.clone())
        .then(just('^').to(Expr::BitXor).then(expr4(fn_call)).repeated())
        .foldl(|lhs, (op, rhs)| {
            let span = lhs.span.union(rhs.span);
            Spanned::new(op(lhs, rhs), span)
        })
        .boxed()
}

fn expr4(fn_call: Option<Rec<Spanned<Expr>>>) -> impl Parser<Spanned<Expr>> + '_ {
    expr3(fn_call.clone())
        .then(just('&').to(Expr::BitAnd).then(expr3(fn_call)).repeated())
        .foldl(|lhs, (op, rhs)| {
            let span = lhs.span.union(rhs.span);
            Spanned::new(op(lhs, rhs), span)
        })
        .boxed()
}

fn expr3(fn_call: Option<Rec<Spanned<Expr>>>) -> impl Parser<Spanned<Expr>> + '_ {
    expr2(fn_call.clone())
        .then(
            choice((
                just("<<").to(Expr::Shl as fn(_, _) -> _),
                just(">>").to(Expr::Shr as fn(_, _) -> _),
            ))
            .then(expr2(fn_call))
            .repeated(),
        )
        .foldl(|lhs, (op, rhs)| {
            let span = lhs.span.union(rhs.span);
            Spanned::new(op(lhs, rhs), span)
        })
        .boxed()
}

fn expr2(fn_call: Option<Rec<Spanned<Expr>>>) -> impl Parser<Spanned<Expr>> + '_ {
    expr1(fn_call.clone())
        .then(
            choice((
                just('+').to(Expr::Add as fn(_, _) -> _),
                just('-').to(Expr::Sub as fn(_, _) -> _),
            ))
            .then(expr1(fn_call))
            .repeated(),
        )
        .foldl(|lhs, (op, rhs)| {
            let span = lhs.span.union(rhs.span);
            Spanned::new(op(lhs, rhs), span)
        })
        .boxed()
}

fn expr1(fn_call: Option<Rec<Spanned<Expr>>>) -> impl Parser<Spanned<Expr>> + '_ {
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
        .foldl(|lhs, (op, rhs)| {
            let span = lhs.span.union(rhs.span);
            Spanned::new(op(lhs, rhs), span)
        })
        .boxed()
}

// cast expr: expr as type as type
pub(crate) fn cast(fn_call_rec: Option<Rec<Spanned<Expr>>>) -> impl Parser<Spanned<Expr>> + '_ {
    let as_expr = just("as").to(Expr::As).then(typeref().padded()).repeated();

    match fn_call_rec {
        None => fn_call()
            .then(as_expr)
            .foldl(|lhs, (op, rhs)| {
                let span = lhs.span.union(rhs.span);
                Spanned::new(op(lhs, rhs), span)
            })
            .boxed(),
        Some(fn_call_rec) => fn_call_rec
            .then(as_expr)
            .foldl(|lhs, (op, rhs)| {
                let span = lhs.span.union(rhs.span);
                Spanned::new(op(lhs, rhs), span)
            })
            .boxed(),
    }
}

// fn(a1, a2)
fn fn_call() -> impl Parser<Spanned<Expr>> {
    recursive(|fn_call| {
        primary()
            .then(
                args(Some(fn_call))
                    .delimited_by(just('('), just(')'))
                    .repeated(),
            )
            .foldl(|name, args| {
                let span = if let Some(first) = args.first() {
                    // Merge args spans into one span
                    name.span.union(
                        args.iter()
                            .map(|f| f.span)
                            .fold(first.span, |acc, xs| acc.union(xs)),
                    )
                } else {
                    // No args found
                    name.span
                };
                Spanned::new(Expr::FnCall { name, args }, span)
            })
    })
    .boxed()
}

fn primary() -> impl Parser<Spanned<Expr>> {
    choice((integer(), character(), string(), variable()))
        .padded()
        .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args() {
        assert_eq!(
            args(None).parse_test("1, var, 1 +1"),
            Ok(vec![
                Spanned::any(Expr::I32(1)),
                Spanned::any(Expr::Variable("var".to_string())),
                Spanned::any(Expr::Add(
                    Spanned::any(Expr::I32(1)),
                    Spanned::any(Expr::I32(1)),
                )),
            ])
        );
        assert_eq!(
            args(None).parse_test("1"),
            Ok(vec![Spanned::any(Expr::I32(1))])
        );
    }

    #[test]
    fn test_expr() {
        assert_eq!(
            expr(None).parse_test("1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10"),
            Ok(Spanned::any(Expr::Or(
                Spanned::any(Expr::I32(1)),
                Spanned::any(Expr::And(
                    Spanned::any(Expr::I32(2)),
                    Spanned::any(Expr::Neq(
                        Spanned::any(Expr::I32(3)),
                        Spanned::any(Expr::BitOr(
                            Spanned::any(Expr::I32(4)),
                            Spanned::any(Expr::BitXor(
                                Spanned::any(Expr::I32(5)),
                                Spanned::any(Expr::BitAnd(
                                    Spanned::any(Expr::I32(6)),
                                    Spanned::any(Expr::Shl(
                                        Spanned::any(Expr::I32(7)),
                                        Spanned::any(Expr::Add(
                                            Spanned::any(Expr::I32(8)),
                                            Spanned::any(Expr::Mul(
                                                Spanned::any(Expr::I32(9)),
                                                Spanned::any(Expr::I32(10))
                                            )),
                                        )),
                                    )),
                                )),
                            )),
                        )),
                    )),
                )),
            )))
        );

        assert_eq!(expr(None).parse_test("1"), Ok(Spanned::any(Expr::I32(1))));
    }

    #[test]
    fn test_expr8() {
        assert_eq!(
            expr8(None).parse_test("1 && 2 != 3 | 4 ^ 5 & 6 << 7 + 8*9"),
            Ok(Spanned::any(Expr::And(
                Spanned::any(Expr::I32(1)),
                Spanned::any(Expr::Neq(
                    Spanned::any(Expr::I32(2)),
                    Spanned::any(Expr::BitOr(
                        Spanned::any(Expr::I32(3)),
                        Spanned::any(Expr::BitXor(
                            Spanned::any(Expr::I32(4)),
                            Spanned::any(Expr::BitAnd(
                                Spanned::any(Expr::I32(5)),
                                Spanned::any(Expr::Shl(
                                    Spanned::any(Expr::I32(6)),
                                    Spanned::any(Expr::Add(
                                        Spanned::any(Expr::I32(7)),
                                        Spanned::any(Expr::Mul(
                                            Spanned::any(Expr::I32(8)),
                                            Spanned::any(Expr::I32(9))
                                        )),
                                    )),
                                )),
                            )),
                        )),
                    )),
                )),
            )))
        );

        assert_eq!(expr8(None).parse_test("1"), Ok(Spanned::any(Expr::I32(1))));
    }

    #[test]
    fn test_expr7() {
        let expr = Spanned::any(Expr::BitOr(
            Spanned::any(Expr::I32(2)),
            Spanned::any(Expr::BitXor(
                Spanned::any(Expr::I32(3)),
                Spanned::any(Expr::BitAnd(
                    Spanned::any(Expr::I32(4)),
                    Spanned::any(Expr::Shl(
                        Spanned::any(Expr::I32(5)),
                        Spanned::any(Expr::Add(
                            Spanned::any(Expr::I32(6)),
                            Spanned::any(Expr::Mul(
                                Spanned::any(Expr::I32(7)),
                                Spanned::any(Expr::I32(8)),
                            )),
                        )),
                    )),
                )),
            )),
        ));

        assert_eq!(
            expr7(None).parse_test("1 != 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Spanned::any(Expr::Neq(
                Spanned::any(Expr::I32(1)),
                expr.clone()
            )))
        );
        assert_eq!(
            expr7(None).parse_test("1 == 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Spanned::any(Expr::Eq(
                Spanned::any(Expr::I32(1)),
                expr.clone()
            )))
        );
        assert_eq!(
            expr7(None).parse_test("1 >= 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Spanned::any(Expr::Gte(
                Spanned::any(Expr::I32(1)),
                expr.clone()
            )))
        );
        assert_eq!(
            expr7(None).parse_test("1 <= 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Spanned::any(Expr::Lte(
                Spanned::any(Expr::I32(1)),
                expr.clone()
            )))
        );
        assert_eq!(
            expr7(None).parse_test("1 > 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Spanned::any(Expr::Gt(
                Spanned::any(Expr::I32(1)),
                expr.clone()
            )))
        );
        assert_eq!(
            expr7(None).parse_test("1 < 2 | 3 ^ 4 & 5 << 6 + 7*8"),
            Ok(Spanned::any(Expr::Lt(Spanned::any(Expr::I32(1)), expr)))
        );

        assert_eq!(expr7(None).parse_test("1"), Ok(Spanned::any(Expr::I32(1))));
    }

    #[test]
    fn test_expr6() {
        assert_eq!(
            expr6(None).parse_test("1 | 2 ^ 3 & 4 << 5 + 6*7"),
            Ok(Spanned::any(Expr::BitOr(
                Spanned::any(Expr::I32(1)),
                Spanned::any(Expr::BitXor(
                    Spanned::any(Expr::I32(2)),
                    Spanned::any(Expr::BitAnd(
                        Spanned::any(Expr::I32(3)),
                        Spanned::any(Expr::Shl(
                            Spanned::any(Expr::I32(4)),
                            Spanned::any(Expr::Add(
                                Spanned::any(Expr::I32(5)),
                                Spanned::any(Expr::Mul(
                                    Spanned::any(Expr::I32(6)),
                                    Spanned::any(Expr::I32(7))
                                )),
                            )),
                        )),
                    )),
                )),
            )))
        );

        assert_eq!(expr6(None).parse_test("1"), Ok(Spanned::any(Expr::I32(1))));
    }

    #[test]
    fn test_expr5() {
        assert_eq!(
            expr5(None).parse_test("1 ^ 2 & 3 << 4 + 5*6"),
            Ok(Spanned::any(Expr::BitXor(
                Spanned::any(Expr::I32(1)),
                Spanned::any(Expr::BitAnd(
                    Spanned::any(Expr::I32(2)),
                    Spanned::any(Expr::Shl(
                        Spanned::any(Expr::I32(3)),
                        Spanned::any(Expr::Add(
                            Spanned::any(Expr::I32(4)),
                            Spanned::any(Expr::Mul(
                                Spanned::any(Expr::I32(5)),
                                Spanned::any(Expr::I32(6))
                            )),
                        )),
                    )),
                )),
            )))
        );

        assert_eq!(expr5(None).parse_test("1"), Ok(Spanned::any(Expr::I32(1))));
    }

    #[test]
    fn test_expr4() {
        assert_eq!(
            expr4(None).parse_test("1 & 2 << 3 + 4*5"),
            Ok(Spanned::any(Expr::BitAnd(
                Spanned::any(Expr::I32(1)),
                Spanned::any(Expr::Shl(
                    Spanned::any(Expr::I32(2)),
                    Spanned::any(Expr::Add(
                        Spanned::any(Expr::I32(3)),
                        Spanned::any(Expr::Mul(
                            Spanned::any(Expr::I32(4)),
                            Spanned::any(Expr::I32(5))
                        )),
                    )),
                )),
            )))
        );

        assert_eq!(expr4(None).parse_test("1"), Ok(Spanned::any(Expr::I32(1))));
    }

    #[test]
    fn test_expr3() {
        assert_eq!(
            expr3(None).parse_test("1 << 2 + 3*4"),
            Ok(Spanned::any(Expr::Shl(
                Spanned::any(Expr::I32(1)),
                Spanned::any(Expr::Add(
                    Spanned::any(Expr::I32(2)),
                    Spanned::any(Expr::Mul(
                        Spanned::any(Expr::I32(3)),
                        Spanned::any(Expr::I32(4))
                    )),
                )),
            )))
        );

        assert_eq!(
            expr3(None).parse_test("1 >> 2 + 3*4"),
            Ok(Spanned::any(Expr::Shr(
                Spanned::any(Expr::I32(1)),
                Spanned::any(Expr::Add(
                    Spanned::any(Expr::I32(2)),
                    Spanned::any(Expr::Mul(
                        Spanned::any(Expr::I32(3)),
                        Spanned::any(Expr::I32(4))
                    )),
                )),
            )))
        );

        assert_eq!(expr3(None).parse_test("1"), Ok(Spanned::any(Expr::I32(1))));
    }

    #[test]
    fn test_expr2() {
        assert_eq!(
            expr2(None).parse_test("1 + 2*3"),
            Ok(Spanned::any(Expr::Add(
                Spanned::any(Expr::I32(1)),
                Spanned::any(Expr::Mul(
                    Spanned::any(Expr::I32(2)),
                    Spanned::any(Expr::I32(3))
                )),
            )))
        );

        assert_eq!(
            expr2(None).parse_test("1 - 2*3"),
            Ok(Spanned::any(Expr::Sub(
                Spanned::any(Expr::I32(1)),
                Spanned::any(Expr::Mul(
                    Spanned::any(Expr::I32(2)),
                    Spanned::any(Expr::I32(3))
                )),
            )))
        );

        assert_eq!(
            expr2(None).parse_test("1*2 + 3*4"),
            Ok(Spanned::any(Expr::Add(
                Spanned::any(Expr::Mul(
                    Spanned::any(Expr::I32(1)),
                    Spanned::any(Expr::I32(2))
                )),
                Spanned::any(Expr::Mul(
                    Spanned::any(Expr::I32(3)),
                    Spanned::any(Expr::I32(4))
                )),
            )))
        );

        assert_eq!(expr2(None).parse_test("1"), Ok(Spanned::any(Expr::I32(1))));
    }

    #[test]
    fn test_expr1() {
        assert_eq!(
            expr1(None).parse_test("1*1"),
            Ok(Spanned::any(Expr::Mul(
                Spanned::any(Expr::I32(1)),
                Spanned::any(Expr::I32(1)),
            )))
        );
        assert_eq!(
            expr1(None).parse_test("1 / 1"),
            Ok(Spanned::any(Expr::Div(
                Spanned::any(Expr::I32(1)),
                Spanned::any(Expr::I32(1)),
            )))
        );
        assert_eq!(
            expr1(None).parse_test("1 %2"),
            Ok(Spanned::any(Expr::Rem(
                Spanned::any(Expr::I32(1)),
                Spanned::any(Expr::I32(2)),
            )))
        );

        assert_eq!(
            expr1(None).parse_test("1 % 2 / 3 * 4"),
            Ok(Spanned::any(Expr::Mul(
                Spanned::any(Expr::Div(
                    Spanned::any(Expr::Rem(
                        Spanned::any(Expr::I32(1)),
                        Spanned::any(Expr::I32(2))
                    )),
                    Spanned::any(Expr::I32(3))
                )),
                Spanned::any(Expr::I32(4)),
            )))
        );

        assert_eq!(expr1(None).parse_test("1"), Ok(Spanned::any(Expr::I32(1))));
    }

    #[test]
    fn test_cast() {
        assert_eq!(
            cast(None).parse_test("var as cast"),
            Ok(Spanned::any(Expr::As(
                Spanned::any(Expr::Variable("var".to_string())),
                Spanned::any(Type::User("cast".to_string()))
            )))
        );
        assert_eq!(
            cast(None).parse_test("127 as u8"),
            Ok(Spanned::any(Expr::As(
                Spanned::any(Expr::I32(127)),
                Spanned::any(Type::U8)
            )))
        );
        // TODO: This should also work
        // assert_eq!(
        //     cast(None).parse_test("fun(a1 as char, a2 as u64) as u8"), todo!()
        // );

        // cast should parse_test fn_call as well
        assert_eq!(cast(None).parse_test("1"), Ok(Spanned::any(Expr::I32(1))));
        assert_eq!(cast(None).parse_test("'a'"), Ok(Spanned::any(Expr::I8(97))));
        assert_eq!(
            cast(None).parse_test("\"a\""),
            Ok(Spanned::any(Expr::String("a".to_string())))
        );
        assert_eq!(
            cast(None).parse_test("var"),
            Ok(Spanned::any(Expr::Variable("var".to_string())))
        );
    }

    #[test]
    fn test_fn_call() {
        assert_eq!(
            fn_call().parse_test("fun(1, a2, '3', \"4\")"),
            Ok(Spanned::any(Expr::FnCall {
                name: Spanned::any(Expr::Variable("fun".to_string())),
                args: vec![
                    Spanned::any(Expr::I32(1)),
                    Spanned::any(Expr::Variable("a2".to_string())),
                    Spanned::any(Expr::I8(51)),
                    Spanned::any(Expr::String("4".to_string())),
                ]
            }))
        );
        assert_eq!(
            fn_call().parse_test("fun(a1 as char, a2 as u64)"),
            Ok(Spanned::any(Expr::FnCall {
                name: Spanned::any(Expr::Variable("fun".to_string())),
                args: vec![
                    Spanned::any(Expr::As(
                        Spanned::any(Expr::Variable("a1".to_string())),
                        Spanned::any(Type::I8)
                    )),
                    Spanned::any(Expr::As(
                        Spanned::any(Expr::Variable("a2".to_string())),
                        Spanned::any(Type::U64)
                    )),
                ]
            }))
        );

        assert_eq!(fn_call().parse_test("1"), Ok(Spanned::any(Expr::I32(1))));
        assert_eq!(fn_call().parse_test("'a'"), Ok(Spanned::any(Expr::I8(97))));
        assert_eq!(
            fn_call().parse_test("\"a\""),
            Ok(Spanned::any(Expr::String("a".to_string())))
        );
        assert_eq!(
            fn_call().parse_test("var"),
            Ok(Spanned::any(Expr::Variable("var".to_string())))
        );
    }

    #[test]
    fn test_primary() {
        assert_eq!(primary().parse_test("1"), Ok(Spanned::any(Expr::I32(1))));
        assert_eq!(primary().parse_test("'a'"), Ok(Spanned::any(Expr::I8(97))));
        assert_eq!(
            primary().parse_test("\"a\""),
            Ok(Spanned::any(Expr::String("a".to_string())))
        );
        assert_eq!(
            primary().parse_test("var"),
            Ok(Spanned::any(Expr::Variable("var".to_string())))
        );
    }
}
