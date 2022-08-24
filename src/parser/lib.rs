use crate::parser::Error;
pub(crate) use chumsky::prelude::*;
pub(crate) use chumsky::Parser as _;

// trait alias under stable version
pub(crate) trait Parser<T>: chumsky::Parser<char, T, Error = Error> + Clone {
    #[cfg(test)]
    fn parse_test(&self, stream: &str) -> Result<T, Vec<Self::Error>> {
        use matc_span::{Span, SrcId};

        let len = stream.chars().count();
        let span = |i| Span::new(SrcId::any(), i..i + 1);

        self.parse(chumsky::Stream::from_iter(
            span(len),
            stream.chars().enumerate().map(|(i, c)| (c, span(i))),
        ))
    }
}
impl<S, T> Parser<T> for S where S: chumsky::Parser<char, T, Error = Error> + Clone {}

pub(crate) type Rec<'a, T> = Recursive<'a, char, T, Error>;
