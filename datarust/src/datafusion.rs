use datafusion::config::JsonOptions;
use datafusion::datasource::file_format::file_compression_type::FileCompressionType;
use datafusion::datasource::file_format::json::JsonFormat;
use datafusion::datasource::listing::ListingOptions;
use datafusion::datasource::listing::ListingTableUrl;
use datafusion::prelude::*;
use std::fs;
use std::sync::Arc;

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
    raw_processing().await
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

    let file_format = JsonFormat::default().with_file_compression_type(FileCompressionType::GZIP);
    let listing_options =
        ListingOptions::new(Arc::new(file_format)).with_file_extension(".json.gz");

    dbg!(&listing_options);

    let resolved_schema = listing_options
        .infer_schema(&session_state, &table_path)
        .await?;

    dbg!(resolved_schema);

    Ok(())
}
