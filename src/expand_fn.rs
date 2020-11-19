use std::fmt::Display;

use crate::compute_consts::{compute_const, compute_consts_in_statement};
use crate::{ast::*, compute_consts::ComputeConstError};
use global_counter::global_counter;
pub const STACK_SIZE: u32 = 1000;
global_counter!(FN_CALL_VARIABLE, u32, 0);

#[derive(Debug)]
pub enum ExpandFnError {
    //CyclicRecursion(Loc<String>, Vec<i32>),
    StackOverflow(Loc<String>),
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

pub fn expand_functions(prog: &mut Program) -> Result<(), ExpandFnError> {
    let mut changed = Some(Loc {
        loc: (0, 0, 0),
        value: String::new(),
    });
    let mut counter = 0;
    while changed.is_some() {
        changed = replace_fn_calls(prog)?;
        counter += 1;
        if counter >= STACK_SIZE && changed.is_some() {
            return Err(ExpandFnError::StackOverflow(changed.unwrap()));
        }
    }
    Ok(())
}

fn replace_fn_calls(prog: &mut Program) -> Result<Option<Loc<String>>, ExpandFnError> {
    let mut changed = None;
    for (_mod_name, module) in prog.modules.iter_mut() {
        for (_node_name, node) in module.automata.iter_mut().flatten() {
            changed = changed.or(replace_fn_calls_in_statements(
                &mut node.statements,
                &mut prog.functions,
            )?);
        }
    }
    Ok(changed)
}

fn replace_fn_calls_in_statements(
    statements: &mut Vec<Statement>,
    functions: &mut HashMap<String, Function>,
) -> Result<Option<Loc<String>>, ExpandFnError> {
    let mut new_vec: Vec<Statement> = Vec::new();
    let mut changed = None;
    for stat in statements.drain(..) {
        match stat {
            Statement::Assign(mut var_assigns) => {
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
                        inline_function(func, fn_call, outputs, &mut new_vec)?;
                        changed = Some(fn_call.name.clone());
                    }
                }
                new_vec.push(Statement::Assign(
                    var_assigns
                        .drain(..)
                        .filter(|v_a| match v_a.expr.value {
                            Expr::FnCall(_) => false,
                            _ => true,
                        })
                        .collect(),
                ));
            }
            Statement::If(mut if_struct) => {
                if let Const::Value(v) = if_struct.condition {
                    if v == 0 {
                        changed = changed.or(replace_fn_calls_in_statements(
                            &mut if_struct.else_block,
                            functions,
                        )?);
                        new_vec.append(&mut if_struct.else_block);
                    } else {
                        changed = changed.or(replace_fn_calls_in_statements(
                            &mut if_struct.if_block,
                            functions,
                        )?);
                        new_vec.append(&mut if_struct.if_block);
                    }
                } else {
                    panic!("Non-constant condition in if condition, when it should be. Should not happen.")
                }
            }
            Statement::FnAssign(mut fn_assign) => {
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
                inline_function(func, &mut fn_assign.f, outputs, &mut new_vec)?;
                changed = Some(fn_assign.f.name.clone());
            }
        }
    }
    *statements = new_vec;
    Ok(changed)
}

fn inline_function(
    func: &Function,
    fncall: &mut FnCall,
    outputs: Loc<Vec<Loc<Var>>>,
    out_statements: &mut Vec<Statement>,
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
    //Makes the vector assignation
    //remember the names given to the input parameters.
    let mut vars_map = HashMap::new();
    let counter = FN_CALL_VARIABLE.get_cloned();
    FN_CALL_VARIABLE.inc();
    let stat_inputs = Statement::Assign(
        fncall
            .args
            .drain(..)
            .enumerate()
            .map(|(i, expr)| {
                let name = format!("arg${}${}${}", *func.name, func.args[i].name, counter);
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

    //simplify the constants, only values should be left.
    let empty_map = HashMap::new();
    let mut static_args_map = HashMap::new();
    for (i, c) in fncall.static_args.iter_mut().enumerate() {
        compute_const(c, &empty_map)?;
        static_args_map.insert(func.static_args[i].clone(), c.clone());
    }

    //Link the output parameters
    let stat_outputs = Statement::Assign(
        outputs
            .iter()
            .enumerate()
            .map(|(i, var)| {
                let name = format!(
                    "ret${}${}${}",
                    *func.name, func.return_vars[i].name, counter
                );
                vars_map.insert(func.return_vars[i].name.clone(), name.clone());
                VarAssign {
                    var: Loc {
                        value: outputs[i].to_string(),
                        loc: outputs[i].loc.clone(),
                    },
                    expr: Loc {
                        value: Expr::Var(Loc {
                            value: name,
                            loc: var.loc,
                        }),
                        loc: var.loc,
                    },
                }
            })
            .collect(),
    );

    //Make the function body
    let mut func_body = func.statements.clone();
    replace_consts(&mut func_body, &static_args_map)?;
    replace_vars(&mut func_body, &vars_map);
    out_statements.append(&mut func_body);
    out_statements.push(stat_outputs);
    Ok(())
}

fn replace_consts(
    statements: &mut Vec<Statement>,
    consts: &HashMap<String, Const>,
) -> Result<(), ComputeConstError> {
    for statement in statements {
        compute_consts_in_statement(statement, consts)?;
    }
    Ok(())
}
fn replace_vars(statements: &mut Vec<Statement>, vars: &HashMap<String, String>) {
    let mut closure = |v: &mut String| {
        if let Some(v_rep) = vars.get(&v.to_string()) {
            *v = v_rep.to_string()
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
        Expr::Reg(e) => map_vars_in_expr(e, f),
        Expr::Ram(RamStruct {
            addr_size: _,
            word_size: _,
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
            addr_size: _,
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
            vars: _,
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
        }
    }
}
