use std::{collections::HashSet, sync::Arc};

#[tokio::main]
async fn main() {
    let n = 1_000_000;
    let threads = 12;
    let mut tasks = Vec::new();
    let set = Arc::new(tokio::sync::Mutex::new(HashSet::new()));
    for _ in 0..threads {
        let set = set.clone();
        let task = tokio::spawn(async move {
            for _ in 0..n {
                let id = idgen::v3::uuid();
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
