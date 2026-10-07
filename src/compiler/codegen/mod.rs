use crate::compiler::sema::HirStmt;

use super::*;
pub mod exprs;
pub mod stmts;

pub struct CodeGen {
    pub(in crate::compiler) functions: Functions,
    pub(in crate::compiler) has_returned: bool,
}

impl CodeGen {
    pub fn new(function_type: FunctionType) -> Self {
        Self {
            functions: Functions {
                function: Function::new(),
                function_type,
            },
            has_returned: false,
        }
    }

    pub fn codegen(&mut self, tree: Vec<HirStmt>, chunk: &mut Chunk) -> Function {
        self.functions.function.chunk = chunk.clone();

        tree.iter()
            .for_each(|hir| self.generate_from_hir_stmts(hir.to_owned()));

        let function = self.end_compiler(0);
        *chunk = self.current_chunk().clone();

        function
    }
}
