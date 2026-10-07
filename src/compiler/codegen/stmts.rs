use crate::{
    chunk::OpCode,
    compiler::{codegen::CodeGen, sema::HirStmt},
};

impl CodeGen {
    pub fn generate_from_hir_stmts(&mut self, typed_tree: HirStmt) {
        match typed_tree {
            HirStmt::Lets { value } => {
                self.generate_from_hir_expr(value);
            }
            HirStmt::Block { body } => {
                body.into_iter()
                    .for_each(move |hirs| self.generate_from_hir_stmts(hirs));
            }
            HirStmt::Fn(x) => self.generate_from_hir_expr(x),
            HirStmt::Expressions(x) => self.generate_from_hir_expr(x),
            HirStmt::Println(x) => {
                self.generate_from_hir_expr(x);

                self.emit_byte((OpCode::Println as u8, 0));
            }
        }
    }
}
