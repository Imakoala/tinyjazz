pub use std::collections::HashMap;
use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
};

#[derive(Debug, Clone)]
pub struct Loc<T> {
    pub loc: (usize, usize),
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
    pub imports: Vec<Import>,                 //all imported files
    pub modules: HashMap<String, Module>,     //all the modules ordered by name
    pub functions: HashMap<String, Function>, //all the functions ordered by name
    pub global_consts: HashMap<String, Const>,
}
pub type Import = PathBuf; //an import is just a Path
#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub inputs: Vec<Arg>,
    pub outputs: Vec<Arg>,
    pub shared: Vec<VarAssign>,
    pub extern_modules: Vec<Loc<ExtModule>>,
    pub automatons: Vec<Automaton>,
}

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
    pub transitions: Vec<(Loc<Expr>, Loc<Var>)>,
}

pub type Statement = Vec<VarAssign>;
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
    If(IfStruct),
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
#[derive(Debug, Clone)]
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
