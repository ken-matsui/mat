/// Local Resolver
use crate::parser::ast::{Ast, Expr, Spanned, Stmt};
use crate::sema::entity::Entity;
use crate::sema::error::SemanticError;
use crate::sema::scope::Scope;
use crate::sema::toplevel_scope::ToplevelScope;
use std::collections::LinkedList;
use std::ops::Deref;

pub(crate) struct LocalResolver {
    scope_stack: LinkedList<Box<dyn Scope>>,
    errors: Vec<SemanticError>,
}

impl LocalResolver {
    pub(crate) fn new() -> Self {
        Self {
            scope_stack: LinkedList::new(),
            errors: Vec::new(),
        }
    }
}

impl LocalResolver {
    pub(crate) fn resolve(&mut self, ast: Ast) -> Result<(), Vec<SemanticError>> {
        // toplevel scope
        let mut toplevel = ToplevelScope::new();

        self.define_entities(&ast, &mut toplevel);
        self.scope_stack.push_back(Box::new(toplevel));
        self.resolve_gvar_initializers(&ast);
        // toplevel scope end

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn define_entities(&mut self, ast: &Ast, toplevel: &mut ToplevelScope) {
        for stmt in &ast.defs {
            // Convert DefVar & DefFn into Entities and define the entity.
            if let Ok(entity) = Entity::try_from(*stmt.value.clone()) {
                if let Err(err) = toplevel.define_entity(entity) {
                    self.errors.push(err);
                }
            }
        }
    }

    fn resolve_gvar_initializers(&mut self, ast: &Ast) {
        for stmt in &ast.defs {
            if let Stmt::DefVar { expr, .. } = stmt.deref() {
                if let Some(expr) = expr.deref() {
                    self.resolve_variable(expr);
                    self.resolve_string(expr);
                }
            }
        }
    }

    fn resolve_variable(&mut self, expr: &Spanned<Expr>) {
        match expr.deref() {
            Expr::Variable(var) => match self.current_scope().get_mut(var, expr.span) {
                Ok(entity) => {
                    entity.referred();
                    // TODO: node.setEntity(ent);
                }
                Err(err) => self.errors.push(err),
            },
            Expr::Or(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::And(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Lt(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Gt(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Lte(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Gte(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Eq(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Neq(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::BitOr(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::BitXor(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::BitAnd(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Shl(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Shr(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Add(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Sub(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Mul(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Div(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::Rem(lhs, rhs) => {
                self.resolve_variable(lhs);
                self.resolve_variable(rhs);
            }
            Expr::As(lhs, _ty) => {
                self.resolve_variable(lhs);
            }
            Expr::FnCall { name, args } => {
                self.resolve_variable(name);
                let _ = args.iter().map(|arg| self.resolve_variable(arg));
            }
            _ => (),
        }
    }

    fn resolve_string(&self, expr: &Spanned<Expr>) {
        if let Expr::String(str) = expr.deref() {}
    }

    fn current_scope(&mut self) -> &mut Box<dyn Scope> {
        self.scope_stack.back_mut().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{Span, Type};

    #[test]
    fn test_define_entities() {
        assert_eq!(
            LocalResolver::new().resolve(Ast {
                imports: vec![],
                defs: vec![]
            }),
            Ok(())
        );
        assert_eq!(
            LocalResolver::new().resolve(Ast {
                imports: vec![],
                defs: vec![Spanned::any(Stmt::DefVar {
                    is_mut: false,
                    name: Spanned::any("foo".to_string()),
                    ty: Spanned::any(Type::I8),
                    expr: None,
                })]
            }),
            Ok(())
        );
        assert_eq!(
            LocalResolver::new().resolve(Ast {
                imports: vec![],
                defs: vec![
                    Spanned::any(Stmt::DefVar {
                        is_mut: false,
                        name: Spanned::any("foo".to_string()),
                        ty: Spanned::any(Type::I8),
                        expr: None,
                    }),
                    Spanned::any(Stmt::DefVar {
                        is_mut: false,
                        name: Spanned::any("foo".to_string()),
                        ty: Spanned::any(Type::I8),
                        expr: None,
                    })
                ]
            }),
            Err(vec![SemanticError::DuplicatedDef {
                pre_span: Span::new(Span::any()),
                span: Span::new(Span::any())
            }])
        );
    }

    #[test]
    fn test_resolve_variable() {
        assert_eq!(
            LocalResolver::new().resolve(Ast {
                imports: vec![],
                defs: vec![Spanned::any(Stmt::DefVar {
                    is_mut: false,
                    name: Spanned::any("foo".to_string()),
                    ty: Spanned::any(Type::I8),
                    expr: Some(Spanned::any(Expr::Variable("bar".to_string()))), // Undefined variable
                })]
            }),
            Err(vec![SemanticError::UnresolvedRef {
                span: Span::new(Span::any())
            }])
        );
        assert_eq!(
            LocalResolver::new().resolve(Ast {
                imports: vec![],
                defs: vec![
                    Spanned::any(Stmt::DefVar {
                        is_mut: false,
                        name: Spanned::any("bar".to_string()),
                        ty: Spanned::any(Type::I8),
                        expr: None,
                    }),
                    Spanned::any(Stmt::DefVar {
                        is_mut: false,
                        name: Spanned::any("foo".to_string()),
                        ty: Spanned::any(Type::I8),
                        expr: Some(Spanned::any(Expr::Variable("bar".to_string()))),
                    })
                ]
            }),
            Ok(())
        );
        assert_eq!(
            LocalResolver::new().resolve(Ast {
                imports: vec![],
                // let foo: i8 = 1 != bar | 2 & buz << 3 || qux;
                defs: vec![Spanned::any(Stmt::DefVar {
                    is_mut: false,
                    name: Spanned::any("foo".to_string()),
                    ty: Spanned::any(Type::I8),
                    expr: Some(Spanned::any(Expr::Or(
                        Spanned::any(Expr::Neq(
                            Spanned::any(Expr::I32(1)),
                            Spanned::any(Expr::BitOr(
                                Spanned::any(Expr::Variable("bar".to_string())), // Undefined variable
                                Spanned::any(Expr::BitAnd(
                                    Spanned::any(Expr::I32(2)),
                                    Spanned::any(Expr::Shl(
                                        Spanned::any(Expr::Variable("buz".to_string())), // Undefined variable
                                        Spanned::any(Expr::I32(3))
                                    ))
                                ))
                            ))
                        )),
                        Spanned::any(Expr::Variable("qux".to_string())) // Undefined variable
                    ))),
                })]
            }),
            Err(vec![
                SemanticError::UnresolvedRef {
                    span: Span::new(Span::any())
                },
                SemanticError::UnresolvedRef {
                    span: Span::new(Span::any())
                },
                SemanticError::UnresolvedRef {
                    span: Span::new(Span::any())
                }
            ])
        );
        assert_eq!(
            LocalResolver::new().resolve(Ast {
                imports: vec![],
                // let foo: i8 = 1 != bar | 2 & buz << 3 || qux;
                defs: vec![
                    Spanned::any(Stmt::DefVar {
                        is_mut: false,
                        name: Spanned::any("bar".to_string()),
                        ty: Spanned::any(Type::I8),
                        expr: None,
                    }),
                    Spanned::any(Stmt::DefVar {
                        is_mut: false,
                        name: Spanned::any("foo".to_string()),
                        ty: Spanned::any(Type::I8),
                        expr: Some(Spanned::any(Expr::Or(
                            Spanned::any(Expr::Neq(
                                Spanned::any(Expr::I32(1)),
                                Spanned::any(Expr::BitOr(
                                    Spanned::any(Expr::Variable("bar".to_string())), // Undefined variable
                                    Spanned::any(Expr::BitAnd(
                                        Spanned::any(Expr::I32(2)),
                                        Spanned::any(Expr::Shl(
                                            Spanned::any(Expr::Variable("buz".to_string())), // Undefined variable
                                            Spanned::any(Expr::I32(3))
                                        ))
                                    ))
                                ))
                            )),
                            Spanned::any(Expr::Variable("qux".to_string())) // Undefined variable
                        ))),
                    })
                ]
            }),
            Err(vec![
                SemanticError::UnresolvedRef {
                    span: Span::new(Span::any())
                },
                SemanticError::UnresolvedRef {
                    span: Span::new(Span::any())
                }
            ])
        );
        assert_eq!(
            LocalResolver::new().resolve(Ast {
                imports: vec![],
                // let foo: i8 = 1 != bar | 2 & buz << 3 || qux;
                defs: vec![
                    Spanned::any(Stmt::DefVar {
                        is_mut: false,
                        name: Spanned::any("bar".to_string()),
                        ty: Spanned::any(Type::I8),
                        expr: None,
                    }),
                    Spanned::any(Stmt::DefVar {
                        is_mut: false,
                        name: Spanned::any("buz".to_string()),
                        ty: Spanned::any(Type::I8),
                        expr: None,
                    }),
                    Spanned::any(Stmt::DefVar {
                        is_mut: false,
                        name: Spanned::any("foo".to_string()),
                        ty: Spanned::any(Type::I8),
                        expr: Some(Spanned::any(Expr::Or(
                            Spanned::any(Expr::Neq(
                                Spanned::any(Expr::I32(1)),
                                Spanned::any(Expr::BitOr(
                                    Spanned::any(Expr::Variable("bar".to_string())), // Undefined variable
                                    Spanned::any(Expr::BitAnd(
                                        Spanned::any(Expr::I32(2)),
                                        Spanned::any(Expr::Shl(
                                            Spanned::any(Expr::Variable("buz".to_string())), // Undefined variable
                                            Spanned::any(Expr::I32(3))
                                        ))
                                    ))
                                ))
                            )),
                            Spanned::any(Expr::Variable("qux".to_string())) // Undefined variable
                        ))),
                    })
                ]
            }),
            Err(vec![SemanticError::UnresolvedRef {
                span: Span::new(Span::any())
            }])
        );
        assert_eq!(
            LocalResolver::new().resolve(Ast {
                imports: vec![],
                // let foo: i8 = 1 != bar | 2 & buz << 3 || qux;
                defs: vec![
                    Spanned::any(Stmt::DefVar {
                        is_mut: false,
                        name: Spanned::any("bar".to_string()),
                        ty: Spanned::any(Type::I8),
                        expr: None,
                    }),
                    Spanned::any(Stmt::DefVar {
                        is_mut: false,
                        name: Spanned::any("buz".to_string()),
                        ty: Spanned::any(Type::I8),
                        expr: None,
                    }),
                    Spanned::any(Stmt::DefVar {
                        is_mut: false,
                        name: Spanned::any("qux".to_string()),
                        ty: Spanned::any(Type::I8),
                        expr: None,
                    }),
                    Spanned::any(Stmt::DefVar {
                        is_mut: false,
                        name: Spanned::any("foo".to_string()),
                        ty: Spanned::any(Type::I8),
                        expr: Some(Spanned::any(Expr::Or(
                            Spanned::any(Expr::Neq(
                                Spanned::any(Expr::I32(1)),
                                Spanned::any(Expr::BitOr(
                                    Spanned::any(Expr::Variable("bar".to_string())), // Undefined variable
                                    Spanned::any(Expr::BitAnd(
                                        Spanned::any(Expr::I32(2)),
                                        Spanned::any(Expr::Shl(
                                            Spanned::any(Expr::Variable("buz".to_string())), // Undefined variable
                                            Spanned::any(Expr::I32(3))
                                        ))
                                    ))
                                ))
                            )),
                            Spanned::any(Expr::Variable("qux".to_string())) // Undefined variable
                        ))),
                    })
                ]
            }),
            Ok(())
        );
    }
}
