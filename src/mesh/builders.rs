impl From<&SharedMesh> for ConnectedMesh {
    fn from(shared_mesh: &SharedMesh) -> Self {
        return ConnectedMesh { positions: Vec::new(), nodes: Vec::new(), face_count: 0 };
    }
}

impl Into<ConnectedMesh> for SharedMesh {
    fn into(self) -> ConnectedMesh {
        return ConnectedMesh::from(&self);
    }
}

impl From<&ConnectedMesh> for SharedMesh {
    fn from(connected_mesh: &ConnectedMesh) -> Self {
        return SharedMesh { positions: Vec::new(), triangles: Vec::new(), groups: Vec::new() };
    }
}

impl Into<SharedMesh> for ConnectedMesh {
    fn into(self) -> SharedMesh {
        return SharedMesh::from(&self);
    }
}

impl From<&SharedMesh> for UnsafeMesh {
    fn from(shared_mesh: &SharedMesh) -> Self {
        unsafe {
            return UnsafeMesh {
                positions_ptr: vec_to_ptr(&shared_mesh.positions),
                positions_len: shared_mesh.positions.len() as i32,
    
                triangles_ptr: vec_to_ptr(&shared_mesh.triangles),
                triangles_len: shared_mesh.triangles.len() as i32,
    
                groups_ptr: vec_to_ptr(&shared_mesh.groups),
                groups_len: shared_mesh.positions.len() as i32,
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
                positions: ptr_to_vec(unsafe_mesh.positions_ptr, unsafe_mesh.positions_len as usize),
                triangles: ptr_to_vec(unsafe_mesh.triangles_ptr, unsafe_mesh.triangles_len as usize),
                groups: ptr_to_vec(unsafe_mesh.groups_ptr, unsafe_mesh.groups_len as usize),
            };
        }
    }
}

impl Into<UnsafeMesh> for SharedMesh {
    fn into(self) -> UnsafeMesh {
        return UnsafeMesh::from(&self);
    }
}