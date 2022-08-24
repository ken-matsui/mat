use chumsky::Span as _;
use internment::Intern;
use std::cmp::{max, min};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut, Range};
use std::path::Path;

#[derive(Clone, PartialEq, Copy, Hash, Eq)]
pub(crate) struct SrcId(Intern<Vec<String>>);

impl SrcId {
    pub(crate) fn from_path<P: AsRef<Path>>(path: P) -> Self {
        SrcId(Intern::new(
            path.as_ref()
                .iter()
                .map(|c| c.to_string_lossy().into_owned())
                .collect(),
        ))
    }

    #[cfg(test)]
    pub(crate) fn any() -> Self {
        Self(Intern::new(Vec::new()))
    }
}

impl fmt::Debug for SrcId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.len() == 0 {
            write!(f, "unknown")
        } else {
            write!(f, "{}", self.0.clone().join("/"))
        }
    }
}

impl fmt::Display for SrcId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Range does not implements `Copy` (#27186), so here we use (usize, usize) instead
#[derive(Clone, PartialEq, Copy, Hash, Eq)]
pub(crate) struct Span {
    src: SrcId,
    range: (usize, usize),
}

impl Span {
    pub(crate) fn range(&self) -> Range<usize> {
        self.start()..self.end()
    }

    pub(crate) fn src(&self) -> SrcId {
        self.context()
    }

    /// Returns a `Span` that would enclose both `self` and `other`.
    pub(crate) fn union(self, other: Self) -> Self {
        assert_eq!(
            self.src, other.src,
            "attempted to union spans with different sources"
        );
        Self {
            range: (
                min(self.start(), other.start()),
                max(self.end(), other.end()),
            ),
            ..self
        }
    }

    /// Bypass equity checks on tests
    #[cfg(test)]
    pub(crate) fn any() -> Self {
        Self::new(SrcId::any(), 0..0)
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}:{:?}", self.src, self.range())
    }
}

impl chumsky::Span for Span {
    type Context = SrcId;
    type Offset = usize;

    fn new(src: Self::Context, range: Range<Self::Offset>) -> Self {
        assert!(range.start <= range.end);
        Self {
            src,
            range: (range.start, range.end),
        }
    }

    fn context(&self) -> Self::Context {
        self.src
    }

    fn start(&self) -> Self::Offset {
        self.range.0
    }
    fn end(&self) -> Self::Offset {
        self.range.1
    }
}

impl ariadne::Span for Span {
    type SourceId = SrcId;

    fn source(&self) -> &Self::SourceId {
        &self.src
    }

    fn start(&self) -> usize {
        self.range.0
    }
    fn end(&self) -> usize {
        self.range.1
    }
}

#[derive(Clone)]
pub(crate) struct Spanned<T> {
    pub(crate) value: Box<T>,
    pub(crate) span: Span,
}

impl<T> Spanned<T> {
    pub(crate) fn new(value: T, span: Span) -> Self {
        Spanned {
            value: Box::new(value),
            span,
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
