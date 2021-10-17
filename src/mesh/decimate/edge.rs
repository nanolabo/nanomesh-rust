use std::fmt::*;

#[derive(Debug, Copy, Clone)]
pub struct Edge {
    node_a: Node,
    node_b: Node,
    collapse_to: Vector3,
}

impl Eq for Edge {

}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.node_a.position == other.node_a.position && self.node_b.position == other.node_b.position)
     || (self.node_a.position == other.node_b.position && self.node_b.position == other.node_a.position)
    }
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node_a.position.hash(state);
        self.node_b.position.hash(state);
    }
}

impl Edge {
    fn new(node_a: Node, node_b: Node) -> Self {
        Self {
            node_a: node_a,
            node_b: node_b,
            collapse_to: Vector3::default()
        }
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<pos A:{} pos B:{}>", self.node_a.position, self.node_b.position)
    }
}