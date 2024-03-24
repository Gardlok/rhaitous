use crate::{point::Point, spatial_vector::SpatialVector};
use rhai::{Dynamic, Engine, EvalAltResult, FLOAT, INT};

pub trait Executor {
    fn execute(&self, script: &str) -> Result<Point, Box<EvalAltResult>>;
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

impl Executor for BasicExecutor {
    fn execute(&self, script: &str) -> Result<Point, Box<EvalAltResult>> {
        let mut engine = Engine::new();

        engine
            .register_type_with_name::<Point>("Point")
            .register_fn("create_point", Point::new)
            .register_get_set("x", |p: &mut Point| p.x, |p: &mut Point, v: INT| p.x = v)
            .register_get_set("y", |p: &mut Point| p.y, |p: &mut Point, v: INT| p.y = v)
            .register_fn("length", Point::length);

        let result = engine.eval::<Point>(script)?;
        Ok(result)
    }
}

impl DynExecutor for BasicExecutor {
    fn dyn_execute(&self, script: &str) -> Result<Dynamic, Box<EvalAltResult>> {
        let mut engine = Engine::new();

        engine
            .register_type_with_name::<Point>("Point")
            .register_fn("create_point", Point::new)
            .register_get_set("x", |p: &mut Point| p.x, |p: &mut Point, v: INT| p.x = v)
            .register_get_set("y", |p: &mut Point| p.y, |p: &mut Point, v: INT| p.y = v)
            .register_fn("length", Point::length)
            .register_type_with_name::<SpatialVector>("SpatialVector")
            .register_fn("new_spatial_vector", SpatialVector::new)
            .register_get_set(
                "x",
                |v: &mut SpatialVector| v.x,
                |v: &mut SpatialVector, val: FLOAT| v.x = val,
            )
            .register_get_set(
                "y",
                |v: &mut SpatialVector| v.y,
                |v: &mut SpatialVector, val: FLOAT| v.y = val,
            )
            .register_fn("magnitude", SpatialVector::magnitude)
            .register_fn("angle", SpatialVector::angle);

        let result = engine.eval::<Dynamic>(script)?;
        Ok(result)
    }
}

// Helpers
pub fn point_executor(script: String) {
    let executor = BasicExecutor::new();
    match executor.execute(&script) {
        Ok(point) => {
            println!("Script executed successfully. Point: {:?}", point);
            println!("Point length: {}", point.length());
        }
        Err(e) => eprintln!("Script execution failed: {:?}", e),
    }
}