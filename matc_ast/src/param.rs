use crate::Type;
use matc_span::Spanned;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Param {
    pub is_mut: bool,
    pub name: Spanned<String>,
    pub ty: Spanned<Type>,
}
