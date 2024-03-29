#![allow(dead_code)]

use std::env;
use std::fs;

mod conduit;

mod point;
use point::PointConfiguration;
mod spatial_vector;
use spatial_vector::SpatialVectorConfiguration;

mod script_executor;
use script_executor::ScriptExecutor;

mod executor;
use executor::{
    BasicExecutor, ConfigurableExecutor, EngineConfiguration, EngineConfigurationStrategy,
};

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

    // configurations (strategies)
    let point_configuration = Box::new(PointConfiguration {});
    // let spatial_vector_configuration = Box::new(SpatialVectorConfiguration {});

    // Instantiate the ConfigurableExecutor with the desired configurations
    // let configurations = vec![point_configuration, spatial_vector_configuration];
    // let configurations = vec![point_configuration];

    let configurations: Vec<Box<dyn EngineConfigurationStrategy>> = vec![
        Box::new(PointConfiguration {}),
        Box::new(SpatialVectorConfiguration {}),
    ];

    let executor = ConfigurableExecutor::new(configurations);

    // Execute the script using the newly configured executor
    ScriptExecutor::execute_script(&executor, &script);

    // let executor = BasicExecutor::new();
    // ScriptExecutor::execute_script(&executor, &script);
}
