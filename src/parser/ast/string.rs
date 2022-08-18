/// String Literal Node
use chumsky::prelude::*;

// pointer for i8
pub(crate) fn string() -> impl Parser<char, String, Error = Simple<char>> + Clone {
    filter(|c: &char| c.is_ascii() && *c != '"')
        .repeated()
        .delimited_by(just('"'), just('"'))
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;

    #[test]
    fn string_test() {
        assert_eq!(string().parse("\"a\""), Ok("a".to_string()));
        assert_eq!(string().parse("\"a\"     "), Ok("a".to_string()));
        assert_eq!(string().parse("\"1 \""), Ok("1 ".to_string()));
        assert_eq!(string().parse("\"\n\""), Ok("\n".to_string()));
        assert!(string().parse("    \"a\"").is_err());
        assert!(string().parse("\"a").is_err());
        assert!(string().parse("a\"").is_err());
        assert!(string().parse("a").is_err());
    }
}
