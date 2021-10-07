pub struct ConnectedMesh {
    nodes: Vec<Node>,
    face_count: u32,

    positions: Vec<Vector3>,
    normals: Vec<Vector3>,
    // uv0: Vec<Vector3>,
    // colors: Vec<Vector3>,
}

macro_rules! loop_relatives {
    ($node_index:expr, $nodes:expr, $relative:ident, $exec:expr) => {{
        let mut $relative: i32 = $node_index;
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
        let mut $sibling: i32 = $node_index;
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
    pub fn decimate(&mut self) {
        let v1 = &self.positions[0];
        let v2 = &self.positions[1];
        let product = v1 ^ v2;
        self.positions.push(product);
    }

    fn collapse_edge(&mut self, node_index_A: i32, node_index_B: i32) {

        let pos_A = self.nodes[node_index_A as usize].position;
        let pos_B = self.nodes[node_index_B as usize].position;

        debug_assert!(pos_A != pos_B);

        loop_siblings!(node_index_A, self.nodes, sibling_of_A, {
            let mut is_face_touched = false;
            let mut face_edge_count = 0;
            let mut node_index_C = -1;

            loop_relatives!(sibling_of_A, self.nodes, relative_of_A, {
                let pos_C = self.nodes[relative_of_A as usize].position;
                if pos_B == pos_C {
                    is_face_touched = true;
                } else if pos_A != pos_C {
                    node_index_C = relative_of_A;
                }
                face_edge_count = face_edge_count + 1;
            });

            debug_assert!(face_edge_count == 3);

            if is_face_touched {
                let pos_C = self.nodes[node_index_C as usize].position;
                loop_relatives!(sibling_of_A, self.nodes, relative_of_A, {
                    self.nodes[relative_of_A as usize].is_removed = true;
                });
                //let valid_node_at_C = reconnect_siblings(node_index_C);
                //update position_to_nodes
                self.face_count = self.face_count - 1;
            }
        });

        //let valid_node_at_A = reconnect_siblings(node_index_A, node_index_B, pos_A);
        //update position_to_nodes
    }
}

pub struct Group {
    first_index: i32,
    index_count: i32,
}

#[derive(Clone)]
pub struct Node {
    sibling: i32,
    relative: i32,

    position: i32,
    normal: i32,
    // uv0: i32,
    // color: i32,

    is_removed: bool,
}

impl Default for Node {
    fn default() -> Self {
        Node{ position: 0, normal: 0, relative: 0, sibling: 0, is_removed: false }
    }
}