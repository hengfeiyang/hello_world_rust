use hash::fnv::new_default_hasher;

#[tokio::main]
async fn main() {
    let hash = new_default_hasher();
    for key in ["hello", "world", "foo", "bar", "baz"].iter() {
        let ret = hash.sum64(key);
        println!("Hash of {} is {}", key, ret);
    } 
}
