use rhai::{Dynamic, Engine, EvalAltResult};

use super::DynExecutor;

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
