use chumsky::prelude::*;

pub(crate) fn comment() -> impl Parser<char, (), Error = Simple<char>> + Clone {
    let single_line_comment = just::<_, _, Simple<char>>("//")
        .then_ignore(take_until(text::newline()))
        .ignored();

    let multi_line_comment = just::<_, _, Simple<char>>("/*")
        .then(take_until(just("*/")))
        .ignored();

    single_line_comment.or(multi_line_comment)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;

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
