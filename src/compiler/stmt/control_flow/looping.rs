use crate::compiler::rules::Precedence::Assignment;

use super::super::*;

// impl Parser {
//     pub fn while_stmt(&mut self, scanner: &mut Scanner) -> Stmt {
//         let loop_start = self.current_chunk().code.len();
//         self.control_flow.loop_starts.push(loop_start);
//         self.control_flow.stops.push(Vec::new());
//
//         let expr = self.parse_precedence(Assignment, scanner);
//         let body = self.statement(scanner);
//
//         self.control_flow.loop_starts.pop();
//         Stmt::While {
//             expr,
//             body: Box::new(body),
//         }
//     } // done
//
//     pub fn loop_stmt(&mut self, scanner: &mut Scanner) -> Stmt {
//         let loop_start = self.current_chunk().code.len();
//         self.control_flow.loop_starts.push(loop_start);
//         self.control_flow.stops.push(Vec::new());
//
//         let body = self.statement(scanner);
//
//         self.control_flow.loop_starts.pop();
//         self.control_flow.stops.pop();
//
//         Stmt::Loop {
//             body: Box::new(body),
//         }
//     } // done
//
//     pub fn for_stmt(&mut self, scanner: &mut Scanner) -> Stmt {
//         self.consume(
//             TokenType::LeftParen,
//             "Expect '(' at the start of 'for' clauses body",
//             scanner,
//         );
//
//         self.consume(
//             TokenType::Let,
//             "Expect 'let' to initialize a clause",
//             scanner,
//         );
//
//         let expr_looped_on = match self.variable_declaration(scanner) {
//             Stmt::Let { value } => value,
//             _ => {
//                 return {
//                     self.error("faild paring the var");
//                     Stmt::NoneStmt
//                 };
//             }
//         };
//
//         let expr = self.parse_precedence(Assignment, scanner);
//
//         self.consume(
//             TokenType::Semicolon,
//             "Expect ';' after a condition clause",
//             scanner,
//         );
//
//         let increment_start = self.current_chunk().code.len();
//         let inc_expr = self.parse_precedence(Assignment, scanner);
//
//         self.consume(
//             TokenType::RigtParen,
//             "Expect ')' at the end of 'for' clauses body",
//             scanner,
//         );
//
//         self.control_flow.loop_starts.push(increment_start);
//         self.control_flow.stops.push(Vec::new());
//         self.control_flow.locals_in.push(self.compiler.local_count);
//
//         let body = self.statement(scanner);
//
//         self.control_flow.loop_starts.pop();
//         self.control_flow.stops.pop();
//         self.control_flow.locals_in.pop();
//
//         Stmt::ForLoop {
//             looped_on: *expr_looped_on,
//             expr,
//             body: Box::new(body),
//             increment: inc_expr,
//         }
//     } // done
// }
