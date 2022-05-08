impl From<&SharedMesh> for ConnectedMesh {
    fn from(shared_mesh: &SharedMesh) -> Self {
        let triangles = &shared_mesh.triangles;
        let mut nodes = vec![Node::default(); triangles.len() * 3];
        let mut vertex_to_nodes = HashMap::<u32, Vec<u32>, _>::with_hasher(
            BuildHasherDefault::<SimpleHasher>::default()
        );
        let mut face_count = 0;
        for i in 0..triangles.len() {
            let triangle = triangles[i];
            {
                let mut a = &mut nodes[i];
                a.position = triangle[0];
                a.normal = triangle[0];
                a.relative = (i as u32) + 1; // B
                if !vertex_to_nodes.contains_key(&a.position) {
                    vertex_to_nodes.insert(a.position, Vec::new());
                }
                vertex_to_nodes.get_mut(&a.position).unwrap().push(i as u32);
            }
            {
                let mut b = &mut nodes[i + 1];
                b.position = triangle[1];
                b.normal = triangle[1];
                b.relative = (i as u32) + 2; // C
                if !vertex_to_nodes.contains_key(&b.position) {
                    vertex_to_nodes.insert(b.position, Vec::new());
                }  
                vertex_to_nodes.get_mut(&b.position).unwrap().push((i as u32) + 1);
            }
            {
                let mut c = &mut nodes[i + 2];
                c.position = triangle[2];
                c.normal = triangle[2];
                c.relative = i as u32; // A
                if !vertex_to_nodes.contains_key(&c.position) {
                    vertex_to_nodes.insert(c.position, Vec::new());
                }
                vertex_to_nodes.get_mut(&c.position).unwrap().push((i as u32) + 2);
            }
            face_count = face_count + 1;
        }

        for x in vertex_to_nodes.values() {
            let mut previous_sibling: u32 = u32::MAX;
            let mut first_sibling: u32 = u32::MAX;
            for node in x.iter() {
                if first_sibling != u32::MAX {
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

        let mut per_vertex_map = HashMap::<[u32; 2], u32>::new();
        let mut browsed_nodes = HashSet::new();
        let mut triangles = Vec::<U32Vec3>::with_capacity((connected_mesh.face_count / 3) as usize);

        for i in 0..connected_mesh.nodes.len() {
            if connected_mesh.nodes[i].is_removed {
                continue;
            }
            //let key = [connected_mesh.nodes[i as usize].position, connected_mesh.nodes[i as usize].normal];
            if browsed_nodes.contains(&(i as u32)) {
                continue; // TODO: Useful ?
            }

            let mut triangle = U32Vec3::default();

            let mut x = 0;
            loop_relatives!(i as u32, connected_mesh.nodes, relative, {
                let key = [connected_mesh.nodes[relative as usize].position, connected_mesh.nodes[relative as usize].normal];
                if !per_vertex_map.contains_key(&key) {
                    per_vertex_map.insert(key, per_vertex_map.len() as u32);
                }
                
                if x > 2 {
                    panic!("caca");
                }
                triangle[x] = *per_vertex_map.get(&key).unwrap();
                browsed_nodes.insert(relative);
                x += 1;
            });

            triangles.push(triangle);
        }

        let mut positions = vec![DVec3::default(); per_vertex_map.len()];
        for (key, value) in &per_vertex_map {
            positions[*value as usize] = connected_mesh.positions[key[0] as usize];
        }

        let normals = match &connected_mesh.normals {
            Some(cm_normals) => {
                let mut snormals = vec![DVec3::default(); per_vertex_map.len()];
                for (key, value) in &per_vertex_map {
                    snormals[*value as usize] = cm_normals[key[1] as usize];
                }
                Some(snormals)
            },
            None => None,
        };

        return SharedMesh {
            groups: Vec::new(),
            triangles: triangles,
            positions: positions,
            normals: normals,
            colors: None,
        };
    }
}

impl Into<SharedMesh> for ConnectedMesh {
    fn into(self) -> SharedMesh {
        return SharedMesh::from(&self);
    }
}

#[cfg(test)]
mod builder_tests {

    use crate::base::*;
    use crate::mesh::*;

    #[test]
    fn shared_mesh_to_connected_mesh() {
        
        return; // todo: Fix this 

        let mut positions = Vec::new();
        // Build a square
        positions.push(DVec3::new(0., 0., 0.));
        positions.push(DVec3::new(1., 0., 0.));
        positions.push(DVec3::new(1., 1., 0.));
        positions.push(DVec3::new(0., 1., 0.));

        let mut triangles = Vec::new();
        // First triangle
        triangles.push(U32Vec3::new(0, 1, 2));
        // Second triangle
        triangles.push(U32Vec3::new(0, 2, 3));

        let shared_mesh = SharedMesh { 
            groups: Vec::new(),
            triangles: triangles,
            colors: None,
            positions: positions,
            normals: None,
        };

        let connected_mesh = ConnectedMesh::from(&shared_mesh);

        assert_eq!(connected_mesh.face_count, 2);
        assert_eq!(connected_mesh.positions.len(), 4);
        assert_eq!(connected_mesh.nodes.len(), 6);

        connected_mesh.nodes[0].relative;

        // Check relatives
        for i in 0..6 {
            let mut relatives = 0;
            loop_relatives!(i, connected_mesh.nodes, relative, {
                relatives += 1; 
            });
            assert_eq!(relatives, 3);
        }

        // Check siblings (connectivity)
        for i in [[0, 2], [1, 1], [2, 2], [3, 2], [4, 2], [5, 1]] {
            let mut siblings = 0;
            loop_siblings!(i[0], connected_mesh.nodes, sibling, {
                siblings += 1;
            });
            assert_eq!(siblings, i[1]);
        }
    }

    #[test]
    fn connected_mesh_to_shared_mesh() {

        let mut positions = Vec::new();
        // Build a square
        positions.push(DVec3::new(0., 0., 0.));
        positions.push(DVec3::new(1., 0., 0.));
        positions.push(DVec3::new(1., 1., 0.));
        positions.push(DVec3::new(0., 1., 0.));

        let mut nodes = Vec::new();
        nodes.push(Node::from_layout(0, 3, 1));
        nodes.push(Node::from_layout(1, 1, 2)); // sibling is itself
        nodes.push(Node::from_layout(2, 4, 0));
        nodes.push(Node::from_layout(0, 0, 4));
        nodes.push(Node::from_layout(2, 2, 5));
        nodes.push(Node::from_layout(3, 5, 3)); // sibling is itself

        let connected_mesh = ConnectedMesh {
            positions: positions,
            normals: None,
            nodes: nodes,
            face_count: 2,
        };

        let shared_mesh = SharedMesh::from(&connected_mesh);

        assert_eq!(shared_mesh.triangles.len(), 2);
        assert_eq!(shared_mesh.positions.len(), 4);
        assert_eq!(shared_mesh.triangles[0], U32Vec3::new(0, 1, 2));
        assert_eq!(shared_mesh.triangles[1], U32Vec3::new(0, 2, 3));
    }
}