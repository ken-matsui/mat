mod comment;
mod expr;
mod ident;
mod integer;
mod span;
mod stmt;
mod string;
mod ty;
mod variable;

pub(crate) use comment::*;
pub(crate) use expr::*;
pub(crate) use ident::*;
pub(crate) use integer::*;
pub(crate) use span::*;
pub(crate) use stmt::*;
pub(crate) use string::*;
pub(crate) use ty::*;
pub(crate) use variable::*;

use chumsky::prelude::{end, Parser, Simple};
use chumsky::text::TextParser;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Ast {
    imports: Vec<Stmt>,
    defs: Vec<Stmt>,
}

pub(crate) fn compilation_unit() -> impl Parser<char, Spanned<Ast>, Error = Simple<char>> + Clone {
    import_stmt()
        .repeated()
        .then(top_defs())
        .padded()
        .then_ignore(end())
        .map_with_span(|(imports, defs), span| (Ast { imports, defs }, span))
        .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::Int;
    use chumsky::Parser;

    #[test]
    fn compilation_unit_test() {
        assert_eq!(
            compilation_unit().parse(
                r#"
import std.io;
import stdio;

let fuga: i32 = 1;

type newint = i32;

fn f1(arg: char, mut arg2: u64) -> u32 {
    return arg as u64 + arg2;
}

fn main() -> i32 {
    let mut hoge: User = 12;
    //boo = 23; // unresolved reference: boo
    //hoge(1, 2, 3); // calling object is not a function
    // 1 = 2 + 4; // invalid lhs expression
    if hoge {
        return 1;
    } else if fuga {
        return f1(fuga as char, hoge as u64);
    } else {
        return 1 + 2 + 2 - 1i64 * hoge;
    }
}
        "#
            ),
            Ok((
                Ast {
                    imports: vec![
                        Stmt::Import("std.io".to_string()),
                        Stmt::Import("stdio".to_string()),
                    ],
                    defs: vec![
                        Stmt::DefVar {
                            is_mut: false,
                            name: "fuga".to_string(),
                            type_ref: Type::I32,
                            expr: Box::new(Expr::Int(Int::I32(1))),
                        },
                        Stmt::TypeDef {
                            new: "newint".to_string(),
                            old: Type::I32,
                        },
                        Stmt::DefFn {
                            name: "f1".to_string(),
                            args: vec![
                                Param {
                                    is_mut: false,
                                    name: "arg".to_string(),
                                    ty: Type::I8
                                },
                                Param {
                                    is_mut: true,
                                    name: "arg2".to_string(),
                                    ty: Type::U64
                                }
                            ],
                            ret_ty: Type::U32,
                            body: Box::new(Stmt::Block(vec![Stmt::Return(Some(Box::new(
                                Expr::Add(
                                    Box::new(Expr::As(
                                        Box::new(Expr::Variable("arg".to_string())),
                                        Type::U64
                                    )),
                                    Box::new(Expr::Variable("arg2".to_string()))
                                )
                            )))])),
                        },
                        Stmt::DefFn {
                            name: "main".to_string(),
                            args: vec![],
                            ret_ty: Type::I32,
                            body: Box::new(Stmt::Block(vec![
                                Stmt::DefVar {
                                    is_mut: true,
                                    name: "hoge".to_string(),
                                    type_ref: Type::User("User".to_string()),
                                    expr: Box::new(Expr::Int(Int::I32(12))),
                                },
                                Stmt::If {
                                    cond: Box::new(Expr::Variable("hoge".to_string())),
                                    then: Box::new(Stmt::Block(vec![Stmt::Return(Some(
                                        Box::new(Expr::Int(Int::I32(1)))
                                    ))])),
                                    els: Some(Box::new(Stmt::If {
                                        cond: Box::new(Expr::Variable("fuga".to_string())),
                                        then: Box::new(Stmt::Block(vec![Stmt::Return(Some(
                                            Box::new(Expr::FnCall {
                                                name: Box::new(Expr::Variable("f1".to_string())),
                                                args: vec![
                                                    Expr::As(
                                                        Box::new(Expr::Variable(
                                                            "fuga".to_string()
                                                        )),
                                                        Type::I8
                                                    ),
                                                    Expr::As(
                                                        Box::new(Expr::Variable(
                                                            "hoge".to_string()
                                                        )),
                                                        Type::U64
                                                    )
                                                ]
                                            })
                                        ))])),
                                        els: Some(Box::new(Stmt::Block(vec![Stmt::Return(Some(
                                            Box::new(Expr::Sub(
                                                Box::new(Expr::Add(
                                                    Box::new(Expr::Add(
                                                        Box::new(Expr::Int(Int::I32(1))),
                                                        Box::new(Expr::Int(Int::I32(2))),
                                                    )),
                                                    Box::new(Expr::Int(Int::I32(2))),
                                                )),
                                                Box::new(Expr::Mul(
                                                    Box::new(Expr::Int(Int::I64(1))),
                                                    Box::new(Expr::Variable("hoge".to_string())),
                                                )),
                                            )),
                                        ))]))),
                                    }))
                                }
                            ])),
                        },
                    ],
                },
                0..507
            ))
        );
    }
}
