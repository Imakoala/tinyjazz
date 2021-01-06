pub mod graph;
pub mod graph_automaton;
pub mod parse_ast;
pub mod typed_ast;
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum BiOp {
    And,  //*
    Or,   //+
    Xor,  //^
    Nand, //-*
    Concat,
}
