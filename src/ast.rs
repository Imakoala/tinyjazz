pub use std::collections::HashMap;
use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
};

//filed_id, left index, right index
pub type Pos = (usize, usize, usize);

//A wrapper to include position information in the tree
//It implements deref for easier use
#[derive(Debug, Clone)]
pub struct Loc<T> {
    pub loc: Pos,
    pub value: T,
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

//A module is basiaclly a group of automata, taking some input and ouputs
#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub inputs: Vec<Arg>,
    pub outputs: Vec<Arg>,
    pub shared: Vec<VarAssign>, //Variables shared across nodes and automata must be declared
    pub extern_modules: Vec<Loc<ExtModule>>, //You can call another module, the inpits must be shared variables and the output are automatically shared
    pub automata: Vec<Automaton>,
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

pub type Automaton = HashMap<String, Loc<Node>>;

#[derive(Debug, Clone)]
pub struct Node {
    pub statements: Vec<Statement>,
    pub transitions: Vec<(Loc<Expr>, Loc<Var>, bool)>,
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
    Unknown(bool, Const), //Const bits initialized to 0
}
#[derive(Debug, Clone)]
pub enum Expr {
    Const(ConstExpr),
    Not(Box<Expr>),
    Slice(Box<Loc<Expr>>, Const, Const),
    BiOp(BiOp, Box<Loc<Expr>>, Box<Loc<Expr>>),
    Mux(Box<Loc<Expr>>, Box<Loc<Expr>>, Box<Loc<Expr>>),
    Var(Loc<Var>),
    Reg(Box<Expr>),
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
    pub addr_size: Const,
    pub word_size: Const,
    pub read_addr: Box<Loc<Expr>>,
    pub write_enable: Box<Loc<Expr>>,
    pub write_addr: Box<Loc<Expr>>,
    pub write_data: Box<Loc<Expr>>,
}
#[derive(Debug, Clone)]
pub struct RomStruct {
    pub addr_size: Const,
    pub word_size: Const,
    pub read_addr: Box<Loc<Expr>>,
}

#[derive(Debug, Clone)]
pub struct IfStruct {
    pub condition: Const,
    pub if_block: Vec<Statement>,
    pub else_block: Vec<Statement>,
}
#[derive(Debug, Clone)]
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
}

//A variable in which the size must be specified (in arguments or return vars)
#[derive(Debug, Clone)]
pub struct Arg {
    pub name: String,
    pub size: Const,
}
#[derive(Debug, Clone)]
pub struct Function {
    pub name: Loc<String>,
    pub static_args: Vec<String>,
    pub args: Vec<Arg>,
    pub return_vars: Vec<Arg>,
    pub statements: Vec<Statement>,
}
