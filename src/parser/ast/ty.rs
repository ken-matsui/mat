use crate::parser::ast::{ident, Spanned};
use crate::parser::lib::*;
/// Type Node
use matc_ast::{Stmt, Type};

pub(crate) fn typeref() -> impl Parser<Spanned<Type>> {
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
    .map_with_span(Spanned::new)
    .boxed()
}

/// type new = old;
pub(crate) fn typedef() -> impl Parser<Spanned<Stmt>> {
    text::keyword("type")
        .padded()
        .ignore_then(ident().map_with_span(Spanned::new).padded())
        .then_ignore(just('='))
        .then(typeref().padded())
        .then_ignore(just(';'))
        .map_with_span(|(name, ty), span| Spanned::new(Stmt::TypeDef { name, ty }, span))
        .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typeref() {
        assert_eq!(typeref().parse_test("void"), Ok(Spanned::any(Type::Void)));
        assert_eq!(typeref().parse_test("char"), Ok(Spanned::any(Type::I8)));
        assert_eq!(typeref().parse_test("i8"), Ok(Spanned::any(Type::I8)));
        assert_eq!(typeref().parse_test("i16"), Ok(Spanned::any(Type::I16)));
        assert_eq!(typeref().parse_test("i32"), Ok(Spanned::any(Type::I32)));
        assert_eq!(typeref().parse_test("i64"), Ok(Spanned::any(Type::I64)));
        assert_eq!(typeref().parse_test("u8"), Ok(Spanned::any(Type::U8)));
        assert_eq!(typeref().parse_test("u16"), Ok(Spanned::any(Type::U16)));
        assert_eq!(typeref().parse_test("u32"), Ok(Spanned::any(Type::U32)));
        assert_eq!(typeref().parse_test("u64"), Ok(Spanned::any(Type::U64)));
        assert_eq!(
            typeref().parse_test("type"),
            Ok(Spanned::any(Type::User("type".to_string())))
        );
        assert!(typeref().parse_test("1type").is_err());
    }

    #[test]
    fn test_typedef() {
        assert_eq!(
            typedef().parse_test("type new = i8;"),
            Ok(Spanned::any(Stmt::TypeDef {
                name: Spanned::any("new".to_string()),
                ty: Spanned::any(Type::I8)
            }))
        );
        assert_eq!(
            typedef().parse_test("type new=i8;"),
            Ok(Spanned::any(Stmt::TypeDef {
                name: Spanned::any("new".to_string()),
                ty: Spanned::any(Type::I8)
            }))
        );
        assert_eq!(
            typedef().parse_test("type new=i8  ;"),
            Ok(Spanned::any(Stmt::TypeDef {
                name: Spanned::any("new".to_string()),
                ty: Spanned::any(Type::I8)
            }))
        );
        assert_eq!(
            typedef().parse_test("type new = old;"),
            Ok(Spanned::any(Stmt::TypeDef {
                name: Spanned::any("new".to_string()),
                ty: Spanned::any(Type::User("old".to_string())),
            }))
        );
        // TODO: For now, this is allowed (will be an error at semantic analysis),
        //   but it would be better to ban at parse phase.
        assert_eq!(
            typedef().parse_test("type i8 = old;"),
            Ok(Spanned::any(Stmt::TypeDef {
                name: Spanned::any("i8".to_string()),
                ty: Spanned::any(Type::User("old".to_string())),
            }))
        );
        assert!(typedef().parse_test("type foo = bar").is_err());
        assert!(typedef().parse_test("type foo = ;").is_err());
        assert!(typedef().parse_test("type foo bar;").is_err());
        assert!(typedef().parse_test("type = bar;").is_err());
        assert!(typedef().parse_test("foo = bar;").is_err());
        assert!(typedef().parse_test("typefoo = bar;").is_err());
    }
}
