use datafusion::datasource::file_format::file_compression_type::FileCompressionType;
use datafusion::datasource::file_format::json::JsonFormat;
use datafusion::datasource::listing::ListingOptions;
use datafusion::datasource::listing::ListingTable;
use datafusion::datasource::listing::ListingTableConfig;
use datafusion::datasource::listing::ListingTableUrl;
use datafusion::prelude::*;
use std::fs;
use std::sync::Arc;
use std::time::Instant;

use crate::JSON_FILE;
use crate::PREP_DIR;
use crate::RAW_DIRECTORY;

fn read_raw_files() -> Vec<String> {
    let files = fs::read_dir(&RAW_DIRECTORY).unwrap();
    let files: Vec<String> = files
        .into_iter()
        .take(1)
        .filter_map(|e| {
            e.ok().and_then(|f| {
                let path = f.path().into_os_string();
                path.to_str().map(String::from)
            })
        })
        .collect();
    files
}

pub async fn main_df() -> datafusion::error::Result<()> {
    let start = Instant::now();
    let res = raw_processing().await;
    let duration = start.elapsed();
    println!("Time elapsed for DuckDB: {:?}", duration);

    res
}

/// Read files using the default [read_json] method.
/// It currently ONLY supports new line defined jsons which is
/// not what we have.
async fn raw_ndjson_processing() -> datafusion::error::Result<()> {
    let ctx = SessionContext::new();
    let options = NdJsonReadOptions::default()
        .file_extension(".json.gz")
        .file_compression_type(FileCompressionType::GZIP);

    println!("{:?}", options.file_compression_type);
    let raw = ctx.read_json(read_raw_files(), options).await;

    println!("{:?}", raw);
    Ok(())
}

/// Load via TableProvider
/// https://docs.rs/datafusion/latest/datafusion/datasource/listing/struct.ListingTable.html
async fn raw_processing() -> datafusion::error::Result<()> {
    let ctx = SessionContext::new();
    let session_state = ctx.state();
    // Parse the path
    let table_path = ListingTableUrl::parse(&RAW_DIRECTORY)?;

    let file_format = JsonFormat::default()
        .with_file_compression_type(FileCompressionType::GZIP)
        .with_schema_infer_max_rec(5);
    let listing_options =
        ListingOptions::new(Arc::new(file_format)).with_file_extension(".json.gz");

    dbg!(&listing_options);

    let resolved_schema = listing_options
        .infer_schema(&session_state, &table_path)
        .await?;

    dbg!(&resolved_schema);

    let config = ListingTableConfig::new(table_path)
        .with_listing_options(listing_options)
        .with_schema(resolved_schema);

    // Create a new TableProvider
    let provider = Arc::new(ListingTable::try_new(config)?);

    // This provider can now be read as a dataframe:

    ctx.register_table("aircraft", provider)?;

    let sql = format!("COPY aircraft to '{PREP_DIR}/datafusion' OPTIONS (format parquet);");
    let results = ctx.sql(&sql).await?.collect().await?;

    println!("{:?}", &results);

    Ok(())
}
