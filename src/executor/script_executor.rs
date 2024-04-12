use rhai::{Dynamic, Engine, EvalAltResult};

// This module handles the execution of scripts using the Rhai
// scripting engine and maps the results to Rust structures.

use super::DynExecutor;
use crate::strategies::{Point, SpatialVector, StringHandler};
use crate::Conduit;

pub struct ScriptExecutor;

impl ScriptExecutor {
    pub fn new() -> Self {
        ScriptExecutor {}
    }

    pub fn execute_script(executor: &dyn DynExecutor, script: &str) {
        match executor.dyn_execute(script) {
            Ok(result) => {
                Self::handle_result(result);
            }
            Err(e) => eprintln!("Script execution failed: {:?}", e),
        }
    }

    // Handles the result of a script execution, casting the dynamic type to
    // known types and acting on them
    fn handle_result(result: Dynamic) {
        // Attempt to cast Dynamic to various types and act accordingly

        // Reduce the type name, removing the namespaces, returning the core type
        let simple_type_name = get_simple_type_name(result.type_name());

        // Determine the type from the Dynamic result
        match simple_type_name {
            "Point" => {
                if let Some(point) = result.try_cast::<Point>() {
                    println!(
                        "Script executed successfully. Point length: {}",
                        point.length()
                    );
                } else {
                    println!("Type matched 'Point' but could not be cast.");
                }
            }
            "SpatialVector" => {
                if let Some(vector) = SpatialVector::create_from_dynamic(result) {
                    println!(
                        "Script executed successfully. SpatialVector magnitude: {}, angle: {}",
                        vector.magnitude(),
                        vector.angle()
                    );
                } else {
                    println!("Type matched 'SpatialVector' but could not be cast.");
                }
            }
            "string" => {
                if let Some(string_result) = result.try_cast::<String>() {
                    println!("Script executed successfully. Result: {}", string_result);
                } else {
                    println!("Expected a 'string' but could not cast to a String.");
                }
            }
            "i64" => {
                if let Some(num) = result.try_cast::<i64>() {
                    println!("Script executed successfully. Integer result: {}", num);
                } else {
                    println!("Type matched 'i64' but could not be cast.");
                }
            }
            _ => println!(
                "Script executed, but the result type '{}' was not recognized.",
                result.type_name()
            ),
        }
    }
}

// Helpers

// Simplifies a type name by removing the namespace components
fn get_simple_type_name(full_type_name: &str) -> &str {
    full_type_name.split("::").last().unwrap_or_default()
}
