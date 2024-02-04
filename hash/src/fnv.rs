use super::hash::Hasher;

/// refer: https://github.com/allegro/bigcache/blob/main/fnv.go

/// new_default_hasher returns a new 64-bit FNV-1a Hasher which makes no memory allocations.
/// Its Sum64 method will lay the value out in big-endian byte order.
/// See https://en.wikipedia.org/wiki/Fowler–Noll–Vo_hash_function
pub fn new_default_hasher() -> Box<dyn Hasher> {
    Box::new(Fnv64a::new())
}

/// offset64 FNVa offset basis. See https://en.wikipedia.org/wiki/Fowler–Noll–Vo_hash_function#FNV-1a_hash
const OFFSET64: u64 = 14695981039346656037;
/// prime64 FNVa prime value. See https://en.wikipedia.org/wiki/Fowler–Noll–Vo_hash_function#FNV-1a_hash
const PRIME64: u64 = 1099511628211;

pub struct Fnv64a {}

impl Fnv64a {
    pub fn new() -> Fnv64a {
        Fnv64a {}
    }
}

impl Hasher for Fnv64a {
    fn sum64(&self, key: &str) -> u64 {
        let mut hash: u64 = OFFSET64;
        for c in key.chars() {
            hash ^= c as u64;
            hash = hash.wrapping_mul(PRIME64);
        }
        hash
    }
    fn bucket_v1(&self, key: &str, bucket_num: u64, _bucket_mask: u64) -> u64 {
        self.sum64(key) % bucket_num
    }
    fn bucket_v2(&self, key: &str, _bucket_num: u64, bucket_mask: u64) -> u64 {
        self.sum64(key) & bucket_mask
    }
    fn get_bucket_mask(&self, num_buckets: u64) -> u64 {
        num_buckets - 1
    }
}
