use std::collections::HashSet;

use crate::{ast::*, typed_ast::Var};

/*
This file aims to make all transitions use only shared variables, to make them reusable in other nodes.
It makes it possible to inline automatons.
*/

fn get_shared_rename(counter: usize, node_name: &str, mod_name: &str) -> String {
    format!("s_r$t{}${}${}", counter, node_name, mod_name)
}

pub fn make_transitions_shared(prog: &mut Program) {
    for (mod_name, module) in prog.modules.iter_mut() {
        for (node_name, node) in module.nodes.iter_mut() {
            let mut statement = Vec::new();
            for (i, (e, _, _)) in node.transitions.iter_mut().enumerate() {
                let new_name = Loc::new(e.loc, format!("s_r$t{}${}${}", i, node_name, mod_name));
                module.shared.push(VarAssign {
                    var: new_name.clone(),
                    expr: Loc::new(new_name.loc, Expr::Const(ConstExpr::Known(vec![false]))),
                });
                let old_expr = std::mem::replace(e, Loc::new(e.loc, Expr::Var(new_name.clone())));
                statement.push(VarAssign {
                    var: new_name,
                    expr: old_expr,
                });
            }
            node.statements.push(Statement::Assign(statement));
        }
    }
}
