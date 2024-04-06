#![allow(dead_code)]

use std::env;
use std::fs;

mod conduit;
pub use conduit::Conduit;

use crate::strategies::{
    FileCatConfiguration, PointConfiguration, SpatialVectorConfiguration, StringHandler,
    StringHandlerConfiguration,
};

mod executor;
use crate::executor::{ConfigurableExecutor, EngineConfigurationStrategy, ScriptExecutor};

mod strategies;
mod utils;

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

    // configurations for engine using configs specified in models/
    let configurations: Vec<Box<dyn EngineConfigurationStrategy>> = vec![
        Box::new(PointConfiguration {}),
        Box::new(SpatialVectorConfiguration {}),
        Box::new(FileCatConfiguration {}),
        Box::new(StringHandlerConfiguration {}),
    ];

    // execute the engine on script
    let executor = ConfigurableExecutor::new(configurations);
    ScriptExecutor::execute_script(&executor, &script);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::executor::{BasicExecutor, DynExecutor};
    use rhai::Engine;
    use std::fs;

    #[test]
    fn test_execute_script_returns_string() {
        let script = r#"return "Hello, World!";"#;
        let executor = BasicExecutor::new();
        let result = executor.dyn_execute(script).unwrap();
        assert_eq!(result.to_string(), "Hello, World!");
    }

    #[test]
    fn test_script_execution_error_handling() {
        let executor = BasicExecutor::new();
        // Deliberately incorrect script to trigger an error
        let script = "let v = undefined_function();";
        let result = executor.dyn_execute(script);
        assert!(result.is_err());
    }

    #[test]
    fn test_string_handler() {
        let mut engine = Engine::new();
        StringHandlerConfiguration.configure_engine(&mut engine);

        let result = engine
            .eval::<StringHandler>(
                r#"
            let s = new_string("Hello");
            s.append(", World!");
            s
            "#,
            )
            .unwrap();

        assert_eq!(result.content, "Hello, World!");
    }
}
