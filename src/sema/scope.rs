use crate::sema::local_scope::LocalScope;
use crate::sema::toplevel_scope::ToplevelScope;

pub(crate) trait Scope: 'static {
    fn is_toplevel() -> bool
    where
        Self: Sized;
    fn toplevel(&self) -> ToplevelScope;
    fn parent(&self) -> Option<Box<dyn Scope>>;

    fn add_child(&mut self, s: LocalScope);

    // fn get() -> Entity;
}
