use crate::compiler::{ast::Stmt, rules::Precedence::Assignment};

use super::super::*;

impl Parser {
    pub fn match_stmt(&mut self, scanner: &mut Scanner) -> Stmt {
        let mut type_tags: Vec<TypeTag> = Vec::new();

        let matched_on_expr = self.parse_precedence(Assignment, scanner);
        let ftype_tag = self.type_tag.pop().expect(TYPETAG_ERR);

        //self.begin_scope();

        self.consume(
            TokenType::LeftBrace,
            "Expect '{' after match body.",
            scanner,
        );

        let case_expr = self.parse_precedence(Assignment, scanner);
        type_tags.push(self.type_tag.pop().expect(TYPETAG_ERR));

        self.consume(TokenType::FatArrowLeft, "Expect '=>' after case.", scanner);

        //self.begin_scope();

        self.consume(
            TokenType::LeftBrace,
            "Expect '{' at the start of the case",
            scanner,
        );

        let case_stmt = self.block(scanner);

        //self.end_scope();

        self.consume(
            TokenType::Comma,
            "Expect ',' at the end of the case's block.",
            scanner,
        );

        let mut branches = Vec::new();
        while self.current.token_type != TokenType::WildCard {
            let stmt = self.branch(scanner, &mut type_tags);
            branches.push(stmt);
        }

        self.consume(
            TokenType::WildCard,
            "Expect '_' at the end of the of the match cases.",
            scanner,
        );

        self.consume(TokenType::FatArrowLeft, "Expect '=>' after a case", scanner);
        //self.begin_scope();
        self.consume(
            TokenType::LeftBrace,
            "Expect '{' after a fat arrow",
            scanner,
        );

        let wild_card_stmt = self.block(scanner);

        //self.end_scope();

        self.consume(
            TokenType::RightBrace,
            "Excpet '}' at the end of the match body.",
            scanner,
        );
        //self.end_scope();

        for i in type_tags {
            if ftype_tag != i {
                self.error(&format!(
                    "Missmatched type expected [{}] due the value matched on was [{}] found [{}]",
                    ftype_tag, ftype_tag, i
                ));
            }
        }

        // Stmt::Match {
        //     matched: matched_on_expr,
        //     case: case_expr,
        //     case_body: Box::new(case_stmt),
        //     branches: Box::new(branches),
        //     wild_card: Box::new(wild_card_stmt),
        // }

        Stmt::NoneStmt
    } // done

    fn branch(&mut self, scanner: &mut Scanner, type_tags: &mut Vec<TypeTag>) -> Stmt {
        let expr = self.parse_precedence(Assignment, scanner);

        type_tags.push(self.type_tag.pop().expect(TYPETAG_ERR));

        self.consume(TokenType::FatArrowLeft, "Expect '=>' after case.", scanner);

        //self.begin_scope();

        self.consume(
            TokenType::LeftBrace,
            "Expect '{' at the start of the case",
            scanner,
        );

        let stmt = self.block(scanner);

        //self.end_scope();
        self.consume(
            TokenType::Comma,
            "Expect ',' at the end of the case's block.",
            scanner,
        );

        Stmt::NoneStmt
    } // done

    pub fn if_stmt(&mut self, scanner: &mut Scanner) -> Stmt {
        let expr = self.parse_precedence(Assignment, scanner);

        let stmt = self.statement(scanner);

        let mut else_if_stmt: Option<Stmt> = None;
        let mut else_stmt: Option<Stmt> = None;

        if self.match_consume(&TokenType::Else, scanner) {
            if self.match_consume(&TokenType::If, scanner) {
                let stmt = self.if_stmt(scanner);
                else_if_stmt = Some(stmt);
            } else {
                let stmt = self.statement(scanner);
                else_stmt = Some(stmt)
            }
        }

        Stmt::If {
            expr,
            body: Box::new(stmt),
            else_stmt: Box::new(else_stmt),
            else_if_stmt: Box::new(else_if_stmt),
        }
    } // done
}
