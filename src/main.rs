extern crate lalrpop_util;

extern crate global_counter;

mod ast;
mod backends;
mod frontend;
mod interpreters;
mod optimization;
mod parser_wrapper;
mod test;
mod util;
use docopt::Docopt;
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf, process::exit};
//Docopt generates a CLI automatically from this usage string. Pretty amazing.
const USAGE: &'static str = include_str!("USAGE.docopt");

#[derive(Debug, Deserialize)]
struct Args {
    arg_file: String,
    flag_version: bool,
    flag_dot: bool,
    flag_print: bool,
    flag_i: Option<String>,
    flag_s: Option<usize>,
    flag_netlist: bool,
}

// fn print_expr(expr: &ast::Expr) -> String {
//     if let ast::Expr::Var(v) = expr {
//         format!("{}", v.value)
//     } else {
//         format!("{:?}", expr)
//     }
// }

// fn print_stat(stat: &ast::Statement) {
//     match stat {
//         ast::Statement::Assign(vec) => {
//             for v in vec {
//                 println!("      {} = {}", v.var.value, print_expr(&v.expr.value));
//             }
//         }
//         ast::Statement::If(a) => {
//             println!("      {:?}", a);
//         }
//         ast::Statement::FnAssign(a) => {
//             println!("      {:?}", a);
//         }
//     }
// }

// pub fn print_prog(prog: &ast::Program) {
//     for (_, modules) in &prog.modules {
//         println!("{} : \n\n", modules.name);
//         for (_, node) in &modules.nodes {
//             println!("  {} : \n\n", node.name.value);
//             for stat in &node.statements {
//                 print_stat(stat)
//             }
//             println!("\n\n  transitions : ");
//             for (expr, a, _b) in &node.transitions {
//                 println!("  |{} -> {}", print_expr(&expr.value), a.value);
//             }
//         }
//     }
// }
//println!("{:#?}", expr);
fn process_file(
    path: PathBuf,
) -> Result<ast::graph_automaton::ProgramGraph, util::errors::TinyjazzError> {
    let (mut prog, files) = parser_wrapper::parse(path)?;
    frontend::constants::compute_consts(&mut prog).map_err(|e| (e, files.clone()))?;
    frontend::hierarchical_automata::collapse_automata(&mut prog)
        .map_err(|e| (e, files.clone()))?;
    frontend::nested_expr::flatten(&mut prog).map_err(|e| (e, files.clone()))?;
    let mut type_map = HashMap::new();
    frontend::functions::expand_functions(&mut prog, &mut type_map)
        .map_err(|e| (e, files.clone()))?;
    prog.functions = HashMap::new(); //the functions are no longer useful
                                     //at this point, the ast is ready to be typed.
    let prog = frontend::typing::type_prog(prog, type_map).map_err(|e| (e, files.clone()))?;
    let graph =
        frontend::make_graph_automaton::make_graph(&prog).map_err(|e| (e, files.clone()))?;
    Ok(graph)
}

fn compile_prog(prog: &ast::graph_automaton::ProgramGraph) -> ast::graph::FlatProgramGraph {
    let graph = frontend::automaton::flatten_automata(&prog);
    graph
}
fn run_interpreter(
    graph: &ast::graph_automaton::ProgramGraph,
    steps: usize,
    input_script_path: Option<String>,
) {
    for outputs in
        interpreters::high_level_interpreter::interprete(graph, input_script_path).take(steps)
    {
        println!("{:?}", outputs);
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    if args.flag_version {
        println!("tinyjazz version 0.0.1");
        return;
    }
    let mut flat_prog = if args.flag_netlist {
        frontend::from_netlist::from_netlist(&*args.arg_file)
    } else {
        let prog_result = process_file(args.arg_file.into());
        if let Err(err) = prog_result {
            err.print().unwrap();
            exit(1)
        }
        let prog = if let Ok(prog) = prog_result {
            prog
        } else {
            panic!()
        };
        let res = compile_prog(&prog);
        if let Some(steps) = args.flag_s {
            run_interpreter(&prog, steps, args.flag_i)
        }
        res
    };
    optimization::basic::optimize(&mut flat_prog);
    let file = std::fs::File::create("out.net").unwrap();
    backends::netlist::to_netlist(&flat_prog, file).unwrap();
    if args.flag_print {
        println!("{:#?}", flat_prog)
    }
    if args.flag_dot {
        util::viz::render(&flat_prog);
    }
}
