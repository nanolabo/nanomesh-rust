pub struct ConnectedMesh {
    positions: Vec<Vector3>,
    nodes: Vec<Node>,
    face_count: u32,
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