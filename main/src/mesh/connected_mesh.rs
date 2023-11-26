use std::fmt::*;

type U32Map = HashMap::<u32, u32>;
type U32Set = HashSet::<u32>;

pub struct ConnectedMesh {
    nodes: Vec<Node>,
    face_count: u32,

    positions: Vec<DVec3>,
    normals: Option<Vec<DVec3>>,
    // uv0: Vec<Vector3>,
    // colors: Vec<Vector3>,
}

impl Default for ConnectedMesh {
    fn default() -> ConnectedMesh {
        ConnectedMesh { 
            positions: Vec::new(),
            normals: None,
            nodes: Vec::new(),
            face_count: 0
        }
    }
}

macro_rules! loop_relatives {
    ($node_index:expr, $nodes:expr, $relative:ident, $exec:expr) => {{
        let mut $relative: u32 = $node_index;
        loop {
            $exec
            $relative = $nodes[$relative as usize].relative;
            if $relative == $node_index {
                break;
            }
        }
    }};
}

macro_rules! loop_siblings {
    ($node_index:expr, $nodes:expr, $sibling:ident, $exec:expr) => {{
        let mut $sibling: u32 = $node_index;
        loop {
            $exec
            $sibling = $nodes[$sibling as usize].sibling;
            if $sibling == $node_index {
                break;
            }
        }
    }};
}

impl ConnectedMesh {

    fn check_siblings(&self, node_index: u32) -> bool {
        let mut i = 0;
        loop_siblings!(node_index, self.nodes, sibling, {
            i += 1;
            if i > 100
            {
                return false;
            }        
        });
        return true;
    }

    fn print_siblings(&self, node_index: u32) -> bool {
        let mut i = 0;
        loop_siblings!(node_index, self.nodes, sibling, {
            print!("{} ", sibling);
            if self.nodes[sibling as usize].is_removed {
                print!("X ");
            }
            print!("> ");
            i += 1;
            if i > 100
            {
                print!("...\n");
                return false;
            }        
        });
        print!("\n");
        return true;
    }

    fn check_relatives(&self, node_index: u32) -> bool {
        let mut i = 0;
        loop_relatives!(node_index, self.nodes, relative, {
            i += 1;
            if i > 100
            {
                return false;
            }        
        });
        return true;
    }

    fn collapse_edge_to_a(&mut self, node_index_a: u32, node_index_b: u32, position_to_node: &mut Option<&mut U32Map>) -> Option<u32> {

        let pos_a = self.nodes[node_index_a as usize].position;
        let pos_b = self.nodes[node_index_b as usize].position;

        debug_assert!(pos_a != pos_b);

        loop_siblings!(node_index_a, self.nodes, sibling_of_a, {
            let mut is_face_touched = false;
            let mut face_edge_count = 0;
            let mut node_index_c = u32::MAX;

            loop_relatives!(sibling_of_a, self.nodes, relative_of_a, {
                let pos_c = self.nodes[relative_of_a as usize].position;
                if pos_b == pos_c {
                    is_face_touched = true;
                } else if pos_a != pos_c {
                    node_index_c = relative_of_a;
                }
                face_edge_count += 1;
            });

            debug_assert!(face_edge_count == 3);

            if is_face_touched {
                loop_relatives!(sibling_of_a, self.nodes, relative_of_a, {
                    self.nodes[relative_of_a as usize].is_removed = true;
                    //println!("mark removed {}", relative_of_a)
                });

                //debug_assert!(self.print_siblings(node_index_c));

                let valid_node_at_c = self.reconnect_sibling(node_index_c);

                match position_to_node {
                    Some(pos_to_node) => { 
                        match valid_node_at_c {
                            Some(valid_node_at_c) => pos_to_node.insert(self.nodes[node_index_c as usize].position, valid_node_at_c),
                            None => pos_to_node.remove(&self.nodes[node_index_c as usize].position),
                        };
                    },
                    None => (),
                };

                //debug_assert!(self.print_siblings(v_c));

                //let pos_c = self.nodes[node_index_c as usize].position;
                //update position_to_nodes
                self.face_count -= 1;
            }
        });

        //debug_assert!(self.print_siblings(node_index_a));

        let valid_node_at_a = self.reconnect_siblings(node_index_a, node_index_b, pos_a);

        match position_to_node {
            Some(pos_to_node) => { 
                match valid_node_at_a {
                    Some(valid_node_at_a) => pos_to_node.insert(pos_a, valid_node_at_a),
                    None => pos_to_node.remove(&pos_a),
                };
                pos_to_node.remove(&pos_b);
            },
            None => (),
        };

        //debug_assert!(self.print_siblings(v_a));
        
        return valid_node_at_a;
    }

    fn reconnect_siblings(&mut self, node_index_a: u32, node_index_b: u32, position: u32) -> Option<u32> {
        let mut last_valid = u32::MAX;
        let mut first_valid = u32::MAX;

        loop_siblings!(node_index_a, self.nodes, sibling, {
            if !self.nodes[sibling as usize].is_removed {
                if first_valid == u32::MAX {
                    first_valid = sibling;
                }
                if last_valid != u32::MAX {
                    self.nodes[last_valid as usize].sibling = sibling;
                    self.nodes[last_valid as usize].position = position;
                }
                last_valid = sibling;
            }
        });

        loop_siblings!(node_index_b, self.nodes, sibling, {
            if !self.nodes[sibling as usize].is_removed {
                if first_valid == u32::MAX {
                    first_valid = sibling;
                }
                if last_valid != u32::MAX {
                    self.nodes[last_valid as usize].sibling = sibling;
                    self.nodes[last_valid as usize].position = position;
                }
                last_valid = sibling;
            }
        });

        if last_valid == u32::MAX {
            return None; // All siblings were removed
        }

        // Close the cloop
        self.nodes[last_valid as usize].sibling = first_valid;
        self.nodes[last_valid as usize].position = position;

        return Some(first_valid);
    }

    fn reconnect_sibling(&mut self, node_index: u32) -> Option<u32> {
        let mut last_valid = u32::MAX;
        let mut first_valid = u32::MAX;
        let mut position = u32::MAX;

        loop_siblings!(node_index, self.nodes, sibling, {
            if !self.nodes[sibling as usize].is_removed {
                if first_valid == u32::MAX {
                    first_valid = sibling;
                    position = self.nodes[sibling as usize].position;
                }
                if last_valid != u32::MAX {
                    self.nodes[last_valid as usize].sibling = sibling;
                    self.nodes[last_valid as usize].position = position;
                }
                last_valid = sibling;
            }
        });

        if last_valid == u32::MAX {
            return None; // All siblings were removed
        }

        // Close the cloop
        self.nodes[last_valid as usize].sibling = first_valid;
        self.nodes[last_valid as usize].position = position;

        return Some(first_valid);
    }

    fn get_edge_topo(&self, node_index_a: u32, node_index_b: u32) -> f64 {
        let pos_b = self.nodes[node_index_b as usize].position;
        let mut faces_attached = 0;
        let mut attribute_at_a: u32 = u32::MAX;
        let mut attribute_at_b: u32 = u32::MAX;
        let mut edge_weight = 0.0;
        
        loop_siblings!(node_index_a, self.nodes, sibling_of_a, {
            if !self.nodes[sibling_of_a as usize].is_removed {
                // Maybe we should begin directly with relative?
                loop_relatives!(sibling_of_a, self.nodes, relative_of_a, {
                    let pos_c = self.nodes[relative_of_a as usize].position;
                    if pos_c == pos_b {
                        faces_attached = faces_attached + 1;
    
                        match &self.normals {
                            Some(normals) => {
                                if attribute_at_b != u32::MAX && normals[attribute_at_b as usize] == normals[self.nodes[relative_of_a as usize].normal as usize] {
                                    edge_weight = edge_weight + 10.0
                                }
                                if attribute_at_a != u32::MAX && normals[attribute_at_a as usize] == normals[self.nodes[sibling_of_a as usize].normal as usize] {
                                    edge_weight = edge_weight + 10.0
                                }
                            },
                            None => ()
                        }
    
                        attribute_at_b = self.nodes[relative_of_a as usize].normal;
                        attribute_at_a = self.nodes[sibling_of_a as usize].normal;
                    }
                });
            }
        });

        // Check if border
        if faces_attached < 2 {
            edge_weight = edge_weight + 100.0;
        }

        return edge_weight;
    }

    fn get_face_normal(&mut self, node_index: u32) -> DVec3 {
        let node_a = self.nodes[node_index as usize];
        let node_b = self.nodes[node_a.relative as usize];
        let node_c = self.nodes[node_b.relative as usize];
        let pos_a = &self.positions[node_a.position as usize];
        let pos_b = &self.positions[node_b.position as usize];
        let pos_c = &self.positions[node_c.position as usize];
        (&(pos_b - pos_a).cross(&(pos_c - pos_a))).normalize()
    }
}

include!("decimate/decimate.rs");

#[derive(Debug, Copy, Clone)]
pub struct Node {
    sibling: u32,
    relative: u32,

    position: u32,
    normal: u32,
    // uv0: u32,
    // color: u32,

    is_removed: bool,
}

impl Node {
    fn from_layout(position: u32, sibling: u32, relative: u32) -> Self {
        Node { position: position, sibling: sibling, relative: relative,  normal: 0, is_removed: false }
    }
}

impl Default for Node {
    fn default() -> Self {
        Node { position: 0, sibling: 0, relative: 0,  normal: 0, is_removed: false }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<sibling:{} relative:{} position:{} removed:{}>", self.sibling, self.relative, self.position, self.is_removed)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod connected_mesh_tests {
    use super::*;

    fn build_test_mesh() -> ConnectedMesh {

        // A────────────────────────B
        // │ ╲           ...---̅ ̅ ̅ ╱ │
        // │  ╲ ...---̅ ̅ ̅         ╱  │
        // │   E────────────────F   │
        // │  ╱ ̅ ̅ ̅ ---...        ╲  │
        // │ ╱           ̅ ̅ ̅ ---...╲ │
        // D────────────────────────C

        let mut positions = Vec::new();
        positions.push(DVec3::new(0., 1., 0.)); // A (0)
        positions.push(DVec3::new(2., 1., 0.)); // B (1)
        positions.push(DVec3::new(2., 0., 0.)); // C (2)
        positions.push(DVec3::new(0., 0., 0.)); // D (3)
        positions.push(DVec3::new(0.25, 0.5, 0.)); // E (4)
        positions.push(DVec3::new(1.75, 0.5, 0.)); // F (5)

        let mut nodes = Vec::new();
        // Face AED
        nodes.push(Node::from_layout(0, 3, 1)); // (0)
        nodes.push(Node::from_layout(4, 5, 2)); // (1)
        nodes.push(Node::from_layout(3, 9, 0)); // (2)
        // Face ABE
        nodes.push(Node::from_layout(0, 0, 4)); // (3)
        nodes.push(Node::from_layout(1, 15, 5)); // (4)
        nodes.push(Node::from_layout(4, 6, 3)); // (5)
        // Face EBF
        nodes.push(Node::from_layout(4, 12, 7)); // (6)
        nodes.push(Node::from_layout(1, 4, 8)); // (7)
        nodes.push(Node::from_layout(5, 17, 6)); // (8)
        // Face DEC
        nodes.push(Node::from_layout(3, 2, 10)); // (9)
        nodes.push(Node::from_layout(4, 1, 11)); // (10)
        nodes.push(Node::from_layout(2, 14, 9)); // (11)
        // Face EFC
        nodes.push(Node::from_layout(4, 10, 13)); // (12)
        nodes.push(Node::from_layout(5, 8, 14)); // (13)
        nodes.push(Node::from_layout(2, 16, 12)); // (14)
        // Face BCF
        nodes.push(Node::from_layout(1, 7, 16)); // (15)
        nodes.push(Node::from_layout(2, 11, 17)); // (16)
        nodes.push(Node::from_layout(5, 13, 15)); // (17)

        let connected_mesh = ConnectedMesh { 
            positions: positions,
            nodes: nodes,
            normals: None,
            face_count: 6 };

        // Verify connectivity
        for i in 0..connected_mesh.nodes.len() {
            assert_eq!(connected_mesh.check_siblings(i as u32), true); 
            assert_eq!(connected_mesh.check_relatives(i as u32), true);
        }

        assert_eq!(connected_mesh.face_count, 6);

        return connected_mesh;
    }

    #[test]
    fn collapse_EF_to_E() {
        let mut connected_mesh = build_test_mesh();

        connected_mesh.collapse_edge_to_a(1 /*a node of E*/, 8 /*a node of F*/, &mut None);

        let mut nodes_removed = 0;

        for i in 0..connected_mesh.nodes.len() {
            if connected_mesh.nodes[i].is_removed {
                nodes_removed += 1;
            } else {
                // Verify that connectivity is valid
                assert_eq!(connected_mesh.check_siblings(i as u32), true); 
                assert_eq!(connected_mesh.check_relatives(i as u32), true);
                // Verify that position of a valid node is never F (5), since it is supposed to be removed
                assert_eq!(connected_mesh.nodes[i].position == 5, false);
            }
        }
        
        // There should be 2 faces removed
        assert_eq!(connected_mesh.face_count, 4);
        // There should be 2 faces removed, which implies 6 nodes
        assert_eq!(nodes_removed, 6); 
    }

    #[test]
    fn collapse_EF_to_F() {
        let mut connected_mesh = build_test_mesh();

        connected_mesh.collapse_edge_to_a(8 /*a node of F*/, 1 /*a node of E*/, &mut None);

        let mut nodes_removed = 0;

        for i in 0..connected_mesh.nodes.len() {
            if connected_mesh.nodes[i].is_removed {
                nodes_removed += 1;
            } else {
                // Verify that connectivity is valid
                assert_eq!(connected_mesh.check_siblings(i as u32), true); 
                assert_eq!(connected_mesh.check_relatives(i as u32), true);
                // Verify that position of a valid node is never E (4), since it is supposed to be removed
                assert_eq!(connected_mesh.nodes[i].position == 4, false);
            }
        }
        
        // There should be 2 faces removed
        assert_eq!(connected_mesh.face_count, 4);
        // There should be 2 faces removed, which implies 6 nodes
        assert_eq!(nodes_removed, 6); 
    }
}