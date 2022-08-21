use crate::parser::ast::Span;
use crate::sema::entity::Entity;
use crate::sema::local_scope::LocalScope;
use crate::sema::toplevel_scope::ToplevelScope;
use crate::SemanticError;

pub(crate) trait Scope: 'static {
    fn is_toplevel() -> bool
    where
        Self: Sized;
    fn toplevel(&self) -> ToplevelScope;
    fn parent(&self) -> Option<Box<dyn Scope>>;

    fn add_child(&mut self, s: LocalScope);

    /// Search and get entity through scopes up to ToplevelScope.
    fn get_mut(&mut self, name: &str, span: Span) -> Result<&mut Entity, SemanticError>;
}
