use super::executors::DynExecutor;

use rhai::{Dynamic, Engine, EvalAltResult};

// This module provides a configurable script executor that allows
// for custom engine configurations through strategies.

pub trait EngineConfigurationStrategy {
    fn configure_engine(&self, engine: &mut Engine);
}

pub struct ConfigurableExecutor {
    configurations: Option<Vec<Box<dyn EngineConfigurationStrategy>>>,
}

impl ConfigurableExecutor {
    pub fn new() -> Self {
        ConfigurableExecutor {
            configurations: None,
        }
    }

    pub fn load_configs(&mut self, configurations: Vec<Box<dyn EngineConfigurationStrategy>>) {
        self.configurations = Some(configurations);
    }

    fn configure_engine(&self) -> Engine {
        let mut engine = Engine::new();

        if let Some(configs) = &self.configurations {
            for config in configs {
                config.configure_engine(&mut engine);
            }
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
