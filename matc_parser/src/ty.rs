/// Type Node
use crate::prelude::*;
use matc_ast::Type;
use matc_span::Spanned;

pub(crate) fn typeref() -> impl Parser<Spanned<Type>> {
    choice((
        text::keyword("void").to(Type::Void),
        text::keyword("char").to(Type::I8),
        text::keyword("i32").to(Type::I32),
    ))
    .map_with_span(Spanned::new)
    .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typeref() {
        assert_eq!(typeref().parse_test("void"), Ok(Spanned::any(Type::Void)));
        assert_eq!(typeref().parse_test("char"), Ok(Spanned::any(Type::I8)));
        assert_eq!(typeref().parse_test("i32"), Ok(Spanned::any(Type::I32)));
    }
}
