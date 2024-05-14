// Copyright 2023 Zinc Labs Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::pin::Pin;

use async_trait::async_trait;
use bytes::Bytes;
use tokio::sync::OnceCell;

type Result<T> = std::result::Result<T, anyhow::Error>;

pub mod etcd;
pub mod nats;

pub static NEED_WATCH: bool = true;
pub static NO_NEED_WATCH: bool = false;

static DEFAULT: OnceCell<Box<dyn Db + 'static>> = OnceCell::const_new();

pub async fn get_db() -> &'static Box<dyn Db + 'static> {
    DEFAULT.get_or_init(default).await
}

pub async fn init() -> Result<()> {
    etcd::init().await?;
    Ok(())
}

static META_STORE: &str = "etcd";

async fn default() -> Box<(dyn Db + 'static)> {
    match META_STORE {
        "etcd" => Box::new(etcd::EtcdDb::default()),
        "nats" => Box::new(nats::NatsDb::default()),
        _ => Box::new(etcd::EtcdDb::default()),
    }
}

pub async fn create_table() -> Result<()> {
    // create for meta store
    let db = get_db().await;
    db.create_table().await?;
    Ok(())
}

pub type UpdateFn = dyn FnOnce(Option<Bytes>) -> Result<Option<Bytes>> + Send;

#[async_trait]
pub trait Db: Sync + Send + 'static {
    async fn create_table(&self) -> Result<()>;
    async fn get(&self, key: &str) -> Result<Bytes>;
    async fn put(
        &self,
        key: &str,
        value: Bytes,
        need_watch: bool,
        start_dt: Option<i64>,
    ) -> Result<()>;
    async fn put_with_callback(
        &self,
        key: &str,
        need_watch: bool,
        start_dt: Option<i64>,
        callback: Box<UpdateFn>,
    ) -> Result<()>;
}

pub enum DbEnum {
    EtcdDb(etcd::EtcdDb),
    NatsDb(nats::NatsDb),
}

#[async_trait]
impl Db for DbEnum {
    async fn create_table(&self) -> Result<()> {
        match self {
            DbEnum::EtcdDb(db) => db.create_table().await,
            DbEnum::NatsDb(db) => db.create_table().await,
        }
    }

    async fn get(&self, key: &str) -> Result<Bytes> {
        match self {
            DbEnum::EtcdDb(db) => db.get(key).await,
            DbEnum::NatsDb(db) => db.get(key).await,
        }
    }

    async fn put(
        &self,
        key: &str,
        value: Bytes,
        need_watch: bool,
        start_dt: Option<i64>,
    ) -> Result<()> {
        match self {
            DbEnum::EtcdDb(db) => db.put(key, value, need_watch, start_dt).await,
            DbEnum::NatsDb(db) => db.put(key, value, need_watch, start_dt).await,
        }
    }
    async fn put_with_callback(
        &self,
        key: &str,
        need_watch: bool,
        start_dt: Option<i64>,
        callback: Box<UpdateFn>,
    ) -> Result<()> {
        match self {
            DbEnum::EtcdDb(db) => {
                db.put_with_callback(key, need_watch, start_dt, callback)
                    .await
            }
            DbEnum::NatsDb(db) => {
                db.put_with_callback(key, need_watch, start_dt, callback)
                    .await
            }
        }
    }
}
