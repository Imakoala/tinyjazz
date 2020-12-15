use crate::{ast::*, expand_fn::WrongNumberType};
use global_counter::global_counter;
/*
This module collapses external modules.
Basically, it makes new shared variables for the output, and replaces the module call by shared var assignation.
Then it renames every shared var and node in the called module, and copies all the nodes and shared variables in the main module.

This repeats until there are no more external modules.
*/

//TODO: reset handling and instant transitions

pub enum CollapseAutomataError {
    CyclicModuleCall(String),
    UnknownModule(Pos, String),
    NoMainModule,
    UnknownVar(Pos, String),
    WrongNumber(WrongNumberType, Pos, usize, usize),
}
global_counter!(INLINE_MODULE_COUNTER, u32, 0);
type Result<T> = std::result::Result<T, CollapseAutomataError>;

pub fn collapse_automata(prog: &mut Program) -> Result<()> {
    let mut changed = true;
    let mut new_nodes = Vec::new();
    let mut new_init_nodes = Vec::new();
    let mut new_shared = Vec::new();
    while changed {
        changed = false;
        let main_module = prog
            .modules
            .get("main")
            .ok_or(CollapseAutomataError::NoMainModule)?;
        let node_parents = compute_nodes_parents(main_module);
        for (_, node) in main_module
            .nodes
            .iter()
            .filter(|(_, n)| !n.extern_modules.is_empty())
        {
            let exit_condition = get_exit_condition(node);
            let enter_condition = node_parents
                .get(&*node.name.value)
                .unwrap()
                .iter()
                .map(|s| get_exit_condition(main_module.nodes.get(*s).unwrap()))
                .fold(
                    loc(node.name.loc, Expr::Const(ConstExpr::Known(vec![true]))),
                    |e1, e2| loc(e1.loc, Expr::BiOp(BiOp::And, Box::new(e1), Box::new(e2))),
                );
            let mut link_node = Node {
                name: node.name.clone(),
                extern_modules: Vec::new(),
                statements: node.statements.clone(),
                transitions: node.transitions.clone(),
                weak: node.weak,
            };
            for ExtModule {
                inputs,
                outputs,
                name,
            } in &node.extern_modules
            {
                if name.value == main_module.name {
                    return Err(CollapseAutomataError::CyclicModuleCall(name.value.clone()));
                }
                let pos = name.loc;
                let inputs: Vec<String> = inputs.value.iter().map(|s| s.to_string()).collect();
                let module =
                    prog.modules
                        .get(&name.value)
                        .ok_or(CollapseAutomataError::UnknownModule(
                            name.loc,
                            name.value.clone(),
                        ))?;
                if module.inputs.len() != inputs.len() {
                    return Err(CollapseAutomataError::WrongNumber(
                        WrongNumberType::Args,
                        name.loc,
                        module.inputs.len(),
                        inputs.len(),
                    ));
                }
                if module.outputs.len() != outputs.len() {
                    return Err(CollapseAutomataError::WrongNumber(
                        WrongNumberType::ReturnVars,
                        name.loc,
                        module.inputs.len(),
                        inputs.len(),
                    ));
                }
                let (mut nodes, mut init_nodes, mut shared, automaton_outputs) = make_automaton(
                    enter_condition.clone(),
                    exit_condition.clone(),
                    inputs,
                    module,
                    main_module.init_nodes.contains(&node.name),
                )?;

                new_init_nodes.append(&mut init_nodes);
                new_nodes.append(&mut nodes);
                new_shared.append(&mut shared);
                link_node.statements.push(Statement::Assign(
                    outputs
                        .iter()
                        .zip(automaton_outputs.iter())
                        .map(|(o, auto_o)| VarAssign {
                            var: o.clone(),
                            expr: loc(pos, Expr::Var(loc(pos, auto_o.to_string()))),
                        })
                        .collect(),
                ))
            }
            new_nodes.push(link_node);
        }
        let main_module = prog.modules.get_mut("main").unwrap();
        main_module.nodes = main_module
            .nodes
            .drain()
            .filter(|(_, n)| n.extern_modules.is_empty())
            .collect();
        main_module.shared.append(&mut new_shared);
        main_module.init_nodes.append(&mut new_init_nodes);
        for node in new_nodes.drain(..) {
            main_module.nodes.insert(node.name.to_string(), node);
        }
    }
    Ok(())
}

fn get_exit_condition(node: &Node) -> Loc<Expr> {
    let mut expr = loc(node.name.loc, Expr::Var(node.name.clone()));
    for (e, _, _) in node.transitions.iter() {
        expr = loc(
            e.loc,
            Expr::BiOp(BiOp::And, Box::new(e.clone()), Box::new(expr)),
        );
    }
    expr
}

fn compute_nodes_parents(module: &Module) -> HashMap<&str, Vec<&str>> {
    let mut node_parents = HashMap::new();
    for (_, node) in module.nodes.iter() {
        if !node_parents.contains_key(&*node.name.value) {
            node_parents.insert(&*node.name.value, Vec::new());
        }
        for (_, n, _) in node.transitions.iter() {
            if !node_parents.contains_key(&*n.value) {
                node_parents.insert(&*n.value, Vec::new());
            }
            node_parents
                .get_mut(&*n.value)
                .unwrap()
                .push(&*node.name.value)
        }
    }
    node_parents
}
fn get_rename(counter: u32, name: &str, namespace: &str) -> String {
    format!("inline_mod${}${}${}$", name, namespace, counter)
}

fn get_pause_name(counter: u32, name: &str, namespace: &str) -> String {
    format!("inline_mod_pause${}${}${}$", name, namespace, counter)
}

pub fn make_automaton(
    enter_condition: Loc<Expr>,
    exit_condition: Loc<Expr>,
    mut inputs: Vec<String>,
    module: &Module,
    is_init: bool,
) -> Result<(Vec<Node>, Vec<Loc<String>>, Vec<VarAssign>, Vec<String>)> {
    //new_nodes, init nodes, new shared, outputs
    let counter = INLINE_MODULE_COUNTER.get_cloned();
    INLINE_MODULE_COUNTER.inc();
    let mut shared_rename_map = HashMap::new();
    let mut shared = Vec::new();
    let mut nodes = Vec::new();
    for (s, rename) in module.inputs.iter().zip(inputs.drain(..)) {
        shared_rename_map.insert(s.name.clone(), rename);
    }
    for var in module.shared.iter() {
        let mut new_var = var.clone();
        let name = get_rename(counter, &new_var.var.value, &module.name);
        new_var.var.value = name.clone();
        shared.push(new_var.clone());
        shared_rename_map.insert(var.var.value.clone(), name);
    }
    for (_, node) in &module.nodes {
        let (new_node, pause_node) = make_node(
            counter,
            &module.name,
            &enter_condition,
            &exit_condition,
            &shared_rename_map,
            node,
        );
        nodes.push(new_node);
        nodes.push(pause_node);
    }
    let outputs = module
        .outputs
        .iter()
        .map(|s| {
            Ok(shared_rename_map
                .get(&s.name)
                .ok_or(CollapseAutomataError::UnknownVar(
                    s.size.loc,
                    s.name.clone(),
                ))?
                .clone())
        })
        .collect::<Result<Vec<String>>>()?;
    let init_nodes = module
        .init_nodes
        .iter()
        .map(|n| {
            if is_init {
                loc(n.loc, get_rename(counter, &*n.value, &*module.name))
            } else {
                loc(n.loc, get_pause_name(counter, &*n.value, &*module.name))
            }
        })
        .collect();
    Ok((nodes, init_nodes, shared, outputs))
}

pub fn make_node(
    counter: u32,
    namespace: &str,
    enter_condition: &Loc<Expr>,
    exit_condition: &Loc<Expr>,
    shared_rename_map: &HashMap<String, String>,
    node: &Node,
) -> (Node, Node) {
    let new_name = get_rename(counter, &node.name, namespace);
    let pos = node.name.loc;
    let statements = node
        .statements
        .iter()
        .map(|s| replace_var_in_statement(s, &shared_rename_map))
        .collect::<Vec<Statement>>();
    let transitions = node
        .transitions
        .iter()
        .map(|(e, n, b)| {
            let renamed_expr = loc(e.loc, replace_var_in_expr(&e.value, shared_rename_map));
            let transition_stay = loc(
                renamed_expr.loc,
                Expr::BiOp(
                    BiOp::And,
                    Box::new(renamed_expr.clone()),
                    Box::new(loc(
                        exit_condition.loc,
                        Expr::Not(Box::new(exit_condition.value.clone())),
                    )),
                ),
            );
            let transition_pause = loc(
                renamed_expr.loc,
                Expr::BiOp(
                    BiOp::And,
                    Box::new(renamed_expr.clone()),
                    Box::new(exit_condition.clone()),
                ),
            );
            vec![
                (
                    transition_stay,
                    loc(n.loc, get_rename(counter, &n.value, namespace)),
                    *b,
                ),
                (
                    transition_pause,
                    loc(n.loc, get_pause_name(counter, &n.value, namespace)),
                    *b,
                ),
            ]
        })
        .flatten()
        .collect::<Vec<(Loc<Expr>, Loc<Var>, bool)>>();
    let pause_node = Node {
        name: loc(pos, get_pause_name(counter, &node.name, namespace)),
        extern_modules: Vec::new(),
        statements: Vec::new(),
        weak: true,
        transitions: vec![(
            loc(pos, enter_condition.value.clone()),
            loc(pos, new_name.clone()),
            false,
        )], //TODO : add reset handling
    };
    let stay_node = Node {
        name: loc(pos, get_rename(counter, &node.name, namespace)),
        extern_modules: node.extern_modules.clone(),
        statements,
        weak: node.weak,
        transitions, //TODO : add reset handling
    };
    (stay_node, pause_node)
}

fn loc<T>(loc: Pos, value: T) -> Loc<T> {
    Loc { loc, value }
}

fn replace_var_in_statement(
    statement: &Statement,
    replace_map: &HashMap<String, String>,
) -> Statement {
    match statement {
        Statement::Assign(var_assigns) => Statement::Assign(
            var_assigns
                .iter()
                .map(|var_assign| {
                    let new_var = replace_map
                        .get(&var_assign.var.value)
                        .unwrap_or(&var_assign.var.value)
                        .clone();
                    let new_expr = replace_var_in_expr(&var_assign.expr, &replace_map);
                    VarAssign {
                        var: loc(var_assign.var.loc, new_var),
                        expr: loc(var_assign.expr.loc, new_expr),
                    }
                })
                .collect(),
        ),
        Statement::If(IfStruct {
            if_block,
            else_block,
            condition,
        }) => Statement::If(IfStruct {
            if_block: if_block
                .iter()
                .map(|s| replace_var_in_statement(s, replace_map))
                .collect(),
            else_block: else_block
                .iter()
                .map(|s| replace_var_in_statement(s, replace_map))
                .collect(),
            condition: condition.clone(),
        }),
        Statement::FnAssign(FnAssign {
            vars,
            f:
                FnCall {
                    name,
                    args,
                    static_args,
                },
        }) => Statement::FnAssign(FnAssign {
            vars: vars
                .iter()
                .map(|v| loc(v.loc, replace_map.get(&v.value).unwrap_or(&v.value).clone()))
                .collect(),
            f: FnCall {
                name: name.clone(),
                args: loc(
                    args.loc,
                    args.value
                        .iter()
                        .map(|e| loc(e.loc, replace_var_in_expr(e, replace_map)))
                        .collect(),
                ),
                static_args: static_args.clone(),
            },
        }),
    }
}

fn replace_var_in_expr(expr: &Expr, replace_map: &HashMap<String, String>) -> Expr {
    match expr {
        Expr::Var(v) => Expr::Var(loc(
            v.loc,
            replace_map.get(&v.value).unwrap_or(&v.value).clone(),
        )),
        Expr::Const(_) => expr.clone(),
        Expr::Not(e) => Expr::Not(Box::new(replace_var_in_expr(e, replace_map))),
        Expr::Slice(e, c1, c2) => Expr::Slice(
            Box::new(loc(e.loc, replace_var_in_expr(e, replace_map))),
            c1.clone(),
            c2.clone(),
        ),
        Expr::BiOp(op, e1, e2) => Expr::BiOp(
            op.clone(),
            Box::new(loc(e1.loc, replace_var_in_expr(e1, replace_map))),
            Box::new(loc(e2.loc, replace_var_in_expr(e2, replace_map))),
        ),
        Expr::Mux(e1, e2, e3) => Expr::Mux(
            Box::new(loc(e1.loc, replace_var_in_expr(e1, replace_map))),
            Box::new(loc(e2.loc, replace_var_in_expr(e2, replace_map))),
            Box::new(loc(e3.loc, replace_var_in_expr(e3, replace_map))),
        ),
        Expr::Reg(c, e) => Expr::Reg(
            c.clone(),
            Box::new(loc(e.loc, replace_var_in_expr(e, replace_map))),
        ),
        Expr::Ram(RamStruct {
            read_addr,
            write_enable,
            write_addr,
            write_data,
        }) => Expr::Ram(RamStruct {
            read_addr: Box::new(loc(
                read_addr.loc,
                replace_var_in_expr(read_addr, replace_map),
            )),
            write_enable: Box::new(loc(
                write_enable.loc,
                replace_var_in_expr(write_enable, replace_map),
            )),
            write_addr: Box::new(loc(
                write_addr.loc,
                replace_var_in_expr(write_addr, replace_map),
            )),
            write_data: Box::new(loc(
                write_data.loc,
                replace_var_in_expr(write_data, replace_map),
            )),
        }),
        Expr::Rom(RomStruct {
            read_addr,
            word_size,
        }) => Expr::Rom(RomStruct {
            read_addr: Box::new(loc(
                read_addr.loc,
                replace_var_in_expr(read_addr, replace_map),
            )),
            word_size: word_size.clone(),
        }),
        Expr::FnCall(FnCall {
            name,
            args,
            static_args,
        }) => Expr::FnCall(FnCall {
            name: name.clone(),
            args: loc(
                args.loc,
                args.value
                    .iter()
                    .map(|e| loc(e.loc, replace_var_in_expr(e, replace_map)))
                    .collect(),
            ),
            static_args: static_args.clone(),
        }),
    }
}
