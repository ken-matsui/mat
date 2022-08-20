use crate::parser::lib::*;

pub(crate) fn comment() -> impl Parser<()> {
    let single_line_comment = just::<_, _, ParserError>("//")
        .then_ignore(take_until(text::newline()))
        .ignored();

    let multi_line_comment = just::<_, _, ParserError>("/*")
        .then(take_until(just("*/")))
        .ignored();

    single_line_comment.or(multi_line_comment).boxed()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comment_test() {
        assert!(comment().parse("// comment\n").is_ok());
        assert!(comment().parse("//\n").is_ok());
        assert!(comment().parse("// comment").is_err());
        assert!(comment().parse("/\n").is_err());

        assert!(comment()
            .parse(
                r#"/*
                comment
                */
            "#
            )
            .is_ok());
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
