use crate::typed_ast::*;
use global_counter::global_counter;
/*
This module collapses external modules.
Basically, it makes new shared variables for the output, and replaces the module call by shared var assignation.
Then it renames every shared var and node in the called module, and copies all the nodes and shared variables in the main module.

This repeats until there are no more external modules.
*/
pub enum CollapseAutomataError {
    CyclicModuleCall(String),
    NoMainModule,
}
global_counter!(INLINE_MODULE_COUNTER, u32, 0);
type Result<T> = std::result::Result<T, CollapseAutomataError>;

pub fn collapse_automata(prog: &mut Program) -> Result<()> {
    loop {
        let mut shared_map = Vec::new();
        let mut node_map = Vec::new();
        let mut changes_ext_mod = Vec::new();
        if let Some(module) = prog.get("main") {
            for (node_name, node) in &module.nodes {
                let mut node_outputs = Vec::new();
                for ext_mod in &node.extern_modules {
                    if ext_mod.name == module.name {
                        return Err(CollapseAutomataError::CyclicModuleCall(module.name.clone()));
                    }
                    let module = prog.get(&ext_mod.name).unwrap();
                    node_outputs.push(inline_module(
                        &ext_mod.inputs,
                        &mut shared_map,
                        &mut node_map,
                        module,
                    ));
                }
                if !node.extern_modules.is_empty() {
                    changes_ext_mod.push((node_name.clone(), node_outputs));
                }
            }
        } else {
            return Err(CollapseAutomataError::NoMainModule);
        }
        let module = prog.get_mut("main").unwrap();
        for (s, value) in shared_map {
            module.shared.insert(s, value);
        }
        for (s, value) in node_map {
            module.nodes.insert(s, value);
        }
        if changes_ext_mod.is_empty() {
            return Ok(());
        }
        for (node_name, mut outputs) in changes_ext_mod {
            let node = module.nodes.get_mut(&node_name).unwrap();
            let init_nodes = &mut module.init_nodes;
            for (var, expr) in node
                .extern_modules
                .drain(..)
                .zip(outputs.drain(..))
                .map(|(mut ext_mod, (mut output, mut init))| {
                    init_nodes.append(&mut init);
                    ext_mod
                        .outputs
                        .drain(..)
                        .zip(output.drain(..))
                        .map(|(out_var, shared_out_var)| {
                            (
                                out_var,
                                Sized {
                                    value: ExprType::Term(Sized {
                                        value: ExprTermType::Var(Var::Shared(shared_out_var.value)),
                                        size: shared_out_var.size,
                                    }),
                                    size: shared_out_var.size,
                                },
                            )
                        })
                        .collect::<Vec<(Var, Expr)>>()
                })
                .flatten()
            {
                node.statements.insert(var, expr);
            }
        }
    }
}

fn get_rename(counter: u32, name: &String, mod_name: &String) -> String {
    format!("in_mod${}${}${}$", name, mod_name, counter)
}

fn inline_module(
    inputs: &Vec<SharedVar>,
    shared_map: &mut Vec<(SharedVar, Value)>,
    node_map: &mut Vec<(Name, Node)>,
    module: &Module,
) -> (Vec<Arg>, Vec<String>) {
    let counter = INLINE_MODULE_COUNTER.get_cloned();
    INLINE_MODULE_COUNTER.inc();
    let mut replace_map = HashMap::new();
    for (name, value) in module.shared.iter() {
        let new_name = get_rename(counter, name, &module.name);
        replace_map.insert(name.clone(), new_name.clone());
        shared_map.push((new_name, value.clone()));
    }
    for (arg, arg_name) in inputs.iter().zip(module.inputs.iter()) {
        replace_map.insert(arg_name.value.clone(), arg.clone());
    }
    let outputs = module
        .outputs
        .iter()
        .map(|s| Sized {
            value: replace_map
                .get(&s.value)
                .expect("Should not happen: output is an unknown var")
                .clone(),
            size: s.size,
        })
        .collect();
    for (_name, node) in module.nodes.iter() {
        let extern_modules = node
            .extern_modules
            .iter()
            .map(|ext_mod| {
                let inputs = ext_mod
                    .inputs
                    .iter()
                    .map(|v| {
                        replace_map
                            .get(v)
                            .expect("Should not happen: output is an unknown var")
                            .clone()
                    })
                    .collect();
                let outputs = ext_mod
                    .outputs
                    .iter()
                    .map(|v| {
                        let s = match v {
                            Var::Local(s) => s,
                            Var::Shared(s) => s,
                        };
                        if let Some(new_v) = replace_map.get(s) {
                            match v {
                                Var::Local(_) => Var::Local(new_v.clone()),
                                Var::Shared(_) => Var::Shared(new_v.clone()),
                            }
                        } else {
                            v.clone()
                        }
                    })
                    .collect();
                ExtModule {
                    inputs,
                    outputs,
                    name: ext_mod.name.clone(),
                }
            })
            .collect();
        let transitions = node
            .transitions
            .iter()
            .map(|(var, name, reset)| match var {
                Var::Shared(s) => {
                    let new_var = replace_map
                        .get(s)
                        .expect("Should not happen : Unknown shared var");
                    (
                        Var::Shared(new_var.clone()),
                        get_rename(counter, name, &module.name),
                        *reset,
                    )
                }
                Var::Local(s) => (
                    Var::Local(s.clone()),
                    get_rename(counter, name, &module.name),
                    *reset,
                ),
            })
            .collect();
        let statements = node
            .statements
            .iter()
            .map(|(var, expr)| {
                let new_var = match var {
                    Var::Shared(s) => {
                        let new_var = replace_map
                            .get(s)
                            .expect("Should not happen : Unknown shared var");
                        Var::Shared(new_var.clone())
                    }
                    Var::Local(_) => var.clone(),
                };
                let new_expr = replace_var_in_expr(expr, &replace_map);
                (new_var, new_expr)
            })
            .collect();
        let new_name = get_rename(counter, &node.name, &module.name);
        node_map.push((
            new_name.clone(),
            Node {
                extern_modules,
                transitions,
                statements,
                weak: node.weak,
                name: new_name,
            },
        ));
    }
    let init_nodes = module
        .init_nodes
        .iter()
        .map(|s| get_rename(counter, s, &module.name))
        .collect();
    (outputs, init_nodes)
}

fn replace_var_in_expr(expr: &Expr, replace_map: &HashMap<String, String>) -> Expr {
    let new_expr = match &expr.value {
        ExprType::Term(t) => ExprType::Term(replace_var_in_expr_term(t, replace_map)),
        ExprType::Not(t) => ExprType::Not(replace_var_in_expr_term(t, replace_map)),
        ExprType::Slice(t, i1, i2) => {
            ExprType::Slice(replace_var_in_expr_term(t, replace_map), *i1, *i2)
        }
        ExprType::BiOp(op, t1, t2) => ExprType::BiOp(
            op.clone(),
            replace_var_in_expr_term(t1, replace_map),
            replace_var_in_expr_term(t2, replace_map),
        ),
        ExprType::Mux(t1, t2, t3) => ExprType::Mux(
            replace_var_in_expr_term(t1, replace_map),
            replace_var_in_expr_term(t2, replace_map),
            replace_var_in_expr_term(t3, replace_map),
        ),
        ExprType::Reg(t) => ExprType::Reg(replace_var_in_expr_term(t, replace_map)),
        ExprType::Ram(RamStruct {
            read_addr: t1,
            write_enable: t2,
            write_addr: t3,
            write_data: t4,
        }) => ExprType::Ram(RamStruct {
            read_addr: replace_var_in_expr_term(t1, replace_map),
            write_enable: replace_var_in_expr_term(t2, replace_map),
            write_addr: replace_var_in_expr_term(t3, replace_map),
            write_data: replace_var_in_expr_term(t4, replace_map),
        }),
        ExprType::Rom(t) => ExprType::Reg(replace_var_in_expr_term(t, replace_map)),
    };
    Sized {
        size: expr.size,
        value: new_expr,
    }
}

fn replace_var_in_expr_term(expr: &ExprTerm, replace_map: &HashMap<String, String>) -> ExprTerm {
    let new_expr = match &expr.value {
        ExprTermType::Const(_) => expr.value.clone(),
        ExprTermType::Var(var) => ExprTermType::Var(match var {
            Var::Shared(s) => {
                let new_var = replace_map
                    .get(s)
                    .expect("Should not happen : Unknown shared var");
                Var::Shared(new_var.clone())
            }
            Var::Local(_) => var.clone(),
        }),
    };
    Sized {
        size: expr.size,
        value: new_expr,
    }
}
