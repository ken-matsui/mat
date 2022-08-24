use crate::sema::diag::{Diagnostics, Error};
use matc_ast::Type;
use matc_span::Spanned;
use std::collections::HashMap;
use std::ops::Deref;

type Mark = bool;
const CHECKING: bool = false;
const CHECKED: bool = true;

pub(crate) struct TypeTable {
    pub(crate) value: HashMap<Spanned<String>, Spanned<Type>>,
}

impl TypeTable {
    pub(crate) fn new() -> Self {
        Self {
            value: HashMap::new(),
        }
    }

    pub(crate) fn semantic_check(&self) -> Diagnostics {
        let mut diag = Diagnostics::new();

        for ty in self.value.values() {
            // TODO:
            // if type (in value) is (struct or union) {
            //     self.check_void_members();
            //     self.check_duplicated_members();
            // } else if type (in value) is array {
            //     self.check_void_members();
            // }
            self.check_recursive_definition(ty, &mut diag);
        }

        diag
    }

    fn check_recursive_definition(&self, ty: &Spanned<Type>, diag: &mut Diagnostics) {
        self.check_recursive_definition_(ty, &mut HashMap::new(), diag);
    }

    fn check_recursive_definition_(
        &self,
        ty: &Spanned<Type>,
        marks: &mut HashMap<Spanned<Type>, Mark>,
        diag: &mut Diagnostics,
    ) {
        if let Some((pre_def, false /* CHECKING */)) = marks.get_key_value(ty) {
            // TODO: pre_def.span & ty.span points the same Span
            diag.push_err(Error::RecursiveTypeDef(pre_def.span, ty.span));
        } else {
            marks.insert(ty.clone(), CHECKING);
            // TODO:
            // if ty is (struct or union) {
            // else if ty is array {
            if let Type::User(name) = ty.deref() {
                // type foo = bar;
                //      ---   --- checking next
                //      | current `ty`
                self.check_recursive_definition_(
                    self.value
                        .get(&Spanned::new(name.clone(), ty.span))
                        .unwrap(),
                    marks,
                    diag,
                );
            }
            marks.insert(ty.clone(), CHECKED);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use matc_span::Span;

    #[test]
    fn test_check_recursive_definition_err() {
        let mut type_table = TypeTable::new();

        type_table.value.insert(
            Spanned::any("baz".to_string()),
            Spanned::any(Type::User("foo".to_string())),
        );
        type_table.value.insert(
            Spanned::any("bar".to_string()),
            Spanned::any(Type::User("baz".to_string())),
        );
        type_table.value.insert(
            Spanned::any("foo".to_string()),
            Spanned::any(Type::User("bar".to_string())),
        );

        assert_eq!(
            type_table.semantic_check(),
            Diagnostics {
                warnings: vec![],
                errors: vec![
                    Error::RecursiveTypeDef(Span::any(), Span::any()),
                    Error::RecursiveTypeDef(Span::any(), Span::any()),
                    Error::RecursiveTypeDef(Span::any(), Span::any()),
                ]
            }
        );
    }

    #[test]
    fn test_check_recursive_definition_ok() {
        let mut type_table = TypeTable::new();

        type_table
            .value
            .insert(Spanned::any("bar".to_string()), Spanned::any(Type::I8));
        type_table.value.insert(
            Spanned::any("foo".to_string()),
            Spanned::any(Type::User("bar".to_string())),
        );
        type_table
            .value
            .insert(Spanned::any("baz".to_string()), Spanned::any(Type::I8));
        type_table.value.insert(
            Spanned::any("bar".to_string()),
            Spanned::any(Type::User("baz".to_string())),
        );

        assert_eq!(type_table.semantic_check(), Diagnostics::new()); // no err
    }
}
