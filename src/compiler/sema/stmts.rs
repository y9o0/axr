use crate::compiler::{ast::Stmt, sema::errors::SemaErr::TypeMismatch};

use super::*;

impl Sema {
    pub fn stmt_to_hir(&mut self, stmt: Stmt) -> SemaResult<HirStmt> {
        match stmt {
            Stmt::Let {
                name,
                value,
                annotation,
                is_mut,
            } => {
                self.declare_variable(name);
                let type_tag = self.extract_type(&value)?;
                let hir_expr = self.expr_to_hir(*value)?;

                self.compiler.locals[(self.compiler.local_count - 1) as usize].type_tag =
                    type_tag.clone();

                self.compiler.locals[(self.compiler.local_count - 1) as usize].is_mut = is_mut;

                if let Some(x) = annotation {
                    if type_tag != x.as_typetag().unwrap() {
                        return Err(Self::error(
                            &format!(
                                "Type missmatch expected '{}' found '{}' ",
                                x.as_typetag().unwrap(),
                                type_tag
                            ),
                            TypeMismatch,
                        ));
                    }
                }

                self.mark_initialized();

                Ok(HirStmt::Lets { value: hir_expr })
            }
            Stmt::Block { block } => {
                self.begin_scope();

                let mut hir_stmts: Vec<HirStmt> = Vec::new();
                for i in block {
                    let hir_stmt = self.stmt_to_hir(i)?;
                    hir_stmts.push(hir_stmt);
                }

                self.end_scope();

                Ok(HirStmt::Block {
                    body: Box::new(hir_stmts),
                })
            }
            Stmt::Fn { function } => {
                let hir_fn = self.expr_to_hir(function)?;

                Ok(HirStmt::Fn(hir_fn))
            }
            Stmt::Expression(x) => {
                let hir_expr = self.expr_to_hir(*x)?;

                Ok(HirStmt::Expressions(hir_expr))
            }
            Stmt::Println(x) => {
                let hir_expr = self.expr_to_hir(*x)?;

                Ok(HirStmt::Println(hir_expr))
            }
            _ => Err(Unexpected),
        }
    }
}
