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

use crate::parser::lib::*;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Ast {
    pub(crate) imports: Vec<Spanned<Stmt>>,
    pub(crate) defs: Vec<Spanned<Stmt>>,
}

pub(crate) fn compilation_unit() -> impl Parser<Ast> {
    import_stmt()
        .repeated()
        .then(top_defs())
        .padded()
        .then_ignore(end())
        .map(|(imports, defs)| Ast { imports, defs })
        .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compilation_unit() {
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
            Ok(Ast {
                imports: vec![
                    Spanned::any(Stmt::Import("std.io".to_string())),
                    Spanned::any(Stmt::Import("stdio".to_string())),
                ],
                defs: vec![
                    Spanned::any(Stmt::DefVar {
                        is_mut: false,
                        name: Spanned::any("fuga".to_string()),
                        ty: Spanned::any(Type::I32),
                        expr: Some(Spanned::any(Expr::I32(1))),
                    }),
                    Spanned::any(Stmt::TypeDef {
                        new: "newint".to_string(),
                        old: Spanned::any(Type::I32),
                    }),
                    Spanned::any(Stmt::DefFn {
                        name: Spanned::any("f1".to_string()),
                        args: vec![
                            Param {
                                is_mut: false,
                                name: "arg".to_string(),
                                ty: Spanned::any(Type::I8)
                            },
                            Param {
                                is_mut: true,
                                name: "arg2".to_string(),
                                ty: Spanned::any(Type::U64)
                            }
                        ],
                        ret_ty: Spanned::any(Type::U32),
                        body: Spanned::any(Stmt::Block(vec![Spanned::any(Stmt::Return(Some(
                            Spanned::any(Expr::Add(
                                Spanned::any(Expr::As(
                                    Spanned::any(Expr::Variable("arg".to_string())),
                                    Spanned::any(Type::U64)
                                )),
                                Spanned::any(Expr::Variable("arg2".to_string()))
                            ))
                        )))])),
                    }),
                    Spanned::any(Stmt::DefFn {
                        name: Spanned::any("main".to_string()),
                        args: vec![],
                        ret_ty: Spanned::any(Type::I32),
                        body: Spanned::any(Stmt::Block(vec![
                            Spanned::any(Stmt::DefVar {
                                is_mut: true,
                                name: Spanned::any("hoge".to_string()),
                                ty: Spanned::any(Type::User("User".to_string())),
                                expr: Some(Spanned::any(Expr::I32(12))),
                            }),
                            Spanned::any(Stmt::If {
                                cond: Spanned::any(Expr::Variable("hoge".to_string())),
                                then: Spanned::any(Stmt::Block(vec![Spanned::any(Stmt::Return(
                                    Some(Spanned::any(Expr::I32(1)))
                                ))])),
                                els: Some(Spanned::any(Stmt::If {
                                    cond: Spanned::any(Expr::Variable("fuga".to_string())),
                                    then: Spanned::any(Stmt::Block(vec![Spanned::any(
                                        Stmt::Return(Some(Spanned::any(Expr::FnCall {
                                            name: Spanned::any(Expr::Variable("f1".to_string())),
                                            args: vec![
                                                Spanned::any(Expr::As(
                                                    Spanned::any(Expr::Variable(
                                                        "fuga".to_string()
                                                    )),
                                                    Spanned::any(Type::I8)
                                                )),
                                                Spanned::any(Expr::As(
                                                    Spanned::any(Expr::Variable(
                                                        "hoge".to_string()
                                                    )),
                                                    Spanned::any(Type::U64)
                                                ))
                                            ]
                                        })))
                                    )])),
                                    els: Some(Spanned::any(Stmt::Block(vec![Spanned::any(
                                        Stmt::Return(Some(Spanned::any(Expr::Sub(
                                            Spanned::any(Expr::Add(
                                                Spanned::any(Expr::Add(
                                                    Spanned::any(Expr::I32(1)),
                                                    Spanned::any(Expr::I32(2)),
                                                )),
                                                Spanned::any(Expr::I32(2)),
                                            )),
                                            Spanned::any(Expr::Mul(
                                                Spanned::any(Expr::I64(1)),
                                                Spanned::any(Expr::Variable("hoge".to_string())),
                                            )),
                                        )),))
                                    )]))),
                                }))
                            }),
                        ])),
                    }),
                ],
            })
        );
    }
}
