pub mod errors;
pub mod exprs;
pub mod stmts;

use crate::compiler::{
    ast::Expr,
    sema::errors::{SemaErr::Unexpected, SemaResult},
    type_safety::{parse_binary_type, parse_range_type},
};

use super::*;

#[derive(Debug, Clone)]
pub struct Compiler {
    pub(in crate::compiler) locals: Vec<Local>,
    pub(in crate::compiler) local_count: i32,
    pub(in crate::compiler) scope_depth: i32,
}

pub struct FunctionInfo {
    parameters_type_tag_table: HashMap<String, Rc<RefCell<Vec<TypeTag>>>>,
    return_type_tag_table: HashMap<String, TypeTag>,
}

impl Compiler {
    pub fn new() -> Self {
        let local = vec![Local {
            name: Token {
                start: "".to_string(),
                ..Default::default()
            },
            depth: 0,
            is_mut: false,
            type_tag: TypeTag::Void,
        }];

        Self {
            locals: local,
            local_count: 1,
            scope_depth: 0,
        }
    }
}

pub struct Sema {
    pub(in crate::compiler) compiler: Compiler,
    pub(in crate::compiler) compiler_stack: Vec<Compiler>,
    pub(in crate::compiler) function_info: FunctionInfo,
}

impl Sema {
    pub fn new() -> Self {
        Self {
            compiler: Compiler::new(),
            compiler_stack: Vec::new(),
            function_info: FunctionInfo {
                parameters_type_tag_table: HashMap::new(),
                return_type_tag_table: HashMap::new(),
            },
        }
    }
}

impl Sema {
    fn info_from_var(&mut self, expr: &Expr) -> SemaResult<(u8, TypeTag, bool)> {
        if expr.is_var() {
            let Expr::Variable { name } = expr else {
                return Err(Self::error(
                    "Couldn't extract 'Token' from variable",
                    Unexpected,
                ));
            };

            let Some((x, tt, mt)) = self.resolve_local(&name) else {
                return Err(Self::error(
                    "Couldn't extract info from variable",
                    Unexpected,
                ));
            };

            Ok((x, tt, mt))
        } else {
            return Err(Self::error("Unexpected targt!", Unexpected));
        }
    }

    fn extract_type(&mut self, expr: &Expr) -> SemaResult<TypeTag> {
        match expr {
            Expr::Const {
                value: _,
                line: _,
                type_tag,
            } => Ok(type_tag.to_owned()),
            Expr::Literal {
                value: _,
                line: _,
                type_tag,
            } => Ok(type_tag.to_owned()),
            Expr::Range {
                left,
                right,
                line: _,
            } => {
                let tt_left = self.extract_type(left);
                let tt_right = self.extract_type(right);

                let range_type = parse_range_type(tt_left?, tt_right?);
                Ok(range_type?)
            }
            Expr::Variable { name: _ } => Ok(self.info_from_var(expr)?.1),
            Expr::Binary {
                left,
                operator,
                right,
                line: _,
            } => {
                let tt_left = self.extract_type(left)?;
                let tt_right = self.extract_type(right)?;

                let result_type = parse_binary_type(tt_left, tt_right, operator.token_type)?;

                Ok(result_type)
            }
            Expr::Grouping(x) => self.extract_type(x),
            Expr::Unary {
                operator: _,
                right,
                line: _,
            } => self.extract_type(right),
            Expr::CompoundAssign {
                left: _,
                operator: _,
                right: _,
                line: _,
            } => Ok(TypeTag::Void),
            Expr::Or {
                left: _,
                right: _,
                line: _,
            }
            | Expr::And {
                left: _,
                right: _,
                line: _,
            } => Ok(TypeTag::Void),

            _ => Err(Self::error(
                &format!(
                    "Unexpected target! the expr '{:?}' doesn't hold a typetag",
                    expr
                ),
                Unexpected,
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub enum HirExpr {
    Variables {
        slot: u8,
        line: u32,
    },
    Globals {
        name: Token,
    },
    Assigns {
        right: Box<HirExpr>,
        slot: u8,
        line: u32,
    },
    Consts {
        value: Value,
        line: u32,
    },
    Literals {
        value: Value,
        line: u32,
    },
    Ranges {
        left: Box<HirExpr>,
        right: Box<HirExpr>,
        type_tag: TypeTag,
        line: u32,
    },
    Grouping(Box<HirExpr>),
    Binarys {
        left: Box<HirExpr>,
        operator: Token,
        right: Box<HirExpr>,
        line: u32,
    },
    Unarys {
        operator: Token,
        right: Box<HirExpr>,
        line: u32,
    },
    CompoundAssigns {
        slot: u8,
        right: Box<HirExpr>,
        operator: TokenType,
        type_tag: TypeTag,
        line: u32,
    },
    Ors {
        right: Box<HirExpr>,
        left: Box<HirExpr>,
        line: u32,
    },
    Ands {
        right: Box<HirExpr>,
        left: Box<HirExpr>,
        line: u32,
    },
    Functions {
        name: String,
        arity: usize,
        exprs: Box<Vec<HirExpr>>,
        function_type: FunctionType,
        body: Vec<HirStmt>,
        line: u32,
    },

    Turbofishs {
        calle: String,
        generic: TypeTag,
    },

    FunctionCalls {
        caller: Box<HirExpr>,
        generic: Option<TypeTag>,
        argument_count: usize,
        arguments: Box<Vec<HirExpr>>,
        line: u32,
    },
}

#[derive(Debug, Clone)]
pub enum HirStmt {
    Lets { value: HirExpr },
    Block { body: Box<Vec<HirStmt>> },
    Fn(HirExpr),
    Expressions(HirExpr),
    Println(HirExpr),
}
