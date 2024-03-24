use crate::conduit::Conduit;
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

impl Conduit for Point {
    fn create_from_dynamic(dynamic: Dynamic) -> Option<Self> {
        dynamic.try_cast::<Self>()
    }
}
