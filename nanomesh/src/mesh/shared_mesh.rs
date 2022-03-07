pub struct SharedMesh {
    pub groups: Vec<Group>,
    pub triangles: Vec<u32>,
    pub positions: Vec<Vector3>,
    pub normals: Option<Vec<Vector3>>,
}