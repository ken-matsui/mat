use crate::ast::Expr;
use crate::parser::ast::{ident, Spanned};
use crate::parser::lib::*;

pub(crate) fn variable() -> impl Parser<Spanned<Expr>> {
    ident()
        .map(Expr::Variable)
        .map_with_span(Spanned::new)
        .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable() {
        assert_eq!(
            variable().parse_test("var"),
            Ok(Spanned::any(Expr::Variable("var".to_string())))
        );
        assert_eq!(
            variable().parse_test("var    "),
            Ok(Spanned::any(Expr::Variable("var".to_string())))
        );
        assert_eq!(
            variable().parse_test("  var"),
            Ok(Spanned::any(Expr::Variable("var".to_string())))
        );
        assert!(variable().parse_test("1var").is_err());
    }
}
