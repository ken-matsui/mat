use crate::parser::ast::ident;
use chumsky::prelude::*;

pub(crate) fn variable() -> impl Parser<char, String, Error = Simple<char>> + Clone {
    ident()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;

    #[test]
    fn variable_test() {
        assert_eq!(variable().parse("var"), Ok("var".to_string()));
        assert_eq!(variable().parse("var    "), Ok("var".to_string()));
        assert_eq!(variable().parse("  var"), Ok("var".to_string()));
        assert!(variable().parse("1var").is_err());
    }
}
