/// Type Node
use crate::parser::ast::{ident, Spanned, Stmt};
use crate::parser::lib::*;

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

pub(crate) fn typeref() -> impl Parser<Type> {
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
        ident().map(Type::User),
    ))
    .boxed()
}

/// type new = old;
pub(crate) fn typedef() -> impl Parser<Spanned<Stmt>> {
    text::keyword("type")
        .padded()
        .ignore_then(ident().padded())
        .then_ignore(just('='))
        .then(typeref().padded())
        .then_ignore(just(';'))
        .map_with_span(|(new, old), span| Spanned::new(Stmt::TypeDef { new, old }, span))
        .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn typedef_test() {
        assert_eq!(
            typedef().parse("type new = i8;"),
            Ok(Spanned::any(Stmt::TypeDef {
                new: "new".to_string(),
                old: Type::I8
            }))
        );
        assert_eq!(
            typedef().parse("type new=i8;"),
            Ok(Spanned::any(Stmt::TypeDef {
                new: "new".to_string(),
                old: Type::I8
            }))
        );
        assert_eq!(
            typedef().parse("type new=i8  ;"),
            Ok(Spanned::any(Stmt::TypeDef {
                new: "new".to_string(),
                old: Type::I8
            }))
        );
        assert_eq!(
            typedef().parse("type new = old;"),
            Ok(Spanned::any(Stmt::TypeDef {
                new: "new".to_string(),
                old: Type::User("old".to_string()),
            }))
        );
        // TODO: For now, this is allowed (will be an error at semantic analysis),
        //   but it would be better to ban at parse time.
        assert_eq!(
            typedef().parse("type i8 = old;"),
            Ok(Spanned::any(Stmt::TypeDef {
                new: "i8".to_string(),
                old: Type::User("old".to_string()),
            }))
        );
        assert!(typedef().parse("type foo = bar").is_err());
        assert!(typedef().parse("type foo = ;").is_err());
        assert!(typedef().parse("type foo bar;").is_err());
        assert!(typedef().parse("type = bar;").is_err());
        assert!(typedef().parse("foo = bar;").is_err());
        assert!(typedef().parse("typefoo = bar;").is_err());
    }
}
