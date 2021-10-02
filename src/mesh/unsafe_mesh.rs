pub struct UnsafeMesh
{ 
    positions_ptr: *mut Vector3,
    positions_len: i32,

    triangles_ptr: *mut i32,
    triangles_len: i32,

    groups_ptr: *mut Group,
    groups_len: i32,
}

impl UnsafeMesh {

    // pub unsafe fn build(connected_mesh: &ConnectedMesh) -> Self {
    //     let ptr: *mut i32 = libc::malloc(std::mem::size_of::<int32>() * (input.intsToSum_len as usize)) as *mut i32;
    // }
}