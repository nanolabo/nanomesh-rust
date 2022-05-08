use nalgebra_glm as glm;
use glm::{DVec3};
use std::fmt::*;

#[derive(Debug, Copy, Clone)]
pub struct Box3 {
    pub min: DVec3,
    pub max: DVec3,
}

impl Box3
{
    pub fn new(min: DVec3, max: DVec3) -> Self {
        return Box3 { 
            min: min,
            max: max
        }
    }

    pub fn diagonal(&self) -> f64 {
        return (&self.max - &self.min).magnitude();
    }

    pub fn zero() -> Self {
        Box3 {
            min: DVec3::default(),
            max: DVec3::default()
        }
    }

    pub fn unfitted() -> Self {
        Box3 {
            min: DVec3::new(f64::MAX, f64::MAX, f64::MAX),
            max: DVec3::new(f64::MIN, f64::MIN, f64::MIN)
        }
    }
}

impl Default for Box3 {
    fn default() -> Self {
        Box3 {
            min: DVec3::default(),
            max: DVec3::default()
        }
    }
}

impl Display for Box3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<min:{} max:{}>", self.min, self.max)
    }
}