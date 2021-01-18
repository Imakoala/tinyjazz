//Module declaration
mod ast;
mod backends;
mod frontend;
mod interpreter;
mod optimization;
mod test;
mod util;
//The standard hashmap is cryptographically secure.
//I use a faster, non-crypto one.
use ahash::AHashMap;
use ast::graph::FlatProgramGraph;
use docopt::Docopt;
use serde::Deserialize;
use std::{path::PathBuf, process::exit};

//Docopt generates a CLI automatically from this usage string. Pretty amazing.
const USAGE: &'static str = include_str!("USAGE.docopt");
//Docopt will generate this struct from the CLI
#[derive(Debug, Deserialize)]
struct Args {
    arg_file: String,
    flag_version: bool,
    flag_dot: bool,
    flag_print: bool,
    flag_i: Option<String>,
    flag_s: Option<usize>,
    flag_netlist: bool,
    flag_o: usize,
}

fn process_file(path: PathBuf) -> Result<FlatProgramGraph, util::errors::TinyjazzError> {
    let (mut prog, files) = frontend::parser_wrapper::parse(path)?;
    frontend::constants::compute_consts(&mut prog).map_err(|e| (e, files.clone()))?;
    frontend::hierarchical_automata::collapse_automata(&mut prog)
        .map_err(|e| (e, files.clone()))?; //this is just error handling
    frontend::nested_expr::flatten(&mut prog);
    //a map the keep the input and output types of function,
    //even when they are inlined
    let mut type_map = AHashMap::new();
    frontend::functions::expand_functions(&mut prog, &mut type_map)
        .map_err(|e| (e, files.clone()))?;
    let prog = frontend::typing::type_prog(prog, type_map).map_err(|e| (e, files.clone()))?;
    let graph = frontend::make_graph_automaton::make_graph(&prog);
    let graph = frontend::automaton::flatten_automata(&graph);
    Ok(graph)
}
fn main() {
    //gets the args from docopt
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    //prints the version
    if args.flag_version {
        println!("tinyjazz version 0.0.1");
        return;
    }
    //compute the intermediate representation from either the netlist,
    //or the .tj file, depending on the arguments
    let mut flat_prog = if args.flag_netlist {
        frontend::from_netlist::from_netlist(&*args.arg_file)
    } else {
        process_file(args.arg_file.into()).unwrap_or_else(|err| {
            err.print().unwrap();
            exit(1)
        })
    };
    //optimises it if necessary
    if args.flag_o >= 1 {
        optimization::basic::optimize(&mut flat_prog);
    }
    //write the output to "out.net"
    let file = std::fs::File::create("out.net").unwrap();
    backends::netlist::to_netlist(&flat_prog, file).unwrap();
    //print it if necessary
    if args.flag_print {
        println!("{:#?}", flat_prog)
    }
    //output the .dot visualisation if necessary
    if args.flag_dot {
        util::viz::render(&flat_prog);
    }
    //interprete the file for <steps> steps
    if let Some(steps) = args.flag_s {
        for outputs in interpreter::interprete(&flat_prog, args.flag_i).take(steps) {
            println!(
                "{:?}",
                outputs
                    .into_iter()
                    .map(|(s, v)| (
                        s,
                        v.into_iter()
                            .map(|b| if b { 1 } else { 0 })
                            .collect::<Vec<u32>>()
                    ))
                    .collect::<Vec<(&String, Vec<u32>)>>()
            );
        }
    }
}
