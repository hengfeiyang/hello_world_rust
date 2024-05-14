use async_nats::jetstream::kv;
use async_nats::jetstream::kv::Config;
use async_nats::ServerAddr;
use futures::TryStreamExt;

use futures::stream::StreamExt;  // Required for then and try_collect
use futures::stream::iter;  // Required for iter
use tokio::time::Duration;

mod dist_lock;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut lock = dist_lock::DistLock::try_new().await?;
    println!("waiting lock");
    lock.acquire("foo", None).await?;
    println!("acuqired lock");
    println!("this is operation with lock need 5 seconds");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    lock.release().await?;
    println!("released lock");

    let client = async_nats::connect("localhost").await?;
    let jetstream = async_nats::jetstream::new(client);
    let mut streams = jetstream.streams();
    while let Some(stream) = streams.try_next().await? {
        println!("stream: {:?}\tstate:{:?}", stream.config.name,stream.state);
    }
    let kv = jetstream.create_key_value(Config {
        bucket: "ttt".to_string(),
        history: 10,
        ..Default::default()
    }).await?;
    println!("create kv: {:?}", kv.status().await.unwrap().info.created);


    
    let values = vec![1, 2, 3, 4, 5];
    
    let stream = iter(values);
    
    let new_stream = stream.then(|value| async move {
        tokio::time::sleep(Duration::from_secs(1)).await;  // Sleep for a second
        Ok::<i32, ()>(value * 2)  // Multiply each value by 2
    });
    
    let result: Result<Vec<_>, _> = new_stream.try_collect().await;
    
    match result {
        Ok(vec) => println!("{:?}", vec),
        Err(e) => eprintln!("Error: {:?}", e),
    }

    let opts = async_nats::ConnectOptions::new()
        .connection_timeout(core::time::Duration::from_secs(5));
    let addrs = "localhost:4222"
        .split(',')
        .map(|a| a.parse().unwrap())
        .collect::<Vec<ServerAddr>>();
    println!("addrs: {:?}", addrs);
    let client = async_nats::connect_with_options(addrs, opts)
        .await
        .expect("Nats connect failed");
    let jetstream = async_nats::jetstream::with_prefix(client, "JS.acc@hub.API");
    let bucket_name = "test";
    let mut bucket = kv::Config {
        bucket: bucket_name.to_string(),
        history: 10,
        ..Default::default()
    };
    let mut kv = jetstream.create_key_value(bucket).await?;
    let val = kv.get("key1").await?;
    println!("val: {:?}", val);
    Ok(())
}
