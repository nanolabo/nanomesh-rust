impl From<&SharedMesh> for ConnectedMesh {
    fn from(shared_mesh: &SharedMesh) -> Self {
        let triangles = &shared_mesh.triangles;
        let mut nodes = vec![Node::default(); triangles.len()];
        let mut vertex_to_nodes = HashMap::<i32, Vec<i32>, _>::with_hasher(
            BuildHasherDefault::<SimpleHasher>::default()
        );
        let mut face_count = 0;
        let mut i: usize = 0;
        loop {
            {
                let mut a = &mut nodes[i];
                a.position = triangles[i];
                a.normal = triangles[i];
                a.relative = (i as i32) + 1; // B
                if !vertex_to_nodes.contains_key(&a.position) {
                    vertex_to_nodes.insert(a.position, Vec::new());
                }
                vertex_to_nodes.get_mut(&a.position).unwrap().push(i as i32);
            }
            {
                let mut b = &mut nodes[i + 1];
                b.position = triangles[i + 1];
                b.normal = triangles[i + 1];
                b.relative = (i as i32) + 2; // C
                if !vertex_to_nodes.contains_key(&b.position) {
                    vertex_to_nodes.insert(b.position, Vec::new());
                }  
                vertex_to_nodes.get_mut(&b.position).unwrap().push((i as i32) + 1);
            }
            {
                let mut c = &mut nodes[i + 2];
                c.position = triangles[i + 2];
                c.normal = triangles[i + 2];
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

        return ConnectedMesh { 
            positions: shared_mesh.positions.clone(),
            normals: shared_mesh.normals.clone(),
            nodes: nodes,
            face_count: face_count };
    }
}

impl Into<ConnectedMesh> for SharedMesh {
    fn into(self) -> ConnectedMesh {
        return ConnectedMesh::from(&self);
    }
}

impl From<&ConnectedMesh> for SharedMesh {
    fn from(connected_mesh: &ConnectedMesh) -> Self {

        let mut per_vertex_map = HashMap::<[i32; 2], i32>::new();
        let mut browsed_nodes = HashSet::new();
        let mut triangles = Vec::<i32>::with_capacity((connected_mesh.face_count * 3) as usize);

        for i in 0..connected_mesh.nodes.len() {
            if connected_mesh.nodes[i].is_removed {
                continue;
            }
            let key = [connected_mesh.nodes[i as usize].position, connected_mesh.nodes[i as usize].normal];
            if browsed_nodes.contains(&(i as i32)) {
                continue; // TODO: Useful ?
            }
            loop_relatives!(i as i32, connected_mesh.nodes, relative, {
                let key = [connected_mesh.nodes[relative as usize].position, connected_mesh.nodes[relative as usize].normal];
                if !per_vertex_map.contains_key(&key) {
                    per_vertex_map.insert(key, per_vertex_map.len() as i32);
                }
                triangles.push(*per_vertex_map.get(&key).unwrap());
                browsed_nodes.insert(relative);
            });
        }

        let mut positions = vec![Vector3::default(); per_vertex_map.len()];
        for (key, value) in &per_vertex_map {
            positions[*value as usize] = connected_mesh.positions[key[0] as usize];
        }

        let mut normals = Vec::new();
        if connected_mesh.normals.len() > 0 {
            normals = vec![Vector3::default(); per_vertex_map.len()];
            for (key, value) in &per_vertex_map {
                normals[*value as usize] = connected_mesh.normals[key[1] as usize];
            }
        }

        return SharedMesh {
            positions: positions,
            normals: normals,
            triangles: triangles,
            groups: Vec::new() };
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
                normals_ptr: vec_to_ptr(&shared_mesh.normals),
                normals_len: shared_mesh.normals.len() as i32,
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
                normals: ptr_to_vec(unsafe_mesh.normals_ptr, unsafe_mesh.normals_len as usize),
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

#[cfg(test)]
mod builder_tests {

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
            normals: Vec::new(),
            groups: Vec::new() 
        };

        let connected_mesh = ConnectedMesh::from(&shared_mesh);

        assert_eq!(connected_mesh.face_count, 2);
        assert_eq!(connected_mesh.positions.len(), 4);
        assert_eq!(connected_mesh.nodes.len(), 6);

        connected_mesh.nodes[0].relative;

        // Check relatives
        for i in 0..6 {
            let mut relatives = 0;
            loop_relatives!(i, connected_mesh.nodes, relative, { relatives = relatives + 1; });
            assert_eq!(relatives, 3);
        }

        // Check siblings (connectivity)
        for i in [[0, 2], [1, 1], [2, 2], [3, 2], [4, 2], [5, 1]] {
            let mut siblings = 0;
            loop_siblings!(i[0], connected_mesh.nodes, sibling, { siblings = siblings + 1; });
            assert_eq!(siblings, i[1]);
        }
    }

    #[test]
    fn connected_mesh_to_shared_mesh() {

        let mut positions = Vec::new();
        // Build a square
        positions.push(Vector3::new(0., 0., 0.));
        positions.push(Vector3::new(1., 0., 0.));
        positions.push(Vector3::new(1., 1., 0.));
        positions.push(Vector3::new(0., 1., 0.));

        let mut nodes = Vec::new();
        nodes.push(Node { position: 0, normal: 0, sibling: 3, relative: 1, is_removed: false });
        nodes.push(Node { position: 1, normal: 0, sibling: 1, relative: 2, is_removed: false }); // sibling is itself
        nodes.push(Node { position: 2, normal: 0, sibling: 4, relative: 0, is_removed: false });
        nodes.push(Node { position: 0, normal: 0, sibling: 0, relative: 4, is_removed: false });
        nodes.push(Node { position: 2, normal: 0, sibling: 2, relative: 5, is_removed: false });
        nodes.push(Node { position: 3, normal: 0, sibling: 5, relative: 3, is_removed: false }); // sibling is itself

        let connected_mesh = ConnectedMesh {
            positions: positions,
            normals: Vec::new(),
            nodes: nodes,
            face_count: 2,
        };

        let shared_mesh = SharedMesh::from(&connected_mesh);

        assert_eq!(shared_mesh.triangles.len(), 6);
        assert_eq!(shared_mesh.positions.len(), 4);

        for i in 0..4 {
            assert!(shared_mesh.triangles.contains(&i));
        }
    }
}