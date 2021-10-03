impl<const A: usize> From<&SharedMesh> for ConnectedMesh<A> {
    fn from(shared_mesh: &SharedMesh) -> Self {
        let triangles = &shared_mesh.triangles;
        let mut nodes = vec![Node{ position: 0, attributes: [0; A], relative: 0, sibling: 0 }; triangles.len()];
        let mut vertex_to_nodes = HashMap::<i32, Vec<i32>, _>::with_hasher(
            BuildHasherDefault::<SimpleHasher>::default()
        );
        let mut face_count = 0;
        let mut i: usize = 0;
        loop {
            {
                let mut a = &mut nodes[i];
                a.position = triangles[i];
                a.attributes = [0; A];
                a.relative = (i as i32) + 1; // B
                if !vertex_to_nodes.contains_key(&a.position) {
                    vertex_to_nodes.insert(a.position, Vec::new());
                }
                vertex_to_nodes.get_mut(&a.position).unwrap().push(i as i32);
            }
            {
                let mut b = &mut nodes[i + 1];
                b.position = triangles[i + 1];
                b.attributes = [0; A];
                b.relative = (i as i32) + 2; // C
                if !vertex_to_nodes.contains_key(&b.position) {
                    vertex_to_nodes.insert(b.position, Vec::new());
                }  
                vertex_to_nodes.get_mut(&b.position).unwrap().push((i as i32) + 1);
            }
            {
                let mut c = &mut nodes[i + 2];
                c.position = triangles[i + 2];
                c.attributes = [0; A];
                c.relative = i as i32; // A
                if !vertex_to_nodes.contains_key(&c.position) {
                    vertex_to_nodes.insert(c.position, Vec::new());
                }
                vertex_to_nodes.get_mut(&c.position).unwrap().push((i as i32) + 2);
            }
            face_count = face_count + 1;

            i = i + 3;
            if i >= triangles.len() {
                break;
            }
        }

        for x in vertex_to_nodes.values() {
            let mut previous_sibling: i32 = -1;
            let mut first_sibling: i32 = -1;
            for node in x.iter() {
                if first_sibling != -1 {
                    nodes[*node as usize].sibling = previous_sibling;
                }
                else {
                    first_sibling = *node;
                }
                previous_sibling = *node;
            }
            nodes[first_sibling as usize].sibling = previous_sibling;
        }

        return ConnectedMesh { positions: shared_mesh.positions.clone(), nodes: nodes, face_count: face_count };
    }
}

impl<const A: usize> Into<ConnectedMesh<A>> for SharedMesh {
    fn into(self) -> ConnectedMesh<A> {
        return ConnectedMesh::from(&self);
    }
}

impl<const A: usize> From<&ConnectedMesh<A>> for SharedMesh {
    fn from(connected_mesh: &ConnectedMesh<A>) -> Self {

        let mut vertex_to_nodes = HashMap::<i32, Vec<i32>, _>::with_hasher(
            BuildHasherDefault::<SimpleHasher>::default()
        );

        return SharedMesh {
            positions: Vec::new(),
            triangles: Vec::new(),
            normals: None,
            groups: Vec::new() };
    }
}

impl<const A: usize> Into<SharedMesh> for ConnectedMesh<A> {
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
                normals_ptr: vec_to_ptr(&Vec::new()),
                normals_len: 0,
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
                normals: None,
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

macro_rules! loop_relatives {
    ($node_index:expr, $nodes:expr, $exec:expr) => {{
        let mut relative = $node_index;
        loop {
            $exec
            relative = $nodes[relative as usize].relative;
            if relative == $node_index {
                break;
            }
        }
    }};
}

macro_rules! loop_siblings {
    ($node_index:expr, $nodes:expr, $exec:expr) => {{
        let mut sibling = $node_index;
        loop {
            $exec
            sibling = $nodes[sibling as usize].sibling;
            if sibling == $node_index {
                break;
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    use crate::base::*;
    use crate::mesh::*;

    #[test]
    fn shared_mesh_to_connected_mesh() {
        
        let mut positions = Vec::new();
        // Build a square
        positions.push(Vector3::new(0., 0., 0.));
        positions.push(Vector3::new(1., 0., 0.));
        positions.push(Vector3::new(1., 1., 0.));
        positions.push(Vector3::new(0., 1., 0.));

        let mut triangles = Vec::new();
        // First triangle
        triangles.push(0);
        triangles.push(1);
        triangles.push(2);
        // Second triangle
        triangles.push(0);
        triangles.push(2);
        triangles.push(3);

        let shared_mesh = SharedMesh { 
            positions: positions,
            triangles: triangles,
            normals: None,
            groups: Vec::new() 
        };
        let connected_mesh = ConnectedMesh::<0>::from(&shared_mesh);

        assert_eq!(connected_mesh.face_count, 2);
        assert_eq!(connected_mesh.positions.len(), 4);
        assert_eq!(connected_mesh.nodes.len(), 6);

        connected_mesh.nodes[0].relative;

        // Check relatives
        for i in 0..6 {
            let mut relatives = 0;
            loop_relatives!(i, connected_mesh.nodes, { relatives = relatives + 1; });
            assert_eq!(relatives, 3);
        }

        // Check siblings (connectivity)
        for i in [[0, 2], [1, 1], [2, 2], [3, 2], [4, 2], [5, 1]] {
            let mut siblings = 0;
            loop_siblings!(i[0], connected_mesh.nodes, { siblings = siblings + 1; });
            assert_eq!(siblings, i[1]);
        }
    }
}