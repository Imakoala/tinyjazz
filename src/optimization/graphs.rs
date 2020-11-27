use std::sync::Arc;

use crate::ast::BiOp;

pub struct ExprNode {
    pub id: usize,
    pub op: ExprOperation,
}

pub enum ExprOperation {
    Input(usize),
    Const(Vec<bool>),
    Not(Arc<ExprNode>),
    Slice(Arc<ExprNode>, usize, usize),
    BiOp(BiOp, Arc<ExprNode>, Arc<ExprNode>),
    Mux(Arc<ExprNode>, Arc<ExprNode>, Arc<ExprNode>),
    Reg(bool, usize),
    Ram(Arc<ExprNode>, Arc<ExprNode>, Arc<ExprNode>, Arc<ExprNode>),
    Rom(Arc<ExprNode>),
}

pub struct ProgramNode {
    pub shared_outputs: Vec<(usize, Arc<ExprNode>)>,
    pub transition_outputs: Vec<(usize, Arc<ExprNode>, bool)>, //node_id, var, reset
    pub reg_outputs: Vec<Arc<ExprNode>>,
    pub inputs: Vec<usize>,
    pub weak: bool,
}

pub struct Program {
    pub init_nodes: Vec<usize>,
    pub nodes: Vec<ProgramNode>,
    pub n_shared: usize, //number of shared variable. They are numbered in order.
}
