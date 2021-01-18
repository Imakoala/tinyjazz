/*
In these files, we optimize the program by cutting unecessary instructions, and build a simpler form.

It builds a graph representation of the program, where the states are operation.
Ex :

a -- \
      + -- c
b -- /

The states are separated in connnected components. Non-connected components are simultaneous.
If a shared variable is defined multiple times during one cycle, it causes undefined behaviour (the compiler will choose one pretty much randomly).
No error message will be thrown, as there are no runtime errors and static analysis can't determine if a variable will be assigned to twice or not.

Each state then determine "inputs" and "outputs" (inputs are shared variables, outputs are shared variables and transition variables), and creates a graph between inputs and outputs.

The transition variables are then linked to other states
Ex :

state 1:
        /-- not -- [out]
[in] --|
        \-- slice 1 -- [state 1]
        \-- slice 0 -- [state 2]

state2:
[in2] -- \
          + -- --- slice1 [state 1]
[in1] -- /     \-- slice2 [state 2]


Internal state graphs cannot have cycles (because they are ordered), and are immutable, and so I will use a custom representation with each state pointing to the next
with Rc. (a state point to its parents only)
With this representation, we keep only the outputs, which keeps a reference to all the states necessary to compute them
and everything else will be dropped by the compiler, hence the free optimisation.

For program states, we keep a vec of states and they are represented by their id. It can't be done like expr states because program states can
contain cycles.
*/

use crate::ast::{graph_automaton::*, typed_ast as typ};

use ahash::AHashMap;
use std::rc::Rc;
use typ::*;

pub fn make_graph(prog: &typ::Program) -> ProgramGraph {
    let state_rename_map = prog
        .states
        .iter()
        .enumerate()
        .map(|(i, (name, _))| (name.clone(), i))
        .collect::<AHashMap<String, usize>>();
    let shared_rename_map = prog
        .inputs
        .iter()
        .map(|s| &s.value)
        .chain(prog.states.iter().map(|(name, _)| name))
        .chain(prog.shared.iter().map(|(s, _)| s))
        .enumerate()
        .map(|(i, s)| (s.clone(), i))
        .collect::<AHashMap<String, usize>>();
    let shared = prog
        .inputs
        .iter()
        .map(|s| vec![false; s.size])
        .chain(
            prog.states
                .iter()
                .map(|(name, _)| vec![prog.init_states.contains(name)]),
        )
        .chain(prog.shared.iter().map(|(_s, init)| init.clone()))
        .collect::<Vec<Vec<bool>>>();
    let states = prog
        .states
        .iter()
        .map(|(_, state)| make_state(state, &state_rename_map, &shared_rename_map))
        .collect::<Vec<ProgramState>>();
    let init_states = prog
        .init_states
        .iter()
        .map(|s| *state_rename_map.get(s).unwrap())
        .collect();
    let outputs = prog
        .outputs
        .iter()
        .map(|v| (v.value.clone(), *shared_rename_map.get(&v.value).unwrap()))
        .collect();
    let inputs = prog.inputs.iter().map(|var| var.size).collect();
    let schedule = Vec::new(); // the scheduler is disabled
    ProgramGraph {
        init_states,
        shared,
        states,
        schedule,
        outputs,
        inputs,
    }
}
//transform a state into a ProgramState
fn make_state(
    state: &State,
    state_rename_map: &AHashMap<String, usize>,
    shared_rename_map: &AHashMap<String, usize>,
) -> ProgramState {
    let mut expr_map = Some(AHashMap::new());
    let mut inputs = Vec::new();
    let local_rename_map = state
        .statements
        .iter()
        .filter_map(|(v, _)| {
            if let Var::Local(s) = v {
                Some(s.clone())
            } else {
                None
            }
        })
        .enumerate()
        .map(|(i, s)| (s, i))
        .collect();
    let transition_outputs = state
        .transitions
        .iter()
        .map(|(var, state_name, reset)| {
            let state_id = if state_name.is_none() {
                None
            } else {
                Some(*state_rename_map.get(state_name.as_ref().unwrap()).unwrap())
            };
            let expr_state = var_to_state(
                None,
                state,
                var,
                shared_rename_map,
                &local_rename_map,
                &mut expr_map,
                &mut None, //shared variables used in transitions are not added as outputs
            );
            (state_id, expr_state, *reset)
        })
        .collect();
    let shared_outputs = state
        .statements
        .iter()
        .filter_map(|(v, expr)| {
            if let Var::Shared(s) = v {
                let var_id = *shared_rename_map.get(s).unwrap();
                Some((
                    var_id,
                    expr_to_state(
                        None,
                        state,
                        expr,
                        shared_rename_map,
                        &local_rename_map,
                        &mut None,
                        &mut Some(&mut inputs),
                    ),
                ))
            } else {
                None
            }
        })
        .collect();
    ProgramState {
        transition_outputs,
        shared_outputs,
        inputs,
        weak: state.weak,
        n_vars: local_rename_map.len(),
    }
}
//inputs = None means that we are inside a register.
//FIXME : this is a very bad idea and doent work with nested registers, which are perfectly legal.
fn expr_to_state(
    var_id: Option<usize>,
    state: &State,
    expr: &Expr,
    shared_rename_map: &AHashMap<String, usize>,
    local_rename_map: &AHashMap<String, usize>,
    expr_map: &mut Option<AHashMap<usize, Rc<ExprNode>>>,
    inputs: &mut Option<&mut Vec<usize>>,
) -> Rc<ExprNode> {
    if let Some(id) = var_id {
        if let Some(state) = expr_map.as_mut().map(|map| map.get(&id)).flatten() {
            return state.clone();
        }
    }
    let op = match &expr.value {
        ExprType::Term(e) => match &e.value {
            ExprTermType::Var(v) => {
                return var_to_state(
                    var_id,
                    state,
                    &v,
                    shared_rename_map,
                    local_rename_map,
                    expr_map,
                    inputs,
                )
            }
            ExprTermType::Const(c) => ExprOperation::Const(c.clone()),
        },
        ExprType::Not(e) => match &e.value {
            ExprTermType::Var(v) => ExprOperation::Not(var_to_state(
                None,
                state,
                &v,
                shared_rename_map,
                local_rename_map,
                expr_map,
                inputs,
            )),
            ExprTermType::Const(c) => ExprOperation::Const(c.iter().map(|b| !*b).collect()),
        },
        ExprType::Slice(e, i1, i2) => match &e.value {
            ExprTermType::Var(v) => ExprOperation::Slice(
                var_to_state(
                    None,
                    state,
                    &v,
                    shared_rename_map,
                    local_rename_map,
                    expr_map,
                    inputs,
                ),
                *i1,
                *i2,
            ),
            ExprTermType::Const(c) => ExprOperation::Const(c.clone()),
        },
        ExprType::BiOp(op, e1, e2) => {
            let n1 = match &e1.value {
                ExprTermType::Var(v) => var_to_state(
                    None,
                    state,
                    &v,
                    shared_rename_map,
                    local_rename_map,
                    expr_map,
                    inputs,
                ),
                ExprTermType::Const(c) => Rc::new(ExprNode {
                    op: ExprOperation::Const(c.clone()),
                    ..Default::default()
                }),
            };
            let n2 = match &e2.value {
                ExprTermType::Var(v) => var_to_state(
                    None,
                    state,
                    &v,
                    shared_rename_map,
                    local_rename_map,
                    expr_map,
                    inputs,
                ),
                ExprTermType::Const(c) => Rc::new(ExprNode {
                    op: ExprOperation::Const(c.clone()),
                    ..Default::default()
                }),
            };
            ExprOperation::BiOp(op.clone(), n1, n2)
        }
        ExprType::Mux(e1, e2, e3) => {
            let n1 = match &e1.value {
                ExprTermType::Var(v) => var_to_state(
                    None,
                    state,
                    &v,
                    shared_rename_map,
                    local_rename_map,
                    expr_map,
                    inputs,
                ),
                ExprTermType::Const(c) => Rc::new(ExprNode {
                    op: ExprOperation::Const(c.clone()),
                    ..Default::default()
                }),
            };
            let n2 = match &e2.value {
                ExprTermType::Var(v) => var_to_state(
                    None,
                    state,
                    &v,
                    shared_rename_map,
                    local_rename_map,
                    expr_map,
                    inputs,
                ),
                ExprTermType::Const(c) => Rc::new(ExprNode {
                    op: ExprOperation::Const(c.clone()),
                    ..Default::default()
                }),
            };
            let n3 = match &e3.value {
                ExprTermType::Var(v) => var_to_state(
                    None,
                    state,
                    &v,
                    shared_rename_map,
                    local_rename_map,
                    expr_map,
                    inputs,
                ),
                ExprTermType::Const(c) => Rc::new(ExprNode {
                    op: ExprOperation::Const(c.clone()),
                    ..Default::default()
                }),
            };
            ExprOperation::Mux(n1, n2, n3)
        }
        ExprType::Reg(e) => match &e.value {
            ExprTermType::Var(v) => {
                if inputs.is_none() {
                    //means we are inside a reg
                    ExprOperation::Reg(e.size, None)
                } else {
                    ExprOperation::Reg(
                        e.size,
                        Some(var_to_state(
                            None,
                            state,
                            v,
                            shared_rename_map,
                            local_rename_map,
                            expr_map,
                            &mut None,
                        )),
                    )
                }
            }
            ExprTermType::Const(c) => ExprOperation::Const(c.iter().map(|b| !*b).collect()),
        },
        ExprType::Ram(RamStruct {
            read_addr: e1,
            write_enable: e2,
            write_addr: e3,
            write_data: e4,
        }) => {
            let n1 = match &e1.value {
                ExprTermType::Var(v) => var_to_state(
                    None,
                    state,
                    &v,
                    shared_rename_map,
                    local_rename_map,
                    expr_map,
                    inputs,
                ),
                ExprTermType::Const(c) => Rc::new(ExprNode {
                    op: ExprOperation::Const(c.clone()),
                    ..Default::default()
                }),
            };
            let n2 = match &e2.value {
                ExprTermType::Var(v) => var_to_state(
                    None,
                    state,
                    &v,
                    shared_rename_map,
                    local_rename_map,
                    expr_map,
                    inputs,
                ),
                ExprTermType::Const(c) => Rc::new(ExprNode {
                    op: ExprOperation::Const(c.clone()),
                    ..Default::default()
                }),
            };
            let n3 = match &e3.value {
                ExprTermType::Var(v) => var_to_state(
                    None,
                    state,
                    &v,
                    shared_rename_map,
                    local_rename_map,
                    expr_map,
                    inputs,
                ),
                ExprTermType::Const(c) => Rc::new(ExprNode {
                    op: ExprOperation::Const(c.clone()),
                    ..Default::default()
                }),
            };
            let n4 = match &e4.value {
                ExprTermType::Var(v) => var_to_state(
                    None,
                    state,
                    &v,
                    shared_rename_map,
                    local_rename_map,
                    expr_map,
                    inputs,
                ),
                ExprTermType::Const(c) => Rc::new(ExprNode {
                    op: ExprOperation::Const(c.clone()),
                    ..Default::default()
                }),
            };
            ExprOperation::Ram(n1, n2, n3, n4)
        }
        ExprType::Rom(e) => {
            let n = match &e.value {
                ExprTermType::Var(v) => var_to_state(
                    None,
                    state,
                    &v,
                    shared_rename_map,
                    local_rename_map,
                    expr_map,
                    inputs,
                ),
                ExprTermType::Const(c) => Rc::new(ExprNode {
                    op: ExprOperation::Const(c.clone()),
                    ..Default::default()
                }),
            };
            ExprOperation::Rom(e.size, n)
        }
        ExprType::Last(v) => ExprOperation::Last(*shared_rename_map.get(v).unwrap()),
    };
    let state = Rc::new(ExprNode {
        id: var_id,
        op,
        ..Default::default()
    });
    if let Some(map) = expr_map {
        if let Some(id) = var_id {
            map.insert(id, state.clone());
        }
    }
    state
}

fn var_to_state(
    var_id: Option<usize>,
    state: &State,
    var: &Var,
    shared_rename_map: &AHashMap<String, usize>,
    local_rename_map: &AHashMap<String, usize>,
    expr_map: &mut Option<AHashMap<usize, Rc<ExprNode>>>,
    inputs: &mut Option<&mut Vec<usize>>,
) -> Rc<ExprNode> {
    match var {
        Var::Local(s) => {
            let id = *local_rename_map.get(s).expect(&*format!("{}", s));
            let expr = state.statements.get(var).unwrap();
            expr_to_state(
                var_id.or(Some(id)),
                state,
                expr,
                shared_rename_map,
                local_rename_map,
                expr_map,
                inputs,
            )
        }
        Var::Shared(s) => {
            let id = *shared_rename_map.get(s).unwrap();
            inputs.as_mut().map(|ins| {
                if !ins.contains(&id) {
                    ins.push(id)
                }
            });
            Rc::new(ExprNode {
                op: ExprOperation::Input(id),
                ..Default::default()
            })
        }
    }
}
