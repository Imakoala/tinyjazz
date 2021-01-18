mod parser;
mod parse_ast {
    use std::collections::{HashMap, HashSet};

    pub use crate::ast::BiOp;
    pub struct Netlist<'a> {
        pub inputs: HashSet<&'a str>,
        pub outputs: HashSet<&'a str>,
        pub vars: HashMap<&'a str, usize>,
        pub instr: HashMap<&'a str, Expr<'a>>,
    }
    pub enum Expr<'a> {
        Const(Vec<bool>),
        Not(&'a str),
        Reg(&'a str),
        Var(&'a str),
        BiOp(BiOp, &'a str, &'a str),
        Mux(&'a str, &'a str, &'a str),
        Ram(&'a str, &'a str, &'a str, &'a str),
        Rom(&'a str),
        Slice(&'a str, usize, usize),
    }
}

use std::{collections::HashMap, fs::read_to_string};

use crate::ast::graph::*;
use parse_ast::*;

use parser::ProgramParser;
//parse the file and convert it into a flatprogramgraph
pub fn from_netlist(path: &str) -> FlatProgramGraph {
    let file = read_to_string(path.clone()).unwrap();
    let netlist = ProgramParser::new().parse(&file).unwrap();
    let mut mem = HashMap::<&str, RCell<Node>>::new();
    let in_map: HashMap<&str, usize> = netlist
        .inputs
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, i))
        .collect();
    let inputs = netlist
        .inputs
        .iter()
        .map(|v| *netlist.vars.get(v).unwrap())
        .collect();
    let mut defer = Vec::new();
    let outputs = netlist
        .outputs
        .iter()
        .map(|v| {
            (
                v.to_string(),
                node_from_var(v, &netlist, &mut mem, &in_map, &mut defer),
            )
        })
        .collect();
    while !defer.is_empty() {
        let cur_defer = defer;
        defer = Vec::new();
        for (v, node) in cur_defer {
            let n = node_from_var(v, &netlist, &mut mem, &in_map, &mut defer);
            *node.borrow_mut() = Node::Reg(*netlist.vars.get(v).unwrap(), n);
        }
    }
    FlatProgramGraph { inputs, outputs }
}

fn node_from_var<'a>(
    var: &'a str,
    netlist: &Netlist<'a>,
    mem: &mut HashMap<&'a str, RCell<Node>>,
    in_map: &HashMap<&str, usize>,
    defer: &mut Vec<(&'a str, RCell<Node>)>,
) -> RCell<Node> {
    if let Some(n) = mem.get(var) {
        return n.clone();
    }
    if let Some(i) = in_map.get(var) {
        let node = RCell::new(Node::Input(*i));
        mem.insert(var, node.clone());
        return node;
    }
    let node = match netlist.instr.get(var).unwrap() {
        Expr::Var(v) => node_from_var(*v, netlist, mem, in_map, defer),
        Expr::Const(c) => RCell::new(Node::Const(c.clone())),
        Expr::Not(v) => RCell::new(Node::Not(node_from_var(*v, netlist, mem, in_map, defer))),
        Expr::Reg(v) => {
            let res = RCell::new(Node::TmpValueHolder(defer.len()));
            defer.push((v, res.clone()));
            res
        }
        Expr::Rom(v) => RCell::new(Node::Rom(
            *netlist.vars.get(v).unwrap(),
            node_from_var(*v, netlist, mem, in_map, defer),
        )),
        Expr::BiOp(op, v1, v2) => RCell::new(Node::BiOp(
            op.clone(),
            node_from_var(*v1, netlist, mem, in_map, defer),
            node_from_var(*v2, netlist, mem, in_map, defer),
        )),
        Expr::Mux(v1, v2, v3) => RCell::new(Node::Mux(
            node_from_var(*v1, netlist, mem, in_map, defer),
            node_from_var(*v2, netlist, mem, in_map, defer),
            node_from_var(*v3, netlist, mem, in_map, defer),
        )),
        Expr::Ram(v1, v2, v3, v4) => RCell::new(Node::Ram(
            node_from_var(*v1, netlist, mem, in_map, defer),
            node_from_var(*v2, netlist, mem, in_map, defer),
            node_from_var(*v3, netlist, mem, in_map, defer),
            node_from_var(*v4, netlist, mem, in_map, defer),
        )),
        Expr::Slice(v, c1, c2) => RCell::new(Node::Slice(
            node_from_var(*v, netlist, mem, in_map, defer),
            *c1,
            *c2,
        )),
    };
    mem.insert(var, node.clone());
    node
}
