use crate::parser::ast::{Expr, Spanned};
use crate::parser::lib::*;
use crate::parser::Error;

pub(crate) fn integer() -> impl Parser<Spanned<Expr>> {
    text::int::<_, Error>(10)
        .then(
            choice((
                just("i8"),
                just("i16"),
                just("i32"),
                just("i64"),
                just("u8"),
                just("u16"),
                just("u32"),
                just("u64"),
            ))
            .or_not(),
        )
        .padded()
        .try_map(|(num, suf), span| {
            match suf {
                // With suffix
                Some("i8") => num.parse().map(Expr::I8),
                Some("i16") => num.parse().map(Expr::I16),
                Some("i32") => num.parse().map(Expr::I32),
                Some("i64") => num.parse().map(Expr::I64),
                Some("u8") => num.parse().map(Expr::U8),
                Some("u16") => num.parse().map(Expr::U16),
                Some("u32") => num.parse().map(Expr::U32),
                Some("u64") => num.parse().map(Expr::U64),
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
        assert_eq!(integer().parse_test("0 i8"), Ok(Spanned::any(Expr::I32(0))));

        assert_eq!(integer().parse_test("0i8 "), Ok(Spanned::any(Expr::I8(0))));
        assert_eq!(
            integer().parse_test("127i8"),
            Ok(Spanned::any(Expr::I8(127)))
        );
        assert!(integer().parse_test("128i8").is_err());

        assert_eq!(
            integer().parse_test("0i16 "),
            Ok(Spanned::any(Expr::I16(0)))
        );
        assert_eq!(
            integer().parse_test("32767i16"),
            Ok(Spanned::any(Expr::I16(32767)))
        );
        assert!(integer().parse_test("32768i16").is_err());

        assert_eq!(
            integer().parse_test("0i32 "),
            Ok(Spanned::any(Expr::I32(0)))
        );
        assert_eq!(
            integer().parse_test("2147483647i32"),
            Ok(Spanned::any(Expr::I32(2147483647)))
        );
        assert!(integer().parse_test("2147483648i32").is_err());

        assert_eq!(
            integer().parse_test("0i64 "),
            Ok(Spanned::any(Expr::I64(0)))
        );
        assert_eq!(
            integer().parse_test("9223372036854775807i64"),
            Ok(Spanned::any(Expr::I64(9223372036854775807)))
        );
        assert!(integer().parse_test("9223372036854775808").is_err());

        assert_eq!(integer().parse_test("0u8 "), Ok(Spanned::any(Expr::U8(0))));
        assert_eq!(
            integer().parse_test("255u8"),
            Ok(Spanned::any(Expr::U8(255)))
        );
        assert!(integer().parse_test("256u8").is_err());

        assert_eq!(
            integer().parse_test("0u16 "),
            Ok(Spanned::any(Expr::U16(0)))
        );
        assert_eq!(
            integer().parse_test("65535u16"),
            Ok(Spanned::any(Expr::U16(65535)))
        );
        assert!(integer().parse_test("65536u16").is_err());

        assert_eq!(
            integer().parse_test("0u32 "),
            Ok(Spanned::any(Expr::U32(0)))
        );
        assert_eq!(
            integer().parse_test("4294967295u32"),
            Ok(Spanned::any(Expr::U32(4294967295)))
        );
        assert!(integer().parse_test("4294967296u32").is_err());

        assert_eq!(
            integer().parse_test("0u64 "),
            Ok(Spanned::any(Expr::U64(0)))
        );
        assert_eq!(
            integer().parse_test("18446744073709551615u64"),
            Ok(Spanned::any(Expr::U64(18446744073709551615)))
        );
        assert!(integer().parse_test("18446744073709551616u64").is_err());
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
