use crate::parser::lib::*;

fn single_line_comment() -> impl Parser<()> {
    just::<_, _, ParserError>("//")
        .then_ignore(take_until(text::newline().or(end())))
        .ignored()
        .boxed()
}

fn multi_line_comment() -> impl Parser<()> {
    just::<_, _, ParserError>("/*")
        .then(take_until(just("*/")))
        .ignored()
        .boxed()
}

pub(crate) fn comment() -> impl Parser<()> {
    single_line_comment().or(multi_line_comment()).boxed()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line_comment() {
        assert!(comment().parse_test("// comment\n").is_ok());
        assert!(comment().parse_test("//\n").is_ok());
        assert!(comment().parse_test("// comment").is_ok());
        assert!(comment().parse_test("/\n").is_err());
    }

    #[test]
    fn test_multi_line_comment() {
        assert!(comment()
            .parse_test(
                r#"/*
                comment
                */
            "#
            )
            .is_ok());
        assert!(comment().parse_test("/**/").is_ok());
        assert!(comment().parse_test("/**/\n").is_ok());
        assert!(comment().parse_test("/* foo */\n").is_ok());
        assert!(comment().parse_test("/** *foo **/\n").is_ok());
        assert!(comment().parse_test("/* foo */").is_ok());
        assert!(comment().parse_test("/* foo *\n").is_err());
        assert!(comment().parse_test("/* foo \n").is_err());
        assert!(comment().parse_test("* foo */\n").is_err());
        assert!(comment().parse_test(" foo */\n").is_err());
    }

    #[test]
    fn test_comment() {
        assert!(comment().parse_test("// comment\n").is_ok());
        assert!(comment().parse_test("//\n").is_ok());
        assert!(comment().parse_test("// comment").is_ok());
        assert!(comment().parse_test("/\n").is_err());

        assert!(comment()
            .parse_test(
                r#"/*
                comment
                */
            "#
            )
            .is_ok());
        assert!(comment().parse_test("/**/").is_ok());
        assert!(comment().parse_test("/**/\n").is_ok());
        assert!(comment().parse_test("/* foo */\n").is_ok());
        assert!(comment().parse_test("/** *foo **/\n").is_ok());
        assert!(comment().parse_test("/* foo */").is_ok());
        assert!(comment().parse_test("/* foo *\n").is_err());
        assert!(comment().parse_test("/* foo \n").is_err());
        assert!(comment().parse_test("* foo */\n").is_err());
        assert!(comment().parse_test(" foo */\n").is_err());
    }
}
