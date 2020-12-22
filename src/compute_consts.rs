use crate::ast::*;
use solvent::DepGraph;
/*
In this file, we try to simplify all the constants as much as possible, to prepare for
recursive expansion.
In particular, the global constants are replaced, and all the constants outiside of functions are replaced by simple numbers.
*/
#[derive(Clone, Debug)]
pub enum ComputeConstError {
    UnknowVariable(Pos, String), //Unknown const var
    DivisionByZero(Pos),         //division by zero in const evaluation
    CyclicDefinition,            //In global const definition
    Other(String),               //Unexpected external error
}

impl From<solvent::SolventError> for ComputeConstError {
    fn from(error: solvent::SolventError) -> Self {
        match error {
            solvent::SolventError::CycleDetected => ComputeConstError::CyclicDefinition,
            solvent::SolventError::NoSuchNode => ComputeConstError::Other(
                "Unknown error while computing constants dependancy graph".to_string(),
            ),
        }
    }
}

//The wrapper function
pub fn compute_consts(prog: &mut Program) -> Result<(), ComputeConstError> {
    //Simplify global consts to single values
    compute_global_consts(&mut prog.global_consts)?;

    //iterate through statements to call appropriate functions
    for (_, m) in &mut prog.modules {
        for arg in &mut m.inputs {
            let res = compute_const(&arg.size, &prog.global_consts)?;
            arg.size.value = Const::Value(res);
        }
        for arg in &mut m.outputs {
            let res = compute_const(&arg.size, &prog.global_consts)?;
            arg.size.value = Const::Value(res);
        }
        for shared in &mut m.shared {
            compute_consts_in_expr(&mut shared.expr, &prog.global_consts)?;
        }
        for (_, node) in &mut m.nodes {
            for statement in &mut node.statements {
                compute_consts_in_statement(statement, &prog.global_consts)?;
            }
            for transition in &mut node.transitions {
                if let TrCond::Expr(expr) = &mut transition.condition.value {
                    compute_consts_in_expr(
                        &mut Loc::new(transition.condition.loc, expr),
                        &prog.global_consts,
                    )?;
                }
            }
        }
    }
    for (_, function) in &mut prog.functions {
        for arg in &mut function.args {
            simplify_const(&mut arg.size, &prog.global_consts, &function.static_args)?;
        }
        for arg in &mut function.return_vars {
            simplify_const(&mut arg.size, &prog.global_consts, &function.static_args)?;
        }
        for statement in &mut function.statements {
            simplify_consts_in_statement(statement, &prog.global_consts, &function.static_args)?;
        }
    }
    Ok(())
}

//simplify a constant as much as possible, without assuming it can be reduced to a constant
//for use in functions only.
fn simplify_const(
    c: &mut Const,
    consts: &HashMap<String, Const>,
    static_args: &Vec<String>,
) -> Result<(), ComputeConstError> {
    let res = match c {
        Const::Value(i) => Some(Const::Value(*i)),
        Const::Var(v) => {
            if static_args.contains(v) {
                None
            } else {
                if let Const::Value(i) = consts
                    .get(&**v)
                    .ok_or(ComputeConstError::UnknowVariable(v.loc, v.to_string()))?
                {
                    Some(Const::Value(*i))
                } else {
                    return Err(ComputeConstError::UnknowVariable(v.loc, v.to_string()));
                }
            }
        }
        Const::BiOp(op, c1, c2) => {
            simplify_const(c1, consts, static_args)?;
            simplify_const(c2, consts, static_args)?;
            match (&**c1, &***c2) {
                (Const::Value(v1), Const::Value(v2)) => {
                    Some(Const::Value(compute_op(op, *v1, *v2, c2.loc)?))
                }
                (Const::Value(v), _) | (_, Const::Value(v))
                    if *op == ConstBiOp::Times && *v == 0 =>
                {
                    Some(Const::Value(0))
                }
                (_, Const::Value(v)) if *op == ConstBiOp::Div && *v == 0 => {
                    return Err(ComputeConstError::DivisionByZero(c2.loc))
                }
                (Const::Value(v), _) if *op == ConstBiOp::Div && *v == 0 => Some(Const::Value(0)),
                _ => None,
            }
        }
    };
    if let Some(res) = res {
        *c = res;
    }
    Ok(())
}

fn simplify_consts_in_statement(
    statement: &mut Statement,
    consts: &HashMap<String, Const>,
    static_args: &Vec<String>,
) -> Result<(), ComputeConstError> {
    let mut closure = |c: &mut Const| simplify_const(c, consts, static_args);
    map_consts_in_statement(statement, &mut closure)
}

pub fn compute_consts_in_statement(
    statement: &mut Statement,
    consts: &HashMap<String, Const>,
) -> Result<(), ComputeConstError> {
    let mut closure = |c: &mut Const| {
        *c = Const::Value(compute_const(c, consts)?);
        Ok(())
    };
    map_consts_in_statement(statement, &mut closure)
}

//Use a generic here, more clear
fn map_consts_in_statement<F>(statement: &mut Statement, f: &mut F) -> Result<(), ComputeConstError>
where
    F: FnMut(&mut Const) -> Result<(), ComputeConstError>,
{
    match statement {
        Statement::Assign(vec) => {
            for assign in vec {
                map_consts_in_expr(&mut assign.expr, f)?;
            }
            Ok(())
        }
        Statement::If(ifstruct) => {
            f(&mut ifstruct.condition)?;
            for stat in &mut ifstruct.if_block {
                map_consts_in_statement(stat, f)?;
            }
            for stat in &mut ifstruct.else_block {
                map_consts_in_statement(stat, f)?;
            }
            Ok(())
        }
        Statement::FnAssign(FnAssign {
            vars: _,
            f:
                FnCall {
                    name: _,
                    args,
                    static_args,
                },
        }) => {
            for arg in &mut **args {
                map_consts_in_expr(arg, f)?;
            }
            for arg in &mut **static_args {
                f(arg)?;
            }
            Ok(())
        }
    }
}

fn compute_op(op: &ConstBiOp, v1: i32, v2: i32, loc: Pos) -> Result<i32, ComputeConstError> {
    match op {
        ConstBiOp::Plus => Ok(v1 + v2),
        ConstBiOp::Times => Ok(v1 * v2),
        ConstBiOp::Div => {
            if v2 != 0 {
                Ok(v1 / v2)
            } else {
                return Err(ComputeConstError::DivisionByZero(loc));
            }
        }
        ConstBiOp::Minus => Ok(v1 - v2),
        ConstBiOp::Eq => Ok((v1 == v2) as i32),
        ConstBiOp::Neq => Ok((v1 != v2) as i32),
        ConstBiOp::Ge => Ok((v1 >= v2) as i32),
        ConstBiOp::Le => Ok((v1 <= v2) as i32),
        ConstBiOp::Gt => Ok((v1 > v2) as i32),
        ConstBiOp::Lt => Ok((v1 < v2) as i32),
        ConstBiOp::And => Ok(((v1 != 0) && (v2 != 0)) as i32),
        ConstBiOp::Or => Ok(((v1 != 0) || (v2 != 0)) as i32),
    }
}

fn compute_consts_in_expr(
    expr: &mut Expr,
    consts: &HashMap<String, Const>,
) -> Result<(), ComputeConstError> {
    let mut closure = |c: &mut Const| {
        *c = Const::Value(compute_const(c, consts)?);
        Ok(())
    };
    map_consts_in_expr(expr, &mut closure)
}

fn map_consts_in_expr<F>(expr: &mut Expr, f: &mut F) -> Result<(), ComputeConstError>
where
    F: FnMut(&mut Const) -> Result<(), ComputeConstError>,
{
    match expr {
        Expr::Const(ConstExpr::Unknown(_, c)) => f(c),
        Expr::Not(e) => map_consts_in_expr(e, f),
        Expr::Slice(e, c1, c2) => {
            f(c1)?;
            f(c2)?;
            map_consts_in_expr(e, f)
        }
        Expr::BiOp(_, e1, e2) => {
            map_consts_in_expr(e1, f)?;
            map_consts_in_expr(e2, f)
        }
        Expr::Mux(e1, e2, e3) => {
            map_consts_in_expr(e1, f)?;
            map_consts_in_expr(e2, f)?;
            map_consts_in_expr(e3, f)
        }
        Expr::Reg(c, e) => {
            f(c)?;
            map_consts_in_expr(e, f)
        }
        Expr::Ram(RamStruct {
            read_addr,
            write_enable,
            write_addr,
            write_data,
        }) => {
            map_consts_in_expr(read_addr, f)?;
            map_consts_in_expr(write_enable, f)?;
            map_consts_in_expr(write_addr, f)?;
            map_consts_in_expr(write_data, f)
        }
        Expr::Rom(RomStruct {
            word_size,
            read_addr,
        }) => {
            map_consts_in_expr(read_addr, f)?;
            f(word_size)
        }
        Expr::FnCall(FnCall {
            name: _,
            args,
            static_args,
        }) => {
            for arg in &mut **args {
                map_consts_in_expr(arg, f)?;
            }
            for arg in &mut **static_args {
                f(arg)?;
            }
            Ok(())
        }
        Expr::Var(_) | Expr::Const(_) | Expr::Last(_) => Ok(()),
    }
}
//replace the constants with simple Value(i32).
//This uses a dpeendancy solver, as the constant definition can be unordered
//(this allows for deterministically using constants from other files)
fn compute_global_consts(consts: &mut HashMap<String, Const>) -> Result<(), ComputeConstError> {
    if consts.is_empty() {
        return Ok(());
    }
    let mut depgraph = DepGraph::<String>::new();
    let start = "0".to_string(); //a const can't be named "0"
    let mut locs = HashMap::new();
    for (s, c) in consts.iter() {
        depgraph.register_dependency(start.clone(), s.to_string());
        let mut deps = Vec::new();
        get_dependancies(c, &mut deps, &mut locs);
        depgraph.register_dependencies(s.to_string(), deps)
    }

    for dep in depgraph.dependencies_of(&start)? {
        let dep = dep?.to_string(); //convert the result of reference to just &str
        if dep == start {
            continue;
        }
        let c = consts.get(&dep).ok_or_else(|| {
            ComputeConstError::UnknowVariable(
                *locs.get(&dep).expect(&format!("{:?}", dep)),
                dep.clone(),
            )
        })?;
        let res = compute_const(c, consts)?;
        consts.insert(dep, Const::Value(res)); //replace the const with its value
    }
    Ok(())
}

//Replace a constant with a single value, fails if it can't
pub fn compute_const(c: &Const, consts: &HashMap<String, Const>) -> Result<i32, ComputeConstError> {
    match c {
        Const::Value(i) => Ok(*i),
        Const::Var(v) => {
            if let Const::Value(i) = consts
                .get(&**v)
                .ok_or(ComputeConstError::UnknowVariable(v.loc, v.to_string()))?
            {
                Ok(*i)
            } else {
                Err(ComputeConstError::UnknowVariable(v.loc, v.to_string()))
            }
        }
        Const::BiOp(op, c1, c2) => {
            let v1 = compute_const(c1, consts)?;
            let v2 = compute_const(c2, consts)?;
            Ok(compute_op(op, v1, v2, c2.loc)?)
        }
    }
}
fn get_dependancies(c: &Const, deps: &mut Vec<String>, locs: &mut HashMap<String, Pos>) {
    match c {
        Const::Value(_) => (),
        Const::Var(s) => {
            locs.insert(s.value.to_string(), s.loc);
            deps.push(s.to_string());
        }
        Const::BiOp(_, c1, c2) => {
            get_dependancies(c1, deps, locs);
            get_dependancies(c2, deps, locs);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser_wrapper::parse;

    use super::compute_consts;
    #[test]
    fn test_consts() {
        let (mut prog, _) = parse("src/tests/compute_consts/pass/test_consts.tj".into()).unwrap();
        compute_consts(&mut prog).unwrap();
        println!("{:#?}", prog)
    }
}
