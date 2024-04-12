use rhai::{Dynamic, Engine, EvalAltResult};

use super::{DynExecutor, SimpleExecutor};

// Defines a simple executor that uses the default configuration of
// the Rhai Engine to execute scripts.
pub struct BasicExecutor;

impl BasicExecutor {
    pub fn new() -> Self {
        BasicExecutor
    }
}

impl DynExecutor for BasicExecutor {
    fn dyn_execute(&self, script: &str) -> Result<Dynamic, Box<EvalAltResult>> {
        let engine = Engine::new();
        let result = engine.eval::<Dynamic>(script)?;
        Ok(result)
    }
}
