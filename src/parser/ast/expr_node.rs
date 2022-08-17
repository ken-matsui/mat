use crate::parser::ast::{
    character, integer, string, variable, IntegerLiteralNode, StringLiteralNode, VariableNode,
};
use chumsky::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Op {
    /// ^
    BitXor,

    /// &
    BitAnd,

    /// <<
    Shl,
    /// >>
    Shr,

    /// +
    Add,
    /// -
    Sub,

    /// *
    Mul,
    /// /
    Div,
    /// %
    Rem,
}

#[derive(Debug, PartialEq)]
pub(crate) struct BinaryOpNode {
    lhs: Box<ExprNode>,
    op: Op,
    rhs: Box<ExprNode>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct CastNode {
    type_node: String, // TODO: TypeNode
    expr: Box<ExprNode>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum ExprNode {
    Integer(IntegerLiteralNode),
    String(StringLiteralNode),
    Variable(VariableNode),
    Cast(CastNode),
    Binary(BinaryOpNode),
    Expr(Box<ExprNode>),
}

pub(crate) fn expr5() -> impl Parser<char, ExprNode, Error = Simple<char>> + Clone {
    expr4()
        .then(just('^').to(Op::BitXor).then(expr4()).repeated())
        .foldl(|lhs, (op, rhs)| {
            ExprNode::Binary(BinaryOpNode {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            })
        })
}

pub(crate) fn expr4() -> impl Parser<char, ExprNode, Error = Simple<char>> + Clone {
    expr3()
        .then(just('&').to(Op::BitAnd).then(expr3()).repeated())
        .foldl(|lhs, (op, rhs)| {
            ExprNode::Binary(BinaryOpNode {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            })
        })
}

pub(crate) fn expr3() -> impl Parser<char, ExprNode, Error = Simple<char>> + Clone {
    expr2()
        .then(
            just("<<")
                .to(Op::Shl)
                .or(just(">>").to(Op::Shr))
                .then(expr2())
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| {
            ExprNode::Binary(BinaryOpNode {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            })
        })
}

pub(crate) fn expr2() -> impl Parser<char, ExprNode, Error = Simple<char>> + Clone {
    expr1()
        .then(
            just('+')
                .to(Op::Add)
                .or(just('-').to(Op::Sub))
                .then(expr1())
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| {
            ExprNode::Binary(BinaryOpNode {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            })
        })
}

pub(crate) fn expr1() -> impl Parser<char, ExprNode, Error = Simple<char>> + Clone {
    term()
        .then(
            just('*')
                .to(Op::Mul)
                .or(just('/').to(Op::Div))
                .or(just('%').to(Op::Rem))
                .then(term())
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| {
            ExprNode::Binary(BinaryOpNode {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            })
        })
}

pub(crate) fn term() -> impl Parser<char, ExprNode, Error = Simple<char>> + Clone {
    suffix()
    // TODO: ident -> type
    // (text::ident()
    //     .delimited_by(just("("), just(")"))
    //     .padded()
    //     .then(term())
    //     .map(|(ty, expr)| {
    //         Cast(CastNode {
    //             type_node: ty.to_string(),
    //             expr: Box::new(expr),
    //         })
    //     }))
    // .or(suffix())
}

pub(crate) fn suffix() -> impl Parser<char, ExprNode, Error = Simple<char>> + Clone {
    primary()
    // or(FnCall)
}

pub(crate) fn primary() -> impl Parser<char, ExprNode, Error = Simple<char>> + Clone {
    choice((
        integer().map(ExprNode::Integer),
        character().map(ExprNode::Integer),
        string().map(ExprNode::String),
        variable().map(ExprNode::Variable),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;

    #[test]
    fn expr5_test() {
        assert_eq!(
            expr5().parse("1 ^ 2 & 3 << 4 + 5*6"),
            Ok(ExprNode::Binary(BinaryOpNode {
                lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                op: Op::BitXor,
                rhs: Box::from(ExprNode::Binary(BinaryOpNode {
                    lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2))),
                    op: Op::BitAnd,
                    rhs: Box::from(ExprNode::Binary(BinaryOpNode {
                        lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3))),
                        op: Op::Shl,
                        rhs: Box::from(ExprNode::Binary(BinaryOpNode {
                            lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(4))),
                            op: Op::Add,
                            rhs: Box::from(ExprNode::Binary(BinaryOpNode {
                                lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(5))),
                                op: Op::Mul,
                                rhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(6)))
                            })),
                        })),
                    })),
                })),
            }))
        );

        assert_eq!(
            expr5().parse("1"),
            Ok(ExprNode::Integer(IntegerLiteralNode::I32(1)))
        );
    }

    #[test]
    fn expr4_test() {
        assert_eq!(
            expr4().parse("1 & 2 << 3 + 4*5"),
            Ok(ExprNode::Binary(BinaryOpNode {
                lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                op: Op::BitAnd,
                rhs: Box::from(ExprNode::Binary(BinaryOpNode {
                    lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2))),
                    op: Op::Shl,
                    rhs: Box::from(ExprNode::Binary(BinaryOpNode {
                        lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3))),
                        op: Op::Add,
                        rhs: Box::from(ExprNode::Binary(BinaryOpNode {
                            lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(4))),
                            op: Op::Mul,
                            rhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(5)))
                        })),
                    })),
                })),
            }))
        );

        assert_eq!(
            expr4().parse("1"),
            Ok(ExprNode::Integer(IntegerLiteralNode::I32(1)))
        );
    }

    #[test]
    fn expr3_test() {
        assert_eq!(
            expr3().parse("1 << 2 + 3*4"),
            Ok(ExprNode::Binary(BinaryOpNode {
                lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                op: Op::Shl,
                rhs: Box::from(ExprNode::Binary(BinaryOpNode {
                    lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2))),
                    op: Op::Add,
                    rhs: Box::from(ExprNode::Binary(BinaryOpNode {
                        lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3))),
                        op: Op::Mul,
                        rhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(4)))
                    })),
                })),
            }))
        );

        assert_eq!(
            expr3().parse("1 >> 2 + 3*4"),
            Ok(ExprNode::Binary(BinaryOpNode {
                lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                op: Op::Shr,
                rhs: Box::from(ExprNode::Binary(BinaryOpNode {
                    lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2))),
                    op: Op::Add,
                    rhs: Box::from(ExprNode::Binary(BinaryOpNode {
                        lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3))),
                        op: Op::Mul,
                        rhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(4)))
                    })),
                })),
            }))
        );

        assert_eq!(
            expr3().parse("1"),
            Ok(ExprNode::Integer(IntegerLiteralNode::I32(1)))
        );
    }

    #[test]
    fn expr2_test() {
        assert_eq!(
            expr2().parse("1 + 2*3"),
            Ok(ExprNode::Binary(BinaryOpNode {
                lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                op: Op::Add,
                rhs: Box::from(ExprNode::Binary(BinaryOpNode {
                    lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2))),
                    op: Op::Mul,
                    rhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3)))
                })),
            }))
        );

        assert_eq!(
            expr2().parse("1 - 2*3"),
            Ok(ExprNode::Binary(BinaryOpNode {
                lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                op: Op::Sub,
                rhs: Box::from(ExprNode::Binary(BinaryOpNode {
                    lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2))),
                    op: Op::Mul,
                    rhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3)))
                })),
            }))
        );

        assert_eq!(
            expr2().parse("1*2 + 3*4"),
            Ok(ExprNode::Binary(BinaryOpNode {
                lhs: Box::from(ExprNode::Binary(BinaryOpNode {
                    lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                    op: Op::Mul,
                    rhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2)))
                })),
                op: Op::Add,
                rhs: Box::from(ExprNode::Binary(BinaryOpNode {
                    lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3))),
                    op: Op::Mul,
                    rhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(4)))
                })),
            }))
        );

        assert_eq!(
            expr2().parse("1"),
            Ok(ExprNode::Integer(IntegerLiteralNode::I32(1)))
        );
    }

    #[test]
    fn expr1_test() {
        assert_eq!(
            expr1().parse("1*1"),
            Ok(ExprNode::Binary(BinaryOpNode {
                lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                op: Op::Mul,
                rhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
            }))
        );
        assert_eq!(
            expr1().parse("1 / 1"),
            Ok(ExprNode::Binary(BinaryOpNode {
                lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                op: Op::Div,
                rhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
            }))
        );
        assert_eq!(
            expr1().parse("1 %2"),
            Ok(ExprNode::Binary(BinaryOpNode {
                lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                op: Op::Rem,
                rhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2))),
            }))
        );

        assert_eq!(
            expr1().parse("1 % 2 / 3 * 4"),
            Ok(ExprNode::Binary(BinaryOpNode {
                lhs: Box::from(ExprNode::Binary(BinaryOpNode {
                    lhs: Box::from(ExprNode::Binary(BinaryOpNode {
                        lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
                        op: Op::Rem,
                        rhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2)))
                    })),
                    op: Op::Div,
                    rhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(3)))
                })),
                op: Op::Mul,
                rhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(4))),
            }))
        );

        assert_eq!(
            expr1().parse("1"),
            Ok(ExprNode::Integer(IntegerLiteralNode::I32(1)))
        );
    }

    #[test]
    fn primary_test() {
        assert_eq!(
            primary().parse("1"),
            Ok(ExprNode::Integer(IntegerLiteralNode::I32(1)))
        );
        assert_eq!(
            primary().parse("'a'"),
            Ok(ExprNode::Integer(IntegerLiteralNode::I8(97)))
        );
        assert_eq!(
            primary().parse("\"a\""),
            Ok(ExprNode::String(StringLiteralNode("a".to_string())))
        );
        assert_eq!(
            primary().parse("var"),
            Ok(ExprNode::Variable(VariableNode("var".to_string())))
        );
    }
}
