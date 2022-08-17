use crate::parser::ast::{
    character, integer, string, variable, IntegerLiteralNode, StringLiteralNode, VariableNode,
};
use chumsky::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Op {
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

pub(crate) fn expr1() -> impl Parser<char, ExprNode, Error = Simple<char>> + Clone {
    choice((
        // recursive(|trm| {
        //     trm.ignored()
        //         .then(term())
        //         .then(one_of("*/%"))
        //         .then(term())
        //         .map(|((((), lhs), op), rhs)| {
        //             ExprNode::Binary(BinaryOpNode {
        //                 lhs: Box::new(lhs),
        //                 op: op.to_string(),
        //                 rhs: Box::new(rhs),
        //             })
        //         })
        // }),
        term()
            .then(
                just('*')
                    .to(Op::Mul)
                    .or(just('/').to(Op::Div))
                    .or(just('%').to(Op::Rem)),
            )
            .then(term())
            .map(|((lhs, op), rhs)| {
                ExprNode::Binary(BinaryOpNode {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(rhs),
                })
            }),
        // term()
        //     .then_ignore(just('/'))
        //     .then(term())
        //     .map(|(lhs, rhs)| {
        //         ExprNode::Binary(BinaryOpNode {
        //             lhs: Box::new(lhs),
        //             op: "/".to_string(),
        //             rhs: Box::new(rhs),
        //         })
        //     }),
        // term()
        //     .then_ignore(just('%'))
        //     .then(term())
        //     .map(|(lhs, rhs)| {
        //         ExprNode::Binary(BinaryOpNode {
        //             lhs: Box::new(lhs),
        //             op: "%".to_string(),
        //             rhs: Box::new(rhs),
        //         })
        //     }),
        term(),
    ))
    // .repeated()
    // .foldl(|lhs, rhs| ExprNode::Expr(Box::new(rhs)))
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
    .padded()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;

    #[test]
    fn expr1_test() {
        assert_eq!(
            expr1().parse("1i8 *1"),
            Ok(ExprNode::Binary(BinaryOpNode {
                lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I8(1))),
                op: Op::Mul,
                rhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
            }))
        );
        assert_eq!(
            expr1().parse("1i8 / 1"),
            Ok(ExprNode::Binary(BinaryOpNode {
                lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I8(1))),
                op: Op::Div,
                rhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(1))),
            }))
        );
        assert_eq!(
            expr1().parse("1i8 % 2"),
            Ok(ExprNode::Binary(BinaryOpNode {
                lhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I8(1))),
                op: Op::Rem,
                rhs: Box::from(ExprNode::Integer(IntegerLiteralNode::I32(2))),
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
