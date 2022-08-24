use crate::hir::Hir;
use crate::parser::ast::{Expr, Spanned, Stmt, Type};
use crate::sema::error::{SemanticDiag, SemanticError};
use crate::sema::type_table::TypeTable;
use std::ops::Deref;

pub(crate) struct TypeResolver<'a> {
    type_table: &'a mut TypeTable,
    diag: SemanticDiag,
}

impl<'a> TypeResolver<'a> {
    pub(crate) fn new(type_table: &'a mut TypeTable) -> Self {
        Self {
            type_table,
            diag: SemanticDiag::new(),
        }
    }

    pub(crate) fn resolve(&mut self, hir: &mut Hir) -> SemanticDiag {
        self.define_types(hir);
        self.resolve_types(hir);
        // TODO: Check references; to warn unused types

        self.diag.clone()
    }

    fn define_types(&mut self, hir: &Hir) {
        for ty in hir.types() {
            if let Some((predef, _)) = self.type_table.value.get_key_value(ty.name) {
                self.diag
                    .push_err(SemanticError::DuplicatedType(predef.span, ty.name.span));
            }
            self.type_table.value.insert(ty.name.clone(), ty.ty.clone());
        }
    }

    fn resolve_types(&mut self, hir: &Hir) {
        for stmt in &hir.defs {
            self.visit_stmt(stmt);
        }
    }

    fn visit_type(&mut self, ty: &Spanned<Type>) {
        if let Type::User(name) = ty.deref() {
            if !self
                .type_table
                .value
                .contains_key(&Spanned::new(name.clone(), ty.span))
            {
                self.diag.push_err(SemanticError::UnresolvedType(ty.span));
            }
        }
    }

    fn visit_stmt(&mut self, stmt: &Spanned<Stmt>) {
        match stmt.deref() {
            Stmt::DefFn {
                args, ret_ty, body, ..
            } => {
                for arg in args.deref() {
                    self.visit_type(&arg.ty);
                }
                self.visit_type(ret_ty);
                self.visit_stmt(body);
            }
            Stmt::DefVar { ty, expr, .. } => {
                self.visit_type(ty);
                if let Some(expr) = expr {
                    self.visit_expr(expr);
                }
            }
            Stmt::TypeDef { ty, .. } => {
                self.visit_type(ty);
            }
            Stmt::Block(stmts) => {
                for s in stmts {
                    self.visit_stmt(s);
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

    fn visit_expr(&mut self, expr: &Spanned<Expr>) {
        match expr.deref() {
            Expr::As(lhs, ty) => {
                self.visit_expr(lhs);
                self.visit_type(ty);
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
            Expr::FnCall { name, args } => {
                self.visit_expr(name);
                let _ = args.iter().map(|arg| self.visit_expr(arg));
            }
            _ => (),
        }
    }
}

// TODO: tests (unres_type.mat)
