use super::*;
use crate::scanner::TokenType::RigtParen;

impl Parser {
    pub fn function(
        &mut self,
        function_type: FunctionType,
        scanner: &mut Scanner,
        function_name: &str,
    ) -> Expr {
        let line = self.previous.line as u32;

        self.consume(
            TokenType::LeftParen,
            "Expect '('  after function name.",
            scanner,
        );

        let mut exprs: Vec<Expr> = Vec::new();
        let mut annotations: Vec<TokenType> = Vec::new();
        let mut arity: usize = 0;

        if !self.check(&TokenType::RigtParen) {
            loop {
                arity += 1;

                if arity > 255 {
                    self.error_at_current("Can't have more than 255 parameters");
                }

                self.parse_variable("Expect a parameter's name", scanner);
                let expr = self.parse_precedence(Precedence::Assignment, scanner);

                exprs.push(expr);

                self.consume(
                    TokenType::Colon,
                    "Expect ':' after paramters, and a type",
                    scanner,
                );

                self.advance(scanner);

                let annotation_type = self.previous.token_type;

                annotations.push(annotation_type);

                if !self.match_consume(&TokenType::Comma, scanner) {
                    break;
                }
            }
        }

        self.consume(
            TokenType::RigtParen,
            "Expect ')' after parameters.",
            scanner,
        );

        let return_type = if self.match_consume(&TokenType::Arrow, scanner) {
            self.advance(scanner);
            self.previous.token_type
        } else {
            TokenType::Void
        };

        self.consume(
            TokenType::LeftBrace,
            "Expect '{' before function body.",
            scanner,
        );

        let stmts = self.block(scanner);

        Expr::Function {
            name: function_name.to_string(),
            arity,
            ftype: function_type,
            exprs: Box::new(exprs),
            body: stmts,
            annotations,
            return_type,
            line,
        }
    }

    pub fn call(&mut self, lhs: &Expr, scanner: &mut Scanner) -> Expr {
        let line = self.previous.line as u32;
        let (arg_count, args) = self.argument_list(scanner);

        Expr::FunctionCall {
            caller: Box::new(lhs.to_owned()),
            argument_count: arg_count,
            arguments: args,
            line,
        }
    }

    pub fn argument_list(&mut self, scanner: &mut Scanner) -> (usize, Vec<Expr>) {
        let mut arg_count = 0;
        let mut args: Vec<Expr> = Vec::new();

        loop {
            if !self.check(&RigtParen) {
                let value = self.parse_precedence(Precedence::Assignment, scanner);

                if arg_count == 255 {
                    self.error("Can't have more than 255 arguments.");
                }

                arg_count += 1;
                args.push(value);

                if !self.match_consume(&TokenType::Comma, scanner) {
                    break;
                }
            } else {
                break;
            }
        }

        self.consume(RigtParen, "Expect ')' after arguments", scanner);

        (arg_count, args)
    }

    pub fn turbofish(&mut self, lhs: &Expr, scanner: &mut Scanner) -> Expr {
        self.consume(
            TokenType::LeftBracket,
            "Expect '[' at the start of a turbofish body",
            scanner,
        );

        self.advance(scanner);
        let generic = self.previous.token_type.as_typetag().unwrap();

        self.consume(
            TokenType::RightBracket,
            "Expect ']' at the end of a turbofish body",
            scanner,
        );

        Expr::Turbofish {
            left: Box::new(lhs.to_owned()),
            generic,
        }
    }
}
