use crate::parser::ast::{Ast, Spanned, Stmt, Type};
use crate::sema::error::{SemanticDiag, SemanticError};
use crate::sema::resolver::Resolver;
use std::collections::HashMap;
use std::ops::Deref;

pub(crate) struct TypeResolver {
    type_table: HashMap<Spanned<String>, Spanned<Type>>,
    diag: SemanticDiag,
}

impl TypeResolver {
    pub(crate) fn new() -> Self {
        Self {
            type_table: HashMap::new(),
            diag: SemanticDiag::new(),
        }
    }
}

impl Resolver for TypeResolver {
    fn resolve(&mut self, ast: &Ast) -> SemanticDiag {
        self.define_types(ast);

        self.diag.clone()
    }
}

impl TypeResolver {
    fn define_types(&mut self, ast: &Ast) {
        for stmt in &ast.defs {
            match stmt.deref() {
                Stmt::TypeDef { name, ty } => {
                    if let Some((predef, _)) = self.type_table.get_key_value(name) {
                        self.diag
                            .push_err(SemanticError::DuplicatedType(predef.span, name.span));
                    }
                    self.type_table.insert(name.clone(), ty.clone());
                }
                // TODO: Stmt::DefStruct
                _ => {}
            }
        }
    }
}
