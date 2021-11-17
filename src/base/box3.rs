use super::vector3::Vector3;
use std::fmt::*;

#[derive(Debug, Copy, Clone)]
pub struct Box3 {
    pub min: Vector3,
    pub max: Vector3,
}

impl Box3
{
    pub fn new(min: Vector3, max: Vector3) -> Self {
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
            min: Vector3::default(),
            max: Vector3::default()
        }
    }

    pub fn unfitted() -> Self {
        Box3 {
            min: Vector3 { x: f64::MAX, y: f64::MAX, z: f64::MAX },
            max: Vector3 { x: f64::MIN, y: f64::MIN, z: f64::MIN }
        }
    }
}

impl Default for Box3 {
    fn default() -> Self {
        Box3 {
            min: Vector3::default(),
            max: Vector3::default()
        }
    }
}

impl Display for Box3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<min:{} max:{}>", self.min, self.max)
    }
}