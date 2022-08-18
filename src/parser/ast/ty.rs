use crate::parser::ast::Stmt;
/// Type Node
use chumsky::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Type {
    Void,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    /// User defined type
    User(String),
}

pub(crate) fn typeref() -> impl Parser<char, Type, Error = Simple<char>> + Clone {
    choice((
        text::keyword("void").to(Type::Void),
        text::keyword("char").to(Type::I8),
        text::keyword("i8").to(Type::I8),
        text::keyword("i16").to(Type::I16),
        text::keyword("i32").to(Type::I32),
        text::keyword("i64").to(Type::I64),
        text::keyword("u8").to(Type::U8),
        text::keyword("u16").to(Type::U16),
        text::keyword("u32").to(Type::U32),
        text::keyword("u64").to(Type::U64),
        text::ident().map(Type::User),
    ))
}

/// type new = old;
pub(crate) fn typedef() -> impl Parser<char, Stmt, Error = Simple<char>> + Clone {
    text::keyword("type")
        .padded()
        .then(text::ident::<_, Simple<char>>())
        .then_ignore(just('='))
        .then(typeref())
        .then_ignore(just(';'))
        .map(|(((), new), old)| Stmt::TypeDef { new, old })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::Int;
    use chumsky::Parser;

    #[test]
    fn typeref_test() {
        assert_eq!(typeref().parse("void"), Ok(Type::Void));
        assert_eq!(typeref().parse("char"), Ok(Type::I8));
        assert_eq!(typeref().parse("i8"), Ok(Type::I8));
        assert_eq!(typeref().parse("i16"), Ok(Type::I16));
        assert_eq!(typeref().parse("i32"), Ok(Type::I32));
        assert_eq!(typeref().parse("i64"), Ok(Type::I64));
        assert_eq!(typeref().parse("u8"), Ok(Type::U8));
        assert_eq!(typeref().parse("u16"), Ok(Type::U16));
        assert_eq!(typeref().parse("u32"), Ok(Type::U32));
        assert_eq!(typeref().parse("u64"), Ok(Type::U64));
        assert_eq!(typeref().parse("type"), Ok(Type::User("type".to_string())));
        assert!(typeref().parse("1type").is_err());
    }
}
