#[cfg(test)]
mod tests {
    use std::{collections::HashSet, sync::Arc};

    #[test]
    fn test_v1_duplicate() {
        let n = 1000000;
        let mut set = HashSet::new();
        for _ in 0..n {
            let id = idgen::v1::generate();
            assert!(!set.contains(&id));
            set.insert(id);
        }
    }

    #[tokio::test]
    async fn test_v1_duplicate_concurrency() {
        let n = 1000000;
        let threads = 8;
        let mut tasks = Vec::new();
        let set = Arc::new(tokio::sync::Mutex::new(HashSet::new()));
        for _ in 0..threads {
            let set = set.clone();
            let task = tokio::spawn(async move {
                for _ in 0..n {
                    let id = idgen::v1::generate();
                    let mut w = set.lock().await;
                    assert!(!w.contains(&id));
                    w.insert(id);
                }
            });
            tasks.push(task);
        }
        for task in tasks {
            task.await.unwrap();
        }
    }
    #[test]
    fn test_v3_duplicate() {
        let n = 1000000;
        let mut set = HashSet::new();
        for _ in 0..n {
            let id = idgen::v3::generate();
            assert!(!set.contains(&id));
            set.insert(id);
        }
    }

    #[tokio::test]
    async fn test_v3_duplicate_concurrency() {
        let n = 1000000;
        let threads = 8;
        let mut tasks = Vec::new();
        let set = Arc::new(tokio::sync::Mutex::new(HashSet::new()));
        for _ in 0..threads {
            let set = set.clone();
            let task = tokio::spawn(async move {
                for _ in 0..n {
                    let id = idgen::v3::generate();
                    let mut w = set.lock().await;
                    assert!(!w.contains(&id));
                    w.insert(id);
                }
            });
            tasks.push(task);
        }
        for task in tasks {
            task.await.unwrap();
        }
    }
}
