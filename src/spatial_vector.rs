use crate::conduit::Conduit;
use rhai::{Dynamic, FLOAT};
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

pub fn new_spatial_vector(x: FLOAT, y: FLOAT) -> SpatialVector {
    SpatialVector { x, y }
}
