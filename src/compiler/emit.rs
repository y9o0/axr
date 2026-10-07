use crate::compiler::codegen::CodeGen;

use super::*;

pub type Byte = (u8, u32);

impl CodeGen {
    pub fn emit_byte(&mut self, byte: Byte) {
        self.current_chunk().write_chunk(byte.0, byte.1);
    }

    pub fn emit_bytes(&mut self, byte1: Byte, byte2: Byte) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    pub fn emit_constant(&mut self, value: Value, line: u32) {
        let constant = self.make_constant(value);
        self.emit_bytes((OpCode::Constant as u8, line), (constant, line));
    }

    pub fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.current_chunk().add_const(value) as u8;

        if constant > u8::MAX {
            //self.error("Too many constants in one chunk.");
            return 0;
        }

        constant
    }

    pub fn end_compiler(&mut self, line: u32) -> Function {
        #[cfg(feature = "DPC")]
        {
            use crate::debug::disassemble_chunk;
            if !self.painc_mode {
                disassemble_chunk(self.current_chunk());
            }
        }

        self.emit_return(line);
        self.functions.function.clone()
    }

    pub fn emit_return(&mut self, line: u32) {
        if self.functions.function_type == FunctionType::Script {
            self.emit_byte((OpCode::Void as u8, line));
            self.emit_byte((OpCode::Return as u8, line));
            return;
        }
    }

    pub fn identifier_constant(&mut self, name: &Token) -> u8 {
        self.make_constant(Value::Str(Arc::from(name.start.to_string())))
    }

    pub fn emit_jump(&mut self, instruction: usize, line: u32) -> usize {
        self.emit_byte((instruction as u8, line));
        self.emit_byte((0xff, line));
        self.emit_byte((0xff, line));
        return &self.current_chunk().code.len() - 2;
    }

    pub fn patch_jump(&mut self, offset: usize) {
        let jump = (self.current_chunk().code.len()) - offset - 2;

        if jump > usize::MAX {
            //self.error("Too much code to jump over.");
        }

        self.current_chunk().code[offset as usize] = ((jump >> 8) & 0xff) as u8;
        self.current_chunk().code[(offset + 1) as usize] = (jump & 0xff) as u8;
    }

    pub fn emit_loop(&mut self, loop_start: usize, line: u32) {
        self.emit_byte((OpCode::Loop as u8, line));

        let offset = self.current_chunk().code.len() - loop_start + 2;

        if offset > u16::MAX as usize {
            //self.error("Loop body too large.");
        }

        self.emit_byte((((offset >> 8) & 0xff) as u8, line));
        self.emit_byte(((offset & 0xff) as u8, line));
    }

    pub fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.functions.function.chunk
    }
}
