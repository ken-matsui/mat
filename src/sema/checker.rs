use crate::sema::error::SemanticDiag;

pub(crate) trait Checker {
    fn check(&mut self) -> SemanticDiag;
}
