use crate::parser::ast::{expr9, ident, term, typedef, typeref, Expr, Type};
use chumsky::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Param {
    is_mut: bool,
    name: String,
    ty: Type,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Stmt {
    Empty,

    Import(String),

    DefFn {
        name: String,
        args: Vec<Param>,
        ret_ty: Type,
        body: Box<Self>,
    },

    DefVar {
        is_mut: bool,
        name: String,
        type_ref: Type,
        expr: Box<Expr>,
    },

    TypeDef {
        new: String,
        old: Type,
    },

    Block(Vec<Self>),

    If {
        cond: Box<Expr>,
        then: Box<Self>,
        els: Option<Box<Self>>,
    },

    Return(Option<Box<Expr>>),

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

    Expr(Box<Expr>),
}

// import std.io;
pub(crate) fn import_stmt() -> impl Parser<char, Stmt, Error = Simple<char>> + Clone {
    text::keyword("import")
        .ignore_then(
            ident()
                .repeated()
                .separated_by(just('.'))
                .map(|i| i.into_iter().flatten().collect::<Vec<String>>().join(".")),
        )
        .then_ignore(just(';'))
        .map(Stmt::Import)
        .labelled("import")
        .padded()
        .boxed()
}

pub(crate) fn top_defs() -> impl Parser<char, Vec<Stmt>, Error = Simple<char>> + Clone {
    choice((defvar(), defn(), typedef())).repeated().boxed()
}

// name1: type1
fn param() -> impl Parser<char, Param, Error = Simple<char>> + Clone {
    text::keyword("mut")
        .or_not()
        .padded()
        .then(ident())
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
fn defn() -> impl Parser<char, Stmt, Error = Simple<char>> + Clone {
    text::keyword("fn")
        .padded()
        .ignore_then(ident())
        .then(
            param()
                .padded()
                .separated_by(just(','))
                .delimited_by(just('('), just(')')),
        )
        .padded()
        .then_ignore(just("->"))
        .then(typeref().padded())
        .then(block())
        .map(|(((name, args), ty), body)| Stmt::DefFn {
            name,
            args,
            ret_ty: ty,
            body: Box::new(body),
        })
        .boxed()
}

// let mut var: type = expr;
fn defvar() -> impl Parser<char, Stmt, Error = Simple<char>> + Clone {
    text::keyword("let")
        .padded()
        .ignore_then(just("mut").or_not())
        .then(ident())
        .then_ignore(just(':'))
        .then(typeref().padded())
        .then_ignore(just('='))
        .padded()
        .then(expr9())
        .then_ignore(just(';'))
        .map(|(((mt, nm), ty), expr)| Stmt::DefVar {
            is_mut: mt.is_some(),
            name: nm,
            type_ref: ty,
            expr: Box::new(expr),
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

fn block() -> impl Parser<char, Stmt, Error = Simple<char>> + Clone {
    defvar()
        .or(stmt())
        .repeated()
        .padded()
        .delimited_by(just('{'), just('}'))
        .map(Stmt::Block)
        .boxed()
}

fn stmt() -> impl Parser<char, Stmt, Error = Simple<char>> + Clone {
    // TODO: Stack overflow on block tests
    // recursive(|_| {
    //     choice((
    //         just(';').padded().to(Stmt::Empty),
    //         assign_stmt(),
    //         block(),
    //         return_stmt(),
    //     ))
    // })
    choice((
        just(';').padded().to(Stmt::Empty),
        assign_stmt(),
        // block(),
        // recursive(|_| if_stmt()),
        return_stmt(),
    ))
    .boxed()
}

// if expr {
// } else if expr {
// } else {
// }
fn if_stmt() -> impl Parser<char, Stmt, Error = Simple<char>> + Clone {
    recursive(|if_| {
        text::keyword("if")
            .padded()
            .ignore_then(expr9())
            .then(block())
            .then(
                text::keyword("else")
                    .padded()
                    .ignore_then(block().or(if_))
                    .or_not(),
            )
            .map(|((cond, then), els)| Stmt::If {
                cond: Box::new(cond),
                then: Box::new(then),
                els: els.map(Box::new),
            })
    })
    .boxed()
}

fn return_stmt() -> impl Parser<char, Stmt, Error = Simple<char>> + Clone {
    text::keyword("return")
        .padded()
        .ignore_then(expr9().or_not())
        .map(|expr| Stmt::Return(expr.map(Box::new)))
        .then_ignore(just(';'))
        .boxed()
}

fn assign_stmt() -> impl Parser<char, Stmt, Error = Simple<char>> + Clone {
    choice((
        term()
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
                .then(expr9()),
            )
            .map(|(lhs, (op, rhs))| op(Box::new(lhs), Box::new(rhs))),
        expr9().map(|expr| Stmt::Expr(Box::new(expr))),
    ))
    .then_ignore(just(';'))
    .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::Int;
    use chumsky::Parser;

    #[test]
    fn import_stmt_test() {
        assert_eq!(
            import_stmt().parse("import std.io;"),
            Ok(Stmt::Import("std.io".to_string()))
        );
        assert_eq!(
            import_stmt().parse("import     std  .   io   ;"),
            Ok(Stmt::Import("std.io".to_string()))
        );
        assert_eq!(
            import_stmt().parse("import stdio;"),
            Ok(Stmt::Import("stdio".to_string()))
        );
        assert!(import_stmt().parse("import 1std.io;").is_err());
        assert!(import_stmt().parse("import std.1io;").is_err());
        assert!(import_stmt().parse("import std.io").is_err());
        assert!(import_stmt().parse("use std.io;").is_err());
    }

    #[test]
    fn block_test1() {
        assert_eq!(block().parse("{}"), Ok(Stmt::Block(vec![])));
        assert_eq!(block().parse("{     }"), Ok(Stmt::Block(vec![])));
        assert_eq!(
            block().parse(
                r#"{
                let var1: type = 10;
    
                let mut var2: type = 10;
            }"#
            ),
            Ok(Stmt::Block(vec![
                Stmt::DefVar {
                    is_mut: false,
                    name: "var1".to_string(),
                    type_ref: Type::User("type".to_string()),
                    expr: Box::new(Expr::Int(Int::I32(10))),
                },
                Stmt::DefVar {
                    is_mut: true,
                    name: "var2".to_string(),
                    type_ref: Type::User("type".to_string()),
                    expr: Box::new(Expr::Int(Int::I32(10))),
                }
            ]))
        );
    }
    #[test]
    fn block_test2() {
        assert!(block().parse("{     ").is_err());
        assert!(block().parse("  }").is_err());
        assert!(block().parse("let var: type = 10;").is_err());
    }

    #[test]
    fn defvar_test() {
        assert_eq!(
            defvar().parse("let var: type = 10;"),
            Ok(Stmt::DefVar {
                is_mut: false,
                name: "var".to_string(),
                type_ref: Type::User("type".to_string()),
                expr: Box::new(Expr::Int(Int::I32(10))),
            })
        );
        assert_eq!(
            defvar().parse("let mut var: type = 10;"),
            Ok(Stmt::DefVar {
                is_mut: true,
                name: "var".to_string(),
                type_ref: Type::User("type".to_string()),
                expr: Box::new(Expr::Int(Int::I32(10))),
            })
        );

        assert!(defvar().parse("let var: type = 10").is_err());
        assert!(defvar().parse("let mut var: type = 10").is_err());

        assert!(defvar().parse("let var := 10;").is_err());
        assert!(defvar().parse("let mut var := 10;").is_err());
    }

    #[test]
    fn if_stmt_test() {
        assert_eq!(
            if_stmt().parse("if foo { 1; }"),
            Ok(Stmt::If {
                cond: Box::new(Expr::Variable("foo".to_string())),
                then: Box::new(Stmt::Block(vec![Stmt::Expr(Box::new(Expr::Int(
                    Int::I32(1)
                )))])),
                els: None,
            })
        );
        assert_eq!(
            if_stmt().parse("if foo { 1; } else { 2; }"),
            Ok(Stmt::If {
                cond: Box::new(Expr::Variable("foo".to_string())),
                then: Box::new(Stmt::Block(vec![Stmt::Expr(Box::new(Expr::Int(
                    Int::I32(1)
                )))])),
                els: Some(Box::new(Stmt::Block(vec![Stmt::Expr(Box::new(
                    Expr::Int(Int::I32(2))
                ))]))),
            })
        );
        assert_eq!(
            if_stmt().parse("if foo { 1; } else if bar { 2; } else { 3; }"),
            Ok(Stmt::If {
                cond: Box::new(Expr::Variable("foo".to_string())),
                then: Box::new(Stmt::Block(vec![Stmt::Expr(Box::new(Expr::Int(
                    Int::I32(1)
                )))])),
                els: Some(Box::new(Stmt::If {
                    cond: Box::new(Expr::Variable("bar".to_string())),
                    then: Box::new(Stmt::Block(vec![Stmt::Expr(Box::new(Expr::Int(
                        Int::I32(2)
                    )))])),
                    els: Some(Box::new(Stmt::Block(vec![Stmt::Expr(Box::new(
                        Expr::Int(Int::I32(3))
                    ))])))
                })),
            })
        );
        assert_eq!(
            if_stmt().parse("if foo { 1; } else if bar { 2; }"),
            Ok(Stmt::If {
                cond: Box::new(Expr::Variable("foo".to_string())),
                then: Box::new(Stmt::Block(vec![Stmt::Expr(Box::new(Expr::Int(
                    Int::I32(1)
                )))])),
                els: Some(Box::new(Stmt::If {
                    cond: Box::new(Expr::Variable("bar".to_string())),
                    then: Box::new(Stmt::Block(vec![Stmt::Expr(Box::new(Expr::Int(
                        Int::I32(2)
                    )))])),
                    els: None,
                })),
            })
        );
        assert_eq!(
            if_stmt()
                .parse("if foo { 1; } else if bar { 2; } else if baz { 3; } else if qux { 4; }"),
            Ok(Stmt::If {
                cond: Box::new(Expr::Variable("foo".to_string())),
                then: Box::new(Stmt::Block(vec![Stmt::Expr(Box::new(Expr::Int(
                    Int::I32(1)
                )))])),
                els: Some(Box::new(Stmt::If {
                    cond: Box::new(Expr::Variable("bar".to_string())),
                    then: Box::new(Stmt::Block(vec![Stmt::Expr(Box::new(Expr::Int(
                        Int::I32(2)
                    )))])),
                    els: Some(Box::new(Stmt::If {
                        cond: Box::new(Expr::Variable("baz".to_string())),
                        then: Box::new(Stmt::Block(vec![Stmt::Expr(Box::new(Expr::Int(
                            Int::I32(3)
                        )))])),
                        els: Some(Box::new(Stmt::If {
                            cond: Box::new(Expr::Variable("qux".to_string())),
                            then: Box::new(Stmt::Block(vec![Stmt::Expr(Box::new(Expr::Int(
                                Int::I32(4)
                            )))])),
                            els: None,
                        })),
                    })),
                })),
            })
        );
        assert!(if_stmt().parse("if foo { 1 }").is_err());
    }

    #[test]
    fn return_stmt_test() {
        assert_eq!(
            return_stmt().parse("return 1 + 2;"),
            Ok(Stmt::Return(Some(Box::new(Expr::Add(
                Box::new(Expr::Int(Int::I32(1))),
                Box::new(Expr::Int(Int::I32(2)))
            )))))
        );
        assert_eq!(
            return_stmt().parse("return 1;"),
            Ok(Stmt::Return(Some(Box::new(Expr::Int(Int::I32(1))))))
        );
        assert_eq!(return_stmt().parse("return ;"), Ok(Stmt::Return(None)));
        assert!(return_stmt().parse("return").is_err());
        assert!(return_stmt().parse("return 1 + 2").is_err());
    }

    fn big_expr() -> Box<Expr> {
        Box::new(Expr::Or(
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
                                            Box::new(Expr::Int(Int::I32(10))),
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
    fn assign_stmt_test1() {
        assert_eq!(
            assign_stmt().parse("var = 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10 ;"),
            Ok(Stmt::Assign(
                Box::new(Expr::Variable("var".to_string())),
                big_expr()
            ))
        );
    }
    #[test]
    fn assign_stmt_test2() {
        assert_eq!(
            assign_stmt().parse("var += 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Stmt::AddAssign(
                Box::new(Expr::Variable("var".to_string())),
                big_expr(),
            ))
        );
    }
    #[test]
    fn assign_stmt_test3() {
        assert_eq!(
            assign_stmt().parse("var -= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Stmt::SubAssign(
                Box::new(Expr::Variable("var".to_string())),
                big_expr(),
            ))
        );
    }
    #[test]
    fn assign_stmt_test4() {
        assert_eq!(
            assign_stmt().parse("var *= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Stmt::MulAssign(
                Box::new(Expr::Variable("var".to_string())),
                big_expr(),
            ))
        );
    }
    #[test]
    fn assign_stmt_test5() {
        assert_eq!(
            assign_stmt().parse("var /= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Stmt::DivAssign(
                Box::new(Expr::Variable("var".to_string())),
                big_expr(),
            ))
        );
    }
    #[test]
    fn assign_stmt_test6() {
        assert_eq!(
            assign_stmt().parse("var %= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Stmt::RemAssign(
                Box::new(Expr::Variable("var".to_string())),
                big_expr(),
            ))
        );
    }
    #[test]
    fn assign_stmt_test7() {
        assert_eq!(
            assign_stmt().parse("var &= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Stmt::BitAndAssign(
                Box::new(Expr::Variable("var".to_string())),
                big_expr(),
            ))
        );
    }
    #[test]
    fn assign_stmt_test8() {
        assert_eq!(
            assign_stmt().parse("var |= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Stmt::BitOrAssign(
                Box::new(Expr::Variable("var".to_string())),
                big_expr(),
            ))
        );
    }
    #[test]
    fn assign_stmt_test9() {
        assert_eq!(
            assign_stmt().parse("var ^= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Stmt::BitXorAssign(
                Box::new(Expr::Variable("var".to_string())),
                big_expr(),
            ))
        );
    }
    #[test]
    fn assign_stmt_test10() {
        assert_eq!(
            assign_stmt().parse("var <<= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Stmt::ShlAssign(
                Box::new(Expr::Variable("var".to_string())),
                big_expr(),
            ))
        );
    }
    #[test]
    fn assign_stmt_test11() {
        assert_eq!(
            assign_stmt().parse("var >>= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Stmt::ShrAssign(
                Box::new(Expr::Variable("var".to_string())),
                big_expr(),
            ))
        );
    }
    #[test]
    fn assign_stmt_test12() {
        assert_eq!(
            assign_stmt().parse("1 ;"),
            Ok(Stmt::Expr(Box::new(Expr::Int(Int::I32(1)))))
        );
    }
    #[test]
    fn assign_stmt_test13() {
        assert!(assign_stmt().parse("1 ").is_err());
    }
}
