// Copyright 2023 Zinc Labs Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use bytes::Bytes;
use hashbrown::HashMap;
use once_cell::sync::Lazy;
use std::collections::VecDeque;
use std::{
    cmp::{max, min},
    ops::Range,
};
use tokio::sync::RwLock;

const MAX_SIZE: usize = 1024 * 1024 * 1024;
const RELEASE_SIZE: usize = 1024 * 1024 * 100;

pub type RwHashMap<K, V> = RwLock<HashMap<K, V>>;

static FILES: Lazy<RwLock<FileData>> = Lazy::new(|| RwLock::new(FileData::new()));
static DATA: Lazy<RwHashMap<String, Bytes>> =
    Lazy::new(|| RwLock::new(HashMap::with_capacity(200000)));

pub struct FileData {
    max_size: usize,
    cur_size: usize,
    data: VecDeque<String>,
    size_data: HashMap<String, usize>,
    lock: RwLock<()>,
}

impl Default for FileData {
    fn default() -> Self {
        Self::new()
    }
}

impl FileData {
    pub fn new() -> FileData {
        FileData::with_capacity(MAX_SIZE)
    }

    pub fn with_capacity(max_size: usize) -> FileData {
        FileData {
            max_size,
            cur_size: 0,
            data: VecDeque::with_capacity(200000),
            size_data: HashMap::with_capacity(200000),
            lock: RwLock::new(()),
        }
    }

    async fn exist(&mut self, file: &str) -> bool {
        self.size_data.contains_key(file)
    }

    async fn get(&self, file: &str, range: Option<Range<usize>>) -> Option<Bytes> {
        let c = DATA.read().await;
        let data = c.get(file)?;
        Some(if let Some(range) = range {
            data.slice(range)
        } else {
            data.clone()
        })
    }

    async fn set(
        &mut self,
        session_id: &str,
        file: &str,
        data: Bytes,
    ) -> Result<(), anyhow::Error> {
        let data_size = file.len() + data.len();
        let mut data_client = DATA.write().await;
        let _permit = self.lock.write().await;
        if self.cur_size + data_size >= self.max_size {
            println!(
                "[session_id {session_id}] File memory cache is full {}/{}, can't cache extra {} bytes",
                self.cur_size,
                self.max_size,
                data_size
            );
            // cache is full, need release some space
            let need_release_size = min(MAX_SIZE, max(RELEASE_SIZE, data_size * 100));
            let mut release_size = 0;
            loop {
                let item = self.data.pop_front();
                if item.is_none() {
                    println!("[session_id {session_id}] File memory cache is corrupt, it shouldn't be none");
                    break;
                }
                let key = item.unwrap();
                let data_size = self.size_data.remove(&key).unwrap();
                // remove file from data cache
                data_client.remove(&key);
                release_size += data_size;
                if release_size >= need_release_size {
                    break;
                }
            }
            self.cur_size -= release_size;
            data_client.shrink_to_fit();
        }

        self.cur_size += data_size;
        self.data.push_back(file.to_string());
        self.size_data.insert(file.to_string(), data_size);
        // write file into cache
        data_client.insert(file.to_string(), data);
        Ok(())
    }

    async fn _pop(&mut self) -> Option<String> {
        self.data.pop_front()
    }

    async fn len(&self) -> (usize, usize, usize) {
        (
            self.data.len(),
            self.size_data.len(),
            DATA.read().await.len(),
        )
    }
}

#[inline]
pub async fn get(file: &str, range: Option<Range<usize>>) -> Option<Bytes> {
    let files = FILES.read().await;
    files.get(file, range).await
}

#[inline]
pub async fn exist(file: &str) -> bool {
    let mut files = FILES.write().await;
    files.exist(file).await
}

#[inline]
pub async fn set(session_id: &str, file: &str, data: Bytes) -> Result<(), anyhow::Error> {
    let mut files = FILES.write().await;
    files.set(session_id, file, data).await
}

#[inline]
pub async fn len() -> (usize, usize, usize) {
    let files = FILES.read().await;
    files.len().await
}

pub async fn download(session_id: &str, file: &str) -> Result<(), anyhow::Error> {
    let data = bytes::Bytes::from("DATA.DATA.".repeat(10240));
    if let Err(e) = set(session_id, file, data).await {
        return Err(anyhow::anyhow!(
            "set file {} to memory cache failed: {}",
            file,
            e
        ));
    };
    Ok(())
}

pub async fn check() -> Result<(), anyhow::Error> {
    //     let (file_len, size_len, data_len) = len().await;
    //     println!("file_len: {}, data_len: {}", file_len, data_len);
    //     let mut files = FILES.write().await;
    //     let mut keys = Vec::new();
    //     while let Some(file) = files.pop().await {
    //         keys.push(file);
    //     }
    //     drop(files);
    //     for key in keys.iter() {
    //         if get(key, None).await.is_none() {
    //             println!("file: {} not exist", key);
    //         }
    //     }
    //     println!("got files: {}", keys.len());
    Ok(())
}
