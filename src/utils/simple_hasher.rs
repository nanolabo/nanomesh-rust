use std::hash::Hasher;
pub struct SimpleHasher(u64);

#[inline]
fn load_u64_le(buf: &[u8], len: usize) -> u64 {
    use std::ptr;
    debug_assert!(len <= buf.len());
    let mut data = 0u64;
    unsafe {
        ptr::copy_nonoverlapping(buf.as_ptr(), &mut data as *mut _ as *mut u8, len);
    }
    data.to_le()
}


impl Default for SimpleHasher {

    #[inline]
    fn default() -> SimpleHasher {
        SimpleHasher(0)
    }
}

impl Hasher for SimpleHasher {

    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        *self = SimpleHasher(load_u64_le(bytes, bytes.len()));
    }
    
    #[inline]
    fn write_i32(&mut self, i: i32) {
        let mut hash: u64 = i as u64;
        hash ^= hash >> 33;
        hash = hash.wrapping_mul(0xff51afd7ed558ccd);
        hash ^= hash >> 33;
        hash = hash.wrapping_mul(0xc4ceb9fe1a85ec53);
        hash ^= hash >> 33;
        self.0 = hash;
    }
}