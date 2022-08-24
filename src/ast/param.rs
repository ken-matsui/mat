use crate::ast::ty::Type;
use matc_span::Spanned;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Param {
    pub(crate) is_mut: bool,
    pub(crate) name: Spanned<String>,
    pub(crate) ty: Spanned<Type>,
}
