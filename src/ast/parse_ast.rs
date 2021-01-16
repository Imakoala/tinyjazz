/*
This is the syntax tree to be used by the parser
and the first steps of compilation. It is quite close
to the tinyjazz syntaxe.
*/

use core::panic;
use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use ahash::AHashMap;

//Binary Operation are the same for every ast, so they are imported
pub use crate::ast::BiOp;

//struct for storing the a position
//the tuple is file_id, left index, right index
pub type Pos = (usize, usize, usize);

//A wrapper to include position information in the tree
//It implements deref for easier use
//It means that we can call methods directly on the "value" field
//of the struct, without deconstructiong it.
//More precisely, if we try to call a method that does no exist
//on the loc struct, the compiler will try to call it
//on the value field instead. It makes it way easier to use.
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
    pub imports: Vec<Import>,                   //all imported files
    pub automata: AHashMap<String, Automaton>,  //all the automata ordered by name
    pub functions: AHashMap<String, Function>,  //all the functions ordered by name
    pub global_consts: AHashMap<String, Const>, //the global constants
}
pub type Import = PathBuf; //an import is just a Path

#[derive(Debug, Clone)]
pub struct Automaton {
    pub name: String,
    pub inputs: Vec<Arg>,
    pub outputs: Vec<Arg>,
    pub shared: Vec<VarAssign>, //Variables shared across states and automata must be declared
    pub states: AHashMap<String, State>,
    pub init_states: Vec<Loc<Var>>,
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

//A call to an extenrla automaton
#[derive(Debug, Clone)]
pub struct ExtAutomaton {
    pub inputs: Loc<Vec<Loc<Expr>>>,
    pub outputs: Loc<Vec<Loc<Var>>>,
    pub name: Loc<Var>,
}

#[derive(Debug, Clone)]
pub struct State {
    pub name: Loc<String>,
    pub statements: Vec<Statement>,
    pub transitions: Vec<Transition>,
    pub weak: bool,
}
#[derive(Debug, Clone)]
pub struct Transition {
    pub condition: Loc<TrCond>,
    pub state: Loc<Option<Var>>,
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
    ExtAutomaton(ExtAutomaton),
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
    Slice(Box<Loc<Expr>>, Option<Const>, Option<Const>),
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
