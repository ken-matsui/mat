use chumsky::prelude::*;

#[derive(Debug, PartialEq)]
pub(crate) struct VariableNode(String);

pub(crate) fn variable() -> impl Parser<char, VariableNode, Error = Simple<char>> + Clone {
    text::ident::<_, Simple<char>>().map(VariableNode).padded()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;

    #[test]
    fn variable_test() {
        assert_eq!(variable().parse("var"), Ok(VariableNode("var".to_string())));
        assert_eq!(
            variable().parse("  var"),
            Ok(VariableNode("var".to_string()))
        );
        assert_eq!(
            variable().parse("var    "),
            Ok(VariableNode("var".to_string()))
        );
        assert!(variable().parse("1var").is_err());
    }
}
