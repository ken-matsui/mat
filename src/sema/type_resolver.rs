use crate::parser::ast::{Ast, Spanned, Stmt};
use crate::sema::error::{SemanticDiag, SemanticError};
use crate::sema::resolver::Resolver;
use std::collections::HashMap;
use std::ops::Deref;

pub(crate) struct TypeResolver {
    type_table: HashMap<String, Spanned<String>>,
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
                Stmt::TypeDef { name, .. } => {
                    if let Some(predef) = self.type_table.get(name.deref()) {
                        self.diag
                            .push_err(SemanticError::DuplicatedType(predef.span, name.span));
                    }
                    self.type_table.insert(name.deref().clone(), name.clone());
                }
                // TODO: Stmt::DefStruct
                _ => {}
            }
        }
    }
}
