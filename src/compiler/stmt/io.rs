use crate::compiler::rules::Precedence::Assignment;

use super::*;

impl Parser {
    pub fn println_statement(&mut self, scanner: &mut Scanner) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' before value.", scanner);
        let value = self.parse_precedence(Assignment, scanner);
        self.consume(TokenType::RigtParen, "Expect ')' after value.", scanner);

        self.consume(
            TokenType::Semicolon,
            "Expect ';' at the end of the statement.",
            scanner,
        );

        Stmt::Println(Box::new(value))
    }
}
