/// String Literal Node
use crate::parser::ast::{Expr, Spanned};
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
    fn string_test() {
        assert_eq!(
            string().parse("\"a\""),
            Ok(Spanned::any(Expr::String("a".to_string())))
        );
        assert_eq!(
            string().parse("\"a\"     "),
            Ok(Spanned::any(Expr::String("a".to_string())))
        );
        assert_eq!(
            string().parse("\"1 \""),
            Ok(Spanned::any(Expr::String("1 ".to_string())))
        );
        assert_eq!(
            string().parse("\"\n\""),
            Ok(Spanned::any(Expr::String("\n".to_string())))
        );
        assert!(string().parse("    \"a\"").is_err());
        assert!(string().parse("\"a").is_err());
        assert!(string().parse("a\"").is_err());
        assert!(string().parse("a").is_err());
    }
}
