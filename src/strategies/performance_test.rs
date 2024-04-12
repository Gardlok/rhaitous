// use super::DynExecutor;
use crate::{executor::DynExecutor, EngineConfigurationStrategy, ScriptExecutor};
use rhai::{Dynamic, Engine, EvalAltResult, OptimizationLevel};
use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

pub struct PerformanceExecutor;

impl PerformanceExecutor {
    pub fn new() -> Self {
        PerformanceExecutor {}
    }
}

pub struct PerformanceEngineConfiguration;

impl EngineConfigurationStrategy for PerformanceEngineConfiguration {
    fn configure_engine(&self, engine: &mut Engine) {
        engine.set_max_call_levels(5000);
        engine.set_optimization_level(OptimizationLevel::Full);
    }
}
