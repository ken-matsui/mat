use crate::parser::ast::{
    character, integer, string, variable, IntegerLiteralNode, StringLiteralNode, VariableNode,
};
use chumsky::prelude::*;

#[derive(Debug, PartialEq)]
pub(crate) enum ExprNode {
    Integer(IntegerLiteralNode),
    String(StringLiteralNode),
    Variable(VariableNode),
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
