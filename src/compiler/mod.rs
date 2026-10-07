pub mod ast;
pub mod codegen;
pub mod core;
pub mod emit;
pub mod expr;
pub mod locals;
pub mod prepass;
pub mod rules;
pub mod sema;
pub mod stmt;
pub mod type_safety;

pub use crate::{
    chunk::{Chunk, OpCode},
    compiler::ast::{Expr, Stmt},
    compiler::{locals::Local, rules::Precedence},
    scanner::{Scanner, Token, TokenType},
    value::{Function, Value},
};

pub const TYPETAG_ERR: &str = "TypeTag stack underflow — compiler emitted unbalanced typetags";

pub use TypeTag::Void;
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc, sync::Arc};

pub struct Parser {
    pub(in crate::compiler) current: Token,
    pub(in crate::compiler) previous: Token,
    pub(in crate::compiler) prevprev: Token,
    pub(in crate::compiler) had_err: bool,
    pub(in crate::compiler) painc_mode: bool,
    pub(in crate::compiler) const_table: HashMap<String, (Value, TypeTag)>,
    pub(in crate::compiler) type_tag: Vec<TypeTag>,
    pub(in crate::compiler) expected_type: Option<TypeTag>,
    pub(in crate::compiler) control_flow: ControlFlow,
    pub(in crate::compiler) info: Info,
}

#[derive(Debug, Clone)]
pub struct Functions {
    function: Function,
    #[allow(warnings)]
    function_type: FunctionType,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum FunctionType {
    Function,
    Script,
}

pub struct ControlFlow {
    pub loop_starts: Vec<usize>,
    pub stops: Vec<Vec<u8>>,
    pub locals_in: Vec<i32>,
    pub jumps: Vec<usize>,
}

pub struct Info {
    pub is_mut: Vec<bool>,
    pub last_local_slot: Option<u8>,
    pub names: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[repr(u8)]
pub enum TypeTag {
    Int,
    Float,
    Str,
    Bool,
    Char,
    Unt,
    Void,
    None,
    Array(Arc<TypeTag>),
    Opt(Arc<TypeTag>),
    Range(Arc<TypeTag>),
    Generic,

    #[default]
    Nai,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            current: Token::default(),
            previous: Token::default(),
            prevprev: Token::default(),
            had_err: false,
            painc_mode: false,
            const_table: HashMap::new(),
            type_tag: Vec::new(),
            expected_type: None,
            control_flow: ControlFlow {
                loop_starts: Vec::new(),
                stops: Vec::new(),
                locals_in: Vec::new(),
                jumps: Vec::new(),
            },
            info: Info {
                is_mut: Vec::new(),
                last_local_slot: Some(0),
                names: Vec::new(),
            },
        }
    }
}
