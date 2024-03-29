#![allow(dead_code)]

use std::env;
use std::fs;

mod conduit;

mod point;
use point::PointConfiguration;

mod spatial_vector;
use spatial_vector::SpatialVectorConfiguration;

mod file_concat;
use file_concat::FileCatConfiguration;

mod script_executor;
use script_executor::ScriptExecutor;

mod executor;
use executor::{ConfigurableExecutor, EngineConfigurationStrategy};

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

    // configurations for engine
    let configurations: Vec<Box<dyn EngineConfigurationStrategy>> = vec![
        Box::new(PointConfiguration {}),
        Box::new(SpatialVectorConfiguration {}),
        Box::new(FileCatConfiguration {}),
    ];

    // execute engine on script
    let executor = ConfigurableExecutor::new(configurations);
    ScriptExecutor::execute_script(&executor, &script);
}
