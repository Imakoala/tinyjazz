use std::collections::HashMap;
use std::sync::Arc;

use crate::ast::{graph::*, graph_automaton::*};
/*
Transforms the automata in a single dataflow graph, using the following process :
-compute all reset condition so regs can be reset appropriatly.
-Replace every node state var "n" with "last n" (as it is the expected behaviour) (this can be done lazily)
-make a table of which node produces which output usign which ExprNode.
for each node:
    -make the mux expression of each Input, using previously computed nodes
    -compute the expressions of its outputs
-Replace them all, in scheduling order.
-Last has disseapeared as it is redundant.
-Everything is wrapped in RCell to stat mutable, for optimisations later on
Once this is done, computes the state variables. They should all be simple shared vars,
and so it should be simple using the previous map.
*/

pub fn flatten_automata(prog: &ProgramGraph) -> FlatProgramGraph {
    let n_input = prog.inputs.len();
    let n_node = n_input + prog.nodes.len();
    let mut shared_map = HashMap::new();
    let mut nodes_mem = vec![HashMap::new(); prog.nodes.len()];
    let reset_conditions = compute_reset_conditions(
        &prog,
        &mut shared_map,
        &prog.shared,
        &mut nodes_mem,
        n_input,
    );
    let init_node = &RCell::new(Node::Reg(1, RCell::new(Node::Const(vec![true]))));
    //Link all the inputs and outputs of shared vars.
    for node_id in &prog.schedule {
        compute_nodes(
            &prog.nodes[*node_id],
            &mut shared_map,
            &prog.shared,
            &mut nodes_mem[*node_id],
            &reset_conditions,
            n_input,
            *node_id,
        );
    }
    //compute all the transitions. (which are node shared variables) order doesn't matter.
    compute_transitions(
        &prog,
        &mut shared_map,
        &prog.shared,
        &mut nodes_mem,
        &reset_conditions,
        n_input,
    );
    add_init_values(&mut shared_map, &prog.shared, n_node, init_node);
    // println!("{:#?}", shared_map);
    remove_tmp_value(&mut shared_map, &prog.shared);
    FlatProgramGraph {
        outputs: prog
            .outputs
            .iter()
            .map(|(s, i)| {
                (
                    s.to_string(),
                    shared_map
                        .remove(i)
                        .unwrap_or(RCell::new(Node::Const(prog.shared[*i].clone()))),
                )
            })
            .collect(),
        inputs: prog.inputs.clone(),
    }
}

fn compute_nodes(
    node: &ProgramNode,
    shared_map: &mut HashMap<usize, RCell<Node>>,
    shared_sizes: &Vec<Vec<bool>>,
    node_mem: &mut HashMap<Arc<ExprNode>, RCell<Node>>,
    reset_conditions: &Vec<Option<Node>>,
    n_input: usize,
    node_id: usize,
) {
    for (id, expr_node) in &node.shared_outputs {
        let node = compute_node(
            expr_node.clone(),
            shared_map,
            shared_sizes,
            node_mem,
            reset_conditions,
            n_input,
            node_id,
        );

        let new_node = if let Some(prev_node) = shared_map.remove(id) {
            Node::Mux(
                RCell::new(Node::TmpValueHolder(n_input + node_id)),
                node,
                prev_node,
            )
        } else {
            Node::Mux(
                RCell::new(Node::TmpValueHolder(n_input + node_id)),
                node,
                RCell::new(Node::Const(shared_sizes[*id].clone())),
            )
        };
        shared_map.insert(*id, RCell::new(new_node));
    }
}
fn compute_reset_conditions(
    prog: &ProgramGraph,
    shared_map: &mut HashMap<usize, RCell<Node>>,
    shared_sizes: &Vec<Vec<bool>>,
    node_mem: &mut Vec<HashMap<Arc<ExprNode>, RCell<Node>>>,
    n_input: usize,
) -> Vec<Option<Node>> {
    let mut reset_conditions = vec![None; prog.nodes.len()];
    for (pred_id, node) in prog.nodes.iter().enumerate() {
        for (next_id, expr_node, b) in &node.transition_outputs {
            if next_id.is_some() && *b {
                let next_id = next_id.unwrap();
                let new_node = compute_node(
                    expr_node.clone(),
                    shared_map,
                    shared_sizes,
                    &mut node_mem[pred_id],
                    &vec![None; prog.nodes.len()],
                    n_input,
                    pred_id,
                );
                let condition = Node::BiOp(
                    BiOp::And,
                    RCell::new(Node::TmpValueHolder(pred_id + prog.inputs.len())),
                    new_node,
                );
                let prev_condition = std::mem::take(&mut reset_conditions[next_id]);
                reset_conditions[next_id] = if prev_condition.is_none() {
                    Some(condition)
                } else {
                    Some(Node::BiOp(
                        BiOp::Or,
                        RCell::new(condition),
                        RCell::new(prev_condition.unwrap()),
                    ))
                };
            }
        }
    }
    reset_conditions
}

fn compute_transitions(
    prog: &ProgramGraph,
    shared_map: &mut HashMap<usize, RCell<Node>>,
    shared_sizes: &Vec<Vec<bool>>,
    node_mem: &mut Vec<HashMap<Arc<ExprNode>, RCell<Node>>>,
    reset_conditions: &Vec<Option<Node>>,
    n_input: usize,
) {
    for (pred_id, node) in prog.nodes.iter().enumerate() {
        for (next_id, expr_node, _) in &node.transition_outputs {
            if let Some(next_id) = next_id {
                let new_node = compute_node(
                    expr_node.clone(),
                    shared_map,
                    shared_sizes,
                    &mut node_mem[pred_id],
                    reset_conditions,
                    n_input,
                    pred_id,
                );
                let condition = RCell::new(Node::BiOp(
                    BiOp::And,
                    RCell::new(Node::TmpValueHolder(pred_id + n_input)),
                    new_node,
                ));
                let prev_condition = shared_map.remove(&(*next_id + n_input));
                shared_map.insert(
                    *next_id + n_input,
                    if prev_condition.is_none() {
                        condition
                    } else {
                        RCell::new(Node::BiOp(BiOp::Or, condition, prev_condition.unwrap()))
                    },
                );
            }
        }
    }
}

fn compute_node(
    expr_node: Arc<ExprNode>,
    shared_map: &mut HashMap<usize, RCell<Node>>,
    shared_size: &Vec<Vec<bool>>,
    node_mem: &mut HashMap<Arc<ExprNode>, RCell<Node>>,
    reset_conditions: &Vec<Option<Node>>,
    n_input: usize,
    node_id: usize,
) -> RCell<Node> {
    if let Some(n) = node_mem.get(&expr_node) {
        return n.clone();
    }
    let ret = match expr_node.op.clone() {
        ExprOperation::Input(i) => {
            if i < n_input {
                RCell::new(Node::Input(i))
            } else {
                RCell::new(Node::TmpValueHolder(i))
            }
        }
        ExprOperation::Const(c) => RCell::new(Node::Const(c)),
        ExprOperation::Not(e) => RCell::new(Node::Not(compute_node(
            e,
            shared_map,
            shared_size,
            node_mem,
            reset_conditions,
            n_input,
            node_id,
        ))),
        ExprOperation::Slice(e, c1, c2) => RCell::new(Node::Slice(
            compute_node(
                e,
                shared_map,
                shared_size,
                node_mem,
                reset_conditions,
                n_input,
                node_id,
            ),
            c1,
            c2,
        )),
        ExprOperation::BiOp(op, e1, e2) => RCell::new(Node::BiOp(
            op,
            compute_node(
                e1,
                shared_map,
                shared_size,
                node_mem,
                reset_conditions,
                n_input,
                node_id,
            ),
            compute_node(
                e2,
                shared_map,
                shared_size,
                node_mem,
                reset_conditions,
                n_input,
                node_id,
            ),
        )),
        ExprOperation::Mux(e1, e2, e3) => RCell::new(Node::Mux(
            compute_node(
                e1,
                shared_map,
                shared_size,
                node_mem,
                reset_conditions,
                n_input,
                node_id,
            ),
            compute_node(
                e2,
                shared_map,
                shared_size,
                node_mem,
                reset_conditions,
                n_input,
                node_id,
            ),
            compute_node(
                e3,
                shared_map,
                shared_size,
                node_mem,
                reset_conditions,
                n_input,
                node_id,
            ),
        )),
        ExprOperation::Reg(s, e) => {
            let new_expr = if let Some(e) = e {
                compute_node(
                    e,
                    shared_map,
                    shared_size,
                    node_mem,
                    reset_conditions,
                    n_input,
                    node_id,
                )
            } else {
                RCell::new(Node::TmpValueHolder(usize::MAX))
            };
            RCell::new(Node::Reg(s, new_expr))
        }
        ExprOperation::Ram(e1, e2, e3, e4) => RCell::new(Node::Ram(
            compute_node(
                e1,
                shared_map,
                shared_size,
                node_mem,
                reset_conditions,
                n_input,
                node_id,
            ),
            compute_node(
                e2,
                shared_map,
                shared_size,
                node_mem,
                reset_conditions,
                n_input,
                node_id,
            ),
            compute_node(
                e3,
                shared_map,
                shared_size,
                node_mem,
                reset_conditions,
                n_input,
                node_id,
            ),
            compute_node(
                e4,
                shared_map,
                shared_size,
                node_mem,
                reset_conditions,
                n_input,
                node_id,
            ),
        )),
        ExprOperation::Rom(s, e) => RCell::new(Node::Rom(
            s,
            compute_node(
                e,
                shared_map,
                shared_size,
                node_mem,
                reset_conditions,
                n_input,
                node_id,
            ),
        )),
        ExprOperation::Last(i) => {
            let inside_node = if let Some(n) = &reset_conditions[node_id] {
                Node::Mux(
                    RCell::new(n.clone()),
                    RCell::new(Node::Const(shared_size[i].clone())),
                    RCell::new(Node::TmpValueHolder(i)),
                )
            } else {
                Node::TmpValueHolder(i)
            };
            RCell::new(Node::Reg(shared_size[i].len(), RCell::new(inside_node)))
        }
    };
    node_mem.insert(expr_node, ret.clone());
    ret
}

fn add_init_values(
    shared_map: &mut HashMap<usize, RCell<Node>>,
    shared_size: &Vec<Vec<bool>>,
    n_nodes: usize,
    init_node: &RCell<Node>,
) {
    for (i, n) in shared_map.iter_mut() {
        if *i >= n_nodes {
            continue;
        }
        if shared_size[*i]
            .iter()
            .fold(false, |prev, next| prev || *next)
        {
            *n = RCell::new(Node::Mux(
                init_node.clone(),
                RCell::new(Node::Reg(1, n.clone())),
                RCell::new(Node::Const(shared_size[*i].clone())),
            ));
        } else {
            *n = RCell::new(Node::Reg(1, n.clone()));
        }
    }
}

fn remove_tmp_value(shared_map: &mut HashMap<usize, RCell<Node>>, shared_size: &Vec<Vec<bool>>) {
    let mut tmp_values = Vec::new();
    for (_, node) in shared_map.iter() {
        fetch_tmp_values(node.clone(), &mut tmp_values)
    }
    for val in tmp_values.drain(..) {
        let i = if let Node::TmpValueHolder(i) = &*val.borrow() {
            *i
        } else {
            continue;
        };
        *val.borrow_mut() = shared_map
            .get(&i)
            .unwrap_or(&RCell::new(Node::Const(shared_size[i].clone())))
            .clone()
            .borrow()
            .clone();
    }
}

fn fetch_tmp_values(node: RCell<Node>, tmp_values: &mut Vec<RCell<Node>>) {
    match &*node.borrow() {
        Node::Input(_) | Node::Const(_) => {}
        Node::Not(e) => fetch_tmp_values(e.clone(), tmp_values),
        Node::Slice(e, _, _) => fetch_tmp_values(e.clone(), tmp_values),
        Node::BiOp(_, e1, e2) => {
            fetch_tmp_values(e1.clone(), tmp_values);
            fetch_tmp_values(e2.clone(), tmp_values)
        }
        Node::Mux(e1, e2, e3) => {
            fetch_tmp_values(e1.clone(), tmp_values);
            fetch_tmp_values(e2.clone(), tmp_values);
            fetch_tmp_values(e3.clone(), tmp_values)
        }
        Node::Reg(_, e) => fetch_tmp_values(e.clone(), tmp_values),
        Node::Ram(e1, e2, e3, e4) => {
            fetch_tmp_values(e1.clone(), tmp_values);
            fetch_tmp_values(e2.clone(), tmp_values);
            fetch_tmp_values(e3.clone(), tmp_values);
            fetch_tmp_values(e4.clone(), tmp_values)
        }
        Node::Rom(_, e) => fetch_tmp_values(e.clone(), tmp_values),
        Node::TmpValueHolder(_) => tmp_values.push(node.clone()),
    }
}
