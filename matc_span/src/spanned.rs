use crate::Span;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct Spanned<T> {
    pub value: Box<T>,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(value: T, span: Span) -> Self {
        Spanned {
            value: Box::new(value),
            span,
        }
    }

    /// Get a reference to the inner value.
    pub fn value(&self) -> &T {
        &self.value
    }
    /// Get a mutable to the inner value.
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Bypass equity checks on tests
    pub fn any(value: T) -> Self {
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

impl<T: Eq> Eq for Spanned<T> {}

impl<T: Hash> Hash for Spanned<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Ignore Span.
        self.value.hash(state);
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
