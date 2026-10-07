use super::*;

impl Parser {
    pub fn parse_variable(&mut self, error_message: &str, scanner: &mut Scanner) -> (bool, Token) {
        let is_mut = self.match_consume(&TokenType::Tilde, scanner);
        self.consume(TokenType::Identifier, error_message, scanner);
        let name = self.previous.clone();
        (is_mut, name)
    }

    pub fn parse_const(&mut self, error_message: &str, scanner: &mut Scanner) -> String {
        self.consume(TokenType::Identifier, error_message, scanner);
        self.previous.start.clone()
    }

    pub fn casting(&mut self, lhs: &Expr, scanner: &mut Scanner) -> Expr {
        let type_tag = self.type_tag.pop().unwrap_or(TypeTag::Void);

        self.advance(scanner);
        let token = self.previous.token_type;

        let target = match token {
            TokenType::Int => TypeTag::Int,
            TokenType::Unt => TypeTag::Unt,
            TokenType::Float => TypeTag::Float,
            TokenType::Str => TypeTag::Str,
            TokenType::Bool => TypeTag::Bool,
            TokenType::Char => TypeTag::Char,
            _ => TypeTag::Void,
        };

        match (type_tag, target.clone()) {
            (TypeTag::Str, _) => {
                self.error(&format!("non-primitive cast: `str` to `{}`", target));
            }
            (TypeTag::Char, _) => {
                self.error(&format!("non-primitive cast: `char` to `{}`", target));
            }
            (TypeTag::Bool, _) => {
                self.error(&format!("non-primitive cast: `bool` to `{}`", target));
            }
            _ => {}
        }

        self.type_tag.push(target.clone());

        Expr::Cast {
            left: Box::new(lhs.to_owned()),
            target,
        }
    }
}
