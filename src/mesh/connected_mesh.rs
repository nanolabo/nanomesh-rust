pub struct ConnectedMesh {
    nodes: Vec<Node>,
    face_count: u32,

    positions: Vec<Vector3>,
    normals: Vec<Vector3>,
    // uv0: Vec<Vector3>,
    // colors: Vec<Vector3>,
}

impl ConnectedMesh {    
    pub fn decimate(&mut self) {
        let v1 = &self.positions[0];
        let v2 = &self.positions[1];
        let product = v1 ^ v2;
        self.positions.push(product);
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