use rhai::{Dynamic, EvalAltResult};

pub trait SimpleExecutor<T> {
    fn execute(&self, script: &str) -> Result<T, Box<EvalAltResult>>;
}

pub trait DynExecutor {
    fn dyn_execute(&self, script: &str) -> Result<Dynamic, Box<EvalAltResult>>;
}
