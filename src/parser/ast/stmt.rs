use crate::parser::ast::{expr9, term, Expr};
use chumsky::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Stmt {
    Return(Option<Expr>),

    /// =
    Assign(Expr, Expr),
    /// +=
    AddAssign(Expr, Expr),
    /// -=
    SubAssign(Expr, Expr),
    /// *=
    MulAssign(Expr, Expr),
    /// /=
    DivAssign(Expr, Expr),
    /// %=
    RemAssign(Expr, Expr),
    /// &=
    BitAndAssign(Expr, Expr),
    /// |=
    BitOrAssign(Expr, Expr),
    /// ^=
    BitXorAssign(Expr, Expr),
    /// <<=
    ShlAssign(Expr, Expr),
    /// >>=
    ShrAssign(Expr, Expr),

    Expr(Expr),
}

pub(crate) fn return_stmt() -> impl Parser<char, Stmt, Error = Simple<char>> + Clone {
    text::keyword("return")
        .padded()
        .then(expr9().or_not())
        .map(|((), expr)| Stmt::Return(expr))
        .then_ignore(just(';'))
}

pub(crate) fn assign_stmt() -> impl Parser<char, Stmt, Error = Simple<char>> + Clone {
    choice((
        term()
            .then(
                just('=')
                    .to(Stmt::Assign as fn(_, _) -> _)
                    .or(just("+=").to(Stmt::AddAssign as fn(_, _) -> _))
                    .or(just("-=").to(Stmt::SubAssign as fn(_, _) -> _))
                    .or(just("*=").to(Stmt::MulAssign as fn(_, _) -> _))
                    .or(just("/=").to(Stmt::DivAssign as fn(_, _) -> _))
                    .or(just("%=").to(Stmt::RemAssign as fn(_, _) -> _))
                    .or(just("&=").to(Stmt::BitAndAssign as fn(_, _) -> _))
                    .or(just("|=").to(Stmt::BitOrAssign as fn(_, _) -> _))
                    .or(just("^=").to(Stmt::BitXorAssign as fn(_, _) -> _))
                    .or(just("<<=").to(Stmt::ShlAssign as fn(_, _) -> _))
                    .or(just(">>=").to(Stmt::ShrAssign as fn(_, _) -> _))
                    // Here, this is not expr() because I would not allow multiple assignments like a = b = c;
                    .then(expr9()),
            )
            .map(|(lhs, (op, rhs))| op(lhs, rhs)),
        expr9().map(Stmt::Expr),
    ))
    .then_ignore(just(';'))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::IntegerLiteralNode;
    use chumsky::Parser;

    #[test]
    fn return_stmt_test() {
        assert_eq!(
            return_stmt().parse("return 1 + 2;"),
            Ok(Stmt::Return(Some(Expr::Add(
                Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
                Box::from(Expr::Integer(IntegerLiteralNode::I32(2)))
            ))))
        );
        assert_eq!(
            return_stmt().parse("return 1;"),
            Ok(Stmt::Return(Some(Expr::Integer(IntegerLiteralNode::I32(
                1
            )))))
        );
        assert_eq!(return_stmt().parse("return ;"), Ok(Stmt::Return(None)));
        assert!(return_stmt().parse("return").is_err());
        assert!(return_stmt().parse("return 1 + 2").is_err());
    }

    fn big_expr() -> Expr {
        Expr::Or(
            Box::from(Expr::Integer(IntegerLiteralNode::I32(1))),
            Box::from(Expr::And(
                Box::from(Expr::Integer(IntegerLiteralNode::I32(2))),
                Box::from(Expr::Neq(
                    Box::from(Expr::Integer(IntegerLiteralNode::I32(3))),
                    Box::from(Expr::BitOr(
                        Box::from(Expr::Integer(IntegerLiteralNode::I32(4))),
                        Box::from(Expr::BitXor(
                            Box::from(Expr::Integer(IntegerLiteralNode::I32(5))),
                            Box::from(Expr::BitAnd(
                                Box::from(Expr::Integer(IntegerLiteralNode::I32(6))),
                                Box::from(Expr::Shl(
                                    Box::from(Expr::Integer(IntegerLiteralNode::I32(7))),
                                    Box::from(Expr::Add(
                                        Box::from(Expr::Integer(IntegerLiteralNode::I32(8))),
                                        Box::from(Expr::Mul(
                                            Box::from(Expr::Integer(IntegerLiteralNode::I32(9))),
                                            Box::from(Expr::Integer(IntegerLiteralNode::I32(10))),
                                        )),
                                    )),
                                )),
                            )),
                        )),
                    )),
                )),
            )),
        )
    }
    #[test]
    fn assign_stmt_test1() {
        assert_eq!(
            assign_stmt().parse("var = 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10 ;"),
            Ok(Stmt::Assign(Expr::Variable("var".to_string()), big_expr(),))
        );
    }
    #[test]
    fn assign_stmt_test2() {
        assert_eq!(
            assign_stmt().parse("var += 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Stmt::AddAssign(
                Expr::Variable("var".to_string()),
                big_expr(),
            ))
        );
    }
    #[test]
    fn assign_stmt_test3() {
        assert_eq!(
            assign_stmt().parse("var -= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Stmt::SubAssign(
                Expr::Variable("var".to_string()),
                big_expr(),
            ))
        );
    }
    #[test]
    fn assign_stmt_test4() {
        assert_eq!(
            assign_stmt().parse("var *= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Stmt::MulAssign(
                Expr::Variable("var".to_string()),
                big_expr(),
            ))
        );
    }
    #[test]
    fn assign_stmt_test5() {
        assert_eq!(
            assign_stmt().parse("var /= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Stmt::DivAssign(
                Expr::Variable("var".to_string()),
                big_expr(),
            ))
        );
    }
    #[test]
    fn assign_stmt_test6() {
        assert_eq!(
            assign_stmt().parse("var %= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Stmt::RemAssign(
                Expr::Variable("var".to_string()),
                big_expr(),
            ))
        );
    }
    #[test]
    fn assign_stmt_test7() {
        assert_eq!(
            assign_stmt().parse("var &= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Stmt::BitAndAssign(
                Expr::Variable("var".to_string()),
                big_expr(),
            ))
        );
    }
    #[test]
    fn assign_stmt_test8() {
        assert_eq!(
            assign_stmt().parse("var |= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Stmt::BitOrAssign(
                Expr::Variable("var".to_string()),
                big_expr(),
            ))
        );
    }
    #[test]
    fn assign_stmt_test9() {
        assert_eq!(
            assign_stmt().parse("var ^= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Stmt::BitXorAssign(
                Expr::Variable("var".to_string()),
                big_expr(),
            ))
        );
    }
    #[test]
    fn assign_stmt_test10() {
        assert_eq!(
            assign_stmt().parse("var <<= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Stmt::ShlAssign(
                Expr::Variable("var".to_string()),
                big_expr(),
            ))
        );
    }
    #[test]
    fn assign_stmt_test11() {
        assert_eq!(
            assign_stmt().parse("var >>= 1 || 2 && 3 != 4 | 5 ^ 6 & 7 << 8 + 9*10;"),
            Ok(Stmt::ShrAssign(
                Expr::Variable("var".to_string()),
                big_expr(),
            ))
        );
    }
    #[test]
    fn assign_stmt_test12() {
        assert_eq!(
            assign_stmt().parse("1 ;"),
            Ok(Stmt::Expr(Expr::Integer(IntegerLiteralNode::I32(1))))
        );
    }
    #[test]
    fn assign_stmt_test13() {
        assert!(assign_stmt().parse("1 ").is_err());
    }
}
