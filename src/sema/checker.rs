use crate::hir::Hir;
use crate::sema::error::SemanticDiag;

pub(crate) trait Checker {
    fn check(&mut self, hir: &mut Hir) -> SemanticDiag;
}
