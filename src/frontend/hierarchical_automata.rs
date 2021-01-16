use std::fmt::Display;

use crate::ast::parse_ast::*;
use ahash::{AHashMap, AHashSet};
use global_counter::global_counter;
/*
This automaton collapses external automata.
Basically, it makes new shared variables for the output, and replaces the automaton call by shared var assignation.
Then it renames every shared var and state in the called automaton, and copies all the states and shared variables in the main automaton.

This repeats until there are no more external automata.
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
    CyclicAutomatonCall(String),
    UnknownAutomaton(Pos, String),
    NoMainAutomaton,
    UnknownVar(Pos, String),
    WrongNumber(WrongNumberType, Pos, usize, usize),
}
global_counter!(INLINE_MODULE_COUNTER, u32, 0);
global_counter!(MODULE_INPUT_COUNTER, u32, 0);
type Result<T> = std::result::Result<T, CollapseAutomataError>;

fn get_input_name(name: &String) -> String {
    let counter = MODULE_INPUT_COUNTER.get_cloned();
    MODULE_INPUT_COUNTER.inc();
    format!("{}#mod_input#{}", name, counter)
}
//makes all transitions shared variables
pub fn make_transitions_shared(prog: &mut Program, iter: u32) {
    for (mod_name, automaton) in prog.automata.iter_mut() {
        let shared_map: AHashSet<String> = automaton
            .shared
            .iter()
            .map(|v_a| v_a.var.value.clone())
            .chain(automaton.states.iter().map(|(name, _)| name.clone()))
            .chain(automaton.inputs.iter().map(|a| a.name.clone()))
            .collect();
        for (state_name, state) in automaton.states.iter_mut() {
            let mut statement = Vec::new();
            for (i, transition) in state.transitions.iter_mut().enumerate() {
                if let TrCond::Expr(e) = &mut transition.condition.value {
                    if let Expr::Var(v) = e {
                        if shared_map.contains(&v.value) {
                            continue;
                        }
                    }
                    let new_name = Loc::new(
                        transition.condition.loc,
                        format!("s_r{}$t{}${}${}", iter, i, state_name, mod_name),
                    );
                    automaton.shared.push(VarAssign {
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
            state.statements.push(Statement::Assign(statement));
        }
    }
}

//replace default transitions
pub fn make_transitions_explicit(prog: &mut Program) {
    for (_mod_name, automaton) in prog.automata.iter_mut() {
        for (_state_name, state) in automaton.states.iter_mut() {
            if !state.transitions.iter().any(|t| {
                if let TrCond::Default = t.condition.value {
                    true
                } else {
                    false
                }
            }) {
                state.transitions.push(Transition {
                    condition: Loc::new(state.name.loc, TrCond::Default),
                    state: Loc::new(state.name.loc, Some(state.name.value.clone())),
                    reset: false,
                })
            }
            let all_conditions = state
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
                    state.name.loc,
                    Expr::Const(ConstExpr::Known(vec![false])),
                ));
            let default_condition = Loc::new(
                all_conditions.loc,
                Expr::Not(Box::new(all_conditions.value)),
            );
            for transition in state.transitions.iter_mut() {
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
    let mut new_states = Vec::new();
    let mut new_init_states = Vec::new();
    let mut new_shared = Vec::new();
    //collapse automaton while something changes.
    //TODO : detect cycles and fail if there is one. Currently the compiler just hangs.
    while changed {
        changed = false;
        let main_automaton = prog
            .automata
            .get("main")
            .ok_or(CollapseAutomataError::NoMainAutomaton)?;
        //It need to be able to "predict" if a state will be entered, and so the parents of each state are needed.
        let state_parents = compute_states_parents(main_automaton);
        //iterates on states with external automaton calls
        for (_, state) in main_automaton.states.iter() {
            let extern_automata = state
                .statements
                .iter()
                .filter_map(|s| {
                    if let Statement::ExtAutomaton(e) = s {
                        Some(e)
                    } else {
                        None
                    }
                })
                .collect::<Vec<&ExtAutomaton>>();
            if extern_automata.is_empty() {
                continue;
            }
            changed = true;
            let exit_condition = get_exit_condition(state);
            //get the global conditions for "entering the state through a [reset/resume] transition"
            let (mut in_conditions_reset, mut in_conditions_resume): (
                Vec<(bool, Loc<Expr>)>,
                Vec<(bool, Loc<Expr>)>,
            ) = state_parents
                .get(&*state.name.value)
                .unwrap()
                .iter()
                .map(|s| {
                    let parent = main_automaton.states.get(*s).unwrap();
                    parent.transitions.iter().filter_map(|transition| {
                        if transition.state.value.is_some()
                            && transition.state.value.clone().unwrap() == state.name.value
                        {
                            Some((
                                transition.reset,
                                Loc::new(
                                    transition.condition.loc,
                                    //condition = the transition condition is true, and the automaton is currently executing the parent state
                                    Expr::BiOp(
                                        BiOp::And,
                                        Box::new(Loc::new(
                                            transition.condition.loc,
                                            transition.condition.clone().value.unwrap(),
                                        )),
                                        Box::new(Loc::new(
                                            state.name.loc,
                                            Expr::Var(state.name.clone()),
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
                    state.name.loc,
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
                    state.name.loc,
                    TrCond::Expr(Expr::Const(ConstExpr::Known(vec![false]))),
                ));
            //This is the state which reads the value of the automaton and write them to shared variables.
            let mut link_state = State {
                name: state.name.clone(),
                statements: state
                    .statements
                    .iter()
                    .filter_map(|s| {
                        if let Statement::ExtAutomaton(_) = s {
                            None
                        } else {
                            Some(s.clone())
                        }
                    })
                    .collect(),
                transitions: state.transitions.clone(),
                weak: state.weak,
            };
            for ExtAutomaton {
                inputs,
                outputs,
                name,
            } in extern_automata
            {
                if name.value == main_automaton.name {
                    return Err(CollapseAutomataError::CyclicAutomatonCall(
                        name.value.clone(),
                    ));
                }
                let pos = name.loc;
                let automaton = prog.automata.get(&name.value).ok_or(
                    CollapseAutomataError::UnknownAutomaton(name.loc, name.value.clone()),
                )?;
                if automaton.inputs.len() != inputs.len() {
                    return Err(CollapseAutomataError::WrongNumber(
                        WrongNumberType::Args,
                        name.loc,
                        automaton.inputs.len(),
                        inputs.len(),
                    ));
                }
                if automaton.outputs.len() != outputs.len() {
                    return Err(CollapseAutomataError::WrongNumber(
                        WrongNumberType::ReturnVars,
                        name.loc,
                        automaton.inputs.len(),
                        inputs.len(),
                    ));
                }

                let mut in_names = Vec::new();
                for (expr, arg) in inputs.value.iter().zip(automaton.inputs.iter()) {
                    let name = get_input_name(&name.value);
                    in_names.push(name.clone());
                    new_shared.push(VarAssign {
                        var: Loc::new(expr.loc, name.clone()),
                        expr: Loc::new(
                            expr.loc,
                            Expr::Const(ConstExpr::Unknown(false, arg.size.clone())),
                        ),
                    });
                    link_state
                        .statements
                        .push(Statement::Assign(vec![VarAssign {
                            var: Loc::new(expr.loc, name.clone()),
                            expr: expr.clone(),
                        }]));
                }
                let (mut states, mut init_states, mut shared, automaton_outputs) = make_automaton(
                    &resume_condition,
                    &reset_condition,
                    exit_condition.clone(),
                    in_names,
                    automaton,
                    main_automaton.init_states.contains(&state.name),
                )?;

                new_init_states.append(&mut init_states);
                new_states.append(&mut states);
                new_shared.append(&mut shared);
                link_state.statements.push(Statement::Assign(
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
            new_states.push(link_state);
        }
        let main_automaton = prog.automata.get_mut("main").unwrap();
        main_automaton.states = main_automaton
            .states
            .drain()
            .filter(|(_, n)| {
                !n.statements.iter().any(|s| {
                    if let Statement::ExtAutomaton(_) = s {
                        true
                    } else {
                        false
                    }
                })
            })
            .collect();
        main_automaton.shared.append(&mut new_shared);
        main_automaton.init_states.append(&mut new_init_states);
        for state in new_states.drain(..) {
            main_automaton.states.insert(state.name.to_string(), state);
        }
    }
    prog.automata = prog.automata.drain().filter(|(s, _)| s == "main").collect();
    // make_transitions_shared(prog, 2);
    // println!("{:#?}", prog);
    Ok(())
}

//Get a condition for the exit of a state.
fn get_exit_condition(state: &State) -> Loc<Expr> {
    let mut expr = Loc::new(state.name.loc, Expr::Var(state.name.clone()));
    for transition in state.transitions.iter() {
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

fn compute_states_parents(automaton: &Automaton) -> AHashMap<&str, Vec<&str>> {
    let mut state_parents = AHashMap::new();
    for (_, state) in automaton.states.iter() {
        if !state_parents.contains_key(&*state.name.value) {
            state_parents.insert(&*state.name.value, Vec::new());
        }
        for transition in state.transitions.iter() {
            if transition.state.is_none() {
                continue;
            }
            if !state_parents.contains_key(&**transition.state.as_ref().unwrap()) {
                state_parents.insert(&**transition.state.as_ref().unwrap(), Vec::new());
            }
            state_parents
                .get_mut(&**transition.state.as_ref().unwrap())
                .unwrap()
                .push(&*state.name.value)
        }
    }
    state_parents
}
fn get_rename(counter: u32, name: &str, namespace: &str) -> String {
    format!("inline_mod${}${}${}$", name, namespace, counter)
}

fn get_pause_name(counter: u32, name: &str, namespace: &str) -> String {
    format!("inline_mod_pause${}${}${}$", name, namespace, counter)
}

//Build the inline automaton corresponding to a automaton and a contexts
pub fn make_automaton(
    resume_condition: &Loc<TrCond>,
    reset_condition: &Loc<Expr>,
    exit_condition: Loc<Expr>,
    mut inputs: Vec<String>,
    automaton: &Automaton,
    is_init: bool,
) -> Result<(Vec<State>, Vec<Loc<String>>, Vec<VarAssign>, Vec<String>)> {
    //new_states, init states, new shared, outputs
    let counter = INLINE_MODULE_COUNTER.get_cloned();
    INLINE_MODULE_COUNTER.inc();
    let mut shared_rename_map = AHashMap::new();
    let mut shared = Vec::new();
    let mut states = Vec::new();
    for (s, rename) in automaton.inputs.iter().zip(inputs.drain(..)) {
        shared_rename_map.insert(s.name.clone(), rename);
    }
    for arg in automaton.outputs.iter() {
        let var = VarAssign {
            var: Loc::new(arg.size.loc, arg.name.clone()),
            expr: Loc::new(
                arg.size.loc,
                Expr::Const(ConstExpr::Unknown(false, arg.size.clone())),
            ),
        };
        let mut new_var = var.clone();
        let name = get_rename(counter, &new_var.var.value, &automaton.name);
        new_var.var.value = name.clone();
        shared.push(new_var.clone());
        shared_rename_map.insert(var.var.value.clone(), name);
    }
    for var in automaton.shared.iter() {
        let mut new_var = var.clone();
        let name = get_rename(counter, &new_var.var.value, &automaton.name);
        new_var.var.value = name.clone();
        shared.push(new_var.clone());
        shared_rename_map.insert(var.var.value.clone(), name);
    }
    //the reset transition is the same for every pause state, so it is pre-computed for the whole automaton here
    let reset_transition = automaton
        .init_states
        .iter()
        .map(|s| {
            let new_name = Loc::new(s.loc, Some(get_rename(counter, &s.value, &automaton.name)));
            Transition {
                condition: Loc::new(
                    reset_condition.loc,
                    TrCond::Expr(reset_condition.value.clone()),
                ),
                state: new_name,
                reset: true,
            }
        })
        .collect();
    for (_, state) in &automaton.states {
        let (new_state, pause_state) = make_state(
            counter,
            &automaton.name,
            &resume_condition,
            &reset_transition,
            &exit_condition,
            &shared_rename_map,
            state,
        );
        states.push(new_state);
        states.push(pause_state);
    }
    let outputs = automaton
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
    let init_states = automaton
        .init_states
        .iter()
        .map(|n| {
            if is_init {
                Loc::new(n.loc, get_rename(counter, &*n.value, &*automaton.name))
            } else {
                Loc::new(n.loc, get_pause_name(counter, &*n.value, &*automaton.name))
            }
        })
        .collect();
    Ok((states, init_states, shared, outputs))
}

//Transform a state into a pause mode and the actual state, for inlining, given a state and its context
pub fn make_state(
    counter: u32,
    namespace: &str,
    resume_condition: &Loc<TrCond>,
    reset_transitions: &Vec<Transition>,
    exit_condition: &Loc<Expr>,
    shared_rename_map: &AHashMap<String, String>,
    state: &State,
) -> (State, State) {
    let new_name = get_rename(counter, &state.name, namespace);
    let pos = state.name.loc;
    let statements = state
        .statements
        .iter()
        .map(|s| replace_var_in_statement(s, &shared_rename_map, counter, namespace))
        .collect::<Vec<Statement>>();
    let transitions = state
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
            let new_state_name = Loc::new(
                transition.state.loc,
                transition
                    .state
                    .value
                    .clone()
                    .map(|s| get_rename(counter, &s, namespace)),
            );
            let pause_state_name = Loc::new(
                transition.state.loc,
                transition
                    .state
                    .value
                    .clone()
                    .map(|s| get_pause_name(counter, &s, namespace)),
            );
            vec![
                Transition {
                    condition: transition_stay,
                    state: new_state_name,
                    reset: transition.reset,
                },
                Transition {
                    condition: transition_pause,
                    state: pause_state_name,
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
        state: Loc::new(pos, Some(new_name.clone())),
        reset: false,
    });
    //Stay paused while we don't come back to the state.
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
        state: Loc::new(pos, Some(get_pause_name(counter, &state.name, namespace))),
        reset: false,
    });
    let pause_state = State {
        name: Loc::new(pos, get_pause_name(counter, &state.name, namespace)),
        statements: Vec::new(),
        weak: true,
        transitions: pause_transitions,
    };
    let stay_state = State {
        name: Loc::new(pos, get_rename(counter, &state.name, namespace)),
        statements,
        weak: state.weak,
        transitions,
    };
    (stay_state, pause_state)
}

//rename vars in a statement
fn replace_var_in_statement(
    statement: &Statement,
    replace_map: &AHashMap<String, String>,
    counter: u32,
    automaton_name: &str,
) -> Statement {
    match statement {
        Statement::Assign(var_assigns) => Statement::Assign(
            var_assigns
                .iter()
                .map(|var_assign| {
                    let new_var = replace_map
                        .get(&var_assign.var.value)
                        .cloned()
                        .unwrap_or(get_rename(counter, &var_assign.var.value, automaton_name));
                    let new_expr = replace_var_in_expr(
                        &var_assign.expr,
                        &replace_map,
                        counter,
                        automaton_name,
                    );
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
                .map(|s| replace_var_in_statement(s, replace_map, counter, automaton_name))
                .collect(),
            else_block: else_block
                .iter()
                .map(|s| replace_var_in_statement(s, replace_map, counter, automaton_name))
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
                            automaton_name,
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
                                replace_var_in_expr(e, replace_map, counter, automaton_name),
                            )
                        })
                        .collect(),
                ),
                static_args: static_args.clone(),
            },
        }),
        Statement::ExtAutomaton(e) => Statement::ExtAutomaton(ExtAutomaton {
            inputs: Loc::new(
                e.inputs.loc,
                e.inputs
                    .iter()
                    .map(|e| {
                        Loc::new(
                            e.loc,
                            replace_var_in_expr(e, replace_map, counter, automaton_name),
                        )
                    })
                    .collect(),
            ),
            outputs: Loc::new(
                e.outputs.loc,
                e.outputs
                    .iter()
                    .map(|v| {
                        Loc::new(
                            v.loc,
                            replace_map.get(&v.value).cloned().unwrap_or(get_rename(
                                counter,
                                &v.value,
                                automaton_name,
                            )),
                        )
                    })
                    .collect(),
            ),
            name: e.name.clone(),
        }),
    }
}

//rename vars in an expression.
fn replace_var_in_expr(
    expr: &Expr,
    replace_map: &AHashMap<String, String>,
    counter: u32,
    automaton_name: &str,
) -> Expr {
    match expr {
        Expr::Var(v) => Expr::Var(Loc::new(
            v.loc,
            replace_map.get(&v.value).cloned().unwrap_or(get_rename(
                counter,
                &v.value,
                automaton_name,
            )),
        )),
        Expr::Last(v) => Expr::Last(Loc::new(
            v.loc,
            replace_map.get(&v.value).cloned().unwrap_or(get_rename(
                counter,
                &v.value,
                automaton_name,
            )),
        )),
        Expr::Const(_) => expr.clone(),
        Expr::Not(e) => Expr::Not(Box::new(replace_var_in_expr(
            e,
            replace_map,
            counter,
            automaton_name,
        ))),
        Expr::Slice(e, c1, c2) => Expr::Slice(
            Box::new(Loc::new(
                e.loc,
                replace_var_in_expr(e, replace_map, counter, automaton_name),
            )),
            c1.clone(),
            c2.clone(),
        ),
        Expr::BiOp(op, e1, e2) => Expr::BiOp(
            op.clone(),
            Box::new(Loc::new(
                e1.loc,
                replace_var_in_expr(e1, replace_map, counter, automaton_name),
            )),
            Box::new(Loc::new(
                e2.loc,
                replace_var_in_expr(e2, replace_map, counter, automaton_name),
            )),
        ),
        Expr::Mux(e1, e2, e3) => Expr::Mux(
            Box::new(Loc::new(
                e1.loc,
                replace_var_in_expr(e1, replace_map, counter, automaton_name),
            )),
            Box::new(Loc::new(
                e2.loc,
                replace_var_in_expr(e2, replace_map, counter, automaton_name),
            )),
            Box::new(Loc::new(
                e3.loc,
                replace_var_in_expr(e3, replace_map, counter, automaton_name),
            )),
        ),
        Expr::Reg(c, e) => Expr::Reg(
            c.clone(),
            Box::new(Loc::new(
                e.loc,
                replace_var_in_expr(e, replace_map, counter, automaton_name),
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
                replace_var_in_expr(read_addr, replace_map, counter, automaton_name),
            )),
            write_enable: Box::new(Loc::new(
                write_enable.loc,
                replace_var_in_expr(write_enable, replace_map, counter, automaton_name),
            )),
            write_addr: Box::new(Loc::new(
                write_addr.loc,
                replace_var_in_expr(write_addr, replace_map, counter, automaton_name),
            )),
            write_data: Box::new(Loc::new(
                write_data.loc,
                replace_var_in_expr(write_data, replace_map, counter, automaton_name),
            )),
        }),
        Expr::Rom(RomStruct {
            read_addr,
            word_size,
        }) => Expr::Rom(RomStruct {
            read_addr: Box::new(Loc::new(
                read_addr.loc,
                replace_var_in_expr(read_addr, replace_map, counter, automaton_name),
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
                            replace_var_in_expr(e, replace_map, counter, automaton_name),
                        )
                    })
                    .collect(),
            ),
            static_args: static_args.clone(),
        }),
    }
}
