use crate::{chunk::OpCode::GetLocal, compiler::sema::HirExpr};

use super::*;

impl CodeGen {
    pub fn generate_from_hir_expr(&mut self, typed_tree: HirExpr) {
        match typed_tree {
            HirExpr::Variables { slot, line } => {
                self.emit_bytes((OpCode::GetLocal as u8, line), (slot, line));
            }
            HirExpr::Globals { name } => {
                let slot = self.identifier_constant(&name);
                self.emit_bytes(
                    (OpCode::GetGlobal as u8, name.line as u32),
                    (slot, name.line as u32),
                );
            }
            HirExpr::Assigns { right, slot, line } => {
                self.generate_from_hir_expr(*right);

                self.emit_bytes((OpCode::SetLocal as u8, line), (slot, line));
            }
            HirExpr::Consts { value, line } => {
                self.emit_constant(value, line);
            }
            HirExpr::Literals { value, line } => {
                self.emit_constant(value, line);
            }
            HirExpr::Grouping(x) => {
                self.generate_from_hir_expr(*x);
            }
            HirExpr::Ranges {
                left,
                right,
                type_tag,
                line,
            } => {
                self.generate_from_hir_expr(*left);
                self.generate_from_hir_expr(*right);

                let idx = self.add_type_tag_to_chunk(type_tag);

                self.emit_byte((OpCode::Range as u8, line));
                self.emit_byte((idx, line));
            }
            HirExpr::Binarys {
                left,
                operator,
                right,
                line,
            } => {
                self.generate_from_hir_expr(*left);
                self.generate_from_hir_expr(*right);

                match operator.token_type {
                    TokenType::Plus => self.emit_byte((OpCode::Add as u8, line)),
                    TokenType::Minus => self.emit_byte((OpCode::Subtract as u8, line)),
                    TokenType::Star => self.emit_byte((OpCode::Multiply as u8, line)),
                    TokenType::Slash => self.emit_byte((OpCode::Divide as u8, line)),
                    TokenType::Modulo => self.emit_byte((OpCode::Modulo as u8, line)),
                    TokenType::BangEqual => self.emit_byte((OpCode::NotEqualTo as u8, line)),
                    TokenType::EqualEqual => self.emit_byte((OpCode::EqualTo as u8, line)),
                    TokenType::Greater => self.emit_byte((OpCode::GreaterThan as u8, line)),
                    TokenType::Lesser => self.emit_byte((OpCode::LessThan as u8, line)),
                    TokenType::GreaterEqual => self.emit_byte((OpCode::GreaterThanEq as u8, line)),
                    TokenType::LesserEqual => self.emit_byte((OpCode::LessThanEq as u8, line)),
                    _ => {}
                }
            }
            HirExpr::Unarys {
                operator,
                right,
                line,
            } => {
                self.generate_from_hir_expr(*right);

                match operator.token_type {
                    TokenType::Minus => self.emit_byte((OpCode::Negate as u8, line)),
                    TokenType::Bang => self.emit_byte((OpCode::Not as u8, line)),
                    _ => {}
                }
            }
            HirExpr::CompoundAssigns {
                slot,
                right,
                operator,
                type_tag,
                line,
            } => {
                self.emit_bytes((GetLocal as u8, line), (slot, line));

                self.generate_from_hir_expr(*right);

                let idx = self.add_type_tag_to_chunk(type_tag);

                match operator {
                    TokenType::AddAdd => {
                        self.emit_byte((OpCode::AddAdd as u8, line));
                        self.emit_byte((OpCode::SetLocal as u8, line));
                        self.emit_byte((idx, line));
                    }
                    TokenType::MinusMinus => {
                        self.emit_byte((OpCode::MinusMinus as u8, line));
                        self.emit_byte((OpCode::SetLocal as u8, line));
                        self.emit_byte((idx, line));
                    }
                    _ => {}
                }
            }
            HirExpr::Ors { right, left, line } => {
                self.generate_from_hir_expr(*left);

                let else_jump = self.emit_jump(OpCode::JumpIfFalse as usize, line);
                let end_jump = self.emit_jump(OpCode::Jump as usize, line);

                self.patch_jump(else_jump);
                self.emit_byte((OpCode::Pop as u8, line));

                self.generate_from_hir_expr(*right);
                self.patch_jump(end_jump);
                self.emit_byte((OpCode::Pop as u8, line));
            }
            HirExpr::Ands { right, left, line } => {
                self.generate_from_hir_expr(*left);
                self.generate_from_hir_expr(*right);

                let jump = self.emit_jump(OpCode::JumpIfFalse as usize, line);
                self.emit_byte((OpCode::Pop as u8, line));

                self.patch_jump(jump);
            }
            HirExpr::Functions {
                name,
                arity,
                exprs,
                function_type,
                body,
                line,
            } => {
                let function = Function {
                    name: name.clone(),
                    arity,
                    chunk: self.functions.function.chunk.clone(),
                };
                self.functions.function_type = function_type;

                exprs
                    .into_iter()
                    .for_each(|e| self.generate_from_hir_expr(e));

                body.into_iter()
                    .for_each(|hirs| self.generate_from_hir_stmts(hirs));

                let const_function = self.make_constant(Value::Function(Arc::new(function)));
                self.emit_bytes((OpCode::Constant as u8, line), (const_function, line));
            }
            HirExpr::FunctionCalls {
                caller,
                generic,
                argument_count,
                arguments,
                line,
            } => {
                self.generate_from_hir_expr(*caller);

                let idx = self.add_type_tag_to_chunk(generic.unwrap_or(TypeTag::Nai));

                arguments
                    .into_iter()
                    .for_each(|hir_e| self.generate_from_hir_expr(hir_e));

                self.emit_bytes((OpCode::Call as u8, line), (argument_count as u8, line));
                self.emit_byte((idx, line));
            }
            _ => {}
        }
    }
}
