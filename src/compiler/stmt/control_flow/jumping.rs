use super::super::*;

impl Parser {
    pub fn stop_stmt(&mut self, scanner: &mut Scanner) -> Stmt {
        if self.control_flow.stops.is_empty() {
            self.error("'stop' used outside of a loop.");
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
            scanner,
        );

        Stmt::Stop
    }

    pub fn skip_stmt(&mut self, scanner: &mut Scanner) -> Stmt {
        if self.control_flow.loop_starts.is_empty() {
            self.error("'skip' used outside of a loop.");
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
            scanner,
        );

        Stmt::Skip
    }

    //     pub fn return_stmt(&mut self, scanner: &mut Scanner) -> Stmt {
    //         // let function_name = &self.compiler.function.function.name;
    //         let stmt: Stmt;
    //
    //         // let expected_return_type = match self
    //         //     .function_info
    //         //     .return_type_tag_table
    //         //     .get(function_name)
    //         //     .cloned()
    //         // {
    //         //     Some(x) => x,
    //         //     None => {
    //         //         return {
    //         //             self.error("Function name was not found in the table!.");
    //         //             NoneStmt
    //         //         };
    //         //     }
    //         // };
    //
    //         // if self.match_consume(&TokenType::Semicolon, scanner) {
    //         //     if expected_return_type != TypeTag::Void {
    //         //         self.error(&format!(
    //         //             "Expected [{}] found [{}]",
    //         //             expected_return_type,
    //         //             TypeTag::Void
    //         //         ));
    //         //     }
    //
    // //             stmt = Stmt::Return(None);
    // //         } else {
    // //             let value = self.parse_precedence(Assignment, scanner);
    // //
    // //             let type_tag = self.type_tag.pop().expect(TYPETAG_ERR);
    // //
    // //             if expected_return_type != type_tag {
    // //                 self.error(&format!(
    // //                     "Expected [{}] found [{}]!. ",
    // //                     expected_return_type, type_tag
    // //                 ));
    // //             }
    //
    //             self.consume(
    //                 TokenType::Semicolon,
    //                 "Expect ';' at the end of the return statement",
    //                 scanner,
    //             );
    //
    //             // stmt = Stmt::Return(Some(Box::new(value)))
    //         }
    //
    //         //self.compiler.has_returned = true;
    //
    //         Stmt::NoneStmt
    //     }
}
