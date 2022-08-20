use crate::parser::ast::{Expr, Spanned};
use crate::parser::lib::*;

pub(crate) fn integer() -> impl Parser<Spanned<Expr>> {
    text::int::<_, ParserError>(10)
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
    fn integer_test() {
        assert_eq!(integer().parse("0"), Ok(Spanned::any(Expr::I32(0))));
        assert_eq!(
            integer().parse("2147483647"),
            Ok(Spanned::any(Expr::I32(2147483647)))
        );
        assert!(integer().parse("2147483648").is_err());
        assert_eq!(integer().parse("0 i8"), Ok(Spanned::any(Expr::I32(0))));

        assert_eq!(integer().parse("0i8 "), Ok(Spanned::any(Expr::I8(0))));
        assert_eq!(integer().parse("127i8"), Ok(Spanned::any(Expr::I8(127))));
        assert!(integer().parse("128i8").is_err());

        assert_eq!(integer().parse("0i16 "), Ok(Spanned::any(Expr::I16(0))));
        assert_eq!(
            integer().parse("32767i16"),
            Ok(Spanned::any(Expr::I16(32767)))
        );
        assert!(integer().parse("32768i16").is_err());

        assert_eq!(integer().parse("0i32 "), Ok(Spanned::any(Expr::I32(0))));
        assert_eq!(
            integer().parse("2147483647i32"),
            Ok(Spanned::any(Expr::I32(2147483647)))
        );
        assert!(integer().parse("2147483648i32").is_err());

        assert_eq!(integer().parse("0i64 "), Ok(Spanned::any(Expr::I64(0))));
        assert_eq!(
            integer().parse("9223372036854775807i64"),
            Ok(Spanned::any(Expr::I64(9223372036854775807)))
        );
        assert!(integer().parse("9223372036854775808").is_err());

        assert_eq!(integer().parse("0u8 "), Ok(Spanned::any(Expr::U8(0))));
        assert_eq!(integer().parse("255u8"), Ok(Spanned::any(Expr::U8(255))));
        assert!(integer().parse("256u8").is_err());

        assert_eq!(integer().parse("0u16 "), Ok(Spanned::any(Expr::U16(0))));
        assert_eq!(
            integer().parse("65535u16"),
            Ok(Spanned::any(Expr::U16(65535)))
        );
        assert!(integer().parse("65536u16").is_err());

        assert_eq!(integer().parse("0u32 "), Ok(Spanned::any(Expr::U32(0))));
        assert_eq!(
            integer().parse("4294967295u32"),
            Ok(Spanned::any(Expr::U32(4294967295)))
        );
        assert!(integer().parse("4294967296u32").is_err());

        assert_eq!(integer().parse("0u64 "), Ok(Spanned::any(Expr::U64(0))));
        assert_eq!(
            integer().parse("18446744073709551615u64"),
            Ok(Spanned::any(Expr::U64(18446744073709551615)))
        );
        assert!(integer().parse("18446744073709551616u64").is_err());
    }

    #[test]
    fn character_test() {
        assert_eq!(character().parse("'a'"), Ok(Spanned::any(Expr::I8(97))));
        assert_eq!(character().parse("'1'"), Ok(Spanned::any(Expr::I8(49))));
        assert_eq!(character().parse("'\n'"), Ok(Spanned::any(Expr::I8(10))));
        assert!(character().parse("'a").is_err());
        assert!(character().parse("a'").is_err());
        assert!(character().parse("a").is_err());
        assert!(character().parse("'aa'").is_err());
        assert!(character().parse("'\nn'").is_err());
    }
}
