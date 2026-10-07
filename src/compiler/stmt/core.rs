use crate::compiler::{Stmt::NoneStmt, rules::Precedence::Assignment};

use super::*;

impl Parser {
    pub fn statement(&mut self, scanner: &mut Scanner) -> Stmt {
        let token = self.current.token_type;

        match token {
            TokenType::LeftBrace => {
                self.match_consume(&token, scanner);
                let stmts = self.block(scanner);

                Stmt::Block { block: stmts }
            }

            TokenType::Let => {
                self.match_consume(&token, scanner);
                self.variable_declaration(scanner)
            }

            TokenType::Println => {
                self.match_consume(&token, scanner);
                self.println_statement(scanner)
            }

            TokenType::Const => {
                self.match_consume(&token, scanner);
                self.const_declaration(scanner);
                NoneStmt
            }

            TokenType::Fn => {
                self.match_consume(&token, scanner);
                self.fn_declaration(scanner)
            }

            TokenType::If => {
                self.match_consume(&token, scanner);
                self.if_stmt(scanner);
                NoneStmt
            }

            TokenType::While => {
                self.match_consume(&token, scanner);

                NoneStmt
            }

            TokenType::Loop => {
                self.match_consume(&token, scanner);
                NoneStmt
            }

            TokenType::Stop => {
                self.match_consume(&token, scanner);
                NoneStmt
            }

            TokenType::Skip => {
                self.match_consume(&token, scanner);
                NoneStmt
            }

            TokenType::Match => {
                self.match_consume(&token, scanner);
                self.match_stmt(scanner);
                NoneStmt
            }

            TokenType::For => {
                self.match_consume(&token, scanner);
                NoneStmt
            }

            TokenType::Return => {
                self.match_consume(&token, scanner);
                NoneStmt
            }

            _ => Stmt::Expression(Box::new(self.expression_statement(scanner))),
        }
    }

    pub fn expression_statement(&mut self, scanner: &mut Scanner) -> Expr {
        let expr = self.parse_precedence(Assignment, scanner);
        self.consume(TokenType::Semicolon, "Expect ';' after value.", scanner);
        expr
    }
}
