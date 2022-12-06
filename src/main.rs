use arrow::datatypes::Schema;
use datafusion::datasource::datasource::TableProvider;
use datafusion::datasource::file_format::file_type::FileType;
use datafusion::datasource::file_format::file_type::GetExt;
use datafusion::datasource::file_format::parquet::ParquetFormat;
use datafusion::datasource::listing::ListingOptions;
use datafusion::datasource::listing::ListingTable;
use datafusion::datasource::listing::ListingTableConfig;
use datafusion::datasource::listing::ListingTableUrl;
use datafusion::datasource::object_store::ObjectStoreRegistry;
use datafusion::datasource::MemTable;
use datafusion::error::{DataFusionError, Result};
use datafusion::execution::context::SessionConfig;
use datafusion::execution::runtime_env::{RuntimeConfig, RuntimeEnv};
use datafusion::prelude::SessionContext;
use hello_world::object_storage::DatafusionCliObjectStoreProvider;
use hello_world::{
    exec, print_format::PrintFormat, print_options::print_timing_info, print_options::PrintOptions,
    DATAFUSION_CLI_VERSION,
};
use std::env;
use std::sync::Arc;
use std::time::Instant;

#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[tokio::main]
pub async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    println!("DataFusion CLI v{}", DATAFUSION_CLI_VERSION);

    for _i in 0..10 {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        main_test_local_list().await.unwrap();
        main_test_local_aggs().await.unwrap();
    }

    Ok(())
}

pub async fn main_test() -> Result<()> {
    let now = Instant::now();

    let mut print_options = PrintOptions {
        format: PrintFormat::Table,
        quiet: false,
    };

    let mut session_config = SessionConfig::from_env().with_information_schema(true);

    let runtime_env = create_runtime_env()?;
    let mut ctx = SessionContext::with_config_rt(session_config.clone(), Arc::new(runtime_env));

    let sql = "create external table tbl stored as parquet location '/Users/yanghengfei/code/rust/github.com/zinclabs/zinc-enl/data/stream'";
    ctx.sql(sql).await?;

    let sql = "select count(*) from tbl";
    let sql = "select * from tbl limit 10";
    let df = ctx.sql(sql).await?;
    let results = df.collect().await?;

    // print the results
    // df.show().await?;

    // let row_count: usize = batches.iter().map(|b| b.num_rows()).sum();
    // print_timing_info(row_count, now);

    print_options.print_batches(&results, now)?;

    Ok(())
}

pub async fn main_s3() -> Result<()> {
    let now = Instant::now();

    let mut print_options = PrintOptions {
        format: PrintFormat::Table,
        quiet: false,
    };

    env::set_var("AWS_DEFAULT_REGION", "us-west-2");

    let mut session_config = SessionConfig::from_env().with_information_schema(true);

    let runtime_env = create_runtime_env()?;
    let mut ctx = SessionContext::with_config_rt(session_config.clone(), Arc::new(runtime_env));

    let sql = "create external table tbl stored as parquet location 's3://zinc-dev-hengfei/default/logs/'";
    ctx.sql(sql).await?;

    let sql = "select count(*) from tbl";
    let sql = "select * from tbl limit 10";
    let df = ctx.sql(sql).await?;
    let results = df.collect().await?;

    // print the results
    // df.show().await?;

    // let row_count: usize = batches.iter().map(|b| b.num_rows()).sum();
    // print_timing_info(row_count, now);

    print_options.print_batches(&results, now)?;

    Ok(())
}

pub async fn main_cli() -> Result<()> {
    let mut session_config = SessionConfig::from_env().with_information_schema(true);

    let runtime_env = create_runtime_env()?;
    let mut ctx = SessionContext::with_config_rt(session_config.clone(), Arc::new(runtime_env));

    let mut print_options = PrintOptions {
        format: PrintFormat::Table,
        quiet: false,
    };

    // TODO maybe we can have thiserror for cli but for now let's keep it simple
    exec::exec_from_repl(&mut ctx, &mut print_options)
        .await
        .map_err(|e| DataFusionError::External(Box::new(e)))
}

pub async fn main_test_local_list() -> Result<()> {
    let now = Instant::now();

    let print_options = PrintOptions {
        format: PrintFormat::NdJson,
        quiet: false,
    };

    let session_config = SessionConfig::from_env().with_information_schema(true);

    let runtime_env = create_runtime_env()?;
    let mut ctx = SessionContext::with_config_rt(session_config.clone(), Arc::new(runtime_env));

    // Configure listing options
    let file_format = ParquetFormat::default().with_enable_pruning(true);
    let listing_options = ListingOptions {
        file_extension: FileType::PARQUET.get_ext(),
        format: Arc::new(file_format),
        table_partition_cols: vec![],
        collect_stat: true,
        target_partitions: 10,
    };

    let mut prefixes = Vec::new();
    prefixes.push(ListingTableUrl::parse("mem:///Users/yanghengfei/code/rust/github.com/zinclabs/zinc-enl/data/stream/default/logs/").unwrap());

    let mut config =
        ListingTableConfig::new_with_multi_paths(prefixes).with_listing_options(listing_options);
    config = config.infer_schema(&ctx.state()).await.unwrap();

    let table = ListingTable::try_new(config)?;
    ctx.register_table("tbl", Arc::new(table))?;

    // let sql = "select count(*) from tbl";
    // let sql = "select * from tbl order by _timestamp desc limit 10";
    // let sql = "select * from tbl where \"kubernetes.container_name\"='ziox' order by _timestamp desc limit 10";
    let sql = "select * from tbl where \"Country\"='USA' order by _timestamp desc limit 10";

    let df = ctx.sql(sql).await?;
    let batches = df.collect().await?;
    print_options.print_batches(&batches, now)?;

    Ok(())
}

pub async fn main_test_local_aggs() -> Result<()> {
    let now = Instant::now();

    let print_options = PrintOptions {
        format: PrintFormat::NdJson,
        quiet: false,
    };

    let session_config = SessionConfig::from_env().with_information_schema(true);

    let runtime_env = create_runtime_env()?;
    let ctx = SessionContext::with_config_rt(session_config.clone(), Arc::new(runtime_env));

    // Configure listing options
    let file_format = ParquetFormat::default().with_enable_pruning(true);
    let listing_options = ListingOptions {
        file_extension: FileType::PARQUET.get_ext(),
        format: Arc::new(file_format),
        table_partition_cols: vec![],
        collect_stat: true,
        target_partitions: 10,
    };

    let mut prefixes = Vec::new();
    prefixes.push(ListingTableUrl::parse("mem:///Users/yanghengfei/code/rust/github.com/zinclabs/zinc-enl/data/stream/default/logs/").unwrap());

    let mut config =
        ListingTableConfig::new_with_multi_paths(prefixes).with_listing_options(listing_options);
    config = config.infer_schema(&ctx.state()).await.unwrap();

    let table = ListingTable::try_new(config)?;
    ctx.register_table("tbl", Arc::new(table))?;

    let sql = "select date_bin(interval '10 second', to_timestamp_micros(\"_timestamp\"), to_timestamp('2001-01-01T00:00:00')) as key ,count(*) as num from tbl where \"kubernetes.container_name\"='ziox' group by  key order by key asc";

    let df = ctx.sql(sql).await?;
    let batches = df.collect().await?;
    print_options.print_batches(&batches, now)?;

    Ok(())
}

fn create_runtime_env() -> Result<RuntimeEnv> {
    let object_store_provider = DatafusionCliObjectStoreProvider {};
    let object_store_registry =
        ObjectStoreRegistry::new_with_provider(Some(Arc::new(object_store_provider)));
    let rn_config =
        RuntimeConfig::new().with_object_store_registry(Arc::new(object_store_registry));
    RuntimeEnv::new(rn_config)
}
