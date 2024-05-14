use std::pin::Pin;

use async_trait::async_trait;
use super::{UpdateFn, Result};
use bytes::Bytes;

pub async fn init() -> Result<()> {
    Ok(())
}

pub struct EtcdDb {
    prefix: String,
}

impl EtcdDb {
    pub fn new() -> Self {
        Self {
            prefix: "etcd".to_string(),
        }
    }
}

impl Default for EtcdDb {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl super::Db for EtcdDb {
    async fn create_table(&self) -> Result<()> {
        println!("create table for {}", self.prefix);
        Ok(())
    }
    async fn get(&self, _key: &str) -> Result<Bytes> {
        println!("get from table {}", self.prefix);
        Ok(Bytes::from(""))
    }
    async fn put(
        &self,
        _key: &str,
        _value: Bytes,
        _need_watch: bool,
        _start_dt: Option<i64>,
    ) -> Result<()> {
        println!("put into table {}", self.prefix);
        Ok(())
    } 
    async fn put_with_callback(
        &self,
        _key: &str,
        _need_watch: bool,
        _start_dt: Option<i64>,
        callback: Box<UpdateFn>,
    ) -> Result<()> {
        println!("put with callback into table {}", self.prefix);
        let v = callback(Some(Bytes::from("")));
        println!("callback: {:?}", v);
        Ok(())
    }
}