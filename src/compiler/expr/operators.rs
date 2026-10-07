use crate::compiler::ast::Expr::{self};

use super::*;

impl Parser {
    pub fn binary(&mut self, lhs: &Expr, scanner: &mut Scanner) -> Expr {
        let op = self.previous.clone();
        let line = self.previous.line as u32;

        let operator_type = self.previous.token_type;
        let rule = Self::get_rule(operator_type);

        let rhs = self.parse_precedence(rule.precedence, scanner);

        Expr::Binary {
            left: Box::new(lhs.clone()),
            operator: op,
            right: Box::new(rhs),
            line,
        }
    }

    pub fn unary(&mut self, scanner: &mut Scanner, _can_assign: bool) -> Expr {
        let operator = self.previous.to_owned();
        let line = self.previous.line as u32;

        let rhs = self.parse_precedence(Precedence::Unary, scanner);

        Expr::Unary {
            operator,
            right: Box::new(rhs),
            line,
        }
    }

    pub fn or_expr(&mut self, lhs: &Expr, scanner: &mut Scanner) -> Expr {
        let line = self.previous.line as u32;

        let rhs = self.parse_precedence(Precedence::Or, scanner);

        Expr::Or {
            left: Box::new(lhs.to_owned()),
            right: Box::new(rhs),
            line,
        }
    }

    pub fn and_expr(&mut self, lhs: &Expr, scanner: &mut Scanner) -> Expr {
        let line = self.previous.line as u32;

        let rhs = self.parse_precedence(Precedence::And, scanner);

        Expr::And {
            left: Box::new(lhs.to_owned()),
            right: Box::new(rhs),
            line,
        }
    }

    pub fn compoundassign_expr(&mut self, lhs: &Expr, scanner: &mut Scanner) -> Expr {
        let line = self.previous.line as u32;
        let operator = self.previous.token_type;

        let rhs = self.parse_precedence(Precedence::Assignment, scanner);

        Expr::CompoundAssign {
            left: Box::new(lhs.to_owned()),
            operator: operator,
            right: Box::new(rhs),
            line,
        }
    }
}
