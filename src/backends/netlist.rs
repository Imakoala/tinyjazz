use crate::ast::graph::*;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io::Write,
};

pub fn to_netlist(source: &FlatProgramGraph, mut dest: impl Write) -> Result<(), std::io::Error> {
    writeln!(
        dest,
        "INPUT {}",
        (0..source.inputs.len()).format_with(", ", |elt, f| f(&format_args!("i_{}", elt)))
    )?;
    writeln!(
        dest,
        "OUTPUT {}",
        source
            .outputs
            .iter()
            .format_with(", ", |elt, f| f(&format_args!("o_{}", elt.0)))
    )?;
    write!(dest, "VAR ")?;
    let mut first = true;
    for (_, n) in source.outputs.iter() {
        write_vars(n, &mut dest, first, &mut HashMap::new(), &source.inputs)?;
        first = false;
    }
    write!(dest, "\nIN\n")?;
    for (s, n) in &source.outputs {
        write_instr(n, &mut dest, &mut HashSet::new())?;
        write!(dest, "o_{} = ", s)?;
        write_var_name(n, &mut dest)?;
        write!(dest, "\n")?;
    }
    Ok(())
}

fn write_vars(
    node: &RCell<Node>,
    dest: &mut impl Write,
    first: bool,
    mem: &mut HashMap<u32, usize>,
    input_sizes: &Vec<usize>,
) -> Result<usize, std::io::Error> {
    if let Some(i) = mem.get(&node.id()) {
        return Ok(*i);
    }
    mem.insert(node.id(), 0);
    let size = match node.borrow().clone() {
        Node::Input(i) => input_sizes[i],
        Node::Const(c) => c.len(),
        Node::Not(e) => write_vars(&e, dest, false, mem, input_sizes)?,
        Node::Slice(e, c1, c2) => {
            write_vars(&e, dest, false, mem, input_sizes)?;
            c2 - c1
        }
        Node::BiOp(_, e1, e2) => {
            write_vars(&e1, dest, false, mem, input_sizes)?;
            write_vars(&e2, dest, false, mem, input_sizes)?
        }
        Node::Mux(e1, e2, e3) => {
            write_vars(&e1, dest, false, mem, input_sizes)?;
            write_vars(&e2, dest, false, mem, input_sizes)?;
            write_vars(&e3, dest, false, mem, input_sizes)?
        }
        Node::Reg(s, e) => {
            mem.insert(node.id(), s);
            write_vars(&e, dest, false, mem, input_sizes)?;
            s
        }
        Node::Ram(e1, e2, e3, e4) => {
            write_vars(&e1, dest, false, mem, input_sizes)?;
            write_vars(&e2, dest, false, mem, input_sizes)?;
            write_vars(&e3, dest, false, mem, input_sizes)?;
            write_vars(&e4, dest, false, mem, input_sizes)?
        }
        Node::Rom(s, e) => {
            write_vars(&e, dest, false, mem, input_sizes)?;
            s
        }
        Node::TmpValueHolder(_) => 0,
    };
    write!(dest, "v_{}", node.id())?;
    if size != 1 {
        write!(dest, " : {}", size)?;
    }
    if !first {
        write!(dest, ", ")?;
    }
    mem.insert(node.id(), size);
    Ok(size)
}

fn write_instr(
    node: &RCell<Node>,
    dest: &mut impl Write,
    mem: &mut HashSet<u32>,
) -> Result<(), std::io::Error> {
    if mem.contains(&node.id()) {
        return Ok(());
    }
    mem.insert(node.id());
    match node.borrow().clone() {
        Node::Input(_) => {}
        Node::Const(v) => {
            write!(dest, "v_{} = ", node.id())?;
            write!(
                dest,
                "{}",
                v.iter()
                    .format_with(" ", |elt, f| f(if *elt { &"1" } else { &"0" }))
            )?
        }
        Node::Not(e) => {
            write_instr(&e, dest, mem)?;
            write!(dest, "v_{} = ", node.id())?;
            write!(dest, "NOT ")?;
            write_var_name(&e, dest)?;
        }
        Node::Slice(e, c1, c2) => {
            write_instr(&e, dest, mem)?;
            write!(dest, "v_{} = ", node.id())?;
            if c2 - c1 != 1 {
                write!(dest, "SLICE {} {} ", c1, c2)?;
            } else {
                write!(dest, "SELECT {} ", c1)?;
            }
            write_var_name(&e, dest)?;
        }
        Node::BiOp(op, e1, e2) => {
            write_instr(&e1, dest, mem)?;
            write_instr(&e2, dest, mem)?;
            write!(dest, "v_{} = ", node.id())?;
            write_op(op, dest)?;
            write_var_name(&e1, dest)?;
            write_var_name(&e2, dest)?;
        }
        Node::Mux(e1, e2, e3) => {
            write_instr(&e1, dest, mem)?;
            write_instr(&e2, dest, mem)?;
            write_instr(&e3, dest, mem)?;
            write!(dest, "v_{} = ", node.id())?;
            write!(dest, "MUX ")?;
            write_var_name(&e1, dest)?;
            write_var_name(&e2, dest)?;
            write_var_name(&e3, dest)?;
        }
        Node::Reg(_, e) => {
            write_instr(&e, dest, mem)?;
            write!(dest, "v_{} = ", node.id())?;
            write!(dest, "REG ")?;
            write_var_name(&e, dest)?;
        }
        Node::Ram(e1, e2, e3, e4) => {
            write_instr(&e1, dest, mem)?;
            write_instr(&e2, dest, mem)?;
            write_instr(&e3, dest, mem)?;
            write_instr(&e4, dest, mem)?;
            write!(dest, "v_{} = ", node.id())?;
            write!(dest, "RAM ")?;
            write_var_name(&e1, dest)?;
            write_var_name(&e2, dest)?;
            write_var_name(&e3, dest)?;
            write_var_name(&e4, dest)?;
        }
        Node::Rom(_, e) => {
            write_instr(&e, dest, mem)?;
            write!(dest, "v_{} = ", node.id())?;
            write!(dest, "ROM ")?;
            write_var_name(&e, dest)?;
        }
        Node::TmpValueHolder(_) => {
            panic!("Should not happen: temp value in codegen")
        }
    };
    write!(dest, "\n")?;
    Ok(())
}

fn write_var_name(node: &RCell<Node>, dest: &mut impl Write) -> Result<(), std::io::Error> {
    match node.borrow().clone() {
        Node::Input(i) => write!(dest, "i_{} ", i),
        _ => write!(dest, "v_{} ", node.id()),
    }
}

fn write_op(op: BiOp, dest: &mut impl Write) -> Result<(), std::io::Error> {
    match op {
        BiOp::And => {
            write!(dest, "AND ")
        }
        BiOp::Or => {
            write!(dest, "OR ")
        }
        BiOp::Xor => {
            write!(dest, "XOR ")
        }
        BiOp::Nand => {
            write!(dest, "NAND ")
        }
        BiOp::Concat => {
            write!(dest, "CONCAT ")
        }
    }
}
