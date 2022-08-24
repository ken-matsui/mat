#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) enum Type {
    Void,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    /// User defined type
    User(String),
}
