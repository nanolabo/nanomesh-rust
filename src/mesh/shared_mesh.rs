pub struct SharedMesh {
    pub positions: Vec<Vector3>,
    pub normals: Vec<Vector3>,
    pub triangles: Vec<u32>,
    pub groups: Vec<Group>,
}