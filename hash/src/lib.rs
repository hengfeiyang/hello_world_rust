
pub mod default_hasher;
pub mod fnv;
pub mod xxhash;

pub trait Sum64 {
    fn sum64(&mut self, key: &str) -> u64;
}
