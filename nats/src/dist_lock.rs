use std::{sync::Arc, time::{Duration, Instant}};

use async_nats::jetstream::kv::{Config, Store};

pub struct DistLock {
    pub(crate) kv: Arc<Store>,
    pub(crate) key: Option<String>,
}

impl DistLock {
    pub async fn try_new() -> Result<Self, anyhow::Error> {
        // Connect to the NATS server
        let client = async_nats::connect("localhost").await?;
        // Create a JetStream context.
        let jetstream = async_nats::jetstream::new(client);
        // Access an existing key-value
        let kv = jetstream.create_key_value(Config {
            bucket: "kvlock".to_string(),
            history: 10,
            ..Default::default()
        }).await?;
        Ok(Self {
            kv:Arc::new(kv),
            key:None,
        })
    }

    pub async fn acquire(&mut self,key:&str, timeout: Option<Duration>) -> Result<(), anyhow::Error> {
        let start = Instant::now();
        self.key = Some(key.into());
        loop {
            match self.kv.create(&key, "".into()).await {
                Ok(_) => {
                    return Ok(());
                }
                Err(_) => {
                    if let Some(timeout) = timeout {
                        if start.elapsed() > timeout {
                            return Err(anyhow::anyhow!("acquire timeout"));
                        }
                    }
                }
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    pub async fn release(&self) -> Result<(), anyhow::Error> {
        if let Some(key) = self.key.as_ref() {
            self.kv.delete(&key).await?;
        } 
            Ok(()) 
    }
}

