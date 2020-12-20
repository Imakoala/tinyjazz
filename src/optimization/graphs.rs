use std::sync::Arc;

use crate::ast::BiOp;
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ExprNode {
    pub id: Option<usize>,
    pub op: ExprOperation,
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
    Rom(Arc<ExprNode>),
}
#[derive(Debug, Clone)]
pub struct ProgramNode {
    pub shared_outputs: Vec<(usize, Arc<ExprNode>)>,
    pub transition_outputs: Vec<(usize, Arc<ExprNode>, bool)>, //node_id, var, reset
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
