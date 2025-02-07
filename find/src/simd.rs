pub fn find(haystack: &str, needle: &str) -> bool {
    memchr::memmem::find(haystack.as_bytes(), needle.as_bytes()).is_some()
}