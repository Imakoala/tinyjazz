use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use scripting::get_inputs_closure;

use crate::{ast::BiOp, optimization::*};
mod scripting;
pub struct InterpreterIterator<'a> {
    graph: &'a ProgramGraph,
    shared: Vec<Vec<bool>>,
    reg_map: Vec<HashMap<usize, Vec<bool>>>,
    to_run: Vec<usize>,
    ram: Arc<Mutex<HashMap<Vec<bool>, Vec<bool>>>>,
    nodes_mem: Vec<Vec<Vec<bool>>>,
    inputs: Box<dyn FnMut() -> Vec<Vec<bool>>>,
}

impl<'a> Iterator for InterpreterIterator<'a> {
    type Item = Vec<(&'a String, Vec<bool>)>;
    fn next(self: &mut Self) -> Option<Vec<(&'a String, Vec<bool>)>> {
        program_step(
            self.graph,
            &mut self.shared,
            &mut self.reg_map,
            &mut self.to_run,
            self.ram.clone(),
            &mut self.nodes_mem,
            &mut self.inputs,
        );
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
    let reg_map: Vec<HashMap<usize, Vec<bool>>> = graph
        .nodes
        .iter()
        .map(|n| {
            n.reg_outputs
                .iter()
                .filter_map(|(size, expr_node)| {
                    if let Some(id) = expr_node.id {
                        Some((id, vec![false; *size]))
                    } else {
                        None
                    }
                })
                .collect::<HashMap<usize, Vec<bool>>>()
        })
        .collect();
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
        to_run,
        ram,
        nodes_mem,
        inputs,
    }
}

fn program_step(
    graph: &ProgramGraph,
    shared: &mut Vec<Vec<bool>>,
    reg_map: &mut Vec<HashMap<usize, Vec<bool>>>,
    to_run: &mut Vec<usize>,
    ram: Arc<Mutex<HashMap<Vec<bool>, Vec<bool>>>>,
    nodes_mem: &mut Vec<Vec<Vec<bool>>>,
    inputs: &mut Box<dyn FnMut() -> Vec<Vec<bool>>>,
) {
    let nodes_to_run = to_run
        .drain(..)
        .map(|i| (i, &graph.nodes[i]))
        .collect::<Vec<(usize, &ProgramNode)>>();
    let new_shared = nodes_to_run
        .iter()
        .map(|(i, node)| node.shared_outputs.iter().map(move |o| (i, o)))
        .flatten();
    let next_reg = nodes_to_run
        .iter()
        .map(|(i, node)| node.reg_outputs.iter().map(move |o| (i, o)))
        .flatten();
    for (node_id, (_size, node)) in next_reg {
        let value = calc_node(
            node.clone(),
            shared,
            &reg_map[*node_id],
            &mut nodes_mem[*node_id],
            ram.clone(),
        );
        if let Some(id) = node.id {
            reg_map[*node_id].insert(id, value);
        }
    }
    for (node_id, (u, n)) in new_shared {
        let value = calc_node(
            n.clone(),
            shared,
            &reg_map[*node_id],
            &mut nodes_mem[*node_id],
            ram.clone(),
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
                &mut nodes_mem[*node_id],
                ram.clone(),
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
    reg_map: &HashMap<usize, Vec<bool>>,
    node_mem: &mut Vec<Vec<bool>>,
    ram: Arc<Mutex<HashMap<Vec<bool>, Vec<bool>>>>,
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
            let mut v = calc_node(nd.clone(), shared, reg_map, node_mem, ram);
            for b in &mut v {
                *b = !*b;
            }
            v
        }
        ExprOperation::Slice(nd, i1, i2) => {
            let v = calc_node(nd.clone(), shared, reg_map, node_mem, ram);
            v[*i1..*i2].to_vec()
        }
        ExprOperation::BiOp(op, n1, n2) => {
            let mut v1 = calc_node(n1.clone(), shared, reg_map, node_mem, ram.clone());
            let v2 = calc_node(n2.clone(), shared, reg_map, node_mem, ram);
            apply_op(op.clone(), &mut v1, v2);
            v1
        }
        ExprOperation::Mux(n1, n2, n3) => {
            let v1 = calc_node(n1.clone(), shared, reg_map, node_mem, ram.clone());
            if v1[0] {
                calc_node(n2.clone(), shared, reg_map, node_mem, ram.clone())
            } else {
                calc_node(n3.clone(), shared, reg_map, node_mem, ram)
            }
        }
        ExprOperation::Reg(share, var_id, size) => {
            if *share {
                shared[*var_id].clone()
            } else {
                reg_map.get(var_id).unwrap_or(&vec![false; *size]).clone()
            }
        }
        ExprOperation::Ram(n1, n2, n3, n4) => {
            let v1 = calc_node(n1.clone(), shared, reg_map, node_mem, ram.clone());
            let v2 = calc_node(n2.clone(), shared, reg_map, node_mem, ram.clone());
            let v4 = calc_node(n4.clone(), shared, reg_map, node_mem, ram.clone());
            let ret = if let Some(value) = ram.lock().unwrap().get(&v1) {
                value.clone()
            } else {
                vec![false; v4.len()]
            };
            if v2[0] {
                let v3 = calc_node(n3.clone(), shared, reg_map, node_mem, ram.clone());
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
