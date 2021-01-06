use core::panic;
use std::{collections::HashMap, io::Write};

use crate::ast::graph::{FlatProgramGraph, Node, RCell};

type Nd = usize;
type Ed<'a> = &'a (usize, usize, String);
struct Graph {
    nodes: Vec<String>,
    edges: Vec<(usize, usize, String)>,
}

pub fn render(prog: &FlatProgramGraph) {
    use std::fs::File;
    let mut f = File::create("viz.dot").unwrap();
    render_prog_to(&mut f, prog);
}

pub fn render_prog_to<W: Write>(output: &mut W, node: &FlatProgramGraph) {
    let mut nodes_mem = HashMap::new();
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    for (s, e) in node.outputs.iter() {
        nodes.push(format!("Output {}", s));
        edges.push((
            nodes.len() - 1,
            mem(nodes.len(), e.clone(), &mut nodes_mem),
            String::new(),
        ));
        render_rec(e.clone(), &mut nodes, &mut edges, &mut nodes_mem);
    }

    let graph = Graph { nodes, edges };

    dot::render(&graph, output).unwrap()
}

fn render_rec(
    e: RCell<Node>,
    nodes: &mut Vec<String>,
    edges: &mut Vec<(usize, usize, String)>,
    nodes_mem: &mut HashMap<RCell<Node>, usize>,
) {
    if nodes_mem.contains_key(&e) {
        return;
    }
    nodes_mem.insert(e.clone(), nodes.len());
    match &*e.borrow() {
        Node::Input(i) => nodes.push(format!("Input : {}", i)),
        Node::Const(c) => nodes.push(format!(
            "{}",
            c.iter()
                .map(|b| if *b { "1" } else { "0" })
                .collect::<String>()
        )),
        Node::Not(e) => {
            nodes.push(format!("Not"));
            edges.push((
                nodes.len() - 1,
                mem(nodes.len(), e.clone(), nodes_mem),
                String::new(),
            ));
            render_rec(e.clone(), nodes, edges, nodes_mem)
        }
        Node::Slice(e, c1, c2) => {
            nodes.push(format!("Slice {} {}", c1, c2));
            edges.push((
                nodes.len() - 1,
                mem(nodes.len(), e.clone(), nodes_mem),
                String::new(),
            ));
            render_rec(e.clone(), nodes, edges, nodes_mem)
        }
        Node::BiOp(op, e1, e2) => {
            nodes.push(format!("{:?}", op));
            let id = nodes.len() - 1;
            edges.push((
                nodes.len() - 1,
                mem(nodes.len(), e1.clone(), nodes_mem),
                String::new(),
            ));
            render_rec(e1.clone(), nodes, edges, nodes_mem);
            edges.push((id, mem(nodes.len(), e2.clone(), nodes_mem), String::new()));
            render_rec(e2.clone(), nodes, edges, nodes_mem);
        }
        Node::Mux(e1, e2, e3) => {
            nodes.push(format!("Mux"));
            let id = nodes.len() - 1;
            edges.push((
                nodes.len() - 1,
                mem(nodes.len(), e1.clone(), nodes_mem),
                "c".to_string(),
            ));
            render_rec(e1.clone(), nodes, edges, nodes_mem);
            edges.push((id, mem(nodes.len(), e2.clone(), nodes_mem), "v".to_string()));
            render_rec(e2.clone(), nodes, edges, nodes_mem);
            edges.push((id, mem(nodes.len(), e3.clone(), nodes_mem), "f".to_string()));
            render_rec(e3.clone(), nodes, edges, nodes_mem);
        }
        Node::Reg(_, e) => {
            nodes.push(format!("Reg"));
            edges.push((
                nodes.len() - 1,
                mem(nodes.len(), e.clone(), nodes_mem),
                String::new(),
            ));
            render_rec(e.clone(), nodes, edges, nodes_mem);
        }
        Node::Ram(e1, e2, e3, e4) => {
            nodes.push(format!("Ram"));
            let id = nodes.len() - 1;
            edges.push((
                nodes.len() - 1,
                mem(nodes.len(), e1.clone(), nodes_mem),
                String::new(),
            ));
            render_rec(e1.clone(), nodes, edges, nodes_mem);
            edges.push((id, mem(nodes.len(), e2.clone(), nodes_mem), String::new()));
            render_rec(e2.clone(), nodes, edges, nodes_mem);
            edges.push((id, mem(nodes.len(), e3.clone(), nodes_mem), String::new()));
            render_rec(e3.clone(), nodes, edges, nodes_mem);
            edges.push((id, mem(nodes.len(), e4.clone(), nodes_mem), String::new()));
            render_rec(e4.clone(), nodes, edges, nodes_mem);
        }
        Node::Rom(e) => {
            nodes.push(format!("Rom"));
            edges.push((
                nodes.len() - 1,
                mem(nodes.len(), e.clone(), nodes_mem),
                String::new(),
            ));
            render_rec(e.clone(), nodes, edges, nodes_mem)
        }
        Node::TmpValueHolder(_) => panic!("Should not happen : temp value in viz"),
    }
}

fn mem(i: usize, e: RCell<Node>, nodes_mem: &mut HashMap<RCell<Node>, usize>) -> usize {
    if let Some(n_id) = nodes_mem.get(&e) {
        return *n_id;
    }
    i
}

impl<'a> dot::Labeller<'a, Nd, Ed<'a>> for Graph {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("example2").unwrap()
    }
    fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", n)).unwrap()
    }
    fn node_label<'b>(&'b self, n: &Nd) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr(self.nodes[*n].clone().into())
    }
    fn edge_label<'b>(&'b self, e: &Ed) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr(format!("{}", e.2).into())
    }
}

impl<'a> dot::GraphWalk<'a, Nd, Ed<'a>> for Graph {
    fn nodes(&self) -> dot::Nodes<'a, Nd> {
        (0..self.nodes.len()).collect()
    }
    fn edges(&'a self) -> dot::Edges<'a, Ed<'a>> {
        self.edges.iter().collect()
    }
    fn source(&self, e: &Ed) -> Nd {
        e.0
    }
    fn target(&self, e: &Ed) -> Nd {
        e.1
    }
}
