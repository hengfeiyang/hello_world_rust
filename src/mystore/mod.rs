//! An in-memory object store implementation
use async_trait::async_trait;
use bytes::Bytes;
use chrono::Utc;
use futures::TryFutureExt;
use futures::{stream::BoxStream, StreamExt};
use object_store::MultipartId;
use object_store::{path::Path, GetResult, ListResult, ObjectMeta, ObjectStore, Result};
use parking_lot::RwLock;
use snafu::{ensure, OptionExt, Snafu};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Read;
use std::io::{self};
use std::ops::Range;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Poll;
use tokio::io::{AsyncReadExt, AsyncWrite};
use walkdir::WalkDir;

/// A specialized `Error` for in-memory object store-related errors
#[derive(Debug, Snafu)]
#[allow(missing_docs)]
enum Error {
    #[snafu(display("No data in memory found. Location: {path}"))]
    NoDataInMemory { path: String },

    #[snafu(display("Out of range"))]
    OutOfRange,

    #[snafu(display("Bad range"))]
    BadRange,

    #[snafu(display("Object already exists at that location: {path}"))]
    AlreadyExists { path: String },
}

impl From<Error> for object_store::Error {
    fn from(source: Error) -> Self {
        match source {
            Error::NoDataInMemory { ref path } => Self::NotFound {
                path: path.into(),
                source: source.into(),
            },
            Error::AlreadyExists { ref path } => Self::AlreadyExists {
                path: path.into(),
                source: source.into(),
            },
            _ => Self::Generic {
                store: "InMemory",
                source: Box::new(source),
            },
        }
    }
}

/// In-memory storage suitable for testing or for opting out of using a cloud
/// storage provider.
#[derive(Debug, Default)]
pub struct InMemory {
    storage: Arc<RwLock<BTreeMap<Path, Bytes>>>,
    root: String,
}

impl std::fmt::Display for InMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InMemory")
    }
}

#[async_trait]
impl ObjectStore for InMemory {
    async fn put(&self, location: &Path, bytes: Bytes) -> Result<()> {
        log::info!("put: {}", location);
        self.storage.write().insert(location.clone(), bytes);
        Ok(())
    }

    async fn put_multipart(
        &self,
        location: &Path,
    ) -> Result<(MultipartId, Box<dyn AsyncWrite + Unpin + Send>)> {
        log::info!("put_multipart: {}", location);
        Ok((
            String::new(),
            Box::new(InMemoryUpload {
                location: location.clone(),
                data: Vec::new(),
                storage: Arc::clone(&self.storage),
            }),
        ))
    }

    async fn abort_multipart(&self, location: &Path, _multipart_id: &MultipartId) -> Result<()> {
        log::info!("abort_multipart: {}", location);
        // Nothing to clean up
        Ok(())
    }

    async fn get(&self, location: &Path) -> Result<GetResult> {
        log::info!("get: {}", location);
        let data = self.get_bytes(location).await?;

        Ok(GetResult::Stream(
            futures::stream::once(async move { Ok(data) }).boxed(),
        ))
    }

    async fn get_range(&self, location: &Path, range: Range<usize>) -> Result<Bytes> {
        // log::info!("get_range: {}", location);
        let data = self.get_bytes(location).await?;
        ensure!(range.end <= data.len(), OutOfRangeSnafu);
        ensure!(range.start <= range.end, BadRangeSnafu);

        Ok(data.slice(range))
    }

    async fn get_ranges(&self, location: &Path, ranges: &[Range<usize>]) -> Result<Vec<Bytes>> {
        // log::info!("get_ranges: {}", location);
        let data = self.get_bytes(location).await?;
        ranges
            .iter()
            .map(|range| {
                ensure!(range.end <= data.len(), OutOfRangeSnafu);
                ensure!(range.start <= range.end, BadRangeSnafu);
                Ok(data.slice(range.clone()))
            })
            .collect()
    }

    async fn head(&self, location: &Path) -> Result<ObjectMeta> {
        log::info!("head: {}", location);
        let last_modified = Utc::now();
        let bytes = self.get_bytes(location).await?;
        Ok(ObjectMeta {
            location: location.clone(),
            last_modified,
            size: bytes.len(),
        })
    }

    async fn delete(&self, location: &Path) -> Result<()> {
        log::info!("delete: {}", location);
        self.storage.write().remove(location);
        Ok(())
    }

    async fn list(&self, prefix: Option<&Path>) -> Result<BoxStream<'_, Result<ObjectMeta>>> {
        log::info!("list: {:?}", prefix);

        let last_modified = Utc::now();

        let storage = self.storage.read();
        let values: Vec<_> = storage
            .iter()
            .filter(move |(key, _)| prefix.map(|p| key.prefix_matches(p)).unwrap_or(true))
            .map(move |(key, value)| {
                Ok(ObjectMeta {
                    location: key.clone(),
                    last_modified,
                    size: value.len(),
                })
            })
            .collect();

        Ok(futures::stream::iter(values).boxed())
    }

    /// The memory implementation returns all results, as opposed to the cloud
    /// versions which limit their results to 1k or more because of API
    /// limitations.
    async fn list_with_delimiter(&self, prefix: Option<&Path>) -> Result<ListResult> {
        log::info!("list_with_delimiter: {:?}", prefix);
        let root = Path::default();
        let prefix = prefix.unwrap_or(&root);

        let mut common_prefixes = BTreeSet::new();
        let last_modified = Utc::now();

        // Only objects in this base level should be returned in the
        // response. Otherwise, we just collect the common prefixes.
        let mut objects = vec![];
        for (k, v) in self.storage.read().range((prefix)..) {
            let mut parts = match k.prefix_match(prefix) {
                Some(parts) => parts,
                None => break,
            };

            // Pop first element
            let common_prefix = match parts.next() {
                Some(p) => p,
                None => continue,
            };

            if parts.next().is_some() {
                common_prefixes.insert(prefix.child(common_prefix));
            } else {
                let object = ObjectMeta {
                    location: k.clone(),
                    last_modified,
                    size: v.len(),
                };
                objects.push(object);
            }
        }

        Ok(ListResult {
            objects,
            common_prefixes: common_prefixes.into_iter().collect(),
        })
    }

    async fn copy(&self, from: &Path, to: &Path) -> Result<()> {
        log::info!("copy: from {} to {}", from, to);
        let data = self.get_bytes(from).await?;
        self.storage.write().insert(to.clone(), data);
        Ok(())
    }

    async fn copy_if_not_exists(&self, from: &Path, to: &Path) -> Result<()> {
        log::info!("copy_if_not_exists: from {} to {}", from, to);
        let data = self.get_bytes(from).await?;
        let mut storage = self.storage.write();
        if storage.contains_key(to) {
            return Err(Error::AlreadyExists {
                path: to.to_string(),
            }
            .into());
        }
        storage.insert(to.clone(), data);
        Ok(())
    }
}

impl InMemory {
    /// Create new in-memory storage.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_path<P: AsRef<std::path::Path>>(&mut self, root_dir: P) -> &mut Self {
        // try to load data into memory
        let root_dir = root_dir.as_ref().to_str().unwrap().to_string();
        let root_dir = root_dir.strip_prefix("mem://").unwrap();
        self.root = root_dir.to_string();
        println!("with_path: {:?}", root_dir);

        let walkdir = WalkDir::new(&self.root)
            // Don't include the root directory itself
            .min_depth(1)
            .follow_links(true);

        let files = walkdir
            .into_iter()
            .flat_map(move |result_dir_entry| {
                let dir_entry = result_dir_entry
                    .as_ref()
                    .unwrap()
                    .path()
                    .to_str()
                    .unwrap()
                    .to_string();
                // println!("Loading data into memory: {}", dir_entry);
                if result_dir_entry.unwrap().file_type().is_dir() {
                    return vec![];
                }
                vec![dir_entry]
            })
            .collect::<Vec<String>>();
        // println!("Loading data into memory: {:?}", files);

        for file in files {
            // println!("Loading data into memory: {}", file);
            let mut f = File::open(&file).unwrap();
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).unwrap();
            self.storage.write().insert(file.clone().into(), buffer.into());
        }

        self
    }

    /// Creates a clone of the store
    pub async fn clone(&self) -> Self {
        let storage = self.storage.read();
        let storage = storage.clone();

        Self {
            storage: Arc::new(RwLock::new(storage)),
            root: self.root.clone(),
        }
    }

    async fn get_bytes(&self, location: &Path) -> Result<Bytes> {
        let storage = self.storage.read();
        let bytes = storage
            .get(location)
            .cloned()
            .context(NoDataInMemorySnafu {
                path: location.to_string(),
            })?;
        Ok(bytes)
    }
}

struct InMemoryUpload {
    location: Path,
    data: Vec<u8>,
    storage: Arc<RwLock<BTreeMap<Path, Bytes>>>,
}

impl AsyncWrite for InMemoryUpload {
    fn poll_write(
        mut self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, io::Error>> {
        self.data.extend_from_slice(buf);
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), io::Error>> {
        let data = Bytes::from(std::mem::take(&mut self.data));
        self.storage.write().insert(self.location.clone(), data);
        Poll::Ready(Ok(()))
    }
}
