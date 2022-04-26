use crate::base::DVec3;
use super::Group;

pub struct SharedMesh {
    pub groups: Vec<Group>,
    pub triangles: Vec<u32>,
    pub positions: Vec<DVec3>,
    pub normals: Option<Vec<DVec3>>,
}