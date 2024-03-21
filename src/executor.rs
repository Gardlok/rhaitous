use rhai::{Dynamic, Engine, EvalAltResult, INT};

#[derive(Debug, Clone)]
pub struct Point {
    pub x: INT,
    pub y: INT,
}

impl Point {
    pub fn new(x: INT, y: INT) -> Self {
        Point { x, y }
    }

    pub fn length(&self) -> f64 {
        ((self.x.pow(2) as f64) + (self.y.pow(2) as f64)).sqrt()
    }
}

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
            .register_fn("length", Point::length);

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
