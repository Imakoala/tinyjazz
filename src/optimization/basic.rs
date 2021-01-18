/*
This file contains optimisations such as "0 and a -> 0", for every optimisation.
It also simplifies bus operations : two following slices can be replaces with a single one.
*/

use ahash::AHashSet;

use crate::ast::graph::*;

pub fn optimize(prog: &mut FlatProgramGraph) {
    let mut changed = true;
    while changed {
        changed = false;
        let mut mem = AHashSet::new();
        for (_name, node) in &prog.outputs {
            try_compute(node.clone(), &mut mem, &mut changed);
        }
    }
}

fn try_compute(
    node: RCell<Node>,
    mem: &mut AHashSet<RCell<Node>>,
    changed: &mut bool,
) -> Option<Vec<bool>> {
    if mem.contains(&node) {
        return None;
    }
    mem.insert(node.clone());
    let mut to_change = None;
    let cloned_node = { node.borrow().clone() };
    let ret = match cloned_node {
        Node::Input(_) => None,
        Node::Const(c) => return Some(c),
        Node::Not(e) => {
            //if e is constant, then not e is too
            if let Some(mut c) = try_compute(e.clone(), mem, changed) {
                for b in &mut c {
                    *b = !*b;
                }
                Some(c)
            } else {
                //Not and -> nand
                if let Node::BiOp(BiOp::And, in_e1, in_e2) = e.borrow().clone() {
                    to_change = Some(RCell::new(Node::BiOp(BiOp::Nand, in_e1, in_e2)))
                }
                None
            }
        }
        Node::Slice(e, c1, c2) => {
            if let Some(c) = try_compute(e.clone(), mem, changed) {
                //if e is constant, propagates
                Some(c[c1..c2].into())
            } else {
                //chaned slice
                if let Node::Slice(in_e, in_c1, _) = e.borrow().clone() {
                    to_change = Some(RCell::new(Node::Slice(in_e, in_c1 + c1, in_c1 + c2)))
                }
                None
            }
        }
        Node::BiOp(op, e1, e2) => {
            let (v1, v2) = (
                try_compute(e1.clone(), mem, changed),
                try_compute(e2.clone(), mem, changed),
            );
            match op {
                //concat : simplify is both members are constant, or if one member has length 0
                BiOp::Concat => match (v1.clone(), v2.clone()) {
                    (None, None) => None,
                    (None, Some(v)) | (Some(v), None) => {
                        if v.len() == 0 {
                            to_change = Some(if v1.is_none() { e1 } else { e2 });
                        }
                        None
                    }
                    (Some(mut v1), Some(mut v2)) => {
                        v1.append(&mut v2);
                        Some(v1)
                    }
                },
                //and : if one member is all 1s or all 0s
                BiOp::And => match (v1.clone(), v2.clone()) {
                    (None, None) => None,
                    (None, Some(v)) | (Some(v), None) => {
                        if !v.iter().fold(false, |p, b| p || *b) {
                            Some(vec![false; v.len()])
                        } else if v.iter().fold(true, |p, b| p && *b) {
                            to_change = Some(if v1.is_none() { e1 } else { e2 });
                            None
                        } else {
                            None
                        }
                    }
                    (Some(v1), Some(v2)) => Some(
                        v1.iter()
                            .zip(v2.iter())
                            .map(|(b1, b2)| *b1 && *b2)
                            .collect(),
                    ),
                },
                //same
                BiOp::Or => match (v1.clone(), v2.clone()) {
                    (None, None) => None,
                    (None, Some(v)) | (Some(v), None) => {
                        if !v.iter().fold(false, |p, b| p || *b) {
                            to_change = Some(if v1.is_none() { e1 } else { e2 });
                            None
                        } else if v.iter().fold(true, |p, b| p && *b) {
                            Some(vec![true; v.len()])
                        } else {
                            None
                        }
                    }
                    (Some(v1), Some(v2)) => Some(
                        v1.iter()
                            .zip(v2.iter())
                            .map(|(b1, b2)| *b1 || *b2)
                            .collect(),
                    ),
                },
                //only simplifies if both members are constant
                BiOp::Xor => match (v1.clone(), v2.clone()) {
                    (None, None) => None,
                    (None, Some(v)) | (Some(v), None) => {
                        if !v.iter().fold(false, |p, b| p || *b) {
                            to_change = Some(if v1.is_none() { e1 } else { e2 });
                            None
                        } else {
                            None
                        }
                    }
                    (Some(v1), Some(v2)) => {
                        Some(v1.iter().zip(v2.iter()).map(|(b1, b2)| *b1 ^ *b2).collect())
                    }
                },
                //if one is all false or all true, same as with and but with a not
                BiOp::Nand => match (v1.clone(), v2) {
                    (None, None) => None,
                    (None, Some(v)) | (Some(v), None) => {
                        if !v.iter().fold(false, |p, b| p || *b) {
                            Some(vec![true; v.len()])
                        } else if v.iter().fold(true, |p, b| p && *b) {
                            to_change =
                                Some(RCell::new(Node::Not(if v1.is_none() { e1 } else { e2 })));
                            None
                        } else {
                            None
                        }
                    }
                    (Some(v1), Some(v2)) => {
                        Some(v1.iter().zip(v2.iter()).map(|(b1, b2)| *b1 ^ *b2).collect())
                    }
                },
            }
        }
        //Can be simplified if the condition is constant, or if one result is constant and of size 1
        //I consider than a Or + Not is better than a Mux
        Node::Mux(e1, e2, e3) => {
            if let Some(v) = try_compute(e1.clone(), mem, changed) {
                if v[0] {
                    to_change = Some(e2.clone())
                } else {
                    to_change = Some(e3.clone())
                }
            };
            let (v2, v3) = (
                try_compute(e2.clone(), mem, changed),
                try_compute(e3.clone(), mem, changed),
            );

            match (v2.clone(), v3.clone()) {
                (None, None) => None,
                (None, Some(v)) | (Some(v), None) => {
                    if v.len() == 1 && !v[0] {
                        let cond = if v3.is_none() {
                            RCell::new(Node::Not(e1))
                        } else {
                            e1
                        };
                        to_change = Some(RCell::new(Node::BiOp(
                            BiOp::And,
                            cond,
                            if v2.is_none() { e2 } else { e3 },
                        )))
                    } else if v.len() == 1 && v[0] {
                        let cond = if v2.is_none() {
                            RCell::new(Node::Not(e1))
                        } else {
                            e1
                        };
                        to_change = Some(RCell::new(Node::BiOp(
                            BiOp::Or,
                            cond,
                            if v2.is_none() { e2 } else { e3 },
                        )))
                    }
                    None
                }
                (Some(v2), Some(v3)) => {
                    if v2 == v3 {
                        Some(v2)
                    } else {
                        if v2.len() == 1 {
                            if v2[0] {
                                to_change = Some(RCell::new(Node::BiOp(BiOp::Or, e1, e3)));
                            } else {
                                to_change = Some(RCell::new(Node::BiOp(BiOp::And, e1, e3)));
                            }
                        }
                        None
                    }
                }
            }
        }
        //reg of something all false is always false
        Node::Reg(_, e) => {
            if let Some(v) = try_compute(e, mem, changed) {
                if !v.iter().fold(false, |prev, b| prev || *b) {
                    Some(v)
                } else {
                    None
                }
            } else {
                None
            }
        }
        Node::Ram(_, _, _, _) => None,
        Node::Rom(_, _) => None,
        Node::TmpValueHolder(_) => None,
    };
    if let Some(v) = &ret {
        *node.borrow_mut() = Node::Const(v.clone());
        *changed = true;
    } else if let Some(e) = to_change {
        *node.borrow_mut() = e.borrow().clone();
        *changed = true;
    }
    ret
}
