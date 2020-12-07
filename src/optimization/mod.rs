/*
In these files, we optimize the program by cutting unecessary instructions, and build a simpler form.

It builds a graph representation of the program, where the nodes are operation.
Ex :

a -- \
      + -- c
b -- /

The nodes are separated in connnected components. Non-connected components are simultaneous.
If a shared variable is defined multiple times during one cycle, it causes undefined behaviour (the compiler will choose one pretty much randomly).
No error message will be thrown, as there are no runtime errors and static analysis can't determine if a variable will be assigned to twice or not.

Each node then determine "inputs" and "outputs" (inputs are shared variables, outputs are shared variables and transition variables), and creates a graph between inputs and outputs.

The transition variables are then linked to other nodes
Ex :

node 1:
        /-- not -- [out]
[in] --|
        \-- slice 1 -- [node 1]
        \-- slice 0 -- [node 2]

node2:
[in2] -- \
          + -- --- slice1 [node 1]
[in1] -- /     \-- slice2 [node 2]


Internal node graphs cannot have cycles (because they are ordered), and are immutable, and so I will use a custom representation with each node pointing to the next
with Arc. (a node point to its parents only)
With this representation, we keep only the outputs, which keeps a reference to all the nodes necessary to compute them
and everything else will be dropped by the compiler, hence the free optimisation.

For program nodes, we keep a vec of nodes and they are represented by their id. It can't be done like expr nodes because program nodes can
contain cycles.

With this representation, simulation can be done with a threadpool : each node is simulated in its own thread.

*/

mod graphs;
use std::sync::Arc;

use crate::typed_ast as typ;
pub use graphs::*;
use typ::*;

pub fn make_graph(prog: &typ::Program) -> graphs::ProgramGraph {
    let main_module = prog.get("main").unwrap();
    let node_rename_map = main_module
        .nodes
        .iter()
        .enumerate()
        .map(|(i, (name, _))| (name.clone(), i))
        .collect::<HashMap<String, usize>>();
    let shared_rename_map = main_module
        .inputs
        .iter()
        .map(|s| &s.value)
        .chain(main_module.shared.iter().map(|(s, _)| s))
        .enumerate()
        .map(|(i, s)| (s.clone(), i))
        .collect::<HashMap<String, usize>>();
    let shared = main_module
        .inputs
        .iter()
        .map(|s| vec![false; s.size])
        .chain(main_module.shared.iter().map(|(_s, init)| init.clone()))
        .collect::<Vec<Vec<bool>>>();
    let nodes = main_module
        .nodes
        .iter()
        .map(|(_, node)| make_node(node, &node_rename_map, &shared_rename_map))
        .collect();
    let init_nodes = main_module
        .init_nodes
        .iter()
        .map(|s| *node_rename_map.get(s).unwrap())
        .collect();
    let outputs = main_module
        .outputs
        .iter()
        .map(|v| (v.value.clone(), *shared_rename_map.get(&v.value).unwrap()))
        .collect();
    let inputs = main_module.inputs.iter().map(|var| var.size).collect();
    graphs::ProgramGraph {
        init_nodes,
        shared,
        nodes,
        outputs,
        inputs,
    }
}

fn make_node(
    node: &Node,
    node_rename_map: &HashMap<String, usize>,
    shared_rename_map: &HashMap<String, usize>,
) -> ProgramNode {
    let mut expr_map = Some(HashMap::new());
    let mut inputs = Vec::new();
    let local_rename_map = node
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
    let transition_outputs = node
        .transitions
        .iter()
        .map(|(var, node_name, reset)| {
            let node_id = node_rename_map.get(node_name).unwrap();
            let expr_node = var_to_node(
                None,
                node,
                var,
                shared_rename_map,
                &local_rename_map,
                &mut expr_map,
                &mut inputs,
            );
            (*node_id, expr_node, *reset)
        })
        .collect();
    let shared_outputs = node
        .statements
        .iter()
        .filter_map(|(v, expr)| {
            if let Var::Shared(s) = v {
                let var_id = *shared_rename_map.get(s).unwrap();
                Some((
                    var_id,
                    expr_to_node(
                        None,
                        node,
                        expr,
                        shared_rename_map,
                        &local_rename_map,
                        &mut None,
                        &mut inputs,
                    ),
                ))
            } else {
                None
            }
        })
        .collect();
    let reg_outputs = node
        .statements
        .iter()
        .filter_map(|(_, expr)| {
            if let ExprType::Reg(s) = &expr.value {
                if let ExprTermType::Var(v) = &s.value {
                    let expr_node = var_to_node(
                        None,
                        node,
                        v,
                        shared_rename_map,
                        &local_rename_map,
                        &mut expr_map,
                        &mut inputs,
                    );
                    Some((expr.size, expr_node))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    ProgramNode {
        transition_outputs,
        shared_outputs,
        reg_outputs,
        inputs,
        weak: node.weak,
        n_vars: local_rename_map.len(),
    }
}

fn expr_to_node(
    var_id: Option<usize>,
    node: &Node,
    expr: &Expr,
    shared_rename_map: &HashMap<String, usize>,
    local_rename_map: &HashMap<String, usize>,
    expr_map: &mut Option<HashMap<usize, Arc<ExprNode>>>,
    inputs: &mut Vec<usize>,
) -> Arc<ExprNode> {
    if let Some(id) = var_id {
        if let Some(node) = expr_map.as_mut().map(|map| map.get(&id)).flatten() {
            return node.clone();
        }
    }
    let op = match &expr.value {
        ExprType::Term(e) => match &e.value {
            ExprTermType::Var(v) => {
                return var_to_node(
                    var_id,
                    node,
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
            ExprTermType::Var(v) => ExprOperation::Not(var_to_node(
                None,
                node,
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
                var_to_node(
                    None,
                    node,
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
                ExprTermType::Var(v) => var_to_node(
                    None,
                    node,
                    &v,
                    shared_rename_map,
                    local_rename_map,
                    expr_map,
                    inputs,
                ),
                ExprTermType::Const(c) => Arc::new(ExprNode {
                    id: None,
                    op: ExprOperation::Const(c.clone()),
                }),
            };
            let n2 = match &e2.value {
                ExprTermType::Var(v) => var_to_node(
                    None,
                    node,
                    &v,
                    shared_rename_map,
                    local_rename_map,
                    expr_map,
                    inputs,
                ),
                ExprTermType::Const(c) => Arc::new(ExprNode {
                    id: None,
                    op: ExprOperation::Const(c.clone()),
                }),
            };
            ExprOperation::BiOp(op.clone(), n1, n2)
        }
        ExprType::Mux(e1, e2, e3) => {
            let n1 = match &e1.value {
                ExprTermType::Var(v) => var_to_node(
                    None,
                    node,
                    &v,
                    shared_rename_map,
                    local_rename_map,
                    expr_map,
                    inputs,
                ),
                ExprTermType::Const(c) => Arc::new(ExprNode {
                    id: None,
                    op: ExprOperation::Const(c.clone()),
                }),
            };
            let n2 = match &e2.value {
                ExprTermType::Var(v) => var_to_node(
                    None,
                    node,
                    &v,
                    shared_rename_map,
                    local_rename_map,
                    expr_map,
                    inputs,
                ),
                ExprTermType::Const(c) => Arc::new(ExprNode {
                    id: None,
                    op: ExprOperation::Const(c.clone()),
                }),
            };
            let n3 = match &e3.value {
                ExprTermType::Var(v) => var_to_node(
                    None,
                    node,
                    &v,
                    shared_rename_map,
                    local_rename_map,
                    expr_map,
                    inputs,
                ),
                ExprTermType::Const(c) => Arc::new(ExprNode {
                    id: None,
                    op: ExprOperation::Const(c.clone()),
                }),
            };
            ExprOperation::Mux(n1, n2, n3)
        }
        ExprType::Reg(e) => match &e.value {
            ExprTermType::Var(Var::Local(s)) => {
                ExprOperation::Reg(false, *local_rename_map.get(s).unwrap(), e.size)
            }
            ExprTermType::Var(Var::Shared(s)) => {
                ExprOperation::Reg(true, *shared_rename_map.get(s).unwrap(), e.size)
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
                ExprTermType::Var(v) => var_to_node(
                    None,
                    node,
                    &v,
                    shared_rename_map,
                    local_rename_map,
                    expr_map,
                    inputs,
                ),
                ExprTermType::Const(c) => Arc::new(ExprNode {
                    id: None,
                    op: ExprOperation::Const(c.clone()),
                }),
            };
            let n2 = match &e2.value {
                ExprTermType::Var(v) => var_to_node(
                    None,
                    node,
                    &v,
                    shared_rename_map,
                    local_rename_map,
                    expr_map,
                    inputs,
                ),
                ExprTermType::Const(c) => Arc::new(ExprNode {
                    id: None,
                    op: ExprOperation::Const(c.clone()),
                }),
            };
            let n3 = match &e3.value {
                ExprTermType::Var(v) => var_to_node(
                    None,
                    node,
                    &v,
                    shared_rename_map,
                    local_rename_map,
                    expr_map,
                    inputs,
                ),
                ExprTermType::Const(c) => Arc::new(ExprNode {
                    id: None,
                    op: ExprOperation::Const(c.clone()),
                }),
            };
            let n4 = match &e4.value {
                ExprTermType::Var(v) => var_to_node(
                    None,
                    node,
                    &v,
                    shared_rename_map,
                    local_rename_map,
                    expr_map,
                    inputs,
                ),
                ExprTermType::Const(c) => Arc::new(ExprNode {
                    id: None,
                    op: ExprOperation::Const(c.clone()),
                }),
            };
            ExprOperation::Ram(n1, n2, n3, n4)
        }
        ExprType::Rom(e) => {
            let n = match &e.value {
                ExprTermType::Var(v) => var_to_node(
                    None,
                    node,
                    &v,
                    shared_rename_map,
                    local_rename_map,
                    expr_map,
                    inputs,
                ),
                ExprTermType::Const(c) => Arc::new(ExprNode {
                    id: None,
                    op: ExprOperation::Const(c.clone()),
                }),
            };
            ExprOperation::Rom(n)
        }
    };
    let node = Arc::new(ExprNode { id: var_id, op });
    if let Some(map) = expr_map {
        if let Some(id) = var_id {
            map.insert(id, node.clone());
        }
    }
    node
}

fn var_to_node(
    var_id: Option<usize>,
    node: &Node,
    var: &Var,
    shared_rename_map: &HashMap<String, usize>,
    local_rename_map: &HashMap<String, usize>,
    expr_map: &mut Option<HashMap<usize, Arc<ExprNode>>>,
    inputs: &mut Vec<usize>,
) -> Arc<ExprNode> {
    match var {
        Var::Local(s) => {
            let id = *local_rename_map.get(s).unwrap();
            let expr = node.statements.get(var).unwrap();
            expr_to_node(
                var_id.or(Some(id)),
                node,
                expr,
                shared_rename_map,
                local_rename_map,
                expr_map,
                inputs,
            )
        }
        Var::Shared(s) => {
            let id = *shared_rename_map.get(s).unwrap();
            inputs.push(id);
            Arc::new(ExprNode {
                op: ExprOperation::Input(id),
                id: None,
            })
        }
    }
}
