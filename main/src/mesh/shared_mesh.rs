use nalgebra_glm as glm;
use glm::{DVec3, U32Vec3};
use super::Group;
use std::convert::TryInto;

pub struct SharedMesh {
    pub groups: Vec<Group>,
    pub triangles: Vec<U32Vec3>,
    pub positions: Vec<DVec3>,
    pub normals: Option<Vec<DVec3>>,
    pub colors: Option<Vec<DVec3>>,
}

impl SharedMesh {
    // Combine two triangulations with an associative binary operator
    // (why yes, this _is_ a monoid)
    pub fn combine(mut a: Self, b: Self) -> Self {
        let dv: u32 = a.positions.len().try_into()
            .expect("Cannot handle more than 4,294,967,295 triangles");
        a.positions.extend(b.positions);
        a.triangles.extend(b.triangles.into_iter()
            .map(|t| U32Vec3::new(t[0] + dv, t[1] + dv, t[2] + dv)));
        a
    }
}

impl Default for SharedMesh {
    fn default() -> Self {
        Self {
            groups: Vec::new(),
            triangles: Vec::new(),
            positions: Vec::new(),
            normals: Some(Vec::new()),
            colors: Some(Vec::new()),
        }
    }
}