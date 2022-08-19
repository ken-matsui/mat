use chumsky::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Int {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

pub(crate) fn integer() -> impl Parser<char, Int, Error = Simple<char>> + Clone {
    text::int::<_, Simple<char>>(10)
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
                Some("i8") => num.parse::<i8>().map(Int::I8),
                Some("i16") => num.parse::<i16>().map(Int::I16),
                Some("i32") => num.parse::<i32>().map(Int::I32),
                Some("i64") => num.parse::<i64>().map(Int::I64),
                Some("u8") => num.parse::<u8>().map(Int::U8),
                Some("u16") => num.parse::<u16>().map(Int::U16),
                Some("u32") => num.parse::<u32>().map(Int::U32),
                Some("u64") => num.parse::<u64>().map(Int::U64),
                // No suffix
                _ => num.parse::<i32>().map(Int::I32),
            }
            .map_err(|e| Simple::custom(span, format!("{}", e)))
        })
        .boxed()
}

pub(crate) fn character() -> impl Parser<char, Int, Error = Simple<char>> + Clone {
    filter(|c: &char| c.is_ascii())
        .delimited_by(just('\''), just('\''))
        .map(|c| Int::I8(c as i8))
        .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;

    #[test]
    fn integer_test() {
        assert_eq!(integer().parse("0"), Ok(Int::I32(0)));
        assert_eq!(integer().parse("2147483647"), Ok(Int::I32(2147483647)));
        assert!(integer().parse("2147483648").is_err());
        assert_eq!(integer().parse("0 i8"), Ok(Int::I32(0)));

        assert_eq!(integer().parse("0i8 "), Ok(Int::I8(0)));
        assert_eq!(integer().parse("127i8"), Ok(Int::I8(127)));
        assert!(integer().parse("128i8").is_err());

        assert_eq!(integer().parse("0i16 "), Ok(Int::I16(0)));
        assert_eq!(integer().parse("32767i16"), Ok(Int::I16(32767)));
        assert!(integer().parse("32768i16").is_err());

        assert_eq!(integer().parse("0i32 "), Ok(Int::I32(0)));
        assert_eq!(integer().parse("2147483647i32"), Ok(Int::I32(2147483647)));
        assert!(integer().parse("2147483648i32").is_err());

        assert_eq!(integer().parse("0i64 "), Ok(Int::I64(0)));
        assert_eq!(
            integer().parse("9223372036854775807i64"),
            Ok(Int::I64(9223372036854775807))
        );
        assert!(integer().parse("9223372036854775808").is_err());

        assert_eq!(integer().parse("0u8 "), Ok(Int::U8(0)));
        assert_eq!(integer().parse("255u8"), Ok(Int::U8(255)));
        assert!(integer().parse("256u8").is_err());

        assert_eq!(integer().parse("0u16 "), Ok(Int::U16(0)));
        assert_eq!(integer().parse("65535u16"), Ok(Int::U16(65535)));
        assert!(integer().parse("65536u16").is_err());

        assert_eq!(integer().parse("0u32 "), Ok(Int::U32(0)));
        assert_eq!(integer().parse("4294967295u32"), Ok(Int::U32(4294967295)));
        assert!(integer().parse("4294967296u32").is_err());

        assert_eq!(integer().parse("0u64 "), Ok(Int::U64(0)));
        assert_eq!(
            integer().parse("18446744073709551615u64"),
            Ok(Int::U64(18446744073709551615))
        );
        assert!(integer().parse("18446744073709551616u64").is_err());
    }

    #[test]
    fn character_test() {
        assert_eq!(character().parse("'a'"), Ok(Int::I8(97)));
        assert_eq!(character().parse("'1'"), Ok(Int::I8(49)));
        assert_eq!(character().parse("'\n'"), Ok(Int::I8(10)));
        assert!(character().parse("'a").is_err());
        assert!(character().parse("a'").is_err());
        assert!(character().parse("a").is_err());
        assert!(character().parse("'aa'").is_err());
        assert!(character().parse("'\nn'").is_err());
    }
}
