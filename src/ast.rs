use core::panic;
pub use std::collections::HashMap;
use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

//filed_id, left index, right index
pub type Pos = (usize, usize, usize);

//A wrapper to include position information in the tree
//It implements deref for easier use
#[derive(Debug, Clone, Eq)]
pub struct Loc<T> {
    pub loc: Pos,
    pub value: T,
}

impl<T> Loc<T> {
    pub fn new(loc: Pos, value: T) -> Self {
        Loc { value, loc }
    }
}

impl<T: PartialEq> PartialEq for Loc<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Hash> Hash for Loc<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

impl<T> Deref for Loc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> DerefMut for Loc<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}
//the main program
#[derive(Debug, Clone)]
pub struct Program {
    pub imports: Vec<Import>,                  //all imported files
    pub modules: HashMap<String, Module>,      //all the modules ordered by name
    pub functions: HashMap<String, Function>,  //all the functions ordered by name
    pub global_consts: HashMap<String, Const>, //the global constants
}
pub type Import = PathBuf; //an import is just a Path

//A module is basically a group of automata, taking some input and ouputs
#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub inputs: Vec<Arg>,
    pub outputs: Vec<Arg>,
    pub shared: Vec<VarAssign>, //Variables shared across nodes and automata must be declared
    pub nodes: HashMap<String, Node>,
    pub init_nodes: Vec<Loc<Var>>,
}

//A variable assignement
#[derive(Debug, Clone)]
pub struct VarAssign {
    pub var: Loc<Var>,
    pub expr: Loc<Expr>,
}
pub type Var = String;

#[derive(Debug, Clone)]
pub struct ConstVarAssign {
    pub var: Var,
    pub cons: Const,
}

#[derive(Debug, Clone)]
pub struct Value(Vec<bool>);

//A call to an extenrla module
#[derive(Debug, Clone)]
pub struct ExtModule {
    pub inputs: Loc<Vec<Loc<Var>>>,
    pub outputs: Loc<Vec<Loc<Var>>>,
    pub name: Loc<Var>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: Loc<String>,
    pub extern_modules: Vec<ExtModule>,
    pub statements: Vec<Statement>,
    pub transitions: Vec<Transition>,
    pub weak: bool,
}
#[derive(Debug, Clone)]
pub struct Transition {
    pub condition: Loc<TrCond>,
    pub node: Loc<Option<Var>>,
    pub reset: bool,
}
#[derive(Debug, Clone)]
pub enum TrCond {
    Default,
    Expr(Expr),
}

impl TrCond {
    pub fn is_default(&self) -> bool {
        if let TrCond::Default = self {
            true
        } else {
            false
        }
    }
    pub fn is_expr(&self) -> bool {
        if let TrCond::Default = self {
            false
        } else {
            true
        }
    }
    pub fn unwrap(self) -> Expr {
        if let TrCond::Expr(e) = self {
            e
        } else {
            panic!("Unwrap TrCond on Default value")
        }
    }
    pub fn unwrap_ref(&self) -> &Expr {
        if let TrCond::Expr(e) = self {
            e
        } else {
            panic!("Unwrap TrCond on Default value")
        }
    }
}
//a statement can be either (tuple) = (tuple), var = expr, or (tuple) = function call
#[derive(Debug, Clone)]
pub enum Statement {
    Assign(Vec<VarAssign>),
    If(IfStruct),
    FnAssign(FnAssign),
}

#[derive(Debug, Clone)]
pub struct FnAssign {
    pub vars: Vec<Loc<Var>>,
    pub f: FnCall,
}

//a constant expression, either [0; n] which mean a bus of size n initialized to 0, or a vector of bits
#[derive(Debug, Clone)]
pub enum ConstExpr {
    Known(Vec<bool>),
    Unknown(bool, Loc<Const>), //Const bits initialized to 0
}
#[derive(Debug, Clone)]
pub enum Expr {
    Const(ConstExpr),
    Not(Box<Expr>),
    Slice(Box<Loc<Expr>>, Const, Const),
    BiOp(BiOp, Box<Loc<Expr>>, Box<Loc<Expr>>),
    Mux(Box<Loc<Expr>>, Box<Loc<Expr>>, Box<Loc<Expr>>),
    Var(Loc<Var>),
    Last(Loc<Var>),
    Reg(Loc<Const>, Box<Loc<Expr>>),
    Ram(RamStruct),
    Rom(RomStruct),
    FnCall(FnCall),
}
#[derive(Debug, Clone)]
pub struct FnCall {
    pub name: Loc<String>,
    pub args: Loc<Vec<Loc<Expr>>>,
    pub static_args: Loc<Vec<Const>>,
}
#[derive(Debug, Clone)]
pub struct RamStruct {
    pub read_addr: Box<Loc<Expr>>,
    pub write_enable: Box<Loc<Expr>>,
    pub write_addr: Box<Loc<Expr>>,
    pub write_data: Box<Loc<Expr>>,
}
#[derive(Debug, Clone)]
pub struct RomStruct {
    pub word_size: Const,
    pub read_addr: Box<Loc<Expr>>,
}

#[derive(Debug, Clone)]
pub struct IfStruct {
    pub condition: Const,
    pub if_block: Vec<Statement>,
    pub else_block: Vec<Statement>,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum BiOp {
    And,  //*
    Or,   //+
    Xor,  //^
    Nand, //-*
    Concat,
}
#[derive(Debug, Clone)]
pub enum Const {
    Value(i32),
    BiOp(ConstBiOp, Box<Const>, Box<Loc<Const>>),
    Var(Loc<String>),
}
#[derive(Debug, Clone, PartialEq)]
pub enum ConstBiOp {
    Plus,
    Times,
    Minus,
    Div,
    Le,
    Lt,
    Ge,
    Gt,
    Eq,
    Neq,
    And,
    Or,
}

//A variable in which the size must be specified (in arguments or return vars)
#[derive(Debug, Clone)]
pub struct Arg {
    pub name: String,
    pub size: Loc<Const>,
}
#[derive(Debug, Clone)]
pub struct Function {
    pub name: Loc<String>,
    pub static_args: Vec<String>,
    pub args: Vec<Arg>,
    pub return_vars: Vec<Arg>,
    pub statements: Vec<Statement>,
}
