pub use std::collections::HashMap;
use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
};
/*
A simpler, typed ast
*/
//a sized value, useful for typing expr.
#[derive(Debug, Clone)]
pub struct Sized<T> {
    value: T,
    size: usize,
}

impl<T> Deref for Sized<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> DerefMut for Sized<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}
//the main program
pub type Program = HashMap<String, Module>;
#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub inputs: Vec<Var>,
    pub outputs: Vec<Var>,
    pub shared: HashMap<Var, Value>,
    pub extern_modules: Vec<ExtModule>,
    pub automata: Vec<Automaton>,
}

pub type Var = String;

pub type Value = Vec<bool>;

#[derive(Debug, Clone)]
pub struct ExtModule {
    pub inputs: Vec<Var>,
    pub outputs: Vec<Var>,
    pub name: Var,
}

pub type Automaton = HashMap<Var, Node>;

#[derive(Debug, Clone)]
pub struct Node {
    pub statements: Vec<Statement>,
    pub transitions: Vec<(Var, Var, bool)>,
}
#[derive(Debug, Clone)]
pub struct Statement {
    var: Var,
    statement: Expr,
}
type Expr = Sized<ExprType>;
type ExprTerm = Sized<ExprTermType>;

//a terminal value for an expression.
#[derive(Debug, Clone)]
pub enum ExprTermType {
    Const(Value),
    Var(Var),
}

//Note on how the Expr type is no longer recursive
#[derive(Debug, Clone)]
pub enum ExprType {
    Term(ExprTerm),
    Not(ExprTerm),
    Slice(ExprTerm, usize, usize),
    BiOp(BiOp, ExprTerm, ExprTerm),
    Mux(ExprTerm, ExprTerm, ExprTerm),
    Reg(ExprTerm),
    Ram(RamStruct),
    Rom(RomStruct),
}
#[derive(Debug, Clone)]
pub struct RamStruct {
    pub addr_size: usize,
    pub word_size: usize,
    pub read_addr: ExprTerm,
    pub write_enable: ExprTerm,
    pub write_addr: ExprTerm,
    pub write_data: ExprTerm,
}
#[derive(Debug, Clone)]
pub struct RomStruct {
    pub addr_size: usize,
    pub word_size: usize,
    pub read_addr: ExprTerm,
}
#[derive(Debug, Clone)]
pub enum BiOp {
    And,  //*
    Or,   //+
    Xor,  //^
    Nand, //-*
    Concat,
}
