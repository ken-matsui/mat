/// String Literal Node
use crate::parser::lib::*;

// pointer for i8
pub(crate) fn string() -> impl Parser<String> {
    filter(|c: &char| c.is_ascii() && *c != '"')
        .repeated()
        .delimited_by(just('"'), just('"'))
        .collect::<String>()
        .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;

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
