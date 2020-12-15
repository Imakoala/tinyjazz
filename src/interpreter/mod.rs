use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
//TODO: add instant shared vars
//TODO: nodes as shared vars
use scripting::get_inputs_closure;

use crate::{ast::BiOp, optimization::*};
mod scripting;
pub struct InterpreterIterator<'a> {
    graph: &'a ProgramGraph,
    shared: Vec<Vec<bool>>,
    reg_map: Vec<HashMap<Arc<ExprNode>, Vec<bool>>>,
    next_reg_map: Vec<HashMap<Arc<ExprNode>, Vec<bool>>>,
    to_run: Vec<usize>,
    ram: Arc<Mutex<HashMap<Vec<bool>, Vec<bool>>>>,
    nodes_mem: Vec<Vec<Vec<bool>>>,
    inputs: Box<dyn FnMut() -> Vec<Vec<bool>>>,
}

impl<'a> Iterator for InterpreterIterator<'a> {
    type Item = Vec<(&'a String, Vec<bool>)>;
    fn next(self: &mut Self) -> Option<Vec<(&'a String, Vec<bool>)>> {
        program_step(self);
        std::mem::swap(&mut self.next_reg_map, &mut self.reg_map);
        Some(
            self.graph
                .outputs
                .iter()
                .map(|(s, i)| (s, self.shared[*i].clone()))
                .collect(),
        )
    }
}

pub fn interprete<'a>(
    graph: &'a ProgramGraph,
    inputs_script_path: Option<String>,
) -> InterpreterIterator {
    let to_run = graph.init_nodes.clone();
    let shared = graph.shared.clone();
    let inputs = get_inputs_closure(inputs_script_path, graph.inputs.clone());
    let reg_map = vec![HashMap::new(); graph.nodes.len()];
    let next_reg_map = vec![HashMap::new(); graph.nodes.len()];
    let ram = Arc::new(Mutex::new(HashMap::new()));
    let nodes_mem = graph
        .nodes
        .iter()
        .map(|n| vec![Vec::new(); n.n_vars])
        .collect();
    InterpreterIterator {
        graph,
        shared,
        reg_map,
        next_reg_map,
        to_run,
        ram,
        nodes_mem,
        inputs,
    }
}

fn program_step(interpreter_state: &mut InterpreterIterator) {
    let graph = interpreter_state.graph;
    let shared = &mut interpreter_state.shared;
    let reg_map = &mut interpreter_state.reg_map;
    let next_reg_map = &mut interpreter_state.next_reg_map;
    let to_run = &mut interpreter_state.to_run;
    let ram = interpreter_state.ram.clone();
    let nodes_mem = &mut interpreter_state.nodes_mem;
    let inputs = &mut interpreter_state.inputs;
    let nodes_to_run = to_run
        .drain(..)
        .map(|i| (i, &graph.nodes[i]))
        .collect::<Vec<(usize, &ProgramNode)>>();
    let new_shared = nodes_to_run
        .iter()
        .map(|(i, node)| node.shared_outputs.iter().map(move |o| (i, o)))
        .flatten();
    for (node_id, (u, n)) in new_shared {
        let value = calc_node(
            n.clone(),
            shared,
            &reg_map[*node_id],
            &mut next_reg_map[*node_id],
            &mut nodes_mem[*node_id],
            ram.clone(),
            None,
        );
        shared[*u] = value
    }
    for (i, v) in inputs().drain(..).enumerate() {
        shared[i] = v
    }
    let mut next_map = vec![false; graph.nodes.len()];
    let next_nodes = nodes_to_run
        .iter()
        .map(|(i, node)| node.transition_outputs.iter().map(move |o| (i, o)))
        .flatten()
        .filter_map(|(node_id, (u, n, b))| {
            let v = calc_node(
                n.clone(),
                shared,
                &reg_map[*node_id],
                &mut next_reg_map[*node_id],
                &mut nodes_mem[*node_id],
                ram.clone(),
                None,
            );
            if v[0] && !next_map[*u] {
                if *b {
                    reg_map[*node_id] = HashMap::new();
                }
                next_map[*u] = true;
                Some(*u)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();
    *to_run = next_nodes;
    //reset the node memoisation
    for n in nodes_mem {
        for v in n {
            v.clear()
        }
    }
}

fn calc_node(
    node: Arc<ExprNode>,
    shared: &Vec<Vec<bool>>,
    reg_map: &HashMap<Arc<ExprNode>, Vec<bool>>,
    next_reg_map: &mut HashMap<Arc<ExprNode>, Vec<bool>>,
    node_mem: &mut Vec<Vec<bool>>,
    ram: Arc<Mutex<HashMap<Vec<bool>, Vec<bool>>>>,
    current_reg: Option<&Vec<bool>>,
) -> Vec<bool> {
    if let Some(id) = node.id {
        if node_mem[id].len() > 0 {
            return node_mem[id].clone();
        }
    }

    match &node.op {
        ExprOperation::Input(i) => shared[*i].clone(),
        ExprOperation::Const(c) => c.clone(),
        ExprOperation::Not(nd) => {
            let mut v = calc_node(
                nd.clone(),
                shared,
                reg_map,
                next_reg_map,
                node_mem,
                ram,
                current_reg,
            );
            for b in &mut v {
                *b = !*b;
            }
            v
        }
        ExprOperation::Slice(nd, i1, i2) => {
            let v = calc_node(
                nd.clone(),
                shared,
                reg_map,
                next_reg_map,
                node_mem,
                ram,
                current_reg,
            );
            v[*i1..*i2].to_vec()
        }
        ExprOperation::BiOp(op, n1, n2) => {
            let mut v1 = calc_node(
                n1.clone(),
                shared,
                reg_map,
                next_reg_map,
                node_mem,
                ram.clone(),
                current_reg,
            );
            let v2 = calc_node(
                n2.clone(),
                shared,
                reg_map,
                next_reg_map,
                node_mem,
                ram,
                current_reg,
            );
            apply_op(op.clone(), &mut v1, v2);
            v1
        }
        ExprOperation::Mux(n1, n2, n3) => {
            let v1 = calc_node(
                n1.clone(),
                shared,
                reg_map,
                next_reg_map,
                node_mem,
                ram.clone(),
                current_reg,
            );
            if v1[0] {
                calc_node(
                    n2.clone(),
                    shared,
                    reg_map,
                    next_reg_map,
                    node_mem,
                    ram.clone(),
                    current_reg,
                )
            } else {
                calc_node(
                    n3.clone(),
                    shared,
                    reg_map,
                    next_reg_map,
                    node_mem,
                    ram,
                    current_reg,
                )
            }
        }
        ExprOperation::Reg(size, nopt) => {
            if let Some(n) = nopt {
                let v = reg_map.get(n).unwrap_or(&vec![false; *size]).clone();
                let v_next = calc_node(
                    n.clone(),
                    shared,
                    reg_map,
                    next_reg_map,
                    node_mem,
                    ram.clone(),
                    Some(&v),
                );
                next_reg_map.insert(n.clone(), v_next);
                v
            } else {
                current_reg
                    .expect("Should not happen: expected a nested reg")
                    .clone()
            }
        }
        ExprOperation::Ram(n1, n2, n3, n4) => {
            let v1 = calc_node(
                n1.clone(),
                shared,
                reg_map,
                next_reg_map,
                node_mem,
                ram.clone(),
                current_reg,
            );
            let v2 = calc_node(
                n2.clone(),
                shared,
                reg_map,
                next_reg_map,
                node_mem,
                ram.clone(),
                current_reg,
            );
            let v4 = calc_node(
                n4.clone(),
                shared,
                reg_map,
                next_reg_map,
                node_mem,
                ram.clone(),
                current_reg,
            );
            let ret = if let Some(value) = ram.lock().unwrap().get(&v1) {
                value.clone()
            } else {
                vec![false; v4.len()]
            };
            if v2[0] {
                let v3 = calc_node(
                    n3.clone(),
                    shared,
                    reg_map,
                    next_reg_map,
                    node_mem,
                    ram.clone(),
                    current_reg,
                );
                ram.lock().unwrap().insert(v3, v4);
            }
            ret
        }
        ExprOperation::Rom(_) => todo!(),
    }
}

fn apply_op(op: BiOp, v1: &mut Vec<bool>, mut v2: Vec<bool>) {
    match op {
        BiOp::And => {
            for i in 0..v1.len() {
                v1[i] = v1[i] && v2[i]
            }
        }
        BiOp::Or => {
            for i in 0..v1.len() {
                v1[i] = v1[i] || v2[i]
            }
        }
        BiOp::Xor => {
            for i in 0..v1.len() {
                v1[i] = v1[i] ^ v2[i]
            }
        }
        BiOp::Nand => {
            for i in 0..v1.len() {
                v1[i] = !(v1[i] && v2[i])
            }
        }
        BiOp::Concat => v1.append(&mut v2),
    }
}
