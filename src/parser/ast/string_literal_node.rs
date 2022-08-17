use chumsky::prelude::*;

// pointer for i8
#[derive(Debug, PartialEq)]
pub(crate) struct StringLiteralNode(String);

pub(crate) fn string() -> impl Parser<char, StringLiteralNode, Error = Simple<char>> + Clone {
    filter(|c: &char| c.is_ascii() && *c != '"')
        .repeated()
        .delimited_by(just('"'), just('"'))
        .collect::<String>()
        .map(StringLiteralNode)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;

    #[test]
    fn string_test() {
        assert_eq!(
            string().parse("\"a\""),
            Ok(StringLiteralNode("a".to_string()))
        );
        assert_eq!(
            string().parse("\"1 \""),
            Ok(StringLiteralNode("1 ".to_string()))
        );
        assert_eq!(
            string().parse("\"\n\""),
            Ok(StringLiteralNode("\n".to_string()))
        );
        assert!(string().parse("\"a").is_err());
        assert!(string().parse("a\"").is_err());
        assert!(string().parse("a").is_err());
    }
}
