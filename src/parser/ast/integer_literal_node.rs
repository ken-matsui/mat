use chumsky::prelude::*;

#[derive(Debug, PartialEq)]
pub(crate) enum IntegerLiteralNode {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

pub(crate) fn integer() -> impl Parser<char, IntegerLiteralNode, Error = Simple<char>> + Clone {
    let dec = text::int::<_, Simple<char>>(10);

    choice((
        // No suffix
        dec.try_map(|s, span| {
            s.parse::<i32>()
                .map_err(|e| Simple::custom(span, format!("{}", e)))
        })
        .then_ignore(end())
        .map(IntegerLiteralNode::I32),
        // With suffix
        dec.then_ignore(just("i8"))
            .try_map(|s, span| {
                s.parse::<i8>()
                    .map_err(|e| Simple::custom(span, format!("{}", e)))
            })
            .map(IntegerLiteralNode::I8),
        dec.then_ignore(just("i16"))
            .try_map(|s, span| {
                s.parse::<i16>()
                    .map_err(|e| Simple::custom(span, format!("{}", e)))
            })
            .map(IntegerLiteralNode::I16),
        dec.then_ignore(just("i32"))
            .try_map(|s, span| {
                s.parse::<i32>()
                    .map_err(|e| Simple::custom(span, format!("{}", e)))
            })
            .map(IntegerLiteralNode::I32),
        dec.then_ignore(just("i64"))
            .try_map(|s, span| {
                s.parse::<i64>()
                    .map_err(|e| Simple::custom(span, format!("{}", e)))
            })
            .map(IntegerLiteralNode::I64),
        dec.then_ignore(just("u8"))
            .try_map(|s, span| {
                s.parse::<u8>()
                    .map_err(|e| Simple::custom(span, format!("{}", e)))
            })
            .map(IntegerLiteralNode::U8),
        dec.then_ignore(just("u16"))
            .try_map(|s, span| {
                s.parse::<u16>()
                    .map_err(|e| Simple::custom(span, format!("{}", e)))
            })
            .map(IntegerLiteralNode::U16),
        dec.then_ignore(just("u32"))
            .try_map(|s, span| {
                s.parse::<u32>()
                    .map_err(|e| Simple::custom(span, format!("{}", e)))
            })
            .map(IntegerLiteralNode::U32),
        dec.then_ignore(just("u64"))
            .try_map(|s, span| {
                s.parse::<u64>()
                    .map_err(|e| Simple::custom(span, format!("{}", e)))
            })
            .map(IntegerLiteralNode::U64),
    ))
}

pub(crate) fn character() -> impl Parser<char, IntegerLiteralNode, Error = Simple<char>> + Clone {
    filter(|c: &char| c.is_ascii())
        .delimited_by(just('\''), just('\''))
        .map(|c| IntegerLiteralNode::I8(c as i8))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;

    #[test]
    fn integer_test() {
        assert_eq!(integer().parse("0"), Ok(IntegerLiteralNode::I32(0)));
        assert_eq!(
            integer().parse("2147483647"),
            Ok(IntegerLiteralNode::I32(2147483647))
        );
        assert!(integer().parse("2147483648").is_err());

        assert_eq!(integer().parse("0i8 "), Ok(IntegerLiteralNode::I8(0)));
        assert!(integer().parse("0 i8").is_err());
        assert_eq!(integer().parse("127i8"), Ok(IntegerLiteralNode::I8(127)));
        assert!(integer().parse("128i8").is_err());

        assert_eq!(integer().parse("0i16 "), Ok(IntegerLiteralNode::I16(0)));
        assert!(integer().parse("0 i16").is_err());
        assert_eq!(
            integer().parse("32767i16"),
            Ok(IntegerLiteralNode::I16(32767))
        );
        assert!(integer().parse("32768i16").is_err());

        assert_eq!(integer().parse("0i32 "), Ok(IntegerLiteralNode::I32(0)));
        assert!(integer().parse("0 i32").is_err());
        assert_eq!(
            integer().parse("2147483647i32"),
            Ok(IntegerLiteralNode::I32(2147483647))
        );
        assert!(integer().parse("2147483648i32").is_err());

        assert_eq!(integer().parse("0i64 "), Ok(IntegerLiteralNode::I64(0)));
        assert!(integer().parse("0 i64").is_err());
        assert_eq!(
            integer().parse("9223372036854775807i64"),
            Ok(IntegerLiteralNode::I64(9223372036854775807))
        );
        assert!(integer().parse("9223372036854775808").is_err());

        assert_eq!(integer().parse("0u8 "), Ok(IntegerLiteralNode::U8(0)));
        assert!(integer().parse("0 u8").is_err());
        assert_eq!(integer().parse("255u8"), Ok(IntegerLiteralNode::U8(255)));
        assert!(integer().parse("256u8").is_err());

        assert_eq!(integer().parse("0u16 "), Ok(IntegerLiteralNode::U16(0)));
        assert!(integer().parse("0 u16").is_err());
        assert_eq!(
            integer().parse("65535u16"),
            Ok(IntegerLiteralNode::U16(65535))
        );
        assert!(integer().parse("65536u16").is_err());

        assert_eq!(integer().parse("0u32 "), Ok(IntegerLiteralNode::U32(0)));
        assert!(integer().parse("0 u32").is_err());
        assert_eq!(
            integer().parse("4294967295u32"),
            Ok(IntegerLiteralNode::U32(4294967295))
        );
        assert!(integer().parse("4294967296u32").is_err());

        assert_eq!(integer().parse("0u64 "), Ok(IntegerLiteralNode::U64(0)));
        assert!(integer().parse("0 u64").is_err());
        assert_eq!(
            integer().parse("18446744073709551615u64"),
            Ok(IntegerLiteralNode::U64(18446744073709551615))
        );
        assert!(integer().parse("18446744073709551616u64").is_err());
    }

    #[test]
    fn character_test() {
        assert_eq!(character().parse("'a'"), Ok(IntegerLiteralNode::I8(97)));
        assert_eq!(character().parse("'1'"), Ok(IntegerLiteralNode::I8(49)));
        assert_eq!(character().parse("'\n'"), Ok(IntegerLiteralNode::I8(10)));
        assert!(character().parse("'a").is_err());
        assert!(character().parse("a'").is_err());
        assert!(character().parse("a").is_err());
        assert!(character().parse("'aa'").is_err());
        assert!(character().parse("'\nn'").is_err());
    }
}
