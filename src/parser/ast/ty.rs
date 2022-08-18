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
