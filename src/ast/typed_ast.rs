pub use crate::ast::BiOp;
use ahash::AHashMap;
use std::ops::{Deref, DerefMut};
/*
A simpler, typed ast
*/
//a sized value, useful for typing expr.
//It works just like "Loc" from the previous ast
#[derive(Debug, Clone)]
pub struct Sized<T> {
    pub value: T,
    pub size: usize,
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
#[derive(Debug, Clone)]
pub struct Program {
    pub inputs: Vec<Arg>,
    pub outputs: Vec<Arg>,
    pub shared: AHashMap<SharedVar, Value>,
    pub states: AHashMap<Name, State>,
    pub init_states: Vec<Name>,
}

//Just to be a bit more explicit in the ast, these are all strings
pub type Arg = Sized<String>;
pub type SharedVar = String;
pub type LocalVar = String;
pub type Name = String;

//SharedVar and LocalVars are differenciated
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Var {
    Local(LocalVar),
    Shared(SharedVar),
}
pub type Value = Vec<bool>;

#[derive(Debug, Clone)]
pub struct State {
    pub name: String,
    pub statements: AHashMap<Var, Expr>,
    pub transitions: Vec<(Var, Option<Name>, bool)>,
    pub weak: bool,
}

pub type Expr = Sized<ExprType>;
pub type ExprTerm = Sized<ExprTermType>;

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
    Last(SharedVar),
    Ram(RamStruct),
    Rom(ExprTerm),
}
#[derive(Debug, Clone)]
pub struct RamStruct {
    pub read_addr: ExprTerm,
    pub write_enable: ExprTerm,
    pub write_addr: ExprTerm,
    pub write_data: ExprTerm,
}
