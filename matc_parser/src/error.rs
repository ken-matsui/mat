use chumsky::error::Simple;
use matc_span::Span;

pub use chumsky::error::SimpleReason;

pub type Error = Simple<char, Span>;
