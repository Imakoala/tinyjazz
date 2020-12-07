use std::fmt::Display;

use crate::compute_consts::{compute_const, compute_consts_in_statement};
use crate::{ast::*, compute_consts::ComputeConstError};
use global_counter::global_counter;

/*
This file recusively inlines functions.
It simply alternates between inlining all functions, then computing all constants.
Then as all constants in modules are values again, it can inlines the remaining functions, and so on.

It stops if the depth exceed a constant currently, probably a cli parameter eventually
*/

pub const REC_DEPTH: u32 = 1000;
global_counter!(FN_CALL_VARIABLE, u32, 0);

#[derive(Debug)]
pub enum ExpandFnError {
    //CyclicRecursion(Loc<String>, Vec<i32>),
    StackOverflow(Loc<String>), //Actually just recusion depth...
    WrongNumber(WrongNumberType, Pos, String, usize, usize),
    ReplaceConstError(ComputeConstError),
    UnknowFunction(Pos, String),
}

#[derive(Debug)]
pub enum WrongNumberType {
    Args,
    StaticArgs,
    ReturnVars,
}
impl Display for WrongNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WrongNumberType::Args => write!(f, "arguments"),
            WrongNumberType::StaticArgs => write!(f, "static arguments"),
            WrongNumberType::ReturnVars => write!(f, "return variables"),
        }
    }
}
impl From<ComputeConstError> for ExpandFnError {
    fn from(err: ComputeConstError) -> Self {
        ExpandFnError::ReplaceConstError(err)
    }
}

pub fn expand_functions(
    prog: &mut Program,
    type_map: &mut HashMap<String, (i32, Pos)>,
) -> Result<(), ExpandFnError> {
    let mut changed = Some(Loc {
        loc: (0, 0, 0),
        value: String::new(),
    });
    let mut counter = 0;
    while changed.is_some() {
        changed = replace_fn_calls(prog, type_map)?;
        counter += 1;
        if counter >= REC_DEPTH && changed.is_some() {
            return Err(ExpandFnError::StackOverflow(changed.unwrap()));
        }
    }
    Ok(())
}

fn replace_fn_calls(
    prog: &mut Program,
    type_map: &mut HashMap<String, (i32, Pos)>,
) -> Result<Option<Loc<String>>, ExpandFnError> {
    let mut changed = None;
    for (_mod_name, module) in prog.modules.iter_mut() {
        for node in module.nodes.iter_mut() {
            changed = changed.or(replace_fn_calls_in_statements(
                &mut node.statements,
                &mut prog.functions,
                type_map,
            )?);
        }
    }
    Ok(changed)
}

//This replaces a vec of statements with a new vec of statements, where function calls are inlined
//it returns the name of a function which was inlined, to report infinite recursion errors.
fn replace_fn_calls_in_statements(
    statements: &mut Vec<Statement>,
    functions: &mut HashMap<String, Function>,
    type_map: &mut HashMap<String, (i32, Pos)>,
) -> Result<Option<Loc<String>>, ExpandFnError> {
    let mut new_vec: Vec<Statement> = Vec::new();
    let mut changed = None;
    for stat in statements.drain(..) {
        match stat {
            Statement::Assign(mut var_assigns) => {
                //handles all the function calls by pushing the right new statements
                for assign in &mut var_assigns {
                    if let Expr::FnCall(fn_call) = &mut assign.expr.value {
                        let func =
                            functions
                                .get(&*fn_call.name)
                                .ok_or(ExpandFnError::UnknowFunction(
                                    fn_call.name.loc,
                                    fn_call.name.to_string(),
                                ))?;
                        let outputs = Loc {
                            loc: assign.var.loc.clone(),
                            value: vec![assign.var.clone()],
                        };
                        inline_function(func, fn_call, outputs, &mut new_vec, type_map)?;
                        changed = Some(fn_call.name.clone());
                    }
                }
                //and the select the other var assign and add them as well
                new_vec.append(
                    &mut var_assigns
                        .drain(..)
                        .filter_map(|v_a| match v_a.expr.value {
                            Expr::FnCall(_) => None,
                            _ => Some(Statement::Assign(vec![v_a])),
                        })
                        .collect(),
                );
            }
            Statement::If(mut if_struct) => {
                //As the condition is always a value, this is a good place select the right block and ignore the other.
                if let Const::Value(v) = if_struct.condition {
                    if v == 0 {
                        changed = changed.or(replace_fn_calls_in_statements(
                            &mut if_struct.else_block,
                            functions,
                            type_map,
                        )?);
                        new_vec.append(&mut if_struct.else_block);
                    } else {
                        changed = changed.or(replace_fn_calls_in_statements(
                            &mut if_struct.if_block,
                            functions,
                            type_map,
                        )?);
                        new_vec.append(&mut if_struct.if_block);
                    }
                } else {
                    panic!("Non-constant condition in if condition, when it should be. Should not happen.")
                }
            }
            Statement::FnAssign(mut fn_assign) => {
                //A simple function assign, just inline it
                let func =
                    functions
                        .get(&*fn_assign.f.name)
                        .ok_or(ExpandFnError::UnknowFunction(
                            fn_assign.f.name.loc,
                            fn_assign.f.name.to_string(),
                        ))?;
                let outputs = Loc {
                    loc: fn_assign.f.name.loc.clone(),
                    value: fn_assign.vars,
                };
                inline_function(func, &mut fn_assign.f, outputs, &mut new_vec, type_map)?;
                changed = Some(fn_assign.f.name.clone());
            }
        }
    }
    *statements = new_vec;
    Ok(changed)
}

//This inlines a function.
//This means generating a certain amount of statements and intermediary variables,
//and the binding those to the outputs, which are specifier
fn inline_function(
    func: &Function,
    fncall: &mut FnCall,
    outputs: Loc<Vec<Loc<Var>>>,
    out_statements: &mut Vec<Statement>,
    type_map: &mut HashMap<String, (i32, Pos)>,
) -> Result<(), ExpandFnError> {
    //check the number of arguments
    if fncall.static_args.len() != func.static_args.len() {
        return Err(ExpandFnError::WrongNumber(
            WrongNumberType::StaticArgs,
            fncall.static_args.loc,
            func.name.to_string(),
            func.static_args.len(),
            fncall.static_args.len(),
        ));
    }
    if fncall.args.len() != func.args.len() {
        return Err(ExpandFnError::WrongNumber(
            WrongNumberType::Args,
            fncall.args.loc,
            func.name.to_string(),
            func.args.len(),
            fncall.args.len(),
        ));
    }
    if outputs.len() != func.return_vars.len() {
        return Err(ExpandFnError::WrongNumber(
            WrongNumberType::ReturnVars,
            outputs.loc.clone(),
            func.name.to_string(),
            func.return_vars.len(),
            outputs.len(),
        ));
    }

    //simplify the constants, only values should be left.
    let empty_map = HashMap::new();
    let mut static_args_map = HashMap::new();
    for (i, c) in fncall.static_args.iter_mut().enumerate() {
        compute_const(c, &empty_map)?;
        static_args_map.insert(func.static_args[i].clone(), c.clone());
    }
    //string with static args
    let args_string: String = static_args_map
        .iter()
        .map(|(s, c)| {
            if let Const::Value(i) = c {
                format!("{}_{}|", s, i).to_string()
            } else {
                String::new()
            }
        })
        .collect();

    //Link the inpute paramters
    //remember the names given to the input parameters.
    let mut vars_map = HashMap::new();
    let counter = FN_CALL_VARIABLE.get_cloned();
    FN_CALL_VARIABLE.inc();

    if fncall.args.len() > 0 {
        let stat_inputs = Statement::Assign(
            fncall
                .args
                .drain(..)
                .enumerate()
                .map(|(i, expr)| {
                    let name = format!(
                        "$arg${}${}${}${}",
                        *func.name, args_string, func.args[i].name, counter
                    );
                    if let Const::Value(size) = func.args[i].size.value {
                        type_map.insert(name.clone(), (size, func.args[i].size.loc));
                    }
                    vars_map.insert(func.args[i].name.clone(), name.clone());
                    VarAssign {
                        var: Loc {
                            value: name,
                            loc: expr.loc,
                        },
                        expr,
                    }
                })
                .collect(),
        );
        out_statements.push(stat_inputs);
    }

    for (i, var) in outputs.iter().enumerate() {
        if let Const::Value(size) = func.return_vars[i].size.value {
            type_map.insert(var.value.clone(), (size, func.return_vars[i].size.loc));
        }
        vars_map.insert(func.return_vars[i].name.clone(), var.value.clone());
    }
    //Link the output parameters
    // let stat_outputs = Statement::Assign(
    //     outputs
    //         .iter()
    //         .enumerate()
    //         .map(|(i, var)| {
    //             // let name = format!(
    //             //     "$ret${}${}${}${}",
    //             //     *func.name, args_string, func.return_vars[i].name, counter
    //             // );

    //             let expr_loc = (fncall.name.loc.0, fncall.name.loc.1, fncall.args.loc.2 + 1);
    //             VarAssign {
    //                 var: Loc {
    //                     value: var.to_string(),
    //                     loc: var.loc,
    //                 },
    //                 expr: Loc {
    //                     value: Expr::Var(Loc {
    //                         value: var.value.clone(),
    //                         loc: expr_loc,
    //                     }),
    //                     loc: expr_loc,
    //                 },
    //             }
    //         })
    //         .collect(),
    // );

    //Make the function body
    let mut func_body = func.statements.clone();
    replace_consts(&mut func_body, &static_args_map)?;
    replace_vars(
        &mut func_body,
        &mut vars_map,
        &func.name.value,
        &args_string,
        counter,
    );

    //push in the right order
    out_statements.append(&mut func_body);
    // if outputs.len() > 0 {
    //     out_statements.push(stat_outputs);
    // }
    Ok(())
}

//replace constants in functions using the provided static parameters
fn replace_consts(
    statements: &mut Vec<Statement>,
    consts: &HashMap<String, Const>,
) -> Result<(), ComputeConstError> {
    for statement in statements {
        compute_consts_in_statement(statement, consts)?;
    }
    Ok(())
}

//replace the variables in functions using the input variables (which might be generated if an expr was passed as the argument)
fn replace_vars(
    statements: &mut Vec<Statement>,
    vars: &mut HashMap<String, String>,
    fn_name: &String,
    static_params: &String,
    counter: u32,
) {
    let mut closure = |v: &mut String| {
        if let Some(v_rep) = vars.get(&v.to_string()) {
            *v = v_rep.to_string()
        } else {
            let name = format!("$in_fn${}${}${}${}", fn_name, static_params, v, counter);
            vars.insert(v.clone(), name.clone());
            *v = name;
        }
    };
    for statement in statements {
        map_vars_in_statement(statement, &mut closure);
    }
}

fn map_vars_in_expr<F>(expr: &mut Expr, f: &mut F)
where
    F: FnMut(&mut String),
{
    match expr {
        Expr::Not(e) => map_vars_in_expr(e, f),
        Expr::Slice(e, _, _) => map_vars_in_expr(e, f),
        Expr::BiOp(_, e1, e2) => {
            map_vars_in_expr(e1, f);
            map_vars_in_expr(e2, f);
        }
        Expr::Mux(e1, e2, e3) => {
            map_vars_in_expr(e1, f);
            map_vars_in_expr(e2, f);
            map_vars_in_expr(e3, f);
        }
        Expr::Reg(_, e) => map_vars_in_expr(e, f),
        Expr::Ram(RamStruct {
            read_addr,
            write_enable,
            write_addr,
            write_data,
        }) => {
            map_vars_in_expr(read_addr, f);
            map_vars_in_expr(write_enable, f);
            map_vars_in_expr(write_addr, f);
            map_vars_in_expr(write_data, f);
        }
        Expr::Rom(RomStruct {
            word_size: _,
            read_addr,
        }) => map_vars_in_expr(read_addr, f),
        Expr::FnCall(FnCall {
            name: _,
            args,
            static_args: _,
        }) => {
            for arg in &mut **args {
                map_vars_in_expr(arg, f);
            }
        }
        Expr::Var(v) => f(&mut **v),
        Expr::Const(_) => (),
    }
}

fn map_vars_in_statement<F>(statement: &mut Statement, f: &mut F)
where
    F: FnMut(&mut String),
{
    match statement {
        Statement::Assign(vec) => {
            for assign in vec {
                f(&mut assign.var);
                map_vars_in_expr(&mut assign.expr, f);
            }
        }
        Statement::If(ifstruct) => {
            for stat in &mut ifstruct.if_block {
                map_vars_in_statement(stat, f);
            }
            for stat in &mut ifstruct.else_block {
                map_vars_in_statement(stat, f);
            }
        }
        Statement::FnAssign(FnAssign {
            vars,
            f:
                FnCall {
                    name: _,
                    args,
                    static_args: _,
                },
        }) => {
            for arg in &mut **args {
                map_vars_in_expr(arg, f);
            }
            for v in vars {
                f(&mut v.value)
            }
        }
    }
}
