use std::hash::Hasher;
        
pub struct EdgeHasher {
    hash: u64,
}

impl Default for EdgeHasher {
    fn default() -> EdgeHasher {
        EdgeHasher { hash: 0 }
    }
}

impl Hasher for EdgeHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }
    #[inline]
    fn write(&mut self, _bytes: &[u8]) { 
    }
    #[inline]
    fn write_i32(&mut self, i: i32) {
        self.hash = self.hash + i as u64;
    }
}