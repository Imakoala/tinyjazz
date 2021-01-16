/*
The ast are used in the following order : parse_ast -> typed_ast -> graph_automaton -> graph
*/

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
