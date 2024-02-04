// Hasher is responsible for generating unsigned, 64 bit hash of provided string. Hasher should minimize collisions
// (generating same hash for different strings) and while performance is also important fast functions are preferable (i.e.
// you can use FarmHash family).
pub trait Hasher {
    fn sum64(&self, key: &str) -> u64;
    fn bucket_v1(&self, key: &str, num_buckets: u64, bucket_mask: u64) -> u64;
    fn bucket_v2(&self, key: &str, num_buckets: u64, bucket_mask: u64) -> u64;
    fn get_bucket_mask(&self, num_buckets: u64) -> u64;
}
