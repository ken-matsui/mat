use crate::prelude::*;
use crate::stmt::top_defs;
use matc_ast::Ast;

pub(crate) fn compilation_unit() -> impl Parser<Ast> {
    top_defs()
        .padded()
        .then_ignore(end())
        .map(|defs| Ast { defs })
        .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;
    use matc_ast::{Expr, Param, Stmt, Type};
    use matc_span::Spanned;

    #[test]
    fn test_compilation_unit() {
        assert_eq!(
            compilation_unit().parse_test(
                r#"
let fuga: i32 = 1;

fn f1(arg: char, mut arg2: i32) -> i32 {
    return arg as i32 + arg2;
}

fn main() -> i32 {
    let mut hoge: i32 = 12;
    //boo = 23; // unresolved reference: boo
    //hoge(1, 2, 3); // calling object is not a function
    // 1 = 2 + 4; // invalid lhs expression
    if hoge {
        return 1;
    } else if fuga {
        return f1(fuga as char, hoge as i32);
    } else {
        return 1 + 2 + 2 - 1i32 * hoge;
    }
}
        "#
            ),
            Ok(Ast {
                defs: vec![
                    Spanned::any(Stmt::DefVar {
                        is_mut: false,
                        name: Spanned::any("fuga".to_string()),
                        ty: Spanned::any(Type::I32),
                        expr: Some(Spanned::any(Expr::I32(1))),
                    }),
                    Spanned::any(Stmt::DefFn {
                        name: Spanned::any("f1".to_string()),
                        args: vec![
                            Param {
                                is_mut: false,
                                name: Spanned::any("arg".to_string()),
                                ty: Spanned::any(Type::I8)
                            },
                            Param {
                                is_mut: true,
                                name: Spanned::any("arg2".to_string()),
                                ty: Spanned::any(Type::I32)
                            }
                        ],
                        ret_ty: Spanned::any(Type::I32),
                        body: Spanned::any(Stmt::Block(vec![Spanned::any(Stmt::Return(Some(
                            Spanned::any(Expr::Add(
                                Spanned::any(Expr::As(
                                    Spanned::any(Expr::Variable("arg".to_string())),
                                    Spanned::any(Type::I32)
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
                                ty: Spanned::any(Type::I32),
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
                                                    Spanned::any(Type::I32)
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
                                                Spanned::any(Expr::I32(1)),
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
