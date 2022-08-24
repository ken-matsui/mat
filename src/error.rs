pub(crate) trait Emit {
    fn emit(&self, code: &str);
    fn count(&self) -> usize;
}
