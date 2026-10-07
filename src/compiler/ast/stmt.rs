use super::*;

#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        name: Token,
        value: Box<Expr>,
        annotation: Option<TokenType>,
        is_mut: bool,
    },

    Fn {
        function: Expr,
    },

    Const {
        name: String,
        value: Value,
        type_tag: TypeTag,
    },

    Block {
        block: Vec<Stmt>,
    },

    Expression(Box<Expr>),

    Println(Box<Expr>),

    Return(Option<Box<Expr>>),

    If {
        expr: Expr,
        body: Box<Stmt>,
        else_stmt: Box<Option<Stmt>>,
        else_if_stmt: Box<Option<Stmt>>,
    },
    Match {
        matched: Expr,
        case: Expr,
        case_body: Box<Vec<Stmt>>,
        branches: Box<Vec<Stmt>>,
        wild_card: Box<Vec<Stmt>>,
    },
    Branch {
        expr: Expr,
        body: Box<Vec<Stmt>>,
    },
    Stop,
    Skip,
    While {
        expr: Expr,
        body: Box<Stmt>,
    },
    Loop {
        body: Box<Stmt>,
    },
    ForLoop {
        looped_on: Expr,
        expr: Expr,
        increment: Expr,
        body: Box<Stmt>,
    },
    End {
        ftype: FunctionType,
        has_returned: bool,
        name: String,
    },
    NoneStmt,
}
