use crate::parser::ast::{Spanned, Stmt, Type};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Entity {
    pub(crate) name: Spanned<String>,
    pub(crate) ty: Spanned<Type>,
    pub(crate) n_referred: usize,
}

impl Entity {
    pub(crate) fn new(name: Spanned<String>, ty: Spanned<Type>) -> Entity {
        Entity {
            name,
            ty,
            n_referred: 0,
        }
    }
}

impl Entity {
    pub(crate) fn referred(&mut self) {
        self.n_referred += 1;
    }
}

impl TryFrom<Stmt> for Entity {
    type Error = &'static str;

    fn try_from(item: Stmt) -> Result<Self, Self::Error> {
        match item {
            Stmt::DefVar { name, ty, .. } => Ok(Entity::new(name, ty)),
            Stmt::DefFn { name, ret_ty, .. } => Ok(Entity::new(name, ret_ty)),
            _ => Err("Converting into entity is only permitted to DefVar & DefFn"),
        }
    }
}
