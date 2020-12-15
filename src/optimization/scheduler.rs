/*
This module orders the nodes so that all shared variables are set before they are used.
It fails if there is a cycle.
TODO : it should also fail if a shared variable can be assigned to twice.
*/

use super::graphs::*;
use solvent::DepGraph;
pub enum ScheduleError {
    CycleError,
    Other(String),
}

impl From<solvent::SolventError> for ScheduleError {
    fn from(error: solvent::SolventError) -> Self {
        match error {
            solvent::SolventError::CycleDetected => ScheduleError::CycleError,
            solvent::SolventError::NoSuchNode => ScheduleError::Other(
                "Unknown error while computing constants dependancy graph".to_string(),
            ),
        }
    }
}

pub fn schedule(prog: &Vec<ProgramNode>, n_shared: usize) -> Result<Vec<usize>, ScheduleError> {
    let mut source_map = vec![Vec::new(); n_shared]; //what node compute each shared var.
    for (id, pnode) in prog.iter().enumerate() {
        for (o, _) in &pnode.shared_outputs {
            source_map[*o].push(id)
        }
    }
    let mut depgraph = DepGraph::<usize>::new();
    for (id, pnode) in prog.iter().enumerate() {
        depgraph.register_dependency(usize::MAX, id);
        for i in &pnode.inputs {
            depgraph.register_dependencies(id, source_map[*i].clone())
        }
    }
    let mut v: Vec<usize> = depgraph
        .dependencies_of(&usize::MAX)?
        .map(|n| match n {
            Ok(u) => Ok(*u),
            Err(e) => Err(e),
        })
        .collect::<Result<Vec<usize>, solvent::SolventError>>()?;
    v.pop(); //remove the usize::MAX dep
    Ok(v)
}
