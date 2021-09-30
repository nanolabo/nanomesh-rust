extern crate libc;

use crate::Vector3;
use crate::utils::r#unsafe::*;

pub struct Group {
    first_index: i32,
    index_count: i32,
}

pub struct SharedMesh {
    positions: Vec<Vector3>,
    triangles: Vec<i32>,
    groups: Vec<Group>,
}

pub struct UnsafeMesh
{ 
    positions_ptr: *mut Vector3,
    positions_len: i32,

    triangles_ptr: *mut i32,
    triangles_len: i32,

    groups_ptr: *mut Group,
    groups_len: i32,
} 

pub struct ConnectedMesh {
    positions: Vec<Vector3>,
    nodes: Vec<Node>,
    face_count: u32,
}

impl UnsafeMesh {

    // pub unsafe fn build(connected_mesh: &ConnectedMesh) -> Self {
    //     let ptr: *mut i32 = libc::malloc(std::mem::size_of::<int32>() * (input.intsToSum_len as usize)) as *mut i32;
    // }
}


impl ConnectedMesh {

    pub unsafe fn build(unsafe_mesh: &UnsafeMesh) -> Self {
        // Iterate over pointer
        for i in 0..unsafe_mesh.positions_len {
            let _position: *mut Vector3 = unsafe_mesh.positions_ptr.offset(i as isize);
        }
        // Buffer copy
        let positions = ptr_to_vec(unsafe_mesh.positions_ptr, unsafe_mesh.positions_len as usize);
        return ConnectedMesh { positions: positions, nodes: Vec::new(), face_count: 0 };
    }

    pub fn decimate(&mut self) {
        let v1 = &self.positions[0];
        let v2 = &self.positions[1];
        let product = v1 ^ v2;
        self.positions.push(product);
    }
}

struct Node {
    position: i32,
    sibling: i32,
    relative: i32,
    attribute: i32,
}

impl Node {
    pub fn mark_removed(&mut self) {
        self.position = 10;
    }
}
