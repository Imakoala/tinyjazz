use std::{collections::HashSet, fmt::Display};

use crate::ast::parse_ast::*;

use global_counter::global_counter;
/*
This module collapses external modules.
Basically, it makes new shared variables for the output, and replaces the module call by shared var assignation.
Then it renames every shared var and node in the called module, and copies all the nodes and shared variables in the main module.

This repeats until there are no more external modules.
*/
//TODO: instant transitions
#[derive(Debug)]
pub enum WrongNumberType {
    Args,
    ReturnVars,
}
impl Display for WrongNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WrongNumberType::Args => write!(f, "arguments"),
            WrongNumberType::ReturnVars => write!(f, "return variables"),
        }
    }
}
pub enum CollapseAutomataError {
    CyclicModuleCall(String),
    UnknownModule(Pos, String),
    NoMainModule,
    UnknownVar(Pos, String),
    WrongNumber(WrongNumberType, Pos, usize, usize),
}
global_counter!(INLINE_MODULE_COUNTER, u32, 0);
type Result<T> = std::result::Result<T, CollapseAutomataError>;

//makes all transitions shared variables
pub fn make_transitions_shared(prog: &mut Program, iter: u32) {
    for (mod_name, module) in prog.modules.iter_mut() {
        let shared_map: HashSet<String> = module
            .shared
            .iter()
            .map(|v_a| v_a.var.value.clone())
            .chain(module.nodes.iter().map(|(name, _)| name.clone()))
            .chain(module.inputs.iter().map(|a| a.name.clone()))
            .collect();
        for (node_name, node) in module.nodes.iter_mut() {
            let mut statement = Vec::new();
            for (i, transition) in node.transitions.iter_mut().enumerate() {
                if let TrCond::Expr(e) = &mut transition.condition.value {
                    if let Expr::Var(v) = e {
                        if shared_map.contains(&v.value) {
                            continue;
                        }
                    }
                    let new_name = Loc::new(
                        transition.condition.loc,
                        format!("s_r{}$t{}${}${}", iter, i, node_name, mod_name),
                    );
                    module.shared.push(VarAssign {
                        var: new_name.clone(),
                        expr: Loc::new(new_name.loc, Expr::Const(ConstExpr::Known(vec![false]))),
                    });
                    let old_expr = std::mem::replace(e, Expr::Var(new_name.clone()));
                    statement.push(VarAssign {
                        var: new_name,
                        expr: Loc::new(transition.condition.loc, old_expr),
                    });
                }
            }
            node.statements.push(Statement::Assign(statement));
        }
    }
}

//replace default transitions
pub fn make_transitions_explicit(prog: &mut Program) {
    for (_mod_name, module) in prog.modules.iter_mut() {
        for (_node_name, node) in module.nodes.iter_mut() {
            let all_conditions = node
                .transitions
                .iter()
                .fold(None, |prev, transition| {
                    match (prev, &transition.condition.value) {
                        (None, TrCond::Default) => None,
                        (Some(p), TrCond::Default) => Some(p),
                        (None, TrCond::Expr(e)) => {
                            Some(Loc::new(transition.condition.loc, e.clone()))
                        }
                        (Some(p), TrCond::Expr(e)) => Some(Loc::new(
                            p.loc,
                            Expr::BiOp(
                                BiOp::And,
                                Box::new(Loc::new(transition.condition.loc, e.clone())),
                                Box::new(p),
                            ),
                        )),
                    }
                })
                .unwrap_or(Loc::new(
                    node.name.loc,
                    Expr::Const(ConstExpr::Known(vec![false])),
                ));
            let default_condition = Loc::new(
                all_conditions.loc,
                Expr::Not(Box::new(all_conditions.value)),
            );
            for transition in node.transitions.iter_mut() {
                if transition.condition.is_default() {
                    transition.condition = Loc::new(
                        default_condition.loc,
                        TrCond::Expr(default_condition.value.clone()),
                    );
                }
            }
        }
    }
}

//collapse all hierarchical automata, to build one big non deterministic automaton (or multiple parallel deterministic automata)
pub fn collapse_automata(prog: &mut Program) -> Result<()> {
    make_transitions_shared(prog, 1);
    make_transitions_explicit(prog);
    let mut changed = true;
    let mut new_nodes = Vec::new();
    let mut new_init_nodes = Vec::new();
    let mut new_shared = Vec::new();
    //collapse automaton while something changes.
    //TODO : detect cycles and fail if there is one. Currently the compiler just hangs.
    while changed {
        changed = false;
        let main_module = prog
            .modules
            .get("main")
            .ok_or(CollapseAutomataError::NoMainModule)?;
        //It need to be able to "predict" if a node will be entered, and so the parents of each node are needed.
        let node_parents = compute_nodes_parents(main_module);
        //iterates on nodes with external module calls
        for (_, node) in main_module
            .nodes
            .iter()
            .filter(|(_, n)| !n.extern_modules.is_empty())
        {
            changed = true;
            let exit_condition = get_exit_condition(node);
            //get the global conditions for "entering the node through a [reset/resume] transition"
            let (mut in_conditions_reset, mut in_conditions_resume): (
                Vec<(bool, Loc<Expr>)>,
                Vec<(bool, Loc<Expr>)>,
            ) = node_parents
                .get(&*node.name.value)
                .unwrap()
                .iter()
                .map(|s| {
                    let parent = main_module.nodes.get(*s).unwrap();
                    parent.transitions.iter().filter_map(|transition| {
                        if transition.node.value.is_some()
                            && transition.node.value.clone().unwrap() == node.name.value
                        {
                            Some((
                                transition.reset,
                                Loc::new(
                                    transition.condition.loc,
                                    //condition = the transition condition is true, and the automaton is currently executing the parent node
                                    Expr::BiOp(
                                        BiOp::And,
                                        Box::new(Loc::new(
                                            transition.condition.loc,
                                            transition.condition.clone().value.unwrap(),
                                        )),
                                        Box::new(Loc::new(
                                            node.name.loc,
                                            Expr::Var(node.name.clone()),
                                        )),
                                    ),
                                ),
                            ))
                        } else {
                            None
                        }
                    })
                })
                .flatten()
                .partition(|(b, _)| *b);
            //Fold the resume and reset conditions in one expression
            let reset_condition = in_conditions_reset
                .drain(..)
                .fold(None, |e, (_, n)| {
                    if let Some(e) = e {
                        Some(Loc::new(
                            n.loc,
                            Expr::BiOp(BiOp::And, Box::new(e), Box::new(n)),
                        ))
                    } else {
                        Some(n)
                    }
                })
                .unwrap_or(Loc::new(
                    node.name.loc,
                    Expr::Const(ConstExpr::Known(vec![false])),
                ));
            let resume_condition = in_conditions_resume
                .drain(..)
                .fold(None, |e: Option<Loc<TrCond>>, (_, n)| {
                    if let Some(e) = e {
                        Some(Loc::new(
                            n.loc,
                            TrCond::Expr(Expr::BiOp(
                                BiOp::And,
                                Box::new(Loc::new(e.loc, e.value.clone().unwrap())),
                                Box::new(n),
                            )),
                        ))
                    } else {
                        Some(Loc::new(n.loc, TrCond::Expr(n.value)))
                    }
                })
                .unwrap_or(Loc::new(
                    node.name.loc,
                    TrCond::Expr(Expr::Const(ConstExpr::Known(vec![false]))),
                ));
            //This is the node which reads the value of the automaton and write them to shared variables.
            let mut link_node = Node {
                name: node.name.clone(),
                extern_modules: Vec::new(),
                statements: node.statements.clone(),
                transitions: node.transitions.clone(),
                weak: node.weak,
            };
            for ExtModule {
                inputs,
                outputs,
                name,
            } in &node.extern_modules
            {
                if name.value == main_module.name {
                    return Err(CollapseAutomataError::CyclicModuleCall(name.value.clone()));
                }
                let pos = name.loc;
                let inputs: Vec<String> = inputs.value.iter().map(|s| s.to_string()).collect();
                let module =
                    prog.modules
                        .get(&name.value)
                        .ok_or(CollapseAutomataError::UnknownModule(
                            name.loc,
                            name.value.clone(),
                        ))?;
                if module.inputs.len() != inputs.len() {
                    return Err(CollapseAutomataError::WrongNumber(
                        WrongNumberType::Args,
                        name.loc,
                        module.inputs.len(),
                        inputs.len(),
                    ));
                }
                if module.outputs.len() != outputs.len() {
                    return Err(CollapseAutomataError::WrongNumber(
                        WrongNumberType::ReturnVars,
                        name.loc,
                        module.inputs.len(),
                        inputs.len(),
                    ));
                }
                let (mut nodes, mut init_nodes, mut shared, automaton_outputs) = make_automaton(
                    &resume_condition,
                    &reset_condition,
                    exit_condition.clone(),
                    inputs,
                    module,
                    main_module.init_nodes.contains(&node.name),
                )?;

                new_init_nodes.append(&mut init_nodes);
                new_nodes.append(&mut nodes);
                new_shared.append(&mut shared);
                link_node.statements.push(Statement::Assign(
                    outputs
                        .iter()
                        .zip(automaton_outputs.iter())
                        .map(|(o, auto_o)| VarAssign {
                            var: o.clone(),
                            expr: Loc::new(pos, Expr::Var(Loc::new(pos, auto_o.to_string()))),
                        })
                        .collect(),
                ))
            }
            new_nodes.push(link_node);
        }
        let main_module = prog.modules.get_mut("main").unwrap();
        main_module.nodes = main_module
            .nodes
            .drain()
            .filter(|(_, n)| n.extern_modules.is_empty())
            .collect();
        main_module.shared.append(&mut new_shared);
        main_module.init_nodes.append(&mut new_init_nodes);
        for node in new_nodes.drain(..) {
            main_module.nodes.insert(node.name.to_string(), node);
        }
    }
    prog.modules = prog.modules.drain().filter(|(s, _)| s == "main").collect();
    // make_transitions_shared(prog, 2);
    // println!("{:#?}", prog);
    Ok(())
}

//Get a condition for the exit of a node.
fn get_exit_condition(node: &Node) -> Loc<Expr> {
    let mut expr = Loc::new(node.name.loc, Expr::Var(node.name.clone()));
    for transition in node.transitions.iter() {
        expr = Loc::new(
            transition.condition.loc,
            Expr::BiOp(
                BiOp::And,
                Box::new(Loc::new(
                    transition.condition.loc,
                    transition.condition.value.clone().unwrap(),
                )),
                Box::new(expr),
            ),
        );
    }
    expr
}

fn compute_nodes_parents(module: &Module) -> HashMap<&str, Vec<&str>> {
    let mut node_parents = HashMap::new();
    for (_, node) in module.nodes.iter() {
        if !node_parents.contains_key(&*node.name.value) {
            node_parents.insert(&*node.name.value, Vec::new());
        }
        for transition in node.transitions.iter() {
            if transition.node.is_none() {
                continue;
            }
            if !node_parents.contains_key(&**transition.node.as_ref().unwrap()) {
                node_parents.insert(&**transition.node.as_ref().unwrap(), Vec::new());
            }
            node_parents
                .get_mut(&**transition.node.as_ref().unwrap())
                .unwrap()
                .push(&*node.name.value)
        }
    }
    node_parents
}
fn get_rename(counter: u32, name: &str, namespace: &str) -> String {
    format!("inline_mod${}${}${}$", name, namespace, counter)
}

fn get_pause_name(counter: u32, name: &str, namespace: &str) -> String {
    format!("inline_mod_pause${}${}${}$", name, namespace, counter)
}

//Build the inline automaton corresponding to a module and a contexts
pub fn make_automaton(
    resume_condition: &Loc<TrCond>,
    reset_condition: &Loc<Expr>,
    exit_condition: Loc<Expr>,
    mut inputs: Vec<String>,
    module: &Module,
    is_init: bool,
) -> Result<(Vec<Node>, Vec<Loc<String>>, Vec<VarAssign>, Vec<String>)> {
    //new_nodes, init nodes, new shared, outputs
    let counter = INLINE_MODULE_COUNTER.get_cloned();
    INLINE_MODULE_COUNTER.inc();
    let mut shared_rename_map = HashMap::new();
    let mut shared = Vec::new();
    let mut nodes = Vec::new();
    for (s, rename) in module.inputs.iter().zip(inputs.drain(..)) {
        shared_rename_map.insert(s.name.clone(), rename);
    }
    for var in module.shared.iter() {
        let mut new_var = var.clone();
        let name = get_rename(counter, &new_var.var.value, &module.name);
        new_var.var.value = name.clone();
        shared.push(new_var.clone());
        shared_rename_map.insert(var.var.value.clone(), name);
    }
    //the reset transition is the same for every pause node, so it is pre-computed for the whole automaton here
    let reset_transition = module
        .init_nodes
        .iter()
        .map(|s| {
            let new_name = Loc::new(s.loc, Some(get_rename(counter, &s.value, &module.name)));
            Transition {
                condition: Loc::new(
                    reset_condition.loc,
                    TrCond::Expr(reset_condition.value.clone()),
                ),
                node: new_name,
                reset: true,
            }
        })
        .collect();
    for (_, node) in &module.nodes {
        let (new_node, pause_node) = make_node(
            counter,
            &module.name,
            &resume_condition,
            &reset_transition,
            &exit_condition,
            &shared_rename_map,
            node,
        );
        nodes.push(new_node);
        nodes.push(pause_node);
    }
    let outputs = module
        .outputs
        .iter()
        .map(|s| {
            Ok(shared_rename_map
                .get(&s.name)
                .ok_or(CollapseAutomataError::UnknownVar(
                    s.size.loc,
                    s.name.clone(),
                ))?
                .clone())
        })
        .collect::<Result<Vec<String>>>()?;
    let init_nodes = module
        .init_nodes
        .iter()
        .map(|n| {
            if is_init {
                Loc::new(n.loc, get_rename(counter, &*n.value, &*module.name))
            } else {
                Loc::new(n.loc, get_pause_name(counter, &*n.value, &*module.name))
            }
        })
        .collect();
    Ok((nodes, init_nodes, shared, outputs))
}

//Transform a node into a pause mode and the actual node, for inlining, given a node and its context
pub fn make_node(
    counter: u32,
    namespace: &str,
    resume_condition: &Loc<TrCond>,
    reset_transitions: &Vec<Transition>,
    exit_condition: &Loc<Expr>,
    shared_rename_map: &HashMap<String, String>,
    node: &Node,
) -> (Node, Node) {
    let new_name = get_rename(counter, &node.name, namespace);
    let pos = node.name.loc;
    let statements = node
        .statements
        .iter()
        .map(|s| replace_var_in_statement(s, &shared_rename_map, counter, namespace))
        .collect::<Vec<Statement>>();
    let transitions = node
        .transitions
        .iter()
        .map(|transition| {
            let renamed_expr = Loc::new(
                transition.condition.loc,
                replace_var_in_expr(
                    transition.condition.unwrap_ref(),
                    shared_rename_map,
                    counter,
                    namespace,
                ),
            );
            let transition_stay = Loc::new(
                renamed_expr.loc,
                TrCond::Expr(Expr::BiOp(
                    BiOp::And,
                    Box::new(renamed_expr.clone()),
                    Box::new(Loc::new(
                        exit_condition.loc,
                        Expr::Not(Box::new(exit_condition.value.clone())),
                    )),
                )),
            );
            let transition_pause = Loc::new(
                renamed_expr.loc,
                TrCond::Expr(Expr::BiOp(
                    BiOp::And,
                    Box::new(renamed_expr.clone()),
                    Box::new(exit_condition.clone()),
                )),
            );
            let new_node_name = Loc::new(
                transition.node.loc,
                transition
                    .node
                    .value
                    .clone()
                    .map(|s| get_rename(counter, &s, namespace)),
            );
            let pause_node_name = Loc::new(
                transition.node.loc,
                transition
                    .node
                    .value
                    .clone()
                    .map(|s| get_pause_name(counter, &s, namespace)),
            );
            vec![
                Transition {
                    condition: transition_stay,
                    node: new_node_name,
                    reset: transition.reset,
                },
                Transition {
                    condition: transition_pause,
                    node: pause_node_name,
                    reset: transition.reset,
                },
            ]
        })
        .flatten()
        .collect::<Vec<Transition>>();
    let mut pause_transitions = reset_transitions.clone();
    let reset_condition = reset_transitions
        .iter()
        .fold(None, |prev: Option<Loc<TrCond>>, transition| {
            if let Some(prev) = prev {
                Some(Loc::new(
                    pos,
                    TrCond::Expr(Expr::BiOp(
                        BiOp::Or,
                        Box::new(Loc::new(prev.loc, prev.value.clone().unwrap())),
                        Box::new(Loc::new(
                            transition.condition.loc,
                            transition.condition.value.clone().unwrap(),
                        )),
                    )),
                ))
            } else {
                Some(transition.condition.clone())
            }
        })
        .unwrap();
    pause_transitions.push(Transition {
        condition: resume_condition.clone(),
        node: Loc::new(pos, Some(new_name.clone())),
        reset: false,
    });
    //Stay paused while we don't come back to the node.
    pause_transitions.push(Transition {
        condition: Loc::new(
            pos,
            TrCond::Expr(Expr::Not(Box::new(Expr::BiOp(
                BiOp::Or,
                Box::new(Loc::new(
                    resume_condition.loc,
                    resume_condition.value.clone().unwrap(),
                )),
                Box::new(Loc::new(
                    reset_condition.loc,
                    reset_condition.value.unwrap(),
                )),
            )))),
        ),
        node: Loc::new(pos, Some(get_pause_name(counter, &node.name, namespace))),
        reset: false,
    });
    let pause_node = Node {
        name: Loc::new(pos, get_pause_name(counter, &node.name, namespace)),
        extern_modules: Vec::new(),
        statements: Vec::new(),
        weak: true,
        transitions: pause_transitions,
    };
    let stay_node = Node {
        name: Loc::new(pos, get_rename(counter, &node.name, namespace)),
        extern_modules: node.extern_modules.clone(),
        statements,
        weak: node.weak,
        transitions,
    };
    (stay_node, pause_node)
}

//rename vars in a statement
fn replace_var_in_statement(
    statement: &Statement,
    replace_map: &HashMap<String, String>,
    counter: u32,
    module_name: &str,
) -> Statement {
    match statement {
        Statement::Assign(var_assigns) => Statement::Assign(
            var_assigns
                .iter()
                .map(|var_assign| {
                    let new_var = replace_map
                        .get(&var_assign.var.value)
                        .cloned()
                        .unwrap_or(get_rename(counter, &var_assign.var.value, module_name));
                    let new_expr =
                        replace_var_in_expr(&var_assign.expr, &replace_map, counter, module_name);
                    VarAssign {
                        var: Loc::new(var_assign.var.loc, new_var),
                        expr: Loc::new(var_assign.expr.loc, new_expr),
                    }
                })
                .collect(),
        ),
        Statement::If(IfStruct {
            if_block,
            else_block,
            condition,
        }) => Statement::If(IfStruct {
            if_block: if_block
                .iter()
                .map(|s| replace_var_in_statement(s, replace_map, counter, module_name))
                .collect(),
            else_block: else_block
                .iter()
                .map(|s| replace_var_in_statement(s, replace_map, counter, module_name))
                .collect(),
            condition: condition.clone(),
        }),
        Statement::FnAssign(FnAssign {
            vars,
            f:
                FnCall {
                    name,
                    args,
                    static_args,
                },
        }) => Statement::FnAssign(FnAssign {
            vars: vars
                .iter()
                .map(|v| {
                    Loc::new(
                        v.loc,
                        replace_map.get(&v.value).cloned().unwrap_or(get_rename(
                            counter,
                            &v.value,
                            module_name,
                        )),
                    )
                })
                .collect(),
            f: FnCall {
                name: name.clone(),
                args: Loc::new(
                    args.loc,
                    args.value
                        .iter()
                        .map(|e| {
                            Loc::new(
                                e.loc,
                                replace_var_in_expr(e, replace_map, counter, module_name),
                            )
                        })
                        .collect(),
                ),
                static_args: static_args.clone(),
            },
        }),
    }
}

//rename vars in an expression.
fn replace_var_in_expr(
    expr: &Expr,
    replace_map: &HashMap<String, String>,
    counter: u32,
    module_name: &str,
) -> Expr {
    match expr {
        Expr::Var(v) | Expr::Last(v) => Expr::Var(Loc::new(
            v.loc,
            replace_map.get(&v.value).cloned().unwrap_or(get_rename(
                counter,
                &v.value,
                module_name,
            )),
        )),
        Expr::Const(_) => expr.clone(),
        Expr::Not(e) => Expr::Not(Box::new(replace_var_in_expr(
            e,
            replace_map,
            counter,
            module_name,
        ))),
        Expr::Slice(e, c1, c2) => Expr::Slice(
            Box::new(Loc::new(
                e.loc,
                replace_var_in_expr(e, replace_map, counter, module_name),
            )),
            c1.clone(),
            c2.clone(),
        ),
        Expr::BiOp(op, e1, e2) => Expr::BiOp(
            op.clone(),
            Box::new(Loc::new(
                e1.loc,
                replace_var_in_expr(e1, replace_map, counter, module_name),
            )),
            Box::new(Loc::new(
                e2.loc,
                replace_var_in_expr(e2, replace_map, counter, module_name),
            )),
        ),
        Expr::Mux(e1, e2, e3) => Expr::Mux(
            Box::new(Loc::new(
                e1.loc,
                replace_var_in_expr(e1, replace_map, counter, module_name),
            )),
            Box::new(Loc::new(
                e2.loc,
                replace_var_in_expr(e2, replace_map, counter, module_name),
            )),
            Box::new(Loc::new(
                e3.loc,
                replace_var_in_expr(e3, replace_map, counter, module_name),
            )),
        ),
        Expr::Reg(c, e) => Expr::Reg(
            c.clone(),
            Box::new(Loc::new(
                e.loc,
                replace_var_in_expr(e, replace_map, counter, module_name),
            )),
        ),
        Expr::Ram(RamStruct {
            read_addr,
            write_enable,
            write_addr,
            write_data,
        }) => Expr::Ram(RamStruct {
            read_addr: Box::new(Loc::new(
                read_addr.loc,
                replace_var_in_expr(read_addr, replace_map, counter, module_name),
            )),
            write_enable: Box::new(Loc::new(
                write_enable.loc,
                replace_var_in_expr(write_enable, replace_map, counter, module_name),
            )),
            write_addr: Box::new(Loc::new(
                write_addr.loc,
                replace_var_in_expr(write_addr, replace_map, counter, module_name),
            )),
            write_data: Box::new(Loc::new(
                write_data.loc,
                replace_var_in_expr(write_data, replace_map, counter, module_name),
            )),
        }),
        Expr::Rom(RomStruct {
            read_addr,
            word_size,
        }) => Expr::Rom(RomStruct {
            read_addr: Box::new(Loc::new(
                read_addr.loc,
                replace_var_in_expr(read_addr, replace_map, counter, module_name),
            )),
            word_size: word_size.clone(),
        }),
        Expr::FnCall(FnCall {
            name,
            args,
            static_args,
        }) => Expr::FnCall(FnCall {
            name: name.clone(),
            args: Loc::new(
                args.loc,
                args.value
                    .iter()
                    .map(|e| {
                        Loc::new(
                            e.loc,
                            replace_var_in_expr(e, replace_map, counter, module_name),
                        )
                    })
                    .collect(),
            ),
            static_args: static_args.clone(),
        }),
    }
}
