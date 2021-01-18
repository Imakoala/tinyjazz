use rhai::{Array, Engine, Scope};
//This returns a closure with no arguments. Each time it is called, it calls the rhai script
//with always the same context (so some form of continuity can be kept)
//and returns the output of the script.
pub fn get_inputs_closure(
    path: Option<String>,
    inputs: Vec<usize>,
) -> Box<dyn FnMut() -> Vec<Vec<bool>>> {
    if let Some(path) = path {
        let engine = Engine::new();
        let mut scope = Scope::new();
        let mut ast = engine.compile_file(path.into()).unwrap();
        Box::new(move || {
            let mut array: Array = engine.eval_ast_with_scope(&mut scope, &mut ast).unwrap();
            array
                .drain(..)
                .map(|d| {
                    d.try_cast::<Array>()
                        .expect("script returned wrong value")
                        .drain(..)
                        .map(|d| d.try_cast::<bool>().expect("script returned wrong value"))
                        .collect()
                })
                .collect()
        })
    } else {
        Box::new(move || inputs.iter().map(|s| vec![false; *s]).collect())
    }
}
