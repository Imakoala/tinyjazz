use crate::ast::parse_ast::*;
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

pub enum FlattenError {
    ConcatInReg(Pos),
    SliceInReg(Pos),
    MemoryInReg(Pos),
}

type Result<T> = std::result::Result<T, FlattenError>;

//wrapper function
pub fn flatten(prog: &mut Program) -> Result<()> {
    for (_, f) in prog.functions.iter_mut() {
        f.statements = f
            .statements
            .drain(..)
            .flat_map(|stat| {
                let (v, r) = match flatten_statement(stat) {
                    Ok(v) => (Some(v.into_iter().map(|s| Ok(s))), None),
                    Err(e) => (None, Some(Err(e))),
                };
                v.into_iter().flatten().chain(r)
            })
            .collect::<Result<Vec<Statement>>>()?;
    }
    for (_, m) in &mut prog.automata {
        for (_, state) in &mut m.states {
            let State {
                name,
                weak: _,
                statements,
                transitions,
            } = state;
            *statements = statements
                .drain(..)
                .flat_map(|stat| {
                    let (v, r) = match flatten_statement(stat) {
                        Ok(v) => (Some(v.into_iter().map(|s| Ok(s))), None),
                        Err(e) => (None, Some(Err(e))),
                    };
                    v.into_iter().flatten().chain(r)
                })
                .collect::<Result<Vec<Statement>>>()?;
            //transition must be handled as well.
            //They are flattened into statements in the end of the state body
            *transitions = transitions
                .drain(..)
                .map(|transition| {
                    //important to avoid creating cycles by taking shared vars out of transitions.
                    if let Expr::Var(_) = transition.condition.unwrap_ref() {
                        return Ok(transition);
                    }
                    let pos = transition.condition.loc;
                    let (mut v, expr) = flatten_expr(
                        name,
                        Loc::new(
                            transition.condition.loc,
                            transition.condition.value.unwrap(),
                        ),
                    )?;
                    statements.append(&mut v);
                    let name = get_name(name);
                    statements.push(Statement::Assign(vec![VarAssign {
                        var: loc(pos, name.clone()),
                        expr: loc(pos, expr),
                    }]));
                    Ok(Transition {
                        condition: loc(pos, TrCond::Expr(Expr::Var(loc(pos, name)))),
                        ..transition
                    })
                })
                .collect::<Result<Vec<Transition>>>()?;
        }
    }
    Ok(())
}
fn flatten_statement(statement: Statement) -> Result<Vec<Statement>> {
    match statement {
        Statement::Assign(var_assign) => flatten_assigns(var_assign),
        Statement::If(IfStruct {
            condition,
            mut if_block,
            mut else_block,
        }) => {
            let v1 = if_block.drain(..).flat_map(|stat| {
                let (v, r) = match flatten_statement(stat) {
                    Ok(v) => (Some(v.into_iter().map(|s| Ok(s))), None),
                    Err(e) => (None, Some(Err(e))),
                };
                v.into_iter().flatten().chain(r)
            });
            let v2 = else_block.drain(..).flat_map(|stat| {
                let (v, r) = match flatten_statement(stat) {
                    Ok(v) => (Some(v.into_iter().map(|s| Ok(s))), None),
                    Err(e) => (None, Some(Err(e))),
                };
                v.into_iter().flatten().chain(r)
            });
            Ok(vec![Statement::If(IfStruct {
                condition: condition,
                if_block: v1.collect::<Result<Vec<Statement>>>()?,
                else_block: v2.collect::<Result<Vec<Statement>>>()?,
            })])
        }
        Statement::FnAssign(mut fn_assign) => {
            let pos = fn_assign.f.name.loc;
            let fn_name = fn_assign.f.name.value.clone();
            let mut res = Vec::new();
            fn_assign.f.args = loc(
                pos,
                fn_assign
                    .f
                    .args
                    .drain(..)
                    .map(|a| {
                        let (mut stmts, e_out) = flatten_expr(&fn_name, a.clone())?;
                        res.append(&mut stmts);
                        Ok(loc(a.loc, e_out))
                    })
                    .collect::<Result<Vec<Loc<Expr>>>>()?,
            );
            res.push(Statement::FnAssign(fn_assign));
            Ok(res)
        }
        Statement::ExtAutomaton(_) => {
            panic!("Should not happen: nested automaton after they are collapsed")
        }
    }
}
pub fn flatten_assigns(mut statement: Vec<VarAssign>) -> Result<Vec<Statement>> {
    let mut res = Vec::new();
    for assign in statement.drain(..) {
        let name = assign.var.value.clone();
        let expr_pos = assign.expr.loc;
        let (mut statements, expr_out) = flatten_expr(&name, assign.expr)?;
        res.append(&mut statements);
        res.push(Statement::Assign(vec![VarAssign {
            expr: loc(expr_pos, expr_out),
            var: assign.var,
        }]))
    }
    Ok(res)
}
fn loc<T>(loc: Pos, value: T) -> Loc<T> {
    Loc { loc, value }
}

//generates a name
fn get_name(name: &String) -> String {
    let counter = FLATTEN_EXPR_COUNTER.get_cloned();
    FLATTEN_EXPR_COUNTER.inc();
    format!("{}#flatten#{}", name, counter)
}

//takes a variable name and an expression as an arg, returns a variable and statements,
//such that if the statements are computed the variable contains the value of the expr
fn flatten_expr(name: &String, expr: Loc<Expr>) -> Result<(Vec<Statement>, Expr)> {
    let mut res = Vec::new();
    let glob_pos = expr.loc;
    let e_ret = match expr.value {
        Expr::Const(_) | Expr::Var(_) => expr.value,
        Expr::Last(v) => {
            let name = loc(v.loc, get_name(&v.value));
            res.push(Statement::Assign(vec![VarAssign {
                var: name.clone(),
                expr: Loc::new(glob_pos, Expr::Last(v)),
            }]));
            Expr::Var(name)
        }
        Expr::FnCall(mut fn_call) => {
            let name = loc(glob_pos, get_name(name));
            let fn_name = fn_call.name.value.clone();
            fn_call.args = loc(
                glob_pos,
                fn_call
                    .args
                    .drain(..)
                    .map(|a| {
                        let (mut stmts, e_out) = flatten_expr(&fn_name, a.clone())?;
                        res.append(&mut stmts);
                        Ok(loc(a.loc, e_out))
                    })
                    .collect::<Result<Vec<Loc<Expr>>>>()?,
            );
            res.push(Statement::FnAssign(FnAssign {
                vars: vec![name.clone()],
                f: fn_call,
            }));
            Expr::Var(name)
        }
        Expr::Not(e_in) => {
            let (mut v, e_out) = flatten_expr(name, loc(expr.loc, *e_in))?;
            let name = loc(expr.loc, get_name(name));
            res.append(&mut v);
            res.push(Statement::Assign(vec![VarAssign {
                var: name.clone(),
                expr: loc(expr.loc, Expr::Not(Box::new(e_out))),
            }]));
            Expr::Var(name)
        }
        Expr::Slice(e_in, c1, c2) => {
            let pos = e_in.loc;
            let (mut v, e_out) = flatten_expr(name, *e_in)?;
            let name = loc(expr.loc, get_name(name));
            res.append(&mut v);
            res.push(Statement::Assign(vec![VarAssign {
                var: name.clone(),
                expr: loc(expr.loc, Expr::Slice(Box::new(loc(pos, e_out)), c1, c2)),
            }]));
            Expr::Var(name)
        }
        Expr::BiOp(op, e1, e2) => {
            let pos1 = e1.loc;
            let pos2 = e2.loc;
            let (mut v1, e_out1) = flatten_expr(name, *e1)?;
            let (mut v2, e_out2) = flatten_expr(name, *e2)?;
            let name = loc(pos1, get_name(name));
            res.append(&mut v1);
            res.append(&mut v2);
            res.push(Statement::Assign(vec![VarAssign {
                var: name.clone(),
                expr: loc(
                    pos1,
                    Expr::BiOp(op, Box::new(loc(pos1, e_out1)), Box::new(loc(pos2, e_out2))),
                ),
            }]));
            Expr::Var(name)
        }
        Expr::Mux(e1, e2, e3) => {
            let pos1 = e1.loc;
            let pos2 = e2.loc;
            let pos3 = e3.loc;
            let (mut v1, e_out1) = flatten_expr(name, *e1)?;
            let (mut v2, e_out2) = flatten_expr(name, *e2)?;
            let (mut v3, e_out3) = flatten_expr(name, *e3)?;
            let name = loc(glob_pos, get_name(name));
            res.append(&mut v1);
            res.append(&mut v2);
            res.append(&mut v3);
            res.push(Statement::Assign(vec![VarAssign {
                var: name.clone(),
                expr: loc(
                    glob_pos,
                    Expr::Mux(
                        Box::new(loc(pos1, e_out1)),
                        Box::new(loc(pos2, e_out2)),
                        Box::new(loc(pos3, e_out3)),
                    ),
                ),
            }]));
            Expr::Var(name)
        }
        Expr::Reg(c, mut e_in) => {
            if let Expr::Var(_) = e_in.value {
                let name = loc(e_in.loc, get_name(name));
                res.push(Statement::Assign(vec![VarAssign {
                    var: name.clone(),
                    expr: loc(e_in.loc, Expr::Reg(c, e_in)),
                }]));
                Expr::Var(name)
            } else {
                regify_expr(&mut e_in.value, e_in.loc, &c)?;
                let (mut v, e_out) = flatten_expr(name, *e_in)?;
                res.append(&mut v);
                e_out
            }
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
            let (mut v1, e_out1) = flatten_expr(name, *e1)?;
            let (mut v2, e_out2) = flatten_expr(name, *e2)?;
            let (mut v3, e_out3) = flatten_expr(name, *e3)?;
            let (mut v4, e_out4) = flatten_expr(name, *e4)?;
            let name = loc(pos1, get_name(name));
            res.append(&mut v1);
            res.append(&mut v2);
            res.append(&mut v3);
            res.append(&mut v4);
            res.push(Statement::Assign(vec![VarAssign {
                var: name.clone(),
                expr: loc(
                    pos1,
                    Expr::Ram(RamStruct {
                        read_addr: Box::new(loc(pos1, e_out1)),
                        write_enable: Box::new(loc(pos2, e_out2)),
                        write_addr: Box::new(loc(pos3, e_out3)),
                        write_data: Box::new(loc(pos4, e_out4)),
                    }),
                ),
            }]));
            Expr::Var(name)
        }
        Expr::Rom(RomStruct {
            word_size,
            read_addr,
        }) => {
            let pos = read_addr.loc;
            let (mut v, e_out) = flatten_expr(name, *read_addr)?;
            let name = loc(pos, get_name(name));
            res.append(&mut v);
            res.push(Statement::Assign(vec![VarAssign {
                var: name.clone(),
                expr: loc(
                    pos,
                    Expr::Rom(RomStruct {
                        read_addr: Box::new(loc(pos, e_out)),
                        word_size,
                    }),
                ),
            }]));
            Expr::Var(name)
        }
    };
    Ok((res, e_ret))
}

fn regify_expr(expr: &mut Expr, pos: Pos, c: &Loc<Const>) -> Result<()> {
    match expr {
        Expr::Var(_) | Expr::Last(_) => {
            *expr = Expr::Reg(c.clone(), Box::new(loc(pos, expr.clone())))
        }
        Expr::Const(_) => {}
        Expr::Not(e) => regify_expr(e, pos, c)?,
        Expr::Slice(_, _, _) => return Err(FlattenError::SliceInReg(pos)),
        Expr::BiOp(op, e1, e2) => {
            if let BiOp::Concat = op {
                return Err(FlattenError::ConcatInReg(pos));
            }
            regify_expr(e1, pos, c)?;
            regify_expr(e2, pos, c)?
        }
        Expr::Mux(e1, e2, e3) => {
            regify_expr(e1, pos, c)?;
            regify_expr(e2, pos, c)?;
            regify_expr(e3, pos, c)?
        }
        Expr::Reg(_, _) => {}
        Expr::Ram(_) => return Err(FlattenError::MemoryInReg(pos)),
        Expr::Rom(_) => return Err(FlattenError::MemoryInReg(pos)),
        Expr::FnCall(FnCall {
            name: _,
            static_args: _,
            args,
        }) => {
            for e in &mut args.value {
                regify_expr(e, pos, c)?
            }
        }
    };
    Ok(())
}
