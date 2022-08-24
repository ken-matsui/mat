use crate::SrcId;
use chumsky::Span as _;
use std::cmp::{max, min};
use std::fmt;
use std::hash::Hash;
use std::ops::Range;

/// Range does not implements `Copy` (#27186), so here we use (usize, usize) instead
#[derive(Clone, PartialEq, Copy, Hash, Eq)]
pub struct Span {
    src: SrcId,
    range: (usize, usize),
}

impl Span {
    pub fn range(&self) -> Range<usize> {
        self.start()..self.end()
    }

    pub fn src(&self) -> SrcId {
        self.context()
    }

    /// Returns a `Span` that would enclose both `self` and `other`.
    pub fn union(self, other: Self) -> Self {
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
    pub fn any() -> Self {
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
