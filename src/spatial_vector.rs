use crate::conduit::Conduit;
use crate::executor::EngineConfigurationStrategy;
use rhai::{Dynamic, Engine, FLOAT};
use std::f64::consts::PI;

#[derive(Clone)]
pub struct SpatialVector {
    pub x: FLOAT,
    pub y: FLOAT,
}

impl SpatialVector {
    pub fn new(x: FLOAT, y: FLOAT) -> Self {
        SpatialVector { x, y }
    }

    pub fn magnitude(&self) -> FLOAT {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    // Returns the angle in degrees
    pub fn angle(&self) -> FLOAT {
        self.y.atan2(self.x) * 180.0 / PI
    }
}

impl Conduit for SpatialVector {
    fn create_from_dynamic(dynamic: Dynamic) -> Option<Self> {
        dynamic.try_cast::<Self>()
    }
}

pub struct SpatialVectorConfiguration;

impl EngineConfigurationStrategy for SpatialVectorConfiguration {
    fn configure_engine(&self, engine: &mut Engine) {
        engine
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
    }
}
