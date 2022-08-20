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
    fn single_line_comment_test() {
        assert!(comment().parse("// comment\n").is_ok());
        assert!(comment().parse("//\n").is_ok());
        assert!(comment().parse("// comment").is_ok());
        assert!(comment().parse("/\n").is_err());
    }

    #[test]
    fn multi_line_comment() {
        assert!(comment()
            .parse(
                r#"/*
                comment
                */
            "#
            )
            .is_ok());
        assert!(comment().parse("/**/").is_ok());
        assert!(comment().parse("/**/\n").is_ok());
        assert!(comment().parse("/* foo */\n").is_ok());
        assert!(comment().parse("/** *foo **/\n").is_ok());
        assert!(comment().parse("/* foo */").is_ok());
        assert!(comment().parse("/* foo *\n").is_err());
        assert!(comment().parse("/* foo \n").is_err());
        assert!(comment().parse("* foo */\n").is_err());
        assert!(comment().parse(" foo */\n").is_err());
    }

    #[test]
    fn comment_test() {
        assert!(comment().parse("// comment\n").is_ok());
        assert!(comment().parse("//\n").is_ok());
        assert!(comment().parse("// comment").is_ok());
        assert!(comment().parse("/\n").is_err());

        assert!(comment()
            .parse(
                r#"/*
                comment
                */
            "#
            )
            .is_ok());
        assert!(comment().parse("/**/").is_ok());
        assert!(comment().parse("/**/\n").is_ok());
        assert!(comment().parse("/* foo */\n").is_ok());
        assert!(comment().parse("/** *foo **/\n").is_ok());
        assert!(comment().parse("/* foo */").is_ok());
        assert!(comment().parse("/* foo *\n").is_err());
        assert!(comment().parse("/* foo \n").is_err());
        assert!(comment().parse("* foo */\n").is_err());
        assert!(comment().parse(" foo */\n").is_err());
    }
}
