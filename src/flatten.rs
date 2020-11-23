use crate::ast::*;
use global_counter::global_counter;

/*
This file is used to "flatten" every statement in the program.
Expr can currently be nested. Netlists cannot be nested, so the tree will need to be
flattened at some point. As it makes the tree simpler to process, it is better done
at the beginning.
The functions in this file transform :
a = 1 + 0 * b + f(0)

in:
x = 0*b
y = 1 + x
z = f(0)
a = y + z

(the names x, y, z are actually generated using $, which is a forbidden character for variables names to avoid conflicts,
and a global counter.)
I choose to use strings for name as netlists allow for it and it makes the generated code a bit cleared, albeit not much
*/

//the global counter
global_counter!(FLATTEN_EXPR_COUNTER, u32, 0);

//wrapper function
pub fn flatten(prog: &mut Program) {
    for (_, f) in prog.functions.iter_mut() {
        f.statements = f
            .statements
            .drain(..)
            .map(|stat| flatten_statement(stat))
            .flatten()
            .collect();
    }
    for (_, m) in &mut prog.modules {
        for automata in &mut m.automata {
            for (name, node) in automata {
                let Node {
                    statements,
                    transitions,
                } = &mut node.value;
                *statements = statements
                    .drain(..)
                    .map(|stat| flatten_statement(stat))
                    .flatten()
                    .collect();
                //transition must be handled as well.
                //They are flattened into statements in the end of the node body
                *transitions = transitions
                    .drain(..)
                    .map(|(expr, goto, reset)| {
                        let pos = expr.loc;
                        let (mut v, expr) = flatten_expr(name, expr);
                        statements.append(&mut v);
                        (loc(pos, expr), goto, reset)
                    })
                    .collect();
            }
        }
    }
}
fn flatten_statement(statement: Statement) -> Vec<Statement> {
    match statement {
        Statement::Assign(var_assign) => flatten_assigns(var_assign),
        Statement::If(IfStruct {
            condition,
            mut if_block,
            mut else_block,
        }) => {
            let v1 = if_block
                .drain(..)
                .map(|stat| flatten_statement(stat))
                .flatten();
            let v2 = else_block
                .drain(..)
                .map(|stat| flatten_statement(stat))
                .flatten();
            vec![Statement::If(IfStruct {
                condition: condition,
                if_block: v1.collect(),
                else_block: v2.collect(),
            })]
        }
        Statement::FnAssign(fn_assign) => vec![Statement::FnAssign(fn_assign)],
    }
}
pub fn flatten_assigns(mut statement: Vec<VarAssign>) -> Vec<Statement> {
    let mut res = Vec::new();
    for assign in statement.drain(..) {
        let name = assign.var.value.clone();
        let expr_pos = assign.expr.loc;
        let (mut statements, expr_out) = flatten_expr(&name, assign.expr);
        res.append(&mut statements);
        res.push(Statement::Assign(vec![VarAssign {
            expr: loc(expr_pos, expr_out),
            var: assign.var,
        }]))
    }
    res
}
fn loc<T>(loc: Pos, value: T) -> Loc<T> {
    Loc { loc, value }
}

//generates a name
fn get_name(name: &String) -> String {
    let counter = FLATTEN_EXPR_COUNTER.get_cloned();
    FLATTEN_EXPR_COUNTER.inc();
    format!("flatten${}$${}", name, counter)
}

//this is very, very verbose. Find a way to simplify it ?
fn flatten_expr(name: &String, expr: Loc<Expr>) -> (Vec<Statement>, Expr) {
    let mut res = Vec::new();
    let e_ret = match expr.value {
        Expr::Const(_) | Expr::Var(_) | Expr::FnCall(_) => expr.value,
        Expr::Not(e_in) => {
            let (mut v, e_out) = flatten_expr(name, loc(expr.loc, *e_in));
            let name = loc(expr.loc, get_name(name));
            res.append(&mut v);
            res.push(Statement::Assign(vec![VarAssign {
                var: name.clone(),
                expr: loc(expr.loc, e_out),
            }]));
            Expr::Not(Box::new(Expr::Var(name)))
        }
        Expr::Slice(e_in, c1, c2) => {
            let pos = e_in.loc;
            let (mut v, e_out) = flatten_expr(name, *e_in);
            let name = loc(expr.loc, get_name(name));
            res.append(&mut v);
            res.push(Statement::Assign(vec![VarAssign {
                var: name.clone(),
                expr: loc(expr.loc, e_out),
            }]));
            Expr::Slice(Box::new(loc(pos, Expr::Var(name))), c1, c2)
        }
        Expr::BiOp(op, e1, e2) => {
            let pos1 = e1.loc;
            let pos2 = e2.loc;
            let (mut v1, e_out1) = flatten_expr(name, *e1);
            let (mut v2, e_out2) = flatten_expr(name, *e2);
            let name1 = loc(pos1, get_name(name));
            let name2 = loc(pos2, get_name(name));
            res.append(&mut v1);
            res.append(&mut v2);
            res.push(Statement::Assign(vec![VarAssign {
                var: name1.clone(),
                expr: loc(pos1, e_out1),
            }]));
            res.push(Statement::Assign(vec![VarAssign {
                var: name2.clone(),
                expr: loc(pos2, e_out2),
            }]));
            Expr::BiOp(
                op,
                Box::new(loc(pos1, Expr::Var(name1))),
                Box::new(loc(pos2, Expr::Var(name2))),
            )
        }
        Expr::Mux(e1, e2, e3) => {
            let pos1 = e1.loc;
            let pos2 = e2.loc;
            let pos3 = e3.loc;
            let (mut v1, e_out1) = flatten_expr(name, *e1);
            let (mut v2, e_out2) = flatten_expr(name, *e2);
            let (mut v3, e_out3) = flatten_expr(name, *e3);
            let name1 = loc(pos1, get_name(name));
            let name2 = loc(pos2, get_name(name));
            let name3 = loc(pos3, get_name(name));
            res.append(&mut v1);
            res.append(&mut v2);
            res.append(&mut v3);
            res.push(Statement::Assign(vec![VarAssign {
                var: name1.clone(),
                expr: loc(pos1, e_out1),
            }]));
            res.push(Statement::Assign(vec![VarAssign {
                var: name2.clone(),
                expr: loc(pos2, e_out2),
            }]));
            res.push(Statement::Assign(vec![VarAssign {
                var: name3.clone(),
                expr: loc(pos3, e_out3),
            }]));
            Expr::Mux(
                Box::new(loc(pos1, Expr::Var(name1))),
                Box::new(loc(pos2, Expr::Var(name2))),
                Box::new(loc(pos3, Expr::Var(name3))),
            )
        }
        Expr::Reg(e_in) => {
            let (mut v, e_out) = flatten_expr(name, loc(expr.loc, *e_in));
            let name = loc(expr.loc, get_name(name));
            res.append(&mut v);
            res.push(Statement::Assign(vec![VarAssign {
                var: name.clone(),
                expr: loc(expr.loc, e_out),
            }]));
            Expr::Reg(Box::new(Expr::Var(name)))
        }
        Expr::Ram(RamStruct {
            read_addr: e1,
            write_enable: e2,
            write_addr: e3,
            write_data: e4,
        }) => {
            let pos1 = e1.loc;
            let pos2 = e2.loc;
            let pos3 = e3.loc;
            let pos4 = e4.loc;
            let (mut v1, e_out1) = flatten_expr(name, *e1);
            let (mut v2, e_out2) = flatten_expr(name, *e2);
            let (mut v3, e_out3) = flatten_expr(name, *e3);
            let (mut v4, e_out4) = flatten_expr(name, *e4);
            let name1 = loc(pos1, get_name(name));
            let name2 = loc(pos2, get_name(name));
            let name3 = loc(pos3, get_name(name));
            let name4 = loc(pos4, get_name(name));
            res.append(&mut v1);
            res.append(&mut v2);
            res.append(&mut v3);
            res.append(&mut v4);
            res.push(Statement::Assign(vec![VarAssign {
                var: name1.clone(),
                expr: loc(pos1, e_out1),
            }]));
            res.push(Statement::Assign(vec![VarAssign {
                var: name2.clone(),
                expr: loc(pos2, e_out2),
            }]));
            res.push(Statement::Assign(vec![VarAssign {
                var: name3.clone(),
                expr: loc(pos3, e_out3),
            }]));
            res.push(Statement::Assign(vec![VarAssign {
                var: name4.clone(),
                expr: loc(pos4, e_out4),
            }]));
            Expr::Ram(RamStruct {
                read_addr: Box::new(loc(pos1, Expr::Var(name1))),
                write_enable: Box::new(loc(pos2, Expr::Var(name2))),
                write_addr: Box::new(loc(pos3, Expr::Var(name3))),
                write_data: Box::new(loc(pos4, Expr::Var(name4))),
            })
        }
        Expr::Rom(RomStruct {
            word_size,
            read_addr,
        }) => {
            let pos = read_addr.loc;
            let (mut v, e_out) = flatten_expr(name, *read_addr);
            let name = loc(pos, get_name(name));
            res.append(&mut v);
            res.push(Statement::Assign(vec![VarAssign {
                var: name.clone(),
                expr: loc(pos, e_out),
            }]));
            Expr::Rom(RomStruct {
                read_addr: Box::new(loc(pos, Expr::Var(name))),
                word_size,
            })
        }
    };
    (res, e_ret)
}
