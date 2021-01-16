pub use crate::ast::BiOp;
use std::{cell::Cell, hash::Hash};
use std::{hash::Hasher, rc::Rc};

//This is a node. It represents an operation. It can have an id, or not.
//It it used in hashtables quite a lot, so it stores its own hash.
//Without the hash storage, we get a x5 slowdown.
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
//All the available operations
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ExprOperation {
    Input(usize),
    Const(Vec<bool>),
    Not(Rc<ExprNode>),
    Slice(Rc<ExprNode>, usize, usize),
    BiOp(BiOp, Rc<ExprNode>, Rc<ExprNode>),
    Mux(Rc<ExprNode>, Rc<ExprNode>, Rc<ExprNode>),
    Reg(usize, Option<Rc<ExprNode>>), //size, state. None means a reference to itself, as we cannot have cycles here.
    Ram(Rc<ExprNode>, Rc<ExprNode>, Rc<ExprNode>, Rc<ExprNode>),
    Rom(usize, Rc<ExprNode>),
    Last(usize),
}
//the default value
impl Default for ExprOperation {
    fn default() -> Self {
        Self::Const(vec![])
    }
}

//This is one state from the automaton
#[derive(Debug, Clone)]
pub struct ProgramState {
    pub shared_outputs: Vec<(usize, Rc<ExprNode>)>, //All the shared variables assigned to by the state
    //The transitions
    pub transition_outputs: Vec<(Option<usize>, Rc<ExprNode>, bool)>, //state_id, var, reset
    pub inputs: Vec<usize>, //All the shared variables used in the state (not in transitions)
    pub weak: bool,         //unused
    pub n_vars: usize,      //maximum node id used
}
#[derive(Debug, Clone)]
pub struct ProgramGraph {
    pub init_states: Vec<usize>,
    pub states: Vec<ProgramState>,
    pub shared: Vec<Vec<bool>>, //init value of each shared variable
    pub schedule: Vec<usize>, //At some point the nodes were scheduled. It is no longer the case, so unused
    pub outputs: Vec<(String, usize)>,
    pub inputs: Vec<usize>,
}
