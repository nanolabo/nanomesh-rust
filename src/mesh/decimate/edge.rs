pub struct Edge {
    node_a: Node,
    node_b: Node,
}

impl Eq for Edge {

}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.node_a.position == other.node_a.position
     && self.node_b.position == other.node_b.position
    }
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node_a.position.hash(state);
        self.node_b.position.hash(state);
    }
}