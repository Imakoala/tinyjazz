fn main() {
    lalrpop::Configuration::new()
        .set_in_dir("src/parser_wrapper")
        .emit_rerun_directives(true)
        .process_current_dir()
        .unwrap();
    lalrpop::Configuration::new()
        .set_in_dir("src/frontend/from_netlist")
        .emit_rerun_directives(true)
        .process_current_dir()
        .unwrap();
}
