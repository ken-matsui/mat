use crate::parser::lib::*;
use crate::parser::{cast, comment, expr, ident, typedef, typeref};
use matc_ast::{Param, Stmt};
use matc_span::Spanned;

// import std.io;
pub(crate) fn import_stmt() -> impl Parser<Spanned<Stmt>> {
    text::keyword("import")
        .ignore_then(
            ident()
                .repeated()
                .separated_by(just('.'))
                .map(|i| i.into_iter().flatten().collect::<Vec<String>>().join(".")),
        )
        .then_ignore(just(';'))
        .map_with_span(|import, span| Spanned::new(Stmt::Import(import), span))
        .labelled("import")
        .padded()
        .boxed()
}

pub(crate) fn top_defs() -> impl Parser<Vec<Spanned<Stmt>>> {
    choice((defvar(), defn(), typedef())).repeated().boxed()
}

// name1: type1
fn param() -> impl Parser<Param> {
    text::keyword("mut")
        .or_not()
        .padded()
        .then(ident().map_with_span(Spanned::new))
        .then_ignore(just(':'))
        .then(typeref().padded())
        .map(|((mt, name), ty)| Param {
            is_mut: mt.is_some(),
            name,
            ty,
        })
        .boxed()
}

// fn name(...) -> type {}
fn defn() -> impl Parser<Spanned<Stmt>> {
    text::keyword("fn")
        .padded()
        .ignore_then(ident().map_with_span(Spanned::new))
        .then(
            param()
                .padded()
                .separated_by(just(','))
                .delimited_by(just('('), just(')')),
        )
        .padded()
        .then_ignore(just("->"))
        .then(typeref().padded())
        .then(block(None))
        .map_with_span(|(((name, args), ret_ty), body), span| {
            Spanned::new(
                Stmt::DefFn {
                    name,
                    args,
                    ret_ty,
                    body,
                },
                span,
            )
        })
        .boxed()
}

// let mut var: type = expr;
fn defvar() -> impl Parser<Spanned<Stmt>> {
    text::keyword("let")
        .padded()
        .ignore_then(just("mut").or_not())
        .then(ident().map_with_span(Spanned::new))
        .then_ignore(just(':'))
        .then(typeref().padded())
        .then(just('=').ignore_then(expr(None)).or_not())
        .then_ignore(just(';'))
        .map_with_span(|(((mt, nm), ty), expr), span| {
            Spanned::new(
                Stmt::DefVar {
                    is_mut: mt.is_some(),
                    name: nm,
                    ty,
                    expr,
                },
                span,
            )
        })
        .labelled("variable")
        .padded()
        .boxed()
}

// TODO: defstruct
// struct name {
//     member: type,
//     ...
// }

fn block(if_stmt: Option<Rec<Spanned<Stmt>>>) -> impl Parser<Spanned<Stmt>> + '_ {
    recursive(|block| {
        defvar()
            .or(stmt(Some(block), if_stmt))
            .padded_by(comment().padded().repeated()) // TODO: implement lexer to simplify comment treatments?
            .repeated()
            .padded()
            .delimited_by(just('{'), just('}'))
            .map_with_span(|block, span| Spanned::new(Stmt::Block(block), span))
            .boxed()
    })
}

fn stmt<'a>(
    block_rec: Option<Rec<'a, Spanned<Stmt>>>,
    if_stmt_rec: Option<Rec<'a, Spanned<Stmt>>>,
) -> impl Parser<Spanned<Stmt>> + 'a {
    let empty = just(';')
        .padded()
        .to(Stmt::Empty)
        .map_with_span(Spanned::new);

    match (block_rec, if_stmt_rec) {
        (None, None) => {
            choice((empty, return_stmt(), assign_stmt(), block(None), if_stmt())).boxed()
        }
        (Some(block_rec), None) => {
            choice((empty, return_stmt(), assign_stmt(), block_rec, if_stmt())).boxed()
        }
        (None, Some(if_stmt_rec)) => choice((
            empty,
            return_stmt(),
            assign_stmt(),
            block(Some(if_stmt_rec.clone())),
            if_stmt_rec,
        ))
        .boxed(),
        (Some(block_rec), Some(if_stmt_rec)) => {
            choice((empty, return_stmt(), assign_stmt(), block_rec, if_stmt_rec)).boxed()
        }
    }
}

// if expr {
// } else if expr {
// } else {
// }
fn if_stmt() -> impl Parser<Spanned<Stmt>> {
    recursive(|if_stmt| {
        text::keyword("if")
            .padded()
            .ignore_then(expr(None))
            .then(block(Some(if_stmt.clone())))
            .then(
                text::keyword("else")
                    .padded()
                    .ignore_then(block(Some(if_stmt.clone())).or(if_stmt))
                    .or_not(),
            )
            .map_with_span(|((cond, then), els), span| {
                Spanned::new(Stmt::If { cond, then, els }, span)
            })
    })
    .boxed()
}

fn return_stmt() -> impl Parser<Spanned<Stmt>> {
    text::keyword("return")
        .padded()
        .ignore_then(expr(None).or_not())
        .map(Stmt::Return)
        .then_ignore(just(';'))
        .map_with_span(Spanned::new)
        .boxed()
}

fn assign_stmt() -> impl Parser<Spanned<Stmt>> {
    choice((
        cast(None)
            .then(
                choice((
                    just('=').to(Stmt::Assign as fn(_, _) -> _),
                    just("+=").to(Stmt::AddAssign as fn(_, _) -> _),
                    just("-=").to(Stmt::SubAssign as fn(_, _) -> _),
                    just("*=").to(Stmt::MulAssign as fn(_, _) -> _),
                    just("/=").to(Stmt::DivAssign as fn(_, _) -> _),
                    just("%=").to(Stmt::RemAssign as fn(_, _) -> _),
                    just("&=").to(Stmt::BitAndAssign as fn(_, _) -> _),
                    just("|=").to(Stmt::BitOrAssign as fn(_, _) -> _),
                    just("^=").to(Stmt::BitXorAssign as fn(_, _) -> _),
                    just("<<=").to(Stmt::ShlAssign as fn(_, _) -> _),
                    just(">>=").to(Stmt::ShrAssign as fn(_, _) -> _),
                ))
                // Here, this is not expr() because I would not allow multiple assignments like a = b = c;
                .then(expr(None)),
            )
            .map(|(lhs, (op, rhs))| op(lhs, rhs)),
        expr(None).map(Stmt::Expr),
    ))
    .then_ignore(just(';'))
    .map_with_span(Spanned::new)
    .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;
    use matc_ast::{Expr, Type};

    #[test]
    fn test_import_stmt() {
        assert_eq!(
            import_stmt().parse_test("import std.io;"),
            Ok(Spanned::any(Stmt::Import("std.io".to_string())))
        );
        assert_eq!(
            import_stmt().parse_test("import     std  .   io   ;"),
            Ok(Spanned::any(Stmt::Import("std.io".to_string())))
        );
        assert_eq!(
            import_stmt().parse_test("import stdio;"),
            Ok(Spanned::any(Stmt::Import("stdio".to_string())))
        );
        assert!(import_stmt().parse_test("import 1std.io;").is_err());
        assert!(import_stmt().parse_test("import std.1io;").is_err());
        assert!(import_stmt().parse_test("import std.io").is_err());
        assert!(import_stmt().parse_test("use std.io;").is_err());
    }

    #[test]
    fn test_top_defs() {
        assert_eq!(top_defs().parse_test(""), Ok(vec![]));
        assert_eq!(
            top_defs().parse_test(
                r#"
                let foo: i8 = 1; type newint = i32;

                fn f1() -> u32 {}
        "#
            ),
            Ok(vec![
                Spanned::any(Stmt::DefVar {
                    is_mut: false,
                    name: Spanned::any("foo".to_string()),
                    ty: Spanned::any(Type::I8),
                    expr: Some(Spanned::any(Expr::I32(1))),
                }),
                Spanned::any(Stmt::TypeDef {
                    name: Spanned::any("newint".to_string()),
                    ty: Spanned::any(Type::I32),
                }),
                Spanned::any(Stmt::DefFn {
                    name: Spanned::any("f1".to_string()),
                    args: vec![],
                    ret_ty: Spanned::any(Type::U32),
                    body: Spanned::any(Stmt::Block(vec![])),
                }),
            ])
        );
    }

    #[test]
    fn test_param() {
        assert_eq!(
            param().parse_test("name: i8"),
            Ok(Param {
                is_mut: false,
                name: Spanned::any("name".to_string()),
                ty: Spanned::any(Type::I8)
            })
        );
        assert_eq!(
            param().parse_test("mut name: i8"),
            Ok(Param {
                is_mut: true,
                name: Spanned::any("name".to_string()),
                ty: Spanned::any(Type::I8)
            })
        );
    }

    #[test]
    fn test_defn() {
        assert_eq!(
            defn().parse_test("fn name() -> i16 {}"),
            Ok(Spanned::any(Stmt::DefFn {
                name: Spanned::any("name".to_string()),
                args: vec![],
                ret_ty: Spanned::any(Type::I16),
                body: Spanned::any(Stmt::Block(vec![])),
            }))
        );
        assert_eq!(
            defn().parse_test("fn name(a1: i8) -> i16 {}"),
            Ok(Spanned::any(Stmt::DefFn {
                name: Spanned::any("name".to_string()),
                args: vec![Param {
                    is_mut: false,
                    name: Spanned::any("a1".to_string()),
                    ty: Spanned::any(Type::I8)
                }],
                ret_ty: Spanned::any(Type::I16),
                body: Spanned::any(Stmt::Block(vec![])),
            }))
        );

        assert!(defn().parse_test("fn name(): i16 {}").is_err());
    }

    #[test]
    fn test_defvar() {
        assert_eq!(
            defvar().parse_test("let var: type = 10;"),
            Ok(Spanned::any(Stmt::DefVar {
                is_mut: false,
                name: Spanned::any("var".to_string()),
                ty: Spanned::any(Type::User("type".to_string())),
                expr: Some(Spanned::any(Expr::I32(10))),
            }))
        );
        assert_eq!(
            defvar().parse_test("let mut var: type = 10;"),
            Ok(Spanned::any(Stmt::DefVar {
                is_mut: true,
                name: Spanned::any("var".to_string()),
                ty: Spanned::any(Type::User("type".to_string())),
                expr: Some(Spanned::any(Expr::I32(10))),
            }))
        );
        assert_eq!(
            defvar().parse_test("let mut var: type;"),
            Ok(Spanned::any(Stmt::DefVar {
                is_mut: true,
                name: Spanned::any("var".to_string()),
                ty: Spanned::any(Type::User("type".to_string())),
                expr: None,
            }))
        );
        assert_eq!(
            defvar().parse_test("let   mut   var    :   type     ;"),
            Ok(Spanned::any(Stmt::DefVar {
                is_mut: true,
                name: Spanned::any("var".to_string()),
                ty: Spanned::any(Type::User("type".to_string())),
                expr: None,
            }))
        );

        assert!(defvar().parse_test("let var: type = 10").is_err());
        assert!(defvar().parse_test("let mut var: type = 10").is_err());

        assert!(defvar().parse_test("let var := 10;").is_err());
        assert!(defvar().parse_test("let mut var := 10;").is_err());
    }

    #[test]
    fn test_block() {
        assert_eq!(
            block(None).parse_test("{}"),
            Ok(Spanned::any(Stmt::Block(vec![])))
        );
        assert_eq!(
            block(None).parse_test("{     }"),
            Ok(Spanned::any(Stmt::Block(vec![])))
        );
        assert_eq!(
            block(None).parse_test(
                r#"{
                let var1: type = 10;
    
                let mut var2: type = 10;
                // comment
                if var1 {}
            }"#
            ),
            Ok(Spanned::any(Stmt::Block(vec![
                Spanned::any(Stmt::DefVar {
                    is_mut: false,
                    name: Spanned::any("var1".to_string()),
                    ty: Spanned::any(Type::User("type".to_string())),
                    expr: Some(Spanned::any(Expr::I32(10))),
                }),
                Spanned::any(Stmt::DefVar {
                    is_mut: true,
                    name: Spanned::any("var2".to_string()),
                    ty: Spanned::any(Type::User("type".to_string())),
                    expr: Some(Spanned::any(Expr::I32(10))),
                }),
                Spanned::any(Stmt::If {
                    cond: Spanned::any(Expr::Variable("var1".to_string())),
                    then: Spanned::any(Stmt::Block(vec![])),
                    els: None,
                }),
            ])))
        );
        assert!(block(None).parse_test("{     ").is_err());
        assert!(block(None).parse_test("  }").is_err());
        assert!(block(None).parse_test("let var: type = 10;").is_err());
    }

    #[test]
    fn test_stmt() {
        assert_eq!(
            stmt(None, None).parse_test(";"),
            Ok(Spanned::any(Stmt::Empty))
        );
        assert_eq!(
            stmt(None, None).parse_test("var = 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10 ;"),
            Ok(Spanned::any(Stmt::Assign(
                Spanned::any(Expr::Variable("var".to_string())),
                big_expr()
            )))
        );
        assert_eq!(
            stmt(None, None).parse_test("{}"),
            Ok(Spanned::any(Stmt::Block(vec![])))
        );
        assert_eq!(
            stmt(None, None).parse_test("if foo {}"),
            Ok(Spanned::any(Stmt::If {
                cond: Spanned::any(Expr::Variable("foo".to_string())),
                then: Spanned::any(Stmt::Block(vec![])),
                els: None,
            }))
        );
        assert_eq!(
            stmt(None, None).parse_test("return;"),
            Ok(Spanned::any(Stmt::Return(None)))
        );

        // complex statement
        assert_eq!(
            stmt(None, None).parse_test("{ if foo { { return 1; } } }"),
            Ok(Spanned::any(Stmt::Block(vec![Spanned::any(Stmt::If {
                cond: Spanned::any(Expr::Variable("foo".to_string())),
                then: Spanned::any(Stmt::Block(vec![Spanned::any(Stmt::Block(vec![
                    Spanned::any(Stmt::Return(Some(Spanned::any(Expr::I32(1)))))
                ]))])),
                els: None,
            })])))
        );
    }

    #[test]
    fn test_if_stmt() {
        assert_eq!(
            if_stmt().parse_test("if foo { 1; }"),
            Ok(Spanned::any(Stmt::If {
                cond: Spanned::any(Expr::Variable("foo".to_string())),
                then: Spanned::any(Stmt::Block(vec![Spanned::any(Stmt::Expr(Spanned::any(
                    Expr::I32(1)
                )))])),
                els: None,
            }))
        );
        assert_eq!(
            if_stmt().parse_test("if foo { if bar {} }"),
            Ok(Spanned::any(Stmt::If {
                cond: Spanned::any(Expr::Variable("foo".to_string())),
                then: Spanned::any(Stmt::Block(vec![Spanned::any(Stmt::If {
                    cond: Spanned::any(Expr::Variable("bar".to_string())),
                    then: Spanned::any(Stmt::Block(vec![])),
                    els: None,
                })])),
                els: None,
            }))
        );
        assert_eq!(
            if_stmt().parse_test("if foo { 1; } else { 2; }"),
            Ok(Spanned::any(Stmt::If {
                cond: Spanned::any(Expr::Variable("foo".to_string())),
                then: Spanned::any(Stmt::Block(vec![Spanned::any(Stmt::Expr(Spanned::any(
                    Expr::I32(1)
                )))])),
                els: Some(Spanned::any(Stmt::Block(vec![Spanned::any(Stmt::Expr(
                    Spanned::any(Expr::I32(2))
                ))]))),
            }))
        );
        assert_eq!(
            if_stmt().parse_test("if foo { 1; } else if bar { 2; } else { 3; }"),
            Ok(Spanned::any(Stmt::If {
                cond: Spanned::any(Expr::Variable("foo".to_string())),
                then: Spanned::any(Stmt::Block(vec![Spanned::any(Stmt::Expr(Spanned::any(
                    Expr::I32(1)
                )))])),
                els: Some(Spanned::any(Stmt::If {
                    cond: Spanned::any(Expr::Variable("bar".to_string())),
                    then: Spanned::any(Stmt::Block(vec![Spanned::any(Stmt::Expr(Spanned::any(
                        Expr::I32(2)
                    )))])),
                    els: Some(Spanned::any(Stmt::Block(vec![Spanned::any(Stmt::Expr(
                        Spanned::any(Expr::I32(3))
                    ))])))
                })),
            }))
        );
        assert_eq!(
            if_stmt().parse_test("if foo { 1; } else if bar { 2; }"),
            Ok(Spanned::any(Stmt::If {
                cond: Spanned::any(Expr::Variable("foo".to_string())),
                then: Spanned::any(Stmt::Block(vec![Spanned::any(Stmt::Expr(Spanned::any(
                    Expr::I32(1)
                )))])),
                els: Some(Spanned::any(Stmt::If {
                    cond: Spanned::any(Expr::Variable("bar".to_string())),
                    then: Spanned::any(Stmt::Block(vec![Spanned::any(Stmt::Expr(Spanned::any(
                        Expr::I32(2)
                    )))])),
                    els: None,
                })),
            }))
        );
        assert_eq!(
            if_stmt().parse_test(
                "if foo { 1; } else if bar { 2; } else if baz { 3; } else if qux { 4; }"
            ),
            Ok(Spanned::any(Stmt::If {
                cond: Spanned::any(Expr::Variable("foo".to_string())),
                then: Spanned::any(Stmt::Block(vec![Spanned::any(Stmt::Expr(Spanned::any(
                    Expr::I32(1)
                )))])),
                els: Some(Spanned::any(Stmt::If {
                    cond: Spanned::any(Expr::Variable("bar".to_string())),
                    then: Spanned::any(Stmt::Block(vec![Spanned::any(Stmt::Expr(Spanned::any(
                        Expr::I32(2)
                    )))])),
                    els: Some(Spanned::any(Stmt::If {
                        cond: Spanned::any(Expr::Variable("baz".to_string())),
                        then: Spanned::any(Stmt::Block(vec![Spanned::any(Stmt::Expr(
                            Spanned::any(Expr::I32(3))
                        ))])),
                        els: Some(Spanned::any(Stmt::If {
                            cond: Spanned::any(Expr::Variable("qux".to_string())),
                            then: Spanned::any(Stmt::Block(vec![Spanned::any(Stmt::Expr(
                                Spanned::any(Expr::I32(4))
                            ))])),
                            els: None,
                        })),
                    })),
                })),
            }))
        );
        assert!(if_stmt().parse_test("if foo { 1 }").is_err());
    }

    #[test]
    fn test_return_stmt() {
        assert_eq!(
            return_stmt().parse_test("return 1 + 2;"),
            Ok(Spanned::any(Stmt::Return(Some(Spanned::any(Expr::Add(
                Spanned::any(Expr::I32(1)),
                Spanned::any(Expr::I32(2)),
            ))))))
        );
        assert_eq!(
            return_stmt().parse_test("return 1;"),
            Ok(Spanned::any(Stmt::Return(Some(Spanned::any(Expr::I32(1))))))
        );
        assert_eq!(
            return_stmt().parse_test("return ;"),
            Ok(Spanned::any(Stmt::Return(None)))
        );
        assert!(return_stmt().parse_test("return").is_err());
        assert!(return_stmt().parse_test("return 1 + 2").is_err());
    }

    fn big_expr() -> Spanned<Expr> {
        Spanned::any(Expr::Or(
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
                                            Spanned::any(Expr::I32(10)),
                                        )),
                                    )),
                                )),
                            )),
                        )),
                    )),
                )),
            )),
        ))
    }
    #[test]
    fn test_assign_stmt() {
        assert_eq!(
            assign_stmt().parse_test("var = 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10 ;"),
            Ok(Spanned::any(Stmt::Assign(
                Spanned::any(Expr::Variable("var".to_string())),
                big_expr()
            )))
        );
        assert_eq!(
            assign_stmt().parse_test("var += 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Spanned::any(Stmt::AddAssign(
                Spanned::any(Expr::Variable("var".to_string())),
                big_expr(),
            )))
        );
        assert_eq!(
            assign_stmt().parse_test("var -= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Spanned::any(Stmt::SubAssign(
                Spanned::any(Expr::Variable("var".to_string())),
                big_expr(),
            )))
        );
        assert_eq!(
            assign_stmt().parse_test("var *= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Spanned::any(Stmt::MulAssign(
                Spanned::any(Expr::Variable("var".to_string())),
                big_expr(),
            )))
        );
        assert_eq!(
            assign_stmt().parse_test("var /= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Spanned::any(Stmt::DivAssign(
                Spanned::any(Expr::Variable("var".to_string())),
                big_expr(),
            )))
        );
        assert_eq!(
            assign_stmt().parse_test("var %= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Spanned::any(Stmt::RemAssign(
                Spanned::any(Expr::Variable("var".to_string())),
                big_expr(),
            )))
        );
        assert_eq!(
            assign_stmt().parse_test("var &= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Spanned::any(Stmt::BitAndAssign(
                Spanned::any(Expr::Variable("var".to_string())),
                big_expr(),
            )))
        );
        assert_eq!(
            assign_stmt().parse_test("var |= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Spanned::any(Stmt::BitOrAssign(
                Spanned::any(Expr::Variable("var".to_string())),
                big_expr(),
            )))
        );
        assert_eq!(
            assign_stmt().parse_test("var ^= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Spanned::any(Stmt::BitXorAssign(
                Spanned::any(Expr::Variable("var".to_string())),
                big_expr(),
            )))
        );
        assert_eq!(
            assign_stmt().parse_test("var <<= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Spanned::any(Stmt::ShlAssign(
                Spanned::any(Expr::Variable("var".to_string())),
                big_expr(),
            )))
        );
        assert_eq!(
            assign_stmt().parse_test("var >>= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Spanned::any(Stmt::ShrAssign(
                Spanned::any(Expr::Variable("var".to_string())),
                big_expr(),
            )))
        );
        assert_eq!(
            assign_stmt().parse_test("1 ;"),
            Ok(Spanned::any(Stmt::Expr(Spanned::any(Expr::I32(1)))))
        );
        assert!(assign_stmt().parse_test("1 ").is_err());
    }
}
