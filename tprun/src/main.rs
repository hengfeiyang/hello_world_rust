use arrow::{json::ReaderBuilder, record_batch::RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use get_size::GetSize;
use std::{mem, sync::Arc, time::Duration};
use tokio::sync::Semaphore;

struct MyStruct {
    x: i32,
    z: Option<RecordBatch>,
    y: String,
}

impl GetSize for MyStruct {
    fn get_size(&self) -> usize {
        let mut size = 0;
        size += mem::size_of_val(&self.x);
        size += self.y.get_size();
        size += mem::size_of_val(&self.z);
        size
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let size = mem::size_of::<i32>();
    println!("Size of MyStruct: {} bytes", size);
    let size = mem::size_of::<String>();
    println!("Size of MyStruct: {} bytes", size);
    let size = mem::size_of::<MyStruct>();
    println!("Size of MyStruct: {} bytes", size);

    let mut my_struct = MyStruct {
        x: 42,
        y: String::from("hellohellohellohellohellohellohellohellohellohll0hello"),
        z: None,
    };
    let actual_size = my_struct.get_size();
    println!("Actual size of my_struct: {} bytes", actual_size);

    my_struct.z = Some(generate_recordbatch());
    let actual_size = my_struct.get_size();
    println!("Actual size of my_struct: {} bytes", actual_size);

    let task1 = test_job_control(1);
    let task2 = test_job_control(2);
    tokio::join!(task1, task2);
    Ok(())
}

fn generate_recordbatch() -> RecordBatch {
    let json_str = r#"
    [
        {"a": 1, "b": "hello", "c": 1.0},
        {"a": 2, "b": "world", "c": 2.0},
        {"a": 3, "b": "!", "c": 3.0}
    ]"#;
    let json_arr: Vec<serde_json::Value> = serde_json::from_str(json_str).unwrap();
    println!("json_str size: {}", json_str.len());
    let schema = Schema::new(vec![
        Field::new("a", DataType::Int64, false),
        Field::new("b", DataType::Utf8, false),
        Field::new("c", DataType::Float64, false),
    ]);
    let schema_size = schema
        .fields()
        .iter()
        .fold(0, |acc, field| acc + field.size());
    println!("schema size: {}", schema_size);
    println!(
        "hashmap size: {}",
        std::mem::size_of::<std::collections::HashMap<String, usize>>()
    );
    let schema = Arc::new(schema);
    let mut decoder = ReaderBuilder::new(schema)
        .with_batch_size(1024)
        .build_decoder()
        .unwrap();
    let _ = decoder.serialize(&json_arr);
    let batch = decoder.flush().unwrap().unwrap();
    println!("recordBatch size: {}", batch.get_array_memory_size());
    batch
}

async fn test_job_control(batch: usize) -> Result<(), anyhow::Error> {
    let semaphore = Arc::new(Semaphore::new(5));
    let mut tasks = Vec::new();
    for i in 0..10 {
        let permit = semaphore.clone().acquire_owned().await.unwrap(); 
        let task = tokio::spawn(async move { 
            println!("task{} -> {} start", batch, i);
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("task{} -> {} end", batch, i);
            drop(permit);
        });
        tasks.push(task);
    }
    println!("task{} -> wait", batch);
    for task in tasks {
        task.await?;
    }
    Ok(())
}
