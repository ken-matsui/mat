use std::ops::{Deref, DerefMut, Range};

#[derive(Debug, Clone)]
pub(crate) struct Span(pub(crate) Range<usize>);

impl Span {
    /// Bypass equity checks on tests
    #[cfg(test)]
    pub(crate) fn any() -> Range<usize> {
        0..0
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Spanned<T> {
    value: Box<T>,
    span: Span,
}

impl<T> Spanned<T> {
    pub(crate) fn new(value: T, span: Range<usize>) -> Self {
        Spanned {
            value: Box::new(value),
            span: Span(span),
        }
    }

    /// Bypass equity checks on tests
    #[cfg(test)]
    pub(crate) fn any(value: T) -> Self {
        Self::new(value, Span::any())
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T: PartialEq> PartialEq for Spanned<T> {
    fn eq(&self, other: &Self) -> bool {
        // Ignore checking Span; nothing makes sense, particularly on tests.
        self.value == other.value
    }
}
