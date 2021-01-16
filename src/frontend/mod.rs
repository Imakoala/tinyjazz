/*
The files in this folder are called in the following order :
-parser_wrapper/parser.lalrpop (don't look at parser.rs, it is auto-generated)
-constants.rs
-hierarchical_automata.rs
-nested_expr.rs
-functions.rs (these and hierarchical automata will be swapped at some point,
as it is necessary to make automata in functions work)
-typing.rs
-make_graph_automaton.rs (this file will be removed and integrated in the next one someday)
(optional : from_netlist/parser.lalrpop)
-automaton.rs


At the beginning of each file, there is a description of what it does.
(schedule.rs is unused)
*/

pub(crate) mod automaton;
pub(crate) mod constants;
pub(crate) mod from_netlist;
pub(crate) mod functions;
pub(crate) mod hierarchical_automata;
pub(crate) mod make_graph_automaton;
pub(crate) mod nested_expr;
pub(crate) mod parser_wrapper;
pub(crate) mod typing;
