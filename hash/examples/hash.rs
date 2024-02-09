use hash::{default_hasher, Sum64};

#[tokio::main]
async fn main() {
    let mut h = default_hasher::new();
    for key in ["hello", "world", "foo", "bar", "baz"].iter() {
        let ret = h.sum64(key);
        println!("Hash of {} is {}", key, ret);
    }
}
