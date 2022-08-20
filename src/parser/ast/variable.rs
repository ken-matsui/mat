use crate::parser::ast::ident;
use crate::parser::lib::*;

pub(crate) fn variable() -> impl Parser<String> {
    ident().boxed()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variable_test() {
        assert_eq!(variable().parse("var"), Ok("var".to_string()));
        assert_eq!(variable().parse("var    "), Ok("var".to_string()));
        assert_eq!(variable().parse("  var"), Ok("var".to_string()));
        assert!(variable().parse("1var").is_err());
    }
}
