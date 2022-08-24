use crate::hir::Hir;
use crate::parser::ast::{Expr, Spanned, Stmt};
use crate::sema::entity::Entity;
use crate::sema::error::SemanticDiag;
use crate::sema::resolver::Resolver;
use crate::sema::scope::Scope;
use std::cell::RefCell;
use std::collections::LinkedList;
use std::ops::Deref;
use std::rc::Rc;

pub(crate) struct LocalResolver {
    scope_stack: LinkedList<Rc<RefCell<Scope>>>,
    diag: SemanticDiag,
}

impl LocalResolver {
    pub(crate) fn new() -> Self {
        Self {
            scope_stack: LinkedList::new(),
            diag: SemanticDiag::new(),
        }
    }
}

impl Resolver for LocalResolver {
    fn resolve(&mut self, hir: &mut Hir) -> SemanticDiag {
        let toplevel = Scope::new(None);
        self.scope_stack.push_back(toplevel.clone());
        self.define_entities(hir, toplevel.clone());

        self.resolve_gvar_initializers(hir);
        self.resolve_functions(hir);
        toplevel.borrow().check_references(&mut self.diag);

        hir.set_scope(toplevel);
        // TODO: ast.set_constant_table(constant_table);

        self.diag.clone()
    }
}

impl LocalResolver {
    fn define_entities(&mut self, hir: &Hir, toplevel: Rc<RefCell<Scope>>) {
        for entity in hir.definitions() {
            if let Err(err) = toplevel.borrow_mut().define_entity(entity) {
                self.diag.push_err(err);
            }
        }
    }

    fn resolve_gvar_initializers(&mut self, hir: &Hir) {
        for var in hir.defined_variables() {
            if let Some(expr) = var.initializer() {
                self.visit_expr(expr);
            }
        }
    }

    fn resolve_functions(&mut self, hir: &mut Hir) {
        for stmt in &hir.defs {
            if let Stmt::DefFn { args, body, .. } = stmt.deref() {
                self.push_scope();
                for arg in args {
                    let maybe_err = self
                        .current_scope()
                        .borrow_mut()
                        .define_entity(Entity::from(arg))
                        .err();
                    if let Some(err) = maybe_err {
                        self.diag.push_err(err);
                    }
                    // TODO: function args do not seem marked as unused variable
                }
                self.visit_stmt(body);
                self.pop_scope(); // TODO: fn.set_scope(self.pop_scope());
            }
        }
    }

    fn visit_expr(&mut self, expr: &Spanned<Expr>) {
        match expr.deref() {
            Expr::Variable(var) => {
                let maybe_err = self
                    .current_scope()
                    .borrow_mut()
                    .refer(var, expr.span)
                    .err();

                // TODO: node.setEntity(ent); when not error; `refer()` should return entity.
                if let Some(err) = maybe_err {
                    self.diag.push_err(err);
                }
            }
            Expr::String(_str) => {
                // node.setEntry(constantTable.intern(node.value()));
                // return null;
                todo!()
            }
            Expr::Or(lhs, rhs)
            | Expr::And(lhs, rhs)
            | Expr::Lt(lhs, rhs)
            | Expr::Gt(lhs, rhs)
            | Expr::Lte(lhs, rhs)
            | Expr::Gte(lhs, rhs)
            | Expr::Eq(lhs, rhs)
            | Expr::Neq(lhs, rhs)
            | Expr::BitOr(lhs, rhs)
            | Expr::BitXor(lhs, rhs)
            | Expr::BitAnd(lhs, rhs)
            | Expr::Shl(lhs, rhs)
            | Expr::Shr(lhs, rhs)
            | Expr::Add(lhs, rhs)
            | Expr::Sub(lhs, rhs)
            | Expr::Mul(lhs, rhs)
            | Expr::Div(lhs, rhs)
            | Expr::Rem(lhs, rhs) => {
                self.visit_expr(lhs);
                self.visit_expr(rhs);
            }
            Expr::As(lhs, _ty) => {
                self.visit_expr(lhs);
            }
            Expr::FnCall { name, args } => {
                self.visit_expr(name);
                let _ = args.iter().map(|arg| self.visit_expr(arg));
            }
            _ => (),
        }
    }

    fn visit_stmt(&mut self, stmt: &Spanned<Stmt>) {
        match stmt.deref() {
            Stmt::Block(stmts) => {
                self.push_scope();
                for s in stmts {
                    self.visit_stmt(s);
                }
                self.pop_scope(); // TODO: stmts.set_scope(self.pop_scope());
            }
            Stmt::DefVar { name, ty, expr, .. } => {
                // should evaluate expr first to support like `let min = min(1, 2)`.
                if let Some(expr) = expr {
                    self.visit_expr(expr);
                }

                let maybe_err = self
                    .current_scope()
                    .borrow_mut()
                    .define_entity(Entity::new(name.clone(), ty.clone()))
                    .err();
                if let Some(err) = maybe_err {
                    self.diag.push_err(err);
                }
            }
            Stmt::If { cond, then, els } => {
                self.visit_expr(cond);
                self.visit_stmt(then);
                if let Some(els) = els {
                    self.visit_stmt(els);
                }
            }
            Stmt::Return(Some(expr)) => self.visit_expr(expr),
            Stmt::Assign(lhs, rhs)
            | Stmt::AddAssign(lhs, rhs)
            | Stmt::SubAssign(lhs, rhs)
            | Stmt::MulAssign(lhs, rhs)
            | Stmt::DivAssign(lhs, rhs)
            | Stmt::RemAssign(lhs, rhs)
            | Stmt::BitAndAssign(lhs, rhs)
            | Stmt::BitOrAssign(lhs, rhs)
            | Stmt::BitXorAssign(lhs, rhs)
            | Stmt::ShlAssign(lhs, rhs)
            | Stmt::ShrAssign(lhs, rhs) => {
                self.visit_expr(lhs);
                self.visit_expr(rhs);
            }
            Stmt::Expr(expr) => self.visit_expr(expr),
            _ => (),
        }
    }

    fn push_scope(&mut self) {
        self.scope_stack
            .push_back(Scope::new(Some(self.current_scope().clone())));
    }
    fn pop_scope(&mut self) -> Option<Rc<RefCell<Scope>>> {
        self.scope_stack.pop_back()
    }
    fn current_scope(&self) -> &Rc<RefCell<Scope>> {
        self.scope_stack.back().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{Span, Type};
    use crate::sema::error::{SemanticError, SemanticWarning};

    fn let_imut_i8(name: &str, expr: Option<Spanned<Expr>>) -> Spanned<Stmt> {
        Spanned::any(Stmt::DefVar {
            is_mut: false,
            name: Spanned::any(name.to_string()),
            ty: Spanned::any(Type::I8),
            expr,
        })
    }

    #[test]
    fn test_define_entities() {
        assert_eq!(
            LocalResolver::new().resolve(&mut Hir::from_defs(vec![])),
            SemanticDiag::new(),
        );
        assert_eq!(
            LocalResolver::new().resolve(&mut Hir::from_defs(vec![let_imut_i8("foo", None)])),
            SemanticDiag {
                warnings: vec![SemanticWarning::UnusedEntity(Span::any())],
                errors: vec![],
            },
        );
        assert_eq!(
            LocalResolver::new().resolve(&mut Hir::from_defs(vec![
                let_imut_i8("foo", None),
                let_imut_i8("foo", None)
            ])),
            SemanticDiag {
                warnings: vec![SemanticWarning::UnusedEntity(Span::any())],
                errors: vec![SemanticError::DuplicatedDef(Span::any(), Span::any())],
            },
        );
    }

    #[test]
    fn test_visit_expr() {
        assert_eq!(
            LocalResolver::new().resolve(&mut Hir::from_defs(vec![
                let_imut_i8("foo", Some(Spanned::any(Expr::Variable("bar".to_string())))), // Undefined variable
            ])),
            SemanticDiag {
                warnings: vec![SemanticWarning::UnusedEntity(Span::any())],
                errors: vec![SemanticError::UnresolvedRef(Span::any())],
            },
        );
        assert_eq!(
            LocalResolver::new().resolve(&mut Hir::from_defs(vec![
                let_imut_i8("bar", None),
                let_imut_i8("foo", Some(Spanned::any(Expr::Variable("bar".to_string())))),
            ])),
            SemanticDiag {
                warnings: vec![SemanticWarning::UnusedEntity(Span::any())],
                errors: vec![],
            },
        );

        // let foo: i8 = 1 != bar | 2 & buz << 3 || qux;
        let let_complex = Spanned::any(Stmt::DefVar {
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
                                Spanned::any(Expr::I32(3)),
                            )),
                        )),
                    )),
                )),
                Spanned::any(Expr::Variable("qux".to_string())), // Undefined variable
            ))),
        });

        assert_eq!(
            LocalResolver::new().resolve(&mut Hir::from_defs(vec![let_complex.clone()])),
            SemanticDiag {
                warnings: vec![SemanticWarning::UnusedEntity(Span::any())],
                errors: vec![
                    SemanticError::UnresolvedRef(Span::any()),
                    SemanticError::UnresolvedRef(Span::any()),
                    SemanticError::UnresolvedRef(Span::any())
                ],
            },
        );
        assert_eq!(
            LocalResolver::new().resolve(&mut Hir::from_defs(vec![
                let_imut_i8("bar", None),
                let_complex.clone()
            ])),
            SemanticDiag {
                warnings: vec![SemanticWarning::UnusedEntity(Span::any())],
                errors: vec![
                    SemanticError::UnresolvedRef(Span::any()),
                    SemanticError::UnresolvedRef(Span::any()),
                ],
            },
        );
        assert_eq!(
            LocalResolver::new().resolve(&mut Hir::from_defs(vec![
                let_imut_i8("bar", None),
                let_imut_i8("buz", None),
                let_complex.clone(),
            ])),
            SemanticDiag {
                warnings: vec![SemanticWarning::UnusedEntity(Span::any())],
                errors: vec![SemanticError::UnresolvedRef(Span::any())],
            },
        );
        assert_eq!(
            LocalResolver::new().resolve(&mut Hir::from_defs(vec![
                let_imut_i8("bar", None),
                let_imut_i8("buz", None),
                let_imut_i8("qux", None),
                let_complex,
            ])),
            SemanticDiag {
                warnings: vec![SemanticWarning::UnusedEntity(Span::any())],
                errors: vec![],
            },
        );
    }

    // TODO: test (unres_block.mat)
    #[test]
    fn test_visit_block() {}
}
