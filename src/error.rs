pub(crate) trait Emit {
    fn emit(&self, code: &str);
    fn count(&self) -> usize;
}

impl<T: Emit> Emit for Vec<T> {
    fn emit(&self, code: &str) {
        for emitter in self {
            emitter.emit(code);
        }
    }

    fn count(&self) -> usize {
        self.len()
    }
}
