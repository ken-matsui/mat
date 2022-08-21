use std::cmp::{max, min};
use std::fmt;
use std::ops::{Deref, DerefMut, Range};

/// Range does not implements `Copy` (#27186), so here we use (usize, usize) instead
#[derive(Clone, PartialEq, Copy)]
pub(crate) struct Span(pub(crate) (usize, usize));

impl Span {
    pub(crate) fn new(range: Range<usize>) -> Self {
        Self((range.start, range.end))
    }

    pub(crate) fn start(&self) -> usize {
        self.0 .0
    }
    pub(crate) fn end(&self) -> usize {
        self.0 .1
    }
    pub(crate) fn range(&self) -> Range<usize> {
        self.start()..self.end()
    }

    /// Returns a `Span` that would enclose both `self` and `other`.
    pub(crate) fn union(&self, other: Span) -> Range<usize> {
        min(self.start(), other.start())..max(self.end(), other.end())
    }

    /// Bypass equity checks on tests
    #[cfg(test)]
    pub(crate) fn any() -> Range<usize> {
        0..0
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.range())
    }
}

#[derive(Clone)]
pub(crate) struct Spanned<T> {
    pub(crate) value: Box<T>,
    pub(crate) span: Span,
}

impl<T> Spanned<T> {
    pub(crate) fn new(value: T, span: Range<usize>) -> Self {
        Spanned {
            value: Box::new(value),
            span: Span::new(span),
        }
    }

    /// Get a reference to the inner value.
    pub(crate) fn value(&self) -> &T {
        &self.value
    }
    /// Get a mutable to the inner value.
    pub(crate) fn value_mut(&mut self) -> &mut T {
        &mut self.value
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
        self.value()
    }
}
impl<T> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value_mut()
    }
}

impl<T: PartialEq> PartialEq for Spanned<T> {
    fn eq(&self, other: &Self) -> bool {
        // Ignore checking Span; nothing makes sense, particularly on tests.
        self.value == other.value
    }
}

impl<T: fmt::Debug> fmt::Debug for Spanned<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:#?} @ {:?}", self.value, self.span)
        } else {
            write!(f, "{:?} @ {:?}", self.value, self.span)
        }
    }
}
