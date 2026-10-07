use crate::{
    compiler::{
        codegen::CodeGen,
        sema::{
            Sema,
            errors::{
                self,
                SemaErr::{InvalidTarget, Unexpected},
                SemaResult,
            },
        },
    },
    value::OptWrapper,
};

use super::*;

impl TokenType {
    pub fn as_typetag(&self) -> Option<TypeTag> {
        match self {
            TokenType::Int => Some(TypeTag::Int),
            TokenType::Unt => Some(TypeTag::Unt),
            TokenType::Float => Some(TypeTag::Float),
            TokenType::Str => Some(TypeTag::Str),
            TokenType::Char => Some(TypeTag::Char),
            TokenType::Void => Some(TypeTag::None),
            TokenType::Bool => Some(TypeTag::Bool),
            _ => None,
        }
    }
}

impl Value {
    pub fn as_typetag(&self) -> Option<TypeTag> {
        match self {
            Value::Int(_) => Some(TypeTag::Int),
            Value::Unt(_) => Some(TypeTag::Unt),
            Value::Float(_) => Some(TypeTag::Float),
            Value::Bool(_) => Some(TypeTag::Bool),
            Value::Char(_) => Some(TypeTag::Char),
            Value::Str(_) => Some(TypeTag::Str),
            Value::Opt(x) => Some(TypeTag::Opt(Arc::new(match x {
                OptWrapper::Some(x) => x.as_typetag().unwrap(),
                OptWrapper::None => TypeTag::None,
            }))),
            Value::Array(x) => Some(TypeTag::Array(Arc::new({
                let mut typetag = TypeTag::Void;

                let x = x.clone();

                for i in x.lock().as_deref().unwrap() {
                    typetag = i.as_typetag().unwrap();
                }

                typetag
            }))),
            Value::Range(x) => Some(TypeTag::Range(Arc::new(match x {
                crate::value::RangeType::RangeFloat(_) => TypeTag::Float,
                crate::value::RangeType::RangeInt(_) => TypeTag::Int,
                crate::value::RangeType::RangeUnt(_) => TypeTag::Unt,
            }))),
            Value::Void => Some(TypeTag::Void),
            _ => None,
        }
    }
}

impl TypeTag {
    pub fn extract(self) -> Arc<TypeTag> {
        match self {
            Self::Array(x) => x,
            Self::Opt(x) => x,
            Self::Range(x) => x,
            _ => Arc::new(Void),
        }
    }

    pub fn is_opt(&self) -> bool {
        match self {
            Self::Opt(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for TypeTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeTag::Int => write!(f, "int"),
            TypeTag::Float => write!(f, "float"),
            TypeTag::Bool => write!(f, "bool"),
            TypeTag::Str => write!(f, "str"),
            TypeTag::Void => write!(f, "void"),
            TypeTag::Char => write!(f, "char"),
            TypeTag::Unt => write!(f, "unt"),
            TypeTag::None => write!(f, "None"),
            TypeTag::Opt(x) => write!(f, "Opt[{}]", x),
            TypeTag::Array(x) => write!(f, "Array[{}]", x),
            TypeTag::Range(x) => write!(f, "Range[{}]", x),
            TypeTag::Generic => write!(f, "generic"),
            TypeTag::Nai => write!(f, "nai!"),
        }
    }
}

impl Parser {
    pub fn parse_array_typetag(&mut self, scanner: &mut Scanner) -> (TypeTag, bool) {
        let array = self.array_type(scanner);
        (array, true)
    }

    pub fn type_check(
        &mut self,
        type_tag: &TypeTag,
        token: &TokenType,
        is_array: bool,
        is_opt: bool,
    ) {
        if let Some(id) = token.as_typetag() {
            let expected = match (is_array, is_opt) {
                (true, false) => TypeTag::Array(Arc::new(id.clone())),
                (true, true) => TypeTag::Opt(Arc::new(TypeTag::Array(Arc::new(id.clone())))),
                (false, true) => TypeTag::Opt(Arc::new(id.clone())),
                (false, false) => id.clone(),
            };

            if &expected != type_tag {
                let kind = match (is_array, is_opt) {
                    (true, false) => format!("Array[{}]", id),
                    (true, true) => format!("Opt[Array[{}]]", id),
                    (false, true) => format!("Opt[{}]", id),
                    (false, false) => format!("{}", id),
                };

                self.error(&format!(
                    "Mismatched types, expected [{}] found [{}]",
                    kind, type_tag
                ));
            }
        } else {
            return;
        }
    }

    pub fn array_type(&mut self, scanner: &mut Scanner) -> TypeTag {
        self.consume(TokenType::LeftBracket, "Exp", scanner);
        self.advance(scanner);

        let array = match self.previous.token_type {
            TokenType::Int => TypeTag::Array(Arc::new(TypeTag::Int)),
            TokenType::Unt => TypeTag::Array(Arc::new(TypeTag::Unt)),
            TokenType::Float => TypeTag::Array(Arc::new(TypeTag::Float)),
            TokenType::Str => TypeTag::Array(Arc::new(TypeTag::Str)),
            TokenType::Bool => TypeTag::Array(Arc::new(TypeTag::Bool)),
            TokenType::Char => TypeTag::Array(Arc::new(TypeTag::Char)),
            TokenType::Array => TypeTag::Array(Arc::new(self.array_type(scanner))),
            _ => TypeTag::Array(Arc::new(TypeTag::Void)),
        };

        self.consume(TokenType::RightBracket, "Exp", scanner);
        array
    }
}

impl CodeGen {
    pub fn add_type_tag_to_chunk(&mut self, type_tag: TypeTag) -> u8 {
        // we cap the curent chunk so we can acc typetag
        let chunk = self.current_chunk();

        // we check if the type tags postion does already exsist and if it does we return it
        if let Some(idx) = chunk.type_tag.iter().position(|t| t == &type_tag) {
            return idx as u8;
        }

        // if it doesn't exsist we push the typetag
        chunk.type_tag.push(type_tag);
        // this is pasicly what push does, the index of the pused value
        (chunk.type_tag.len() - 1) as u8

        // we return those idxs so we can emit them later!.
        // we need those idxs to index in the typetag stack and get the value we need.
    }
}

pub fn parse_range_type(lt: TypeTag, rt: TypeTag) -> sema::errors::SemaResult<TypeTag> {
    match (lt, rt) {
        (TypeTag::Int, TypeTag::Int) => Ok(TypeTag::Range(Arc::new(TypeTag::Int))),
        (TypeTag::Unt, TypeTag::Unt) => Ok(TypeTag::Range(Arc::new(TypeTag::Unt))),
        (TypeTag::Float, TypeTag::Float) => Ok(TypeTag::Range(Arc::new(TypeTag::Float))),
        _ => Err(sema::Sema::error("Unexpected range type !", Unexpected)),
    }
}

pub fn parse_binary_type(
    lt: TypeTag,
    rt: TypeTag,
    optype: TokenType,
) -> sema::errors::SemaResult<TypeTag> {
    let is_comp = matches!(
        &optype,
        TokenType::BangEqual
            | TokenType::EqualEqual
            | TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Lesser
            | TokenType::LesserEqual
    );

    match (&lt, &rt) {
        (TypeTag::Int, TypeTag::Int)
        | (TypeTag::Int, TypeTag::Float)
        | (TypeTag::Float, TypeTag::Int)
        | (TypeTag::Str, TypeTag::Str)
        | (TypeTag::Unt, TypeTag::Unt)
        | (TypeTag::Unt, TypeTag::Float)
        | (TypeTag::Float, TypeTag::Unt)
        | (TypeTag::Float, TypeTag::Float)
        | (TypeTag::Bool, TypeTag::Bool)
        | (TypeTag::Char, TypeTag::Char)
            if is_comp =>
        {
            Ok(TypeTag::Bool)
        }
        (TypeTag::Int, TypeTag::Int) => Ok(TypeTag::Int),
        (TypeTag::Int, TypeTag::Float) => Ok(TypeTag::Float),
        (TypeTag::Float, TypeTag::Int) => Ok(TypeTag::Float),
        (TypeTag::Str, TypeTag::Str) => Ok(TypeTag::Str),
        (TypeTag::Unt, TypeTag::Unt) => Ok(TypeTag::Unt),
        (TypeTag::Unt, TypeTag::Float) => Ok(TypeTag::Float),
        (TypeTag::Float, TypeTag::Unt) => Ok(TypeTag::Unt),
        (TypeTag::Float, TypeTag::Float) => Ok(TypeTag::Float),
        _ => Err(sema::Sema::error(
            &format!("mismatched types cannot use [{}] with [{}]", lt, rt),
            sema::errors::SemaErr::TypeMismatch,
        )),
    }
}

pub fn parse_compoundassign_type(lt: TypeTag, rt: TypeTag) -> SemaResult<TypeTag> {
    match (&lt, &rt) {
        (TypeTag::Int, TypeTag::Int) => Ok(TypeTag::Int),
        (TypeTag::Unt, TypeTag::Unt) => Ok(TypeTag::Unt),
        (TypeTag::Float, &TypeTag::Float) => Ok(TypeTag::Float),
        (TypeTag::Int | TypeTag::Unt | TypeTag::Float, _) => {
            return Err(Sema::error(
                &format!("Missmatched types expected [{}] found [{}]", lt, rt),
                errors::SemaErr::TypeMismatch,
            ));
        }
        (_, TypeTag::Int | TypeTag::Unt | TypeTag::Float) => {
            return Err(Sema::error(
                &format!("Missmatched types expected [{}] found [{}]", lt, rt),
                errors::SemaErr::TypeMismatch,
            ));
        }
        _ => {
            return Err(Sema::error(
                "modifers like += and -= can be used only on numbers",
                InvalidTarget,
            ));
        }
    }
}
