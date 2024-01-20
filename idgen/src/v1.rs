use std::sync::Mutex;

use once_cell::sync::Lazy;
use snowflake::SnowflakeIdBucket;

static IDER: Lazy<Mutex<SnowflakeIdBucket>> =
    Lazy::new(|| Mutex::new(SnowflakeIdBucket::new(1, 1)));

pub fn init() -> Result<(), anyhow::Error> {
    _ = generate();
    Ok(())
}

pub fn generate() -> String {
    let id = IDER.lock().unwrap().get_id();
    id.to_string()
}
