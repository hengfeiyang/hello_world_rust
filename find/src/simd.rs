use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

static CACHE: Lazy<RwLock<HashMap<String, Arc<memchr::memmem::Finder>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub fn find(haystack: &str, needle: &str) -> bool {
    memchr::memmem::find(haystack.as_bytes(), needle.as_bytes()).is_some()
}

pub fn find_cached(haystack: &str, needle: &'static str) -> bool {
    // fast path
    let cache = CACHE.read();
    if let Some(finder) = cache.get(needle) {
        return finder.find(haystack.as_bytes()).is_some();
    }
    drop(cache);

    // slow path
    let finder = Arc::new(memchr::memmem::Finder::new(needle.as_bytes()));
    let mut cache = CACHE.write();
    cache.insert(needle.to_string(), finder.clone());
    finder.find(haystack.as_bytes()).is_some()
}
