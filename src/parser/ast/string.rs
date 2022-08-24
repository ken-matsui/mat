/// String Literal Node
use crate::ast::expr::Expr;
use crate::parser::ast::Spanned;
use crate::parser::lib::*;

// pointer for i8
pub(crate) fn string() -> impl Parser<Spanned<Expr>> {
    filter(|c: &char| c.is_ascii() && *c != '"')
        .repeated()
        .delimited_by(just('"'), just('"'))
        .collect::<String>()
        .map(Expr::String)
        .map_with_span(Spanned::new)
        .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        assert_eq!(
            string().parse_test("\"a\""),
            Ok(Spanned::any(Expr::String("a".to_string())))
        );
        assert_eq!(
            string().parse_test("\"a\"     "),
            Ok(Spanned::any(Expr::String("a".to_string())))
        );
        assert_eq!(
            string().parse_test("\"1 \""),
            Ok(Spanned::any(Expr::String("1 ".to_string())))
        );
        assert_eq!(
            string().parse_test("\"\n\""),
            Ok(Spanned::any(Expr::String("\n".to_string())))
        );
        assert!(string().parse_test("    \"a\"").is_err());
        assert!(string().parse_test("\"a").is_err());
        assert!(string().parse_test("a\"").is_err());
        assert!(string().parse_test("a").is_err());
    }
}
