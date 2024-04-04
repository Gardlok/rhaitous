use rhai::EvalAltResult;

pub trait Executor<T> {
    fn execute(&self, script: &str) -> Result<T, Box<EvalAltResult>>;
}
