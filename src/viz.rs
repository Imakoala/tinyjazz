use std::{collections::HashMap, io::Write, sync::Arc};

use crate::optimization::*;

type Nd = usize;
type Ed<'a> = &'a (usize, usize, String);
struct Graph {
    nodes: Vec<String>,
    edges: Vec<(usize, usize, String)>,
}

pub fn render(prog: &ProgramGraph) {
    use std::fs::File;
    let mut f = File::create("viz.dot").unwrap();
    render_prog_to(&mut f, prog);
    for (i, n) in prog.nodes.iter().enumerate() {
        let mut f = File::create(format!("viz_node{}.dot", i)).unwrap();
        render_node_to(&mut f, n);
    }
}

pub fn render_prog_to<W: Write>(output: &mut W, prog: &ProgramGraph) {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    for (i, node) in prog.nodes.iter().enumerate() {
        nodes.push(i.to_string());
        for (k, (j, _, _)) in node.transition_outputs.iter().enumerate() {
            if let Some(j) = j {
                edges.push((i, *j, format!("t{}", k)))
            }
        }
    }

    let graph = Graph { nodes, edges };

    dot::render(&graph, output).unwrap()
}

pub fn render_node_to<W: Write>(output: &mut W, node: &ProgramNode) {
    let mut nodes_mem = HashMap::new();
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    for (k, (_, e, _)) in node.transition_outputs.iter().enumerate() {
        nodes.push(format!("t{}", k));
        edges.push((
            nodes.len() - 1,
            mem(nodes.len(), e.id, &mut nodes_mem),
            String::new(),
        ));
        render_rec(e.clone(), &mut nodes, &mut edges, &mut nodes_mem);
    }
    for (i, e) in node.shared_outputs.iter() {
        nodes.push(format!("Output {}", i));
        edges.push((
            nodes.len() - 1,
            mem(nodes.len(), e.id, &mut nodes_mem),
            String::new(),
        ));
        render_rec(e.clone(), &mut nodes, &mut edges, &mut nodes_mem);
    }

    let graph = Graph { nodes, edges };

    dot::render(&graph, output).unwrap()
}

fn render_rec(
    e: Arc<ExprNode>,
    nodes: &mut Vec<String>,
    edges: &mut Vec<(usize, usize, String)>,
    nodes_mem: &mut HashMap<usize, usize>,
) {
    if let Some(id) = e.id {
        if nodes_mem.contains_key(&id) {
            return;
        }
        nodes_mem.insert(id, nodes.len());
    }
    match &e.op {
        ExprOperation::Input(i) => nodes.push(format!("Input : {}", i)),
        ExprOperation::Const(c) => nodes.push(format!("Const : {:?}", c)),
        ExprOperation::Not(e) => {
            nodes.push(format!("Not"));
            edges.push((
                nodes.len() - 1,
                mem(nodes.len(), e.id, nodes_mem),
                String::new(),
            ));
            render_rec(e.clone(), nodes, edges, nodes_mem)
        }
        ExprOperation::Slice(e, c1, c2) => {
            nodes.push(format!("Slice {} {}", c1, c2));
            edges.push((
                nodes.len() - 1,
                mem(nodes.len(), e.id, nodes_mem),
                String::new(),
            ));
            render_rec(e.clone(), nodes, edges, nodes_mem)
        }
        ExprOperation::BiOp(op, e1, e2) => {
            nodes.push(format!("{:?}", op));
            let id = nodes.len() - 1;
            edges.push((
                nodes.len() - 1,
                mem(nodes.len(), e1.id, nodes_mem),
                String::new(),
            ));
            render_rec(e1.clone(), nodes, edges, nodes_mem);
            edges.push((id, mem(nodes.len(), e2.id, nodes_mem), String::new()));
            render_rec(e2.clone(), nodes, edges, nodes_mem);
        }
        ExprOperation::Mux(e1, e2, e3) => {
            nodes.push(format!("Mux"));
            let id = nodes.len() - 1;
            edges.push((
                nodes.len() - 1,
                mem(nodes.len(), e1.id, nodes_mem),
                String::new(),
            ));
            render_rec(e1.clone(), nodes, edges, nodes_mem);
            edges.push((id, mem(nodes.len(), e2.id, nodes_mem), String::new()));
            render_rec(e2.clone(), nodes, edges, nodes_mem);
            edges.push((id, mem(nodes.len(), e3.id, nodes_mem), String::new()));
            render_rec(e3.clone(), nodes, edges, nodes_mem);
        }
        ExprOperation::Reg(_, e) => {
            if e.is_none() {
                edges.push((nodes.len() - 1, nodes.len() - 1, String::new()));
            } else {
                nodes.push(format!("Reg"));
                edges.push((
                    nodes.len() - 1,
                    mem(nodes.len(), e.as_ref().unwrap().id, nodes_mem),
                    String::new(),
                ));
                render_rec(e.clone().unwrap(), nodes, edges, nodes_mem);
            }
        }
        ExprOperation::Ram(e1, e2, e3, e4) => {
            nodes.push(format!("Ram"));
            let id = nodes.len() - 1;
            edges.push((
                nodes.len() - 1,
                mem(nodes.len(), e1.id, nodes_mem),
                String::new(),
            ));
            render_rec(e1.clone(), nodes, edges, nodes_mem);
            edges.push((id, mem(nodes.len(), e2.id, nodes_mem), String::new()));
            render_rec(e2.clone(), nodes, edges, nodes_mem);
            edges.push((id, mem(nodes.len(), e3.id, nodes_mem), String::new()));
            render_rec(e3.clone(), nodes, edges, nodes_mem);
            edges.push((id, mem(nodes.len(), e4.id, nodes_mem), String::new()));
            render_rec(e4.clone(), nodes, edges, nodes_mem);
        }
        ExprOperation::Rom(e) => {
            nodes.push(format!("Rom"));
            edges.push((
                nodes.len() - 1,
                mem(nodes.len(), e.id, nodes_mem),
                String::new(),
            ));
            render_rec(e.clone(), nodes, edges, nodes_mem)
        }
        ExprOperation::Last(i) => {
            nodes.push(format!("Last : {}", i));
        }
    }
}
fn mem(i: usize, id: Option<usize>, nodes_mem: &mut HashMap<usize, usize>) -> usize {
    if let Some(id) = id {
        if let Some(n_id) = nodes_mem.get(&id) {
            return *n_id;
        }
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
