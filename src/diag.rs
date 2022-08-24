mod emit;
mod parser;

pub(crate) use emit::{emit, Emit};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Diagnostics<W, E> {
    pub(crate) warnings: Vec<W>,
    pub(crate) errors: Vec<E>,
}

impl<W, E> Emit for Diagnostics<W, E>
where
    Vec<W>: Emit,
    Vec<E>: Emit,
{
    fn emit(&self, code: &str) {
        self.warnings.emit(code);
        self.errors.emit(code);
    }

    fn count(&self) -> usize {
        // Ignore warnings
        self.errors.count()
    }
}

impl<W, E> Diagnostics<W, E> {
    pub(crate) fn new() -> Self {
        Self {
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub(crate) fn has_err(&self) -> bool {
        !self.errors.is_empty()
    }

    pub(crate) fn push_warn(&mut self, warn: W) {
        self.warnings.push(warn);
    }
    pub(crate) fn push_err(&mut self, err: E) {
        self.errors.push(err);
    }
}
