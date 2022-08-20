pub(crate) use chumsky::prelude::*;
pub(crate) use chumsky::Parser as _;

pub(crate) type ParserError = Simple<char>;

// trait alias under stable version
pub(crate) trait Parser<T>: chumsky::Parser<char, T, Error = ParserError> + Clone {}
impl<S, T> Parser<T> for S where S: chumsky::Parser<char, T, Error = ParserError> + Clone {}

pub(crate) type Rec<'a, T> = Recursive<'a, char, T, ParserError>;
