use super::*;
pub mod expr;
pub mod stmt;

pub use expr::Expr;
pub use stmt::Stmt;

impl Expr {
    pub fn is_var(&self) -> bool {
        let Expr::Variable { name: _ } = self else {
            return false;
        };

        true
    }
}
