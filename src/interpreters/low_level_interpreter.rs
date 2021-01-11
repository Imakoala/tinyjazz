use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::ast::graph::*;

pub struct InterpreterIterator<'a> {
    graph: &'a FlatProgramGraph,
    reg_map: HashMap<RCell<Node>, Vec<bool>>,
    next_reg_map: HashMap<RCell<Node>, Vec<bool>>,
    ram: Arc<Mutex<HashMap<Vec<bool>, Vec<bool>>>>,
    mem: HashMap<RCell<Node>, Vec<bool>>,
    inputs: Box<dyn FnMut() -> Vec<Vec<bool>>>,
}

impl<'a> Iterator for InterpreterIterator<'a> {
    type Item = Vec<(&'a String, Vec<bool>)>;
    fn next(self: &mut Self) -> Option<Vec<(&'a String, Vec<bool>)>> {
        self.reg_map = self.next_reg_map.clone();
        let inputs = (self.inputs)();
        self.mem.clear();
        Some(
            self.graph
                .outputs
                .iter()
                .map(|(s, i)| {
                    (
                        s,
                        get_value(
                            i,
                            &self.reg_map,
                            &mut self.next_reg_map,
                            &mut self.mem,
                            &inputs,
                            self.ram.clone(),
                        ),
                    )
                })
                .collect(),
        )
    }
}

pub fn interprete<'a>(
    graph: &'a FlatProgramGraph,
    inputs_script_path: Option<String>,
) -> InterpreterIterator {
    let inputs =
        crate::util::scripting::get_inputs_closure(inputs_script_path, graph.inputs.clone());
    let ram = Arc::new(Mutex::new(HashMap::new()));
    InterpreterIterator {
        graph,
        reg_map: HashMap::new(),
        next_reg_map: HashMap::new(),
        ram,
        mem: HashMap::new(),
        inputs,
    }
}

fn get_value(
    node: &RCell<Node>,
    reg_map: &HashMap<RCell<Node>, Vec<bool>>,
    next_reg_map: &mut HashMap<RCell<Node>, Vec<bool>>,
    mem: &mut HashMap<RCell<Node>, Vec<bool>>,
    inputs: &Vec<Vec<bool>>,
    ram: Arc<Mutex<HashMap<Vec<bool>, Vec<bool>>>>,
) -> Vec<bool> {
    if let Some(v) = mem.get(node) {
        return v.clone();
    }
    let res = match node.borrow().clone() {
        Node::Input(i) => inputs[i].clone(),
        Node::Const(c) => c,
        Node::Not(n) => {
            let mut v = get_value(&n, reg_map, next_reg_map, mem, inputs, ram);
            for b in &mut v {
                *b = !*b
            }
            v
        }
        Node::Slice(n, c1, c2) => {
            let v = get_value(&n, reg_map, next_reg_map, mem, inputs, ram);
            v[c1..c2].into()
        }
        Node::BiOp(op, n1, n2) => {
            let mut v1 = get_value(&n1, reg_map, next_reg_map, mem, inputs, ram.clone());
            let v2 = get_value(&n2, reg_map, next_reg_map, mem, inputs, ram);
            apply_op(op, &mut v1, v2);
            v1
        }
        Node::Mux(n1, n2, n3) => {
            let v1 = get_value(&n1, reg_map, next_reg_map, mem, inputs, ram.clone());
            let v2 = get_value(&n2, reg_map, next_reg_map, mem, inputs, ram.clone());
            let v3 = get_value(&n3, reg_map, next_reg_map, mem, inputs, ram.clone());
            if v1[0] {
                v2
            } else {
                v3
            }
        }
        Node::Reg(s, n) => {
            let prev_v = reg_map.get(&n).unwrap_or(&vec![false; s]).clone();
            mem.insert(node.clone(), prev_v.clone());
            let v = get_value(&n, reg_map, next_reg_map, mem, inputs, ram);
            next_reg_map.insert(n, v);
            prev_v
        }
        Node::Ram(n1, n2, n3, n4) => {
            let read_addr = get_value(&n1, reg_map, next_reg_map, mem, inputs, ram.clone());
            let write_enable = get_value(&n2, reg_map, next_reg_map, mem, inputs, ram.clone());
            let write_addr = get_value(&n3, reg_map, next_reg_map, mem, inputs, ram.clone());
            let write_data = get_value(&n4, reg_map, next_reg_map, mem, inputs, ram.clone());
            let mut ram = ram.lock().unwrap();
            let v = ram
                .get(&read_addr)
                .cloned()
                .unwrap_or(vec![false; write_data.len()]);
            if write_enable[0] {
                ram.insert(write_addr, write_data);
            }
            v
        }
        Node::Rom(_, _) => {
            todo!()
        }
        Node::TmpValueHolder(_) => {
            panic!("Should not happen : tmp value in interpreter")
        }
    };
    mem.insert(node.clone(), res.clone());
    res
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
