pub struct UnsafeMesh
{ 
    positions_ptr: *mut Vector3,
    positions_len: i32,

    normals_ptr: *mut Vector3,
    normals_len: i32,

    triangles_ptr: *mut u32,
    triangles_len: i32,

    groups_ptr: *mut Group,
    groups_len: i32,
}