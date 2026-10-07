use crate::compiler::{
    ast::expr::Expr,
    sema::errors::{
        SemaErr::{InvalidTarget, StackCorruption, TypeMismatch},
        SemaResult,
    },
    type_safety::parse_compoundassign_type,
};

use super::*;

impl Sema {
    pub fn expr_to_hir(&mut self, expr: Expr) -> SemaResult<HirExpr> {
        match expr {
            Expr::Variable { name } => {
                let Some((arg, _, _)) = self.resolve_local(&name) else {
                    return Ok(HirExpr::Globals { name });
                };

                Ok(HirExpr::Variables {
                    slot: arg,
                    line: name.line as u32,
                })
            }
            Expr::Assign { name, right } => {
                let Some((arg, typetag, is_mut)) = self.resolve_local(&name) else {
                    return Ok(HirExpr::Globals { name });
                };

                if !is_mut {
                    return Err(Self::error(
                        "varible must be mututed on order to assign to it",
                        Unexpected,
                    ));
                }

                let rhs_type = self.extract_type(right.as_ref())?;

                if rhs_type != typetag {
                    return Err(Self::error(
                        &format!(
                            "Mismatched types, expected [{}] found [{}]",
                            typetag, rhs_type
                        ),
                        errors::SemaErr::TypeMismatch,
                    ));
                }

                Ok(HirExpr::Assigns {
                    right: Box::new(self.expr_to_hir(*right)?),
                    slot: arg,
                    line: name.line as u32,
                })
            }
            Expr::Const {
                value,
                type_tag: _,
                line,
            } => Ok(HirExpr::Consts { value, line }),
            Expr::Literal {
                value,
                line,
                type_tag: _,
            } => Ok(HirExpr::Literals { value, line }),
            Expr::Range { left, right, line } => {
                let tt_left = self.extract_type(left.as_ref())?;
                let tt_right = self.extract_type(right.as_ref())?;

                let hir_left = self.expr_to_hir(*left);
                let hir_right = self.expr_to_hir(*right);

                if tt_left != tt_right {
                    return Err(Self::error(
                        &format!("Type mismatch expected '{}' found '{}' ", tt_right, tt_left),
                        errors::SemaErr::TypeMismatch,
                    ));
                }

                let range_type = parse_range_type(tt_left, tt_right)?;

                Ok(HirExpr::Ranges {
                    left: Box::new(hir_left?),
                    right: Box::new(hir_right?),
                    type_tag: range_type,
                    line,
                })
            }
            Expr::Grouping(x) => Ok(HirExpr::Grouping(Box::new(self.expr_to_hir(*x)?))),
            Expr::Binary {
                left,
                operator,
                right,
                line,
            } => {
                let tt_left = self.extract_type(&left)?;
                let tt_right = self.extract_type(&right)?;

                parse_binary_type(tt_left.to_owned(), tt_right.to_owned(), operator.token_type)?;

                let hir_left = self.expr_to_hir(*left)?;
                let hir_right = self.expr_to_hir(*right)?;

                Ok(HirExpr::Binarys {
                    left: Box::new(hir_left),
                    operator,
                    right: Box::new(hir_right),
                    line,
                })
            }
            Expr::Unary {
                operator,
                right,
                line,
            } => {
                let hir_right = self.expr_to_hir(*right)?;

                Ok(HirExpr::Unarys {
                    operator,
                    right: Box::new(hir_right),
                    line,
                })
            }
            Expr::CompoundAssign {
                left,
                operator,
                right,
                line,
            } => {
                let tt_left = self.extract_type(&left)?;
                let tt_right = self.extract_type(&right)?;

                let result_type = parse_compoundassign_type(tt_left, tt_right)?;

                if !left.is_var() {
                    return Err(Self::error(
                        &format!("Can not use '{:?}' on anything bur a varible", operator),
                        InvalidTarget,
                    ));
                }

                let slot = self.info_from_var(&left)?.0;

                let hir_right = self.expr_to_hir(*right)?;

                Ok(HirExpr::CompoundAssigns {
                    slot,
                    right: Box::new(hir_right),
                    operator,
                    line,
                    type_tag: result_type,
                })
            }
            Expr::Or { left, right, line } => {
                let tt_left = self.extract_type(&left)?;
                let tt_right = self.extract_type(&right)?;

                if tt_left != TypeTag::Bool || tt_right != TypeTag::Bool {
                    return Err(Self::error(
                        &format!(
                            "mismatched types cannot use [{}] with [{}]",
                            tt_left, tt_right
                        ),
                        InvalidTarget,
                    ));
                }

                let hir_left = self.expr_to_hir(*left)?;
                let hir_right = self.expr_to_hir(*right)?;

                Ok(HirExpr::Ors {
                    right: Box::new(hir_right),
                    left: Box::new(hir_left),
                    line,
                })
            }
            Expr::And { left, right, line } => {
                let tt_left = self.extract_type(&left)?;
                let tt_right = self.extract_type(&right)?;

                if tt_left != TypeTag::Bool || tt_right != TypeTag::Bool {
                    return Err(Self::error(
                        &format!(
                            "mismatched types cannot use [{}] with [{}]",
                            tt_left, tt_right
                        ),
                        InvalidTarget,
                    ));
                }

                let hir_left = self.expr_to_hir(*left)?;
                let hir_right = self.expr_to_hir(*right)?;

                Ok(HirExpr::Ands {
                    right: Box::new(hir_right),
                    left: Box::new(hir_left),
                    line,
                })
            }
            Expr::Function {
                name,
                arity,
                exprs,
                annotations,
                return_type,
                ftype,
                body,
                line,
            } => {
                let enclosing = std::mem::replace(&mut self.compiler, Compiler::new());
                self.compiler_stack.push(enclosing);

                let mut hir_exprs: Vec<HirExpr> = Vec::new();
                let mut param_types: Vec<TypeTag> = Vec::new();

                for e in exprs.into_iter() {
                    for a in annotations.to_owned() {
                        let hir_expr = self.expr_to_hir(e.to_owned())?;
                        hir_exprs.push(hir_expr);

                        let r#type = match a {
                            TokenType::Int => TypeTag::Int,
                            TokenType::Unt => TypeTag::Unt,
                            TokenType::Float => TypeTag::Float,
                            TokenType::Str => TypeTag::Str,
                            TokenType::Char => TypeTag::Char,
                            TokenType::Bool => TypeTag::Bool,
                            _ => {
                                return Err(Self::error(
                                    &format!("Unexpected annotation type : '{:?}'", a),
                                    Unexpected,
                                ));
                            }
                        };

                        self.compiler.locals[(self.compiler.local_count - 1) as usize].type_tag =
                            r#type.clone();

                        param_types.push(r#type);
                    }
                }

                let return_type = match return_type {
                    TokenType::Int => TypeTag::Int,
                    TokenType::Unt => TypeTag::Unt,
                    TokenType::Float => TypeTag::Float,
                    TokenType::Str => TypeTag::Str,
                    TokenType::Char => TypeTag::Char,
                    TokenType::Bool => TypeTag::Bool,
                    TokenType::Void => TypeTag::Void,
                    _ => {
                        return Err(Self::error(
                            &format!("Unexpected annotation type : '{:?}'", return_type),
                            Unexpected,
                        ));
                    }
                };

                self.function_info
                    .return_type_tag_table
                    .insert(name.clone(), return_type);

                self.function_info
                    .parameters_type_tag_table
                    .insert(name.clone(), Rc::new(RefCell::new(param_types)));

                self.begin_scope();

                let mut hir_stmts: Vec<HirStmt> = Vec::new();
                for s in body {
                    let hir_stmt = self.stmt_to_hir(s)?;
                    hir_stmts.push(hir_stmt);
                }

                self.end_scope();

                self.compiler = self
                    .compiler_stack
                    .pop()
                    .ok_or_else(|| Self::error("Compiler not found!", StackCorruption))?;

                Ok(HirExpr::Functions {
                    name,
                    arity,
                    exprs: Box::new(hir_exprs),
                    function_type: ftype,
                    body: hir_stmts,
                    line,
                })
            }
            Expr::Turbofish { left, generic } => {
                let name = if left.is_var() {
                    let Expr::Variable { name } = *left else {
                        return Err(Self::error(
                            &format!("Unexpected : Couldn't extract the calle"),
                            Unexpected,
                        ));
                    };

                    name
                } else {
                    return Err(Self::error(&format!("Invaild target"), InvalidTarget));
                };

                if self
                    .function_info
                    .return_type_tag_table
                    .contains_key(&name.start)
                {
                    return Err(Self::error(
                        &format!("caller '{}' does not need Turbofish ::[]", name.start),
                        InvalidTarget,
                    ));
                }

                self.function_info
                    .return_type_tag_table
                    .insert(name.start.clone(), generic.clone());

                Ok(HirExpr::Turbofishs {
                    calle: name.start,
                    generic,
                })
            }
            Expr::FunctionCall {
                caller,
                argument_count,
                arguments,
                line,
            } => {
                let table = self.function_info.parameters_type_tag_table.clone();
                let mut generic_param = TypeTag::Void;

                let hir_caller = self.expr_to_hir(*caller.clone())?;

                let (name, generic) = match hir_caller.clone() {
                    HirExpr::Turbofishs { calle, generic } => (calle, Some(generic)),
                    _ => {
                        let name = if caller.is_var() {
                            let Expr::Variable { name } = *caller else {
                                return Err(Self::error(
                                    &format!("Unexpected : Couldn't extract the calle"),
                                    Unexpected,
                                ));
                            };

                            name
                        } else {
                            return Err(Self::error(&format!("Invaild target"), InvalidTarget));
                        };
                        (name.start, None)
                    }
                };

                let stack = table
                    .get(&name)
                    .expect("Expected function found nothing.")
                    .as_ref();

                let mut hir_args: Vec<HirExpr> = Vec::new();

                for e in arguments {
                    let type_tag = self.info_from_var(&e)?.1;

                    let mut expected_type_tag = {
                        let expected_stack = stack.borrow();
                        expected_stack.get(argument_count).cloned().unwrap()
                    };

                    if expected_type_tag == TypeTag::Generic {
                        let idx = stack.borrow().iter().position(|t| t == &expected_type_tag);

                        if let Some(idx) = idx {
                            stack.borrow_mut()[idx] = type_tag.clone();
                        }

                        expected_type_tag = type_tag.clone();
                        generic_param = type_tag.clone();
                    }

                    if type_tag != expected_type_tag {
                        return Err(Self::error(
                            &format!(
                                "Mismatched types expected [{}] found [{}]",
                                expected_type_tag, type_tag
                            ),
                            TypeMismatch,
                        ));
                    }

                    let hir = self.expr_to_hir(e)?;
                    hir_args.push(hir);
                }

                if self
                    .function_info
                    .return_type_tag_table
                    .get(&name)
                    .is_some_and(|t| t == &TypeTag::Generic)
                {
                    self.function_info
                        .return_type_tag_table
                        .insert(name.clone(), generic_param.clone());
                }

                Ok(HirExpr::FunctionCalls {
                    caller: Box::new(hir_caller),
                    generic,
                    argument_count,
                    arguments: Box::new(hir_args),
                    line,
                })
            }
            _ => Err(Self::error("Unexpected AST!", errors::SemaErr::Unexpected)),
        }
    }
}
