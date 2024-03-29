use crate::conduit::Conduit;
use crate::executor::EngineConfigurationStrategy;
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

// pub struct PointConfiguration;

// impl EngineConfiguration for PointConfiguration {
//     fn configure_engine(&self, engine: &mut Engine) {
//         // Configure engine for Point operations
//         engine
//             .register_type_with_name::<Point>("Point")
//             .register_fn("create_point", Point::new)
//             .register_get_set("x", |p: &mut Point| p.x, |p: &mut Point, v: INT| p.x = v)
//             .register_get_set("y", |p: &mut Point| p.y, |p: &mut Point, v: INT| p.y = v)
//             .register_fn("length", Point::length);
//     }
// }

pub struct PointConfiguration;

impl EngineConfigurationStrategy for PointConfiguration {
    fn configure_engine(&self, engine: &mut Engine) {
        engine
            .register_type_with_name::<Point>("Point")
            .register_fn("create_point", Point::new)
            .register_get_set("x", |p: &mut Point| p.x, |p: &mut Point, v: INT| p.x = v)
            .register_get_set("y", |p: &mut Point| p.y, |p: &mut Point, v: INT| p.y = v)
            .register_fn("length", Point::length);
    }
}
