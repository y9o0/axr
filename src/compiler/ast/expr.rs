use super::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
        line: u32,
    },

    Literal {
        value: Value,
        line: u32,
        type_tag: TypeTag,
    },

    Const {
        value: Value,
        line: u32,
        type_tag: TypeTag,
    },

    Grouping(Box<Expr>),

    Variable {
        name: Token,
    },

    Assign {
        name: Token,
        right: Box<Expr>,
    },

    Range {
        left: Box<Expr>,
        right: Box<Expr>,
        line: u32,
    },

    Unary {
        operator: Token,
        right: Box<Expr>,
        line: u32,
    },

    CompoundAssign {
        left: Box<Expr>,
        operator: TokenType,
        right: Box<Expr>,
        line: u32,
    },

    Or {
        left: Box<Expr>,
        right: Box<Expr>,
        line: u32,
    },

    And {
        left: Box<Expr>,
        right: Box<Expr>,
        line: u32,
    },

    Array {
        len: u8,
    },

    FunctionCall {
        caller: Box<Expr>,
        argument_count: usize,
        arguments: Vec<Expr>,
        line: u32,
    },

    Turbofish {
        left: Box<Expr>,
        generic: TypeTag,
    },

    Cast {
        left: Box<Expr>,
        target: TypeTag,
    },

    Function {
        name: String,
        arity: usize,
        exprs: Box<Vec<Expr>>,
        annotations: Vec<TokenType>,
        return_type: TokenType,
        ftype: FunctionType,
        body: Vec<Stmt>,
        line: u32,
    },

    EndScope {
        locals_len: usize,
        depth: i32,
        scope_depth: i32,
    },

    NoneExpr,
}
