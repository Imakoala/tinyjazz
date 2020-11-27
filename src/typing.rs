use std::collections::HashMap;

use crate::typed_ast as typ;
use crate::{ast as untyp, expand_fn::WrongNumberType};
use std::convert::TryFrom;
use untyp::Pos;
/*
This file aims to type the program.
This is mostly veryfying all operations are between expressions of the right size, and attributing a bus size
to every expression.

It also transforms the ast in a typed_ast, which is a lot simpler and without location information.

Scoping is also done in this file. It just means shared variables are particularized.

The compiler / interpreter is unable to fail after this point, and so every possible error must be detected and reported now.
*/

#[derive(Debug)]
pub enum TypingError {
    NegativeSizeBus(Pos, i32),
    MismatchedBusSize(Token, Token), //expected, got
    UnknownVar(String, Pos),
    DuplicateVar(String, Pos, Pos),
    UnknownModule(String, Pos),
    UnknownNode(String, Pos),
    WrongNumber(WrongNumberType, Pos, usize, usize),
    ExpectedSizeOne(Pos, usize),
    IndexOutOfRange(Pos, i32, usize),
}
#[derive(Debug)]
pub struct Token {
    pub loc: Pos,
    pub name: Option<String>,
    pub length: usize,
}
type Result<T> = std::result::Result<T, TypingError>;
pub fn type_prog(
    mut prog: untyp::Program,
    mut type_constraints: HashMap<String, (i32, Pos)>,
) -> Result<typ::Program> {
    let mut mod_types_map = HashMap::new();
    //make the pre_typing and allocates it in a vec (I did not find a way to do it without a new allocation)
    let mut mod_in_out_map = prog
        .modules
        .drain()
        .map(|(n, mut m)| {
            let (mod_type, mod_pre_typing) = type_mod_inputs_and_outputs(&mut m)?;
            mod_types_map.insert(n.clone(), mod_type);
            Ok((mod_pre_typing, m))
        })
        .collect::<Result<Vec<(ModulePreTyping, untyp::Module)>>>()?;
    //Now type it
    mod_in_out_map
        .drain(..)
        .map(|(pre_typing, m)| {
            Ok((
                m.name.clone(),
                type_mod(m, pre_typing, &mod_types_map, &mut type_constraints)?,
            ))
        })
        .collect()
}

/*
To type the external module calls, all module need to be typed.
So we first type the inputs and outputs of all module (and everything which comes first),
and then we can type the rest of the module, including the external calls
*/

//this type is too long to write.
type ModulePreTyping = (
    HashMap<String, (usize, Pos)>,
    HashMap<String, Vec<bool>>,
    Vec<typ::Sized<String>>,
    Vec<typ::Sized<String>>,
);
fn type_mod_inputs_and_outputs(
    module: &mut untyp::Module,
) -> Result<((Vec<(usize, Pos)>, Vec<(usize, Pos)>), ModulePreTyping)> {
    let mut shared_types: HashMap<String, (usize, Pos)> = HashMap::new();
    //build the map from shared variables, and type them as well.
    let shared_map = module
        .shared
        .drain(..)
        .map(|untyp::VarAssign { var, expr }| {
            let loc = expr.loc;
            if let untyp::Expr::Const(c) = expr.value {
                let v = match c {
                    untyp::ConstExpr::Known(vec) => vec,
                    untyp::ConstExpr::Unknown(b, c) => {
                        if let untyp::Const::Value(i) = c.value {
                            //try i into usize, if it fails then i is negative and an error is thrown
                            let j = usize::try_from(i)
                                .map_err(|_| TypingError::NegativeSizeBus(c.loc, i))?;
                            vec![b; j]
                        } else {
                            panic!("Should not happen : unknown const in typing");
                        }
                    }
                };
                if let Some((_, other_loc)) = shared_types.get(&var.value) {
                    return Err(TypingError::DuplicateVar(
                        var.value.clone(),
                        loc,
                        *other_loc,
                    ));
                }
                shared_types.insert(var.value.clone(), (v.len(), loc));
                Ok((var.value, v))
            } else {
                panic!("Should not happen : non-value constant encountered while typing")
            }
        })
        .collect::<Result<HashMap<String, Vec<bool>>>>()?;
    //inputs are added as shared variables.
    //the types are also added in a vector for use in external module typing.
    let mut in_types = Vec::new();
    let inputs = module
        .inputs
        .drain(..)
        .map(|arg| {
            let loc = arg.size.loc;
            if let untyp::Const::Value(i) = arg.size.value {
                let j = usize::try_from(i).map_err(|_| TypingError::NegativeSizeBus(loc, i))?;
                in_types.push((j, loc));
                if let Some((_, other_loc)) = shared_types.get(&arg.name) {
                    return Err(TypingError::DuplicateVar(arg.name.clone(), loc, *other_loc));
                }
                shared_types.insert(arg.name.clone(), (j, loc));
                Ok(typ::Sized {
                    value: arg.name,
                    size: j,
                })
            } else {
                panic!("Should not happen : unknown const in typing");
            }
        })
        .collect::<Result<Vec<typ::Sized<String>>>>()?;

    //outputs must be shared variables or inputs.
    //the types are also added in a vector for use in external module typing.
    let mut out_types = Vec::new();
    let outputs = module
        .outputs
        .drain(..)
        .map(|arg| {
            let loc = arg.size.loc;
            if let untyp::Const::Value(i) = arg.size.value {
                let j = usize::try_from(i).map_err(|_| TypingError::NegativeSizeBus(loc, i))?;
                if let Some((i_decl, loc_decl)) = shared_types.get(&arg.name) {
                    out_types.push((j, loc));
                    if *i_decl != j {
                        let token1 = Token {
                            loc: *loc_decl,
                            name: Some(format_var(arg.name.clone())),
                            length: *i_decl,
                        };
                        let token2 = Token {
                            loc,
                            name: Some(format_var(arg.name.clone())),
                            length: j,
                        };
                        Err(TypingError::MismatchedBusSize(token1, token2))
                    } else {
                        Ok(typ::Sized {
                            value: arg.name,
                            size: j,
                        })
                    }
                } else {
                    Err(TypingError::UnknownVar(arg.name, loc))
                }
            } else {
                panic!("Should not happen : unknown const in typing");
            }
        })
        .collect::<Result<Vec<typ::Sized<String>>>>()?;
    //for module
    Ok((
        (in_types, out_types),
        (shared_types, shared_map, outputs, inputs),
    ))
}

//type a module. The biggest part is to tpe external modules calls, as we have to check every input and output.
fn type_mod(
    mut module: untyp::Module,
    pre_typing: ModulePreTyping,
    module_types: &HashMap<String, (Vec<(usize, Pos)>, Vec<(usize, Pos)>)>,
    type_constraints: &mut HashMap<String, (i32, Pos)>,
) -> Result<typ::Module> {
    let (shared_types, shared, outputs, inputs) = pre_typing;
    let nodes_map = module
        .nodes
        .iter()
        .map(|node| (node.name.to_string(), node.name.loc))
        .collect::<HashMap<String, Pos>>();
    let nodes = module
        .nodes
        .drain(..)
        .map(|node| {
            Ok((
                node.name.value.clone(),
                type_node(
                    node,
                    &nodes_map,
                    &shared_types,
                    type_constraints,
                    module_types,
                )?,
            ))
        })
        .collect::<Result<HashMap<typ::Name, typ::Node>>>()?;

    //If init nodes were specified, use them. Else, use the first node.
    let init_nodes = if module.init_nodes.len() > 0 {
        module
            .init_nodes
            .drain(..)
            .map(|s| {
                if module_types.contains_key(&s.value) {
                    Ok(s.value)
                } else {
                    Err(TypingError::UnknownModule(s.value, s.loc))
                }
            })
            .collect::<Result<Vec<String>>>()?
    } else {
        if let Some(n) = module.nodes.get(0) {
            vec![n.name.value.clone()]
        } else {
            Vec::new()
        }
    };
    Ok(typ::Module {
        name: module.name,
        inputs,
        outputs,
        nodes,
        shared,
        init_nodes,
    })
}

//type a node. It has to type external module calls again, plus all the statements and transitions
fn type_node(
    mut node: untyp::Node,
    nodes_map: &HashMap<String, Pos>,
    shared_types: &HashMap<String, (usize, Pos)>,
    type_constraints: &mut HashMap<String, (i32, Pos)>,
    module_types: &HashMap<String, (Vec<(usize, Pos)>, Vec<(usize, Pos)>)>,
) -> Result<typ::Node> {
    let mut var_types: HashMap<String, (usize, Pos)> = HashMap::new();
    let statements = node
        .statements
        .drain(..)
        .map(|s| {
            Ok(type_statement(
                s,
                &mut var_types,
                &shared_types,
                type_constraints,
            )?)
        })
        .collect::<Result<HashMap<typ::Var, typ::Expr>>>()?;
    let transitions = node
        .transitions
        .drain(..)
        .map(|(expr, name, reset)| {
            if let untyp::Expr::Var(s) = expr.value {
                if nodes_map.contains_key(&name.value) {
                    return Err(TypingError::UnknownNode(name.value, name.loc));
                }
                if let Some((size, _loc)) = var_types.get(&s.value) {
                    if *size != 1 {
                        return Err(TypingError::ExpectedSizeOne(s.loc, *size));
                    }
                    Ok((typ::Var::Local(s.value), name.value, reset))
                } else {
                    if let Some((size, _loc)) = shared_types.get(&s.value) {
                        if *size != 1 {
                            return Err(TypingError::ExpectedSizeOne(s.loc, *size));
                        }
                        Ok((typ::Var::Shared(s.value), name.value, reset))
                    } else {
                        Err(TypingError::UnknownVar(s.value, s.loc))
                    }
                }
            } else {
                panic!("Should not happen : Expected a variable in transition")
            }
        })
        .collect::<Result<Vec<(typ::Var, typ::Name, bool)>>>()?;

    let extern_modules = node
        .extern_modules
        .drain(..)
        .map(|mut ext_module| {
            let (in_types, out_types) =
                module_types
                    .get(&ext_module.name.value)
                    .ok_or(TypingError::UnknownModule(
                        ext_module.name.value.clone(),
                        ext_module.name.loc,
                    ))?;

            //check that the right number of argument and return vars are supplied
            if in_types.len() != ext_module.inputs.len() {
                return Err(TypingError::WrongNumber(
                    WrongNumberType::Args,
                    ext_module.inputs.loc,
                    in_types.len(),
                    ext_module.inputs.len(),
                ));
            }
            if out_types.len() != ext_module.outputs.len() {
                return Err(TypingError::WrongNumber(
                    WrongNumberType::ReturnVars,
                    ext_module.outputs.loc,
                    out_types.len(),
                    ext_module.outputs.len(),
                ));
            }
            let inputs = ext_module
                .inputs
                .drain(..)
                .zip(in_types.iter())
                .map(|(arg, in_type)| {
                    //An input must be a shared variable. It must be the same type as the module input.
                    let internal_in_type = shared_types
                        .get(&arg.value)
                        .ok_or(TypingError::UnknownVar(arg.value.clone(), arg.loc))?;
                    if in_type.0 != internal_in_type.0 {
                        let token1 = Token {
                            loc: internal_in_type.1,
                            name: Some(format_var(arg.value.clone())),
                            length: internal_in_type.0,
                        };
                        let token2 = Token {
                            loc: in_type.1,
                            name: Some(format_var(arg.value.clone())),
                            length: in_type.0,
                        };
                        Err(TypingError::MismatchedBusSize(token1, token2))
                    } else {
                        Ok(arg.value)
                    }
                })
                .collect::<Result<Vec<String>>>()?;
            let outputs = ext_module
                .outputs
                .drain(..)
                .zip(out_types.iter())
                .map(|(arg, out_type)| {
                    //an output can be a shared or local variable. If it is, it must be the same type as the module output.
                    if let Some(internal_out_type) = shared_types.get(&arg.value) {
                        if out_type.0 != internal_out_type.0 {
                            let token1 = Token {
                                loc: internal_out_type.1,
                                name: Some(format_var(arg.value.clone())),
                                length: internal_out_type.0,
                            };
                            let token2 = Token {
                                loc: out_type.1,
                                name: Some(format_var(arg.value.clone())),
                                length: out_type.0,
                            };
                            Err(TypingError::MismatchedBusSize(token1, token2))
                        } else {
                            Ok(typ::Var::Shared(arg.value))
                        }
                    } else {
                        var_types.insert(arg.value.clone(), *out_type);
                        Ok(typ::Var::Local(arg.value))
                    }
                })
                .collect::<Result<Vec<typ::Var>>>()?;
            Ok(typ::ExtModule {
                inputs,
                outputs,
                name: ext_module.name.to_string(),
            })
        })
        .collect::<Result<Vec<typ::ExtModule>>>()?;
    Ok(typ::Node {
        transitions,
        name: node.name.value,
        weak: node.weak,
        extern_modules,
        statements,
    })
}

//type a statement, not much to say here.
fn type_statement(
    statement: untyp::Statement,
    var_types: &mut HashMap<String, (usize, Pos)>,
    shared_types: &HashMap<String, (usize, Pos)>,
    type_constraints: &mut HashMap<String, (i32, Pos)>,
) -> Result<(typ::Var, typ::Expr)> {
    match statement {
        untyp::Statement::Assign(mut var_assigns) => {
            assert_eq!(
                var_assigns.len(),
                1,
                "Should not happen : Var assign of size different from 1"
            );
            let untyp::VarAssign { var, expr } = var_assigns.drain(..).next().unwrap();
            let sized_expr = type_expr(expr.value, shared_types, var_types, type_constraints)?;
            if let Some((_, loc)) = var_types.get(&var.value) {
                Err(TypingError::DuplicateVar(var.value, var.loc, *loc))
            } else if let Some((size, loc)) = shared_types.get(&var.value) {
                if *size == sized_expr.size {
                    Ok((typ::Var::Shared(var.value), sized_expr))
                } else {
                    let token1 = Token {
                        loc: *loc,
                        name: Some(format_var(var.value.to_string())),
                        length: *size,
                    };
                    let token2 = Token {
                        loc: expr.loc,
                        name: None,
                        length: sized_expr.size,
                    };
                    Err(TypingError::MismatchedBusSize(token1, token2))
                }
            } else if let Some((size, loc)) = type_constraints.get(&var.value) {
                let size_u = usize::try_from(*size)
                    .map_err(|_| TypingError::NegativeSizeBus(*loc, *size))?;
                if size_u == sized_expr.size {
                    Ok((typ::Var::Shared(var.value), sized_expr))
                } else {
                    let token1 = Token {
                        loc: *loc,
                        name: Some(format_var(var.value.to_string())),
                        length: size_u,
                    };
                    let token2 = Token {
                        loc: expr.loc,
                        name: None,
                        length: sized_expr.size,
                    };
                    Err(TypingError::MismatchedBusSize(token1, token2))
                }
            } else {
                var_types.insert(var.value.clone(), (sized_expr.size, var.loc));
                Ok((typ::Var::Local(var.value), sized_expr))
            }
        }
        _ => panic!(format!(
            "Should not happen : Statement with fn call or if struct when typing {:?}",
            statement
        )),
    }
}

fn type_expr(
    expr: untyp::Expr,
    var_types: &HashMap<String, (usize, Pos)>,
    shared_types: &HashMap<String, (usize, Pos)>,
    type_constraints: &mut HashMap<String, (i32, Pos)>,
) -> Result<typ::Expr> {
    match expr {
        untyp::Expr::Const(_) | untyp::Expr::Var(_) => {
            let sized_expr = type_expr_term(expr, var_types, shared_types, type_constraints)?;
            Ok(typ::Sized {
                size: sized_expr.size,
                value: typ::ExprType::Term(sized_expr),
            })
        }
        untyp::Expr::Not(expr_term) => {
            let sized_expr = type_expr_term(*expr_term, var_types, shared_types, type_constraints)?;
            Ok(typ::Sized {
                size: sized_expr.size,
                value: typ::ExprType::Not(sized_expr),
            })
        }
        untyp::Expr::Slice(expr_term, c1, c2) => {
            if let (untyp::Const::Value(i1), untyp::Const::Value(i2)) = (c1, c2) {
                let loc = expr_term.loc;
                let sized_expr =
                    type_expr_term(expr_term.value, var_types, shared_types, type_constraints)?;
                let j1 = usize::try_from(i1)
                    .map_err(|_| TypingError::IndexOutOfRange(loc, i1, sized_expr.size))?;
                let j2 = usize::try_from(i2)
                    .map_err(|_| TypingError::IndexOutOfRange(loc, i2, sized_expr.size))?;
                if j2 >= sized_expr.size {
                    Err(TypingError::IndexOutOfRange(
                        loc,
                        j2 as i32,
                        sized_expr.size,
                    ))
                } else {
                    let mut new_size = j2 as i32 - j1 as i32;
                    if new_size < 0 {
                        new_size = 0
                    }
                    Ok(typ::Sized {
                        size: new_size as usize,
                        value: typ::ExprType::Slice(sized_expr, j1, j2),
                    })
                }
            } else {
                panic!("Should not happen : unknown const in typing")
            }
        }
        untyp::Expr::BiOp(op, e1, e2) => {
            let loc1 = e1.loc;
            let loc2 = e2.loc;
            let sized_e1 = type_expr_term(e1.value, var_types, shared_types, type_constraints)?;
            let sized_e2 = type_expr_term(e2.value, var_types, shared_types, type_constraints)?;
            if let untyp::BiOp::Concat = op {
                Ok(typ::Sized {
                    size: sized_e1.size + sized_e2.size,
                    value: typ::ExprType::BiOp(op, sized_e1, sized_e2),
                })
            } else {
                if sized_e1.size != sized_e2.size {
                    let token1 = Token {
                        loc: loc1,
                        name: None,
                        length: sized_e1.size,
                    };
                    let token2 = Token {
                        loc: loc2,
                        name: None,
                        length: sized_e2.size,
                    };
                    Err(TypingError::MismatchedBusSize(token1, token2))
                } else {
                    Ok(typ::Sized {
                        size: sized_e1.size,
                        value: typ::ExprType::BiOp(op, sized_e1, sized_e2),
                    })
                }
            }
        }
        untyp::Expr::Mux(e1, e2, e3) => {
            let loc1 = e1.loc;
            let loc2 = e2.loc;
            let loc3 = e3.loc;
            let sized_e1 = type_expr_term(e1.value, var_types, shared_types, type_constraints)?;
            let sized_e2 = type_expr_term(e2.value, var_types, shared_types, type_constraints)?;
            let sized_e3 = type_expr_term(e3.value, var_types, shared_types, type_constraints)?;
            if sized_e1.size != 1 {
                Err(TypingError::ExpectedSizeOne(loc1, sized_e1.size))
            } else if sized_e2.size != sized_e3.size {
                let token1 = Token {
                    loc: loc2,
                    name: None,
                    length: sized_e2.size,
                };
                let token2 = Token {
                    loc: loc3,
                    name: None,
                    length: sized_e3.size,
                };
                Err(TypingError::MismatchedBusSize(token1, token2))
            } else {
                Ok(typ::Sized {
                    size: sized_e2.size,
                    value: typ::ExprType::Mux(sized_e1, sized_e2, sized_e3),
                })
            }
        }
        untyp::Expr::Reg(expr_term) => {
            let sized_expr = type_expr_term(*expr_term, var_types, shared_types, type_constraints)?;
            Ok(typ::Sized {
                size: sized_expr.size,
                value: typ::ExprType::Reg(sized_expr),
            })
        }
        untyp::Expr::Ram(untyp::RamStruct {
            read_addr: e1,
            write_enable: e2,
            write_addr: e3,
            write_data: e4,
        }) => {
            let loc1 = e1.loc;
            let loc2 = e2.loc;
            let loc3 = e3.loc;
            let sized_e1 = type_expr_term(e1.value, var_types, shared_types, type_constraints)?;
            let sized_e2 = type_expr_term(e2.value, var_types, shared_types, type_constraints)?;
            let sized_e3 = type_expr_term(e3.value, var_types, shared_types, type_constraints)?;
            let sized_e4 = type_expr_term(e4.value, var_types, shared_types, type_constraints)?;
            if sized_e2.size != 1 {
                Err(TypingError::ExpectedSizeOne(loc2, sized_e2.size))
            } else if sized_e1.size != sized_e3.size {
                let token1 = Token {
                    loc: loc1,
                    name: None,
                    length: sized_e1.size,
                };
                let token2 = Token {
                    loc: loc3,
                    name: None,
                    length: sized_e3.size,
                };
                Err(TypingError::MismatchedBusSize(token1, token2))
            } else {
                Ok(typ::Sized {
                    size: sized_e4.size,
                    value: typ::ExprType::Ram(typ::RamStruct {
                        read_addr: sized_e1,
                        write_enable: sized_e2,
                        write_addr: sized_e3,
                        write_data: sized_e4,
                    }),
                })
            }
        }
        untyp::Expr::Rom(untyp::RomStruct {
            word_size,
            read_addr,
        }) => {
            let loc = read_addr.loc;
            let sized_expr =
                type_expr_term(read_addr.value, var_types, shared_types, type_constraints)?;
            if let untyp::Const::Value(i) = word_size {
                let j = usize::try_from(i).map_err(|_| TypingError::NegativeSizeBus(loc, i))?;
                Ok(typ::Sized {
                    size: j,
                    value: typ::ExprType::Rom(sized_expr),
                })
            } else {
                panic!("Should not happen : unknown const while typing")
            }
        }
        untyp::Expr::FnCall(_) => panic!("Should not happen : fn call in typing"),
    }
}

fn type_expr_term(
    expr: untyp::Expr,
    var_types: &HashMap<String, (usize, Pos)>,
    shared_types: &HashMap<String, (usize, Pos)>,
    type_constraints: &mut HashMap<String, (i32, Pos)>,
) -> Result<typ::ExprTerm> {
    match expr {
        untyp::Expr::Const(c) => match c {
            untyp::ConstExpr::Known(v) => Ok(typ::Sized {
                size: v.len(),
                value: typ::ExprTermType::Const(v),
            }),
            untyp::ConstExpr::Unknown(b, c) => {
                if let untyp::Const::Value(i) = c.value {
                    let j =
                        usize::try_from(i).map_err(|_| TypingError::NegativeSizeBus(c.loc, i))?;
                    Ok(typ::Sized {
                        size: j,
                        value: typ::ExprTermType::Const(vec![b; j]),
                    })
                } else {
                    panic!("Should not happen : unknown const while typing")
                }
            }
        },
        untyp::Expr::Var(v) => {
            if let Some((size, _loc)) = var_types.get(&v.value) {
                Ok(typ::Sized {
                    size: *size,
                    value: typ::ExprTermType::Var(typ::Var::Local(v.value)),
                })
            } else if let Some((size, _loc)) = shared_types.get(&v.value) {
                Ok(typ::Sized {
                    size: *size,
                    value: typ::ExprTermType::Var(typ::Var::Shared(v.value)),
                })
            } else if let Some((size, loc)) = type_constraints.get(&v.value) {
                let size_u = usize::try_from(*size)
                    .map_err(|_| TypingError::NegativeSizeBus(*loc, *size))?;
                Ok(typ::Sized {
                    size: size_u,
                    value: typ::ExprTermType::Var(typ::Var::Local(v.value)),
                })
            } else {
                Err(TypingError::UnknownVar(v.value, v.loc))
            }
        }
        _ => panic!("Should not happen : non terminal expr at depth 1 in typing"),
    }
}

fn format_var(var: String) -> String {
    if var.starts_with('$') {
        println!("{:#?}", var);
        let vec: Vec<&str> = var.split('$').filter(|s| *s != "").collect();
        let _typ = vec[0];
        let fn_name = vec[1];
        let args = vec[2];
        let var_name = vec[3];
        format!(
            "\"{}\" in call of function \"{}\" with arguments{}",
            var_name,
            fn_name,
            format_args(args)
        )
    } else {
        var
    }
}

fn format_args(args: &str) -> String {
    args.split('|')
        .filter(|s| *s != "")
        .map(|v| v.split('_').collect::<Vec<&str>>())
        .map(|v| {
            println!("{:#?}", v);
            format!(" {} = {},", v[0], v[1])
        })
        .collect::<String>()
}
