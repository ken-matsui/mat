use inflector::Inflector;

pub(crate) fn pluralize(s: &str, len: usize) -> String {
    if len > 1 {
        s.to_plural()
    } else {
        s.to_string()
    }
}
