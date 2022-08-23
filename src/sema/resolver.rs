use crate::hir::lib::Hir;
use crate::sema::error::SemanticDiag;

pub(crate) trait Resolver {
    fn resolve(&mut self, hir: &mut Hir) -> SemanticDiag;
}
