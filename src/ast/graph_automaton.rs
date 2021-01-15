pub use crate::ast::BiOp;
use std::{cell::Cell, hash::Hash};
use std::{hash::Hasher, rc::Rc};
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ExprNode {
    pub id: Option<usize>,
    pub op: ExprOperation,
    pub hash: Cell<Option<u64>>,
}

impl Hash for ExprNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let inner_h = self.hash.get();
        if let Some(h) = inner_h {
            h.hash(state)
        } else {
            let mut hasher = ahash::AHasher::default();
            self.op.hash(&mut hasher);
            let h = hasher.finish();
            h.hash(state);
            self.hash.set(Some(h));
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ExprOperation {
    Input(usize),
    Const(Vec<bool>),
    Not(Rc<ExprNode>),
    Slice(Rc<ExprNode>, usize, usize),
    BiOp(BiOp, Rc<ExprNode>, Rc<ExprNode>),
    Mux(Rc<ExprNode>, Rc<ExprNode>, Rc<ExprNode>),
    Reg(usize, Option<Rc<ExprNode>>), //size, node. None means a reference to itself.
    Ram(Rc<ExprNode>, Rc<ExprNode>, Rc<ExprNode>, Rc<ExprNode>),
    Rom(usize, Rc<ExprNode>),
    Last(usize),
}
impl Default for ExprOperation {
    fn default() -> Self {
        Self::Const(vec![])
    }
}
#[derive(Debug, Clone)]
pub struct ProgramNode {
    pub shared_outputs: Vec<(usize, Rc<ExprNode>)>,
    pub transition_outputs: Vec<(Option<usize>, Rc<ExprNode>, bool)>, //node_id, var, reset
    pub inputs: Vec<usize>, //does not include the inputs in transitions
    pub weak: bool,
    pub n_vars: usize,
}
#[derive(Debug, Clone)]
pub struct ProgramGraph {
    pub init_nodes: Vec<usize>,
    pub nodes: Vec<ProgramNode>,
    pub shared: Vec<Vec<bool>>, //size of each shared variable
    pub schedule: Vec<usize>,
    pub outputs: Vec<(String, usize)>,
    pub inputs: Vec<usize>,
}
