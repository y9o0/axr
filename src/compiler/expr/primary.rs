use crate::compiler::{TypeTag::Bool, ast::Expr};

use super::*;

impl Parser {
    pub fn grouping(&mut self, scanner: &mut Scanner, _can_assign: bool) -> Expr {
        let inner = self.parse_precedence(Precedence::Assignment, scanner);

        self.consume(
            TokenType::RigtParen,
            "Expect ')' after expression.",
            scanner,
        );

        Expr::Grouping(Box::new(inner))
    }

    pub fn variable(&mut self, scanner: &mut Scanner, can_assign: bool) -> Expr {
        let token = &self.previous.clone();
        self.named_variable(token, scanner, can_assign)
    }

    pub fn named_variable(
        &mut self,
        name: &Token,
        scanner: &mut Scanner,
        can_assign: bool,
    ) -> Expr {
        if let Some((value, type_tag)) = self.const_table.get(&name.start).cloned() {
            return Expr::Const {
                value,
                type_tag,
                line: name.line as u32,
            };
        }

        if can_assign && self.match_consume(&TokenType::Equal, scanner) {
            let rhs = self.parse_precedence(Precedence::Assignment, scanner);

            Expr::Assign {
                name: name.to_owned(),
                right: Box::new(rhs),
            }
        } else {
            Expr::Variable {
                name: name.to_owned(),
            }
        }
    }

    pub fn number(&mut self, _scanner: &mut Scanner, _can_assign: bool) -> Expr {
        let value = &self.previous.start;
        let line = self.previous.line as u32;

        if value.contains(".") {
            let float_value: f64 = value.parse::<f64>().unwrap_or_default();

            Expr::Literal {
                value: Value::Float(float_value),
                line,
                type_tag: TypeTag::Float,
            }
        } else if self
            .expected_type
            .clone()
            .is_some_and(|t| t == TypeTag::Unt)
        {
            let unt_value = value.parse::<u64>().unwrap_or_default();

            Expr::Literal {
                value: Value::Unt(unt_value),
                line,
                type_tag: TypeTag::Unt,
            }
        } else {
            let int_value = value.parse::<i64>();

            if let Ok(int) = int_value {
                Expr::Literal {
                    value: Value::Int(int),
                    line,
                    type_tag: TypeTag::Int,
                }
            } else {
                let unt_value = value.parse::<u64>().unwrap_or_default();

                Expr::Literal {
                    value: Value::Unt(unt_value),
                    line,
                    type_tag: TypeTag::Unt,
                }
            }
        }
    }

    pub fn literal(&mut self, _scanner: &mut Scanner, _can_assign: bool) -> Expr {
        let line = self.previous.line as u32;

        match self.previous.token_type {
            TokenType::True => Expr::Literal {
                value: Value::Bool(true),
                line,
                type_tag: Bool,
            },
            TokenType::False => Expr::Literal {
                value: Value::Bool(false),
                line,
                type_tag: TypeTag::Bool,
            },
            TokenType::Void => Expr::Literal {
                value: Value::Void,
                line,
                type_tag: TypeTag::Void,
            },
            TokenType::None => Expr::Literal {
                value: Value::Void,
                line,
                type_tag: TypeTag::None,
            },
            _ => {
                return {
                    self.error(&format!(
                        "Unexpected token '{:?}' ",
                        &self.previous.token_type
                    ));
                    Expr::NoneExpr
                };
            }
        }
    }

    pub fn strings(&mut self, _scanner: &mut Scanner, _can_assign: bool) -> Expr {
        let line = self.previous.line as u32;
        let raw = &self.previous.start;
        let trimmed = &raw[1..raw.len() - 1];
        self.type_tag.push(TypeTag::Str);

        Expr::Literal {
            value: Value::Str(Arc::from(trimmed)),
            line,
            type_tag: TypeTag::Str,
        }
    }

    pub fn char(&mut self, _scanner: &mut Scanner, _can_assign: bool) -> Expr {
        let line = self.previous.line as u32;
        let raw = &self.previous.start;
        let trimmed = &raw[1..raw.len() - 1];
        let into_chars: Vec<char> = trimmed.chars().collect();

        if into_chars.len() != 1 {
            self.error("Char type cannot contain more than one char.");
            return Expr::NoneExpr;
        }

        Expr::Literal {
            value: Value::Char(into_chars[0]),
            line,
            type_tag: TypeTag::Char,
        }
    }

    pub fn range(&mut self, lhs: &Expr, scanner: &mut Scanner) -> Expr {
        let line = self.previous.line as u32;

        let rhs = self.parse_precedence(Precedence::Term, scanner);

        Expr::Range {
            left: Box::new(lhs.to_owned()),
            right: Box::new(rhs),
            line,
        }
    }
}
