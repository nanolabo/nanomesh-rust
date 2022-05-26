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

    pub fn new_from_points<'a>(vals: impl IntoIterator<Item = &'a DVec3>) -> Self {

        let mut min = DVec3::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max = DVec3::new(f64::MIN, f64::MIN, f64::MIN);

        for val in vals.into_iter() {
            min.x = min.x.min(val.x);
            min.y = min.y.min(val.y);
            min.z = min.z.min(val.z);
            max.x = max.x.max(val.x);
            max.y = max.y.max(val.y);
            max.z = max.z.max(val.z);
        }

        return Box3::new(min, max);
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