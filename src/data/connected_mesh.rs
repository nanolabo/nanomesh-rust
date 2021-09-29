use crate::Vector3;

pub struct ConnectedMesh {
    positions: Vec<Vector3>,
    nodes: Vec<Node>,
    faceCount: u32,
}

impl ConnectedMesh {
    pub fn Decimate(&mut self) {
        let v1 = &self.positions[0];
        let v2 = &self.positions[1];
        self.positions.push(v1 ^ v2);
    }
}

struct Node {
    position: i32,
    sibling: i32,
    relative: i32,
    attribute: i32,
}

impl Node {
    pub fn MarkRemoved(&mut self) {
        self.position = 10;
    }
}
