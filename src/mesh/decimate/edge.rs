use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub struct Edge {
    pos_a: i32,
    pos_b: i32,
    collapse_to: Vector3,
}

impl Eq for Edge {

}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.pos_a == other.pos_a && self.pos_b == other.pos_b)
     || (self.pos_a == other.pos_b && self.pos_b == other.pos_a)
    }
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut hash: u64 = (self.pos_a + self.pos_b) as u64;
        hash *= 13 * (self.pos_a - self.pos_b).abs() as u64; // TODO: Fine tune for optimal performance
        hash.hash(state);
    }
}

impl Edge {
    fn new(pos_a: i32, pos_b: i32) -> Self {
        Self {
            pos_a: pos_a,
            pos_b: pos_b,
            collapse_to: Vector3::default()
        }
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<pos A:{} pos B:{}>", self.pos_a, self.pos_b)
    }
}