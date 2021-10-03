pub struct SharedMesh {
    positions: Vec<Vector3>,
    normals: Option<Vec<Vector3>>,
    triangles: Vec<i32>,
    groups: Vec<Group>,
}