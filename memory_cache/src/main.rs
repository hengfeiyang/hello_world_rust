use arrow::{ipc::reader::StreamReader, util::pretty};
use memory_cache::errors::MyError;
use memory_cache::errors::{OpenFileSnafu, ReadDataSnafu};
use snafu::ResultExt; // for the context method
use std::fs::File;
use std::io::Read;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let a = std::path::Path::new("../Cargo.toml.rs");
    println!("to_strs: {:?}", a.to_str().unwrap().to_string());
    println!("display: {:?}", a.display().to_string());
    let file = std::env::args().nth(1).unwrap_or_default();
    if file.is_empty() {
        println!("Usage: arrow_read <file>");
        return Ok(());
    }
    let file = std::fs::File::open(file).unwrap();
    let stream_reader = StreamReader::try_new(&file, None)?;
    let mut batches = vec![];
    for read_result in stream_reader {
        let record_batch = read_result?;
        batches.push(record_batch);
    }
    // print record batches as table
    pretty::print_batches(&batches)?;
    Ok(())
}

fn _read_file(path: &str) -> Result<String, MyError> {
    let mut file = File::open(path).context(OpenFileSnafu {
        path: path.to_string(),
    })?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).context(ReadDataSnafu {
        path: path.to_string(),
    })?;
    Ok(contents)
}
