use once_cell::sync::Lazy;
use rand::distributions::{Alphanumeric, DistString};
use snowflake::SnowflakeIdBucket;

static mut IDER: Lazy<SnowflakeIdBucket> = Lazy::new(|| SnowflakeIdBucket::new(1, 1));

pub fn init() -> Result<(), anyhow::Error> {
    _ = generate();
    Ok(())
}

pub fn generate() -> String {
    let id = unsafe { IDER.get_id() };
    format!("{}{}", id, generate_random_string(6))
}

pub fn generate_random_string(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), len)
}
