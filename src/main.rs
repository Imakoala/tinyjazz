extern crate lalrpop_util;

extern crate global_counter;

mod ast;
mod compute_consts;
mod errors;
mod expand_fn;
mod flatten;
mod parser_wrapper;
mod typed_ast;
mod typing;

use compute_consts::compute_consts;
use docopt::Docopt;
use errors::TinyjazzError;
use expand_fn::expand_functions;
use flatten::flatten;
use parser_wrapper::parse;
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf, process::exit};
use typed_ast::*;
use typing::type_prog;

//Docopt generates a CLI automatically from this usage string. Pretty amazing.
const USAGE: &'static str = include_str!("USAGE.docopt");

#[derive(Debug, Deserialize)]
struct Args {
    arg_file: String,
    flag_version: bool,
}

fn process_file(path: PathBuf) -> Result<Program, TinyjazzError> {
    let (mut prog, files) = parse(path)?;
    compute_consts(&mut prog).map_err(|e| (e, files.clone()))?;
    flatten(&mut prog);
    let mut type_map = HashMap::new();
    expand_functions(&mut prog, &mut type_map).map_err(|e| (e, files.clone()))?;
    prog.functions = HashMap::new(); //the functions are no longer useful
                                     //at this point, the ast is ready to be typed.
    let prog = type_prog(prog, type_map).map_err(|e| (e, files.clone()))?;
    Ok(prog)
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    if args.flag_version {
        println!("tinyjazz version 0.0.1");
        return;
    }
    let prog_result = process_file(args.arg_file.into());
    match prog_result {
        Err(err) => {
            err.print().unwrap();
            exit(1)
        }
        Ok(prog) => {
            println!(
                "{:#?}",
                prog.get("mod").unwrap().automata[0]
                    .get("n")
                    .unwrap()
                    .statements
                    .len()
            );
        }
    }
}
