use super::*;
use std::sync::Mutex;

mod array;
mod r#const;
mod decl_support;
mod function;
mod operators;
mod primary;

impl Parser {
    pub fn expression(&mut self, scanner: &mut Scanner) {
        self.parse_precedence(Precedence::Assignment, scanner);
    }
}
