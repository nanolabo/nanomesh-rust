use super::shared_mesh::SharedMesh;
use super::Group;
use crate::base::DVec3;
use crate::utils::r#unsafe::*;

pub struct UnsafeMesh
{ 
    positions_ptr: *mut DVec3,
    positions_len: i32,

    normals_ptr: *mut DVec3,
    normals_len: i32,

    triangles_ptr: *mut u32,
    triangles_len: i32,

    groups_ptr: *mut Group,
    groups_len: i32,
}

impl From<&SharedMesh> for UnsafeMesh {
    fn from(shared_mesh: &SharedMesh) -> Self {
        unsafe {
            return UnsafeMesh {
                groups_ptr: vec_to_ptr(&shared_mesh.groups),
                groups_len: shared_mesh.positions.len() as i32,
                triangles_ptr: vec_to_ptr(&shared_mesh.triangles),
                triangles_len: shared_mesh.triangles.len() as i32,
                positions_ptr: vec_to_ptr(&shared_mesh.positions),
                positions_len: shared_mesh.positions.len() as i32,
                normals_ptr: match &shared_mesh.normals { Some(normals) => vec_to_ptr(&normals), None => std::ptr::null_mut() },
                normals_len: match &shared_mesh.normals { Some(normals) => normals.len() as i32, None => 0 },
            };
        }
    }
}

impl Into<SharedMesh> for UnsafeMesh {
    fn into(self) -> SharedMesh {
        return SharedMesh::from(&self);
    }
}

impl From<&UnsafeMesh> for SharedMesh {
    fn from(unsafe_mesh: &UnsafeMesh) -> Self {
        unsafe {
            return SharedMesh {
                groups: ptr_to_vec(unsafe_mesh.groups_ptr, unsafe_mesh.groups_len as usize),
                triangles: ptr_to_vec(unsafe_mesh.triangles_ptr, unsafe_mesh.triangles_len as usize),
                positions: ptr_to_vec(unsafe_mesh.positions_ptr, unsafe_mesh.positions_len as usize),
                normals: match unsafe_mesh.normals_ptr.is_null() { false => Some(ptr_to_vec(unsafe_mesh.normals_ptr, unsafe_mesh.normals_len as usize)), true => None },
            };
        }
    }
}

impl Into<UnsafeMesh> for SharedMesh {
    fn into(self) -> UnsafeMesh {
        return UnsafeMesh::from(&self);
    }
}