use std::fmt::{Display, Formatter};
use std::hash::Hasher;

#[derive(Debug, Copy, Clone)]
pub struct Edge {
    pos_a: u32,
    pos_b: u32,
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
        let (lo, hi) = if self.pos_a < self.pos_b {
            (self.pos_a, self.pos_b)
        } else {
            (self.pos_b, self.pos_a)
        };
        (((hi as u64) << 32) | (lo as u64)).hash(state);
    }
}

impl Edge {
    fn new(pos_a: u32, pos_b: u32) -> Self {
        Self {
            pos_a: pos_a,
            pos_b: pos_b,
        }
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<pos A:{} pos B:{}>", self.pos_a, self.pos_b)
    }
}