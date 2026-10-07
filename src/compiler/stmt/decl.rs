use crate::compiler::{
    ast::Stmt::{self, NoneStmt},
    rules::Precedence::Assignment,
};

use super::*;

impl Parser {
    pub fn declaration(&mut self, scanner: &mut Scanner) -> Stmt {
        let stmt = self.statement(scanner);

        if self.painc_mode {
            self.synchronize(scanner);
        }

        stmt
    }

    pub fn fn_declaration(&mut self, scanner: &mut Scanner) -> Stmt {
        self.consume(TokenType::Identifier, "Expect a functions name", scanner);
        let function_name = self.previous.clone();

        let expr = self.function(FunctionType::Function, scanner, &function_name.start);

        Stmt::Fn { function: expr }
    }

    pub fn variable_declaration(&mut self, scanner: &mut Scanner) -> Stmt {
        let (is_mut, name) = self.parse_variable("Expect variable name.", scanner);

        let type_annotation = self.match_consume(&TokenType::Colon, scanner);

        let annotation_type = if type_annotation {
            self.advance(scanner);
            Some(self.previous.token_type)
        } else {
            None
        };

        self.expected_type = annotation_type.map(|t| match t {
            TokenType::Int => TypeTag::Int,
            TokenType::Str => TypeTag::Str,
            TokenType::Bool => TypeTag::Bool,
            TokenType::Float => TypeTag::Float,
            TokenType::Char => TypeTag::Char,
            TokenType::Unt => TypeTag::Unt,
            _ => TypeTag::Void,
        });

        self.consume(TokenType::Equal, "Expect '=' after varible name", scanner);
        let value: Expr = self.parse_precedence(Assignment, scanner);

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
            scanner,
        );

        Stmt::Let {
            name,
            value: Box::new(value),
            annotation: annotation_type,
            is_mut,
        }
    }

    pub fn const_declaration(&mut self, scanner: &mut Scanner) -> Stmt {
        let const_name = self.parse_const("Expect const name.", scanner);

        if !const_name.chars().all(|c| c.is_uppercase()) {
            self.error("Const name must be all in uppercase");
            return NoneStmt;
        }

        self.consume(
            TokenType::Colon,
            "expected a type annotation for const values",
            scanner,
        );

        self.advance(scanner);
        let annotation_type = self.previous.token_type;

        let mut is_array = false;
        let mut is_opt = false;

        let array = if annotation_type == TokenType::Array {
            let array = self.array_type(scanner);
            is_array = true;
            array
        } else {
            TypeTag::Array(Arc::new(TypeTag::Void))
        };

        let opt = if annotation_type == TokenType::Opt {
            self.consume(TokenType::LeftBracket, "Exp", scanner);

            let opt = match self.current.token_type {
                TokenType::Int => TypeTag::Opt(Arc::new(TypeTag::Int)),
                TokenType::Unt => TypeTag::Opt(Arc::new(TypeTag::Unt)),
                TokenType::Float => TypeTag::Opt(Arc::new(TypeTag::Float)),
                TokenType::Str => TypeTag::Opt(Arc::new(TypeTag::Str)),
                TokenType::Bool => TypeTag::Opt(Arc::new(TypeTag::Bool)),
                TokenType::Char => TypeTag::Opt(Arc::new(TypeTag::Char)),
                TokenType::Array => TypeTag::Opt(Arc::new(array.clone())),
                _ => TypeTag::Void,
            };

            self.advance(scanner);
            self.consume(TokenType::RightBracket, "Exp", scanner);
            is_opt = true;
            opt
        } else {
            TypeTag::Opt(Arc::new(Void))
        };

        self.expected_type = match annotation_type {
            TokenType::Int => Some(TypeTag::Int),
            TokenType::Str => Some(TypeTag::Str),
            TokenType::Bool => Some(TypeTag::Bool),
            TokenType::Float => Some(TypeTag::Float),
            TokenType::Char => Some(TypeTag::Char),
            TokenType::Unt => Some(TypeTag::Unt),
            TokenType::Array => Some(array.clone()),
            TokenType::Opt => Some(opt.clone()),
            _ => Some(TypeTag::Void),
        };

        self.consume(TokenType::Equal, "Expect '=' after const name.", scanner);

        let (const_value, type_tag) = self.const_value(scanner);

        match annotation_type {
            TokenType::Opt => {
                if type_tag != TypeTag::Opt(Arc::new(TypeTag::None)) && type_tag != opt {
                    self.error(&format!(
                        "Mismatched types, expected [{}] found [{}]",
                        opt, type_tag
                    ));
                }
            }
            TokenType::Array => {
                if type_tag != array {
                    self.error(&format!(
                        "Mismatched types, expected [{}] found [{}]",
                        opt, type_tag
                    ));
                }
            }
            _ => self.type_check(&type_tag, &annotation_type, is_array, is_opt),
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
            scanner,
        );

        // Stmt::Const {
        //     name: const_name,
        //     value: const_value,
        //     type_tag,
        // }
        //
        Stmt::NoneStmt
    }
}
