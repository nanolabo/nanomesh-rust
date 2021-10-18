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
        panic!("Not supposed to be called");
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.hash = i;
    }
}