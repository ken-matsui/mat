use std::ops::Range;

pub(crate) type Span = Range<usize>;

pub(crate) type Spanned<T> = (T, Span);
