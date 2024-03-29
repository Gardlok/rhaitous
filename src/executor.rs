use rhai::{Dynamic, Engine, EvalAltResult};

pub trait EngineConfiguration {
    fn configure_engine(&self, engine: &mut Engine);
}

pub trait Executor<T> {
    fn execute(&self, script: &str) -> Result<T, Box<EvalAltResult>>;
}

pub trait DynExecutor {
    fn dyn_execute(&self, script: &str) -> Result<Dynamic, Box<EvalAltResult>>;
}

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

pub trait EngineConfigurationStrategy {
    fn configure_engine(&self, engine: &mut Engine);
}

pub struct ConfigurableExecutor {
    configurations: Vec<Box<dyn EngineConfigurationStrategy>>,
}

impl ConfigurableExecutor {
    pub fn new(configurations: Vec<Box<dyn EngineConfigurationStrategy>>) -> Self {
        ConfigurableExecutor { configurations }
    }

    fn configure_engine(&self) -> Engine {
        let mut engine = Engine::new();
        for config in &self.configurations {
            config.configure_engine(&mut engine);
        }
        engine
    }
}

impl DynExecutor for ConfigurableExecutor {
    fn dyn_execute(&self, script: &str) -> Result<Dynamic, Box<EvalAltResult>> {
        let engine = self.configure_engine();
        engine.eval::<Dynamic>(script)
    }
}
