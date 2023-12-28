use crate::error::Error;
use crate::prelude::*;
use matc_ast::Expr;
use matc_span::Spanned;

pub(crate) fn integer() -> impl Parser<Spanned<Expr>> {
    text::int::<_, Error>(10)
        .then(just("i32").or_not())
        .padded()
        .try_map(|(num, suf), span| {
            match suf {
                // With suffix
                Some("i32") => num.parse().map(Expr::I32),
                // No suffix
                _ => num.parse().map(Expr::I32),
            }
            .map_err(|e| Simple::custom(span, format!("{}", e)))
        })
        .map_with_span(Spanned::new)
        .boxed()
}

pub(crate) fn character() -> impl Parser<Spanned<Expr>> {
    filter(|c: &char| c.is_ascii())
        .delimited_by(just('\''), just('\''))
        .map(|c| Expr::I8(c as i8))
        .map_with_span(Spanned::new)
        .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer() {
        assert_eq!(integer().parse_test("0"), Ok(Spanned::any(Expr::I32(0))));
        assert_eq!(
            integer().parse_test("2147483647"),
            Ok(Spanned::any(Expr::I32(2147483647)))
        );
        assert!(integer().parse_test("2147483648").is_err());

        assert_eq!(
            integer().parse_test("0i32 "),
            Ok(Spanned::any(Expr::I32(0)))
        );
        assert_eq!(
            integer().parse_test("2147483647i32"),
            Ok(Spanned::any(Expr::I32(2147483647)))
        );
        assert!(integer().parse_test("2147483648i32").is_err());
    }

    #[test]
    fn test_character() {
        assert_eq!(
            character().parse_test("'a'"),
            Ok(Spanned::any(Expr::I8(97)))
        );
        assert_eq!(
            character().parse_test("'1'"),
            Ok(Spanned::any(Expr::I8(49)))
        );
        assert_eq!(
            character().parse_test("'\n'"),
            Ok(Spanned::any(Expr::I8(10)))
        );
        assert!(character().parse_test("'a").is_err());
        assert!(character().parse_test("a'").is_err());
        assert!(character().parse_test("a").is_err());
        assert!(character().parse_test("'aa'").is_err());
        assert!(character().parse_test("'\nn'").is_err());
    }
}
