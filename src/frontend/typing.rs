use ahash::AHashMap;

use crate::ast::parse_ast as untyp;
use crate::ast::typed_ast as typ;
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
    UnknownNode(String, Pos),
    ExpectedSizeOne(Pos, usize),
    IndexOutOfRange(Pos, i32, usize),
    LocalVarInUnless(Pos, String),
    NonSharedInLast(Pos, String),
    ConflictingNodeShared(Pos, String, Pos),
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
    mut type_constraints: AHashMap<String, (i32, Pos)>,
) -> Result<typ::Program> {
    let mut shared_types: AHashMap<String, (usize, Pos)> = AHashMap::new();
    //build the map from shared variables, and type them as well.
    let main_module = prog.modules.get_mut("main").unwrap();
    let mut shared_map = main_module
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
        .collect::<Result<AHashMap<String, Vec<bool>>>>()?;
    //Nodes name are shared variable which indicated the state of the node (running or not)
    for (name, node) in main_module.nodes.iter() {
        if shared_map.contains_key(name) {
            let (_, loc) = shared_types.get(name).unwrap();
            return Err(TypingError::ConflictingNodeShared(
                node.name.loc,
                node.name.value.clone(),
                *loc,
            ));
        }
        shared_types.insert(name.clone(), (1, node.name.loc));
    }
    //inputs are added as shared variables.
    //the types are also added in a vector for use in external module typing.
    let mut in_types = Vec::new();
    let inputs = main_module
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

    //outputs can be shared variables or inputs.
    //the types are also added in a vector for use in external module typing.
    let mut out_types = Vec::new();
    let outputs = main_module
        .outputs
        .drain(..)
        .map(|arg| {
            let loc = arg.size.loc;
            if let untyp::Const::Value(i) = arg.size.value {
                let j = usize::try_from(i).map_err(|_| TypingError::NegativeSizeBus(loc, i))?;
                out_types.push((j, loc));
                if let Some((i_decl, loc_decl)) = shared_types.get(&arg.name) {
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
                    shared_map.insert(arg.name.clone(), vec![false; j]);
                    shared_types.insert(arg.name.clone(), (j, loc));
                    Ok(typ::Sized {
                        value: arg.name,
                        size: j,
                    })
                }
            } else {
                panic!("Should not happen : unknown const in typing");
            }
        })
        .collect::<Result<Vec<typ::Sized<String>>>>()?;
    let nodes_map = main_module
        .nodes
        .iter()
        .map(|(_, node)| (node.name.to_string(), node.name.loc))
        .collect::<AHashMap<String, Pos>>();
    //If init nodes were specified, use them. Else, use the first node.
    let init_nodes = main_module
        .init_nodes
        .drain(..)
        .map(|s| {
            if nodes_map.contains_key(&s.value) {
                Ok(s.value)
            } else {
                Err(TypingError::UnknownNode(s.value, s.loc))
            }
        })
        .collect::<Result<Vec<String>>>()?;
    let nodes = main_module
        .nodes
        .drain()
        .map(|(_, node)| {
            Ok((
                node.name.value.clone(),
                type_node(node, &nodes_map, &shared_types, &mut type_constraints)?,
            ))
        })
        .collect::<Result<AHashMap<typ::Name, typ::Node>>>()?;
    Ok(typ::Program {
        inputs,
        outputs,
        nodes,
        shared: shared_map,
        init_nodes,
    })
}

//type a node. It has to type all the statements and transitions
fn type_node(
    mut node: untyp::Node,
    nodes_map: &AHashMap<String, Pos>,
    shared_types: &AHashMap<String, (usize, Pos)>,
    type_constraints: &mut AHashMap<String, (i32, Pos)>,
) -> Result<typ::Node> {
    let mut var_types: AHashMap<String, (usize, Pos)> = AHashMap::new();
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
        .collect::<Result<AHashMap<typ::Var, typ::Expr>>>()?;
    let weak = node.weak;
    let transitions = node
        .transitions
        .drain(..)
        .map(|transition| {
            if let untyp::Expr::Var(s) = transition.condition.unwrap_ref() {
                if transition.node.is_some()
                    && !nodes_map.contains_key(transition.node.as_ref().unwrap())
                {
                    return Err(TypingError::UnknownNode(
                        transition.node.value.unwrap(),
                        transition.node.loc,
                    ));
                }
                if let Some((size, _loc)) = var_types.get(&s.value) {
                    if weak == false {
                        return Err(TypingError::LocalVarInUnless(s.loc, s.value.clone()));
                    }
                    if *size != 1 {
                        return Err(TypingError::ExpectedSizeOne(s.loc, *size));
                    }
                    Ok((
                        typ::Var::Local(s.value.clone()),
                        transition.node.value,
                        transition.reset,
                    ))
                } else {
                    if let Some((size, _loc)) = shared_types.get(&s.value) {
                        if *size != 1 {
                            return Err(TypingError::ExpectedSizeOne(s.loc, *size));
                        }
                        Ok((
                            typ::Var::Shared(s.value.clone()),
                            transition.node.value,
                            transition.reset,
                        ))
                    } else {
                        Err(TypingError::UnknownVar(s.value.clone(), s.loc))
                    }
                }
            } else {
                panic!("Should not happen : Expected a variable in transition")
            }
        })
        .collect::<Result<Vec<(typ::Var, Option<typ::Name>, bool)>>>()?;
    Ok(typ::Node {
        transitions,
        name: node.name.value,
        weak: node.weak,
        statements,
    })
}

//type a statement, not much to say here.
fn type_statement(
    statement: untyp::Statement,
    var_types: &mut AHashMap<String, (usize, Pos)>,
    shared_types: &AHashMap<String, (usize, Pos)>,
    type_constraints: &mut AHashMap<String, (i32, Pos)>,
) -> Result<(typ::Var, typ::Expr)> {
    match statement {
        untyp::Statement::Assign(mut var_assigns) => {
            assert_eq!(
                var_assigns.len(),
                1,
                "Should not happen : Var assign of size different from 1"
            );
            let untyp::VarAssign { var, expr } = var_assigns.drain(..).next().unwrap();
            let sized_expr = type_expr(expr.value, var_types, shared_types, type_constraints)?;
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
                    var_types.insert(var.value.clone(), (sized_expr.size, var.loc));
                    Ok((typ::Var::Local(var.value), sized_expr))
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
    var_types: &AHashMap<String, (usize, Pos)>,
    shared_types: &AHashMap<String, (usize, Pos)>,
    type_constraints: &mut AHashMap<String, (i32, Pos)>,
) -> Result<typ::Expr> {
    match expr {
        untyp::Expr::Const(_) | untyp::Expr::Var(_) => {
            let sized_expr = type_expr_term(expr, var_types, shared_types, type_constraints)?;
            Ok(typ::Sized {
                size: sized_expr.size,
                value: typ::ExprType::Term(sized_expr),
            })
        }
        untyp::Expr::Last(v) => {
            if let Some(s) = shared_types.get(&v.value) {
                Ok(typ::Sized {
                    size: s.0,
                    value: typ::ExprType::Last(v.value),
                })
            } else {
                Err(TypingError::NonSharedInLast(v.loc, v.value))
            }
        }
        untyp::Expr::Not(expr_term) => {
            let sized_expr = type_expr_term(*expr_term, var_types, shared_types, type_constraints)?;
            Ok(typ::Sized {
                size: sized_expr.size,
                value: typ::ExprType::Not(sized_expr),
            })
        }
        untyp::Expr::Slice(expr_term, c1, c2) => {
            let sized_expr =
                type_expr_term(expr_term.value, var_types, shared_types, type_constraints)?;
            let c1 = c1.unwrap_or(untyp::Const::Value(0));
            let c2 = c2.unwrap_or(untyp::Const::Value(sized_expr.size as i32));
            if let (untyp::Const::Value(i1), untyp::Const::Value(i2)) = (c1, c2) {
                let loc = expr_term.loc;

                let j1 = usize::try_from(i1)
                    .map_err(|_| TypingError::IndexOutOfRange(loc, i1, sized_expr.size))?;
                let j2 = usize::try_from(i2)
                    .map_err(|_| TypingError::IndexOutOfRange(loc, i2, sized_expr.size))?;
                if j2 > sized_expr.size {
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
        untyp::Expr::Reg(c, expr_term) => {
            let loc1 = c.loc;
            let loc2 = expr_term.loc;
            let (size, expr) =
                type_expr_term_reg(expr_term.value, var_types, shared_types, type_constraints)?;
            if let untyp::Const::Value(i) = c.value {
                let j = usize::try_from(i).map_err(|_| TypingError::NegativeSizeBus(c.loc, i))?;
                if let Some(size) = size {
                    if size != j {
                        let token1 = Token {
                            loc: loc1,
                            name: None,
                            length: j,
                        };
                        let token2 = Token {
                            loc: loc2,
                            name: None,
                            length: size,
                        };
                        return Err(TypingError::MismatchedBusSize(token1, token2));
                    }
                }
                Ok(typ::Sized {
                    size: j,
                    value: typ::ExprType::Reg(typ::Sized {
                        size: j,
                        value: expr,
                    }),
                })
            } else {
                panic!("Should not happen : unknown const while typing")
            }
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
    var_types: &AHashMap<String, (usize, Pos)>,
    shared_types: &AHashMap<String, (usize, Pos)>,
    type_constraints: &mut AHashMap<String, (i32, Pos)>,
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
        e => panic!(format!(
            "Should not happen : non terminal expr at depth 1 in typing {:#?}",
            e
        )),
    }
}

fn type_expr_term_reg(
    expr: untyp::Expr,
    var_types: &AHashMap<String, (usize, Pos)>,
    shared_types: &AHashMap<String, (usize, Pos)>,
    type_constraints: &mut AHashMap<String, (i32, Pos)>,
) -> Result<(Option<usize>, typ::ExprTermType)> {
    match expr {
        untyp::Expr::Const(c) => match c {
            untyp::ConstExpr::Known(v) => Ok((Some(v.len()), typ::ExprTermType::Const(v))),
            untyp::ConstExpr::Unknown(b, c) => {
                if let untyp::Const::Value(i) = c.value {
                    let j =
                        usize::try_from(i).map_err(|_| TypingError::NegativeSizeBus(c.loc, i))?;
                    Ok((Some(j), typ::ExprTermType::Const(vec![b; j])))
                } else {
                    panic!("Should not happen : unknown const while typing")
                }
            }
        },
        untyp::Expr::Var(v) => {
            if let Some((size, _loc)) = var_types.get(&v.value) {
                Ok((
                    Some(*size),
                    typ::ExprTermType::Var(typ::Var::Local(v.value)),
                ))
            } else if let Some((size, _loc)) = shared_types.get(&v.value) {
                Ok((
                    Some(*size),
                    typ::ExprTermType::Var(typ::Var::Shared(v.value)),
                ))
            } else if let Some((size, loc)) = type_constraints.get(&v.value) {
                let size_u = usize::try_from(*size)
                    .map_err(|_| TypingError::NegativeSizeBus(*loc, *size))?;
                Ok((
                    Some(size_u),
                    typ::ExprTermType::Var(typ::Var::Local(v.value)),
                ))
            } else {
                Ok((None, typ::ExprTermType::Var(typ::Var::Local(v.value))))
            }
        }
        _ => panic!("Should not happen : non terminal expr at depth 1 in typing"),
    }
}

fn format_var(var: String) -> String {
    if var.starts_with('$') {
        let vec: Vec<&str> = var.split('$').filter(|s| *s != "").collect();
        let _typ = vec[0];
        let fn_name = vec[1];
        let args = vec[2];
        let var_name = vec[3];
        format!(
            "\"{}\" in call of function \"{}\" with arguments{}",
            var_name.split('#').next().unwrap(),
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
        .map(|v| format!(" {} = {},", v[0], v[1]))
        .collect::<String>()
}
