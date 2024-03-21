use std::env;
use std::fs;

mod script_executor;
use script_executor::ScriptExecutor;

mod executor;
//use executor::point_dyn_executor;
use executor::{point_executor, BasicExecutor, DynExecutor, Executor};

fn main() {
    // Grab the location of RHAI file from CLI args
    // and turn it into the script
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <path_to_script.rhai>");
        std::process::exit(1);
    }
    let script_path = &args[1];
    let script = fs::read_to_string(script_path).expect("Failed to read script file");

    let executor = BasicExecutor::new();
    ScriptExecutor::execute_script(&executor, &script);
}
