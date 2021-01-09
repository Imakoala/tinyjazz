pub use crate::ast::BiOp;
use std::{cell::RefCell, hash::Hash};
use std::{hash::Hasher, sync::Arc};
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ExprNode {
    pub id: Option<usize>,
    pub op: ExprOperation,
    pub hash: RefCell<Option<u64>>,
}

impl Hash for ExprNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let cloned_h = self.hash.borrow().clone();
        if let Some(h) = cloned_h {
            h.hash(state)
        } else {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            self.op.hash(&mut hasher);
            let h = hasher.finish();
            h.hash(state);
            *self.hash.borrow_mut() = Some(h)
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ExprOperation {
    Input(usize),
    Const(Vec<bool>),
    Not(Arc<ExprNode>),
    Slice(Arc<ExprNode>, usize, usize),
    BiOp(BiOp, Arc<ExprNode>, Arc<ExprNode>),
    Mux(Arc<ExprNode>, Arc<ExprNode>, Arc<ExprNode>),
    Reg(usize, Option<Arc<ExprNode>>), //size, node. None means a reference to itself.
    Ram(Arc<ExprNode>, Arc<ExprNode>, Arc<ExprNode>, Arc<ExprNode>),
    Rom(usize, Arc<ExprNode>),
    Last(usize),
}
impl Default for ExprOperation {
    fn default() -> Self {
        Self::Const(vec![])
    }
}
#[derive(Debug, Clone)]
pub struct ProgramNode {
    pub shared_outputs: Vec<(usize, Arc<ExprNode>)>,
    pub transition_outputs: Vec<(Option<usize>, Arc<ExprNode>, bool)>, //node_id, var, reset
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
