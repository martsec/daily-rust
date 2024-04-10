#![allow(dead_code, unused_variables, unused_imports)]

mod datafusion;
mod duckdb;
mod polars;

use datafusion::main_df;
use duckdb::main_rust;
use polars::main_polars;

const RAW_DIRECTORY: &str = "/home/marti/greenwashing/data/readsb_hist/date=20230101";
const JSON_FILE: &str = "/home/marti/greenwashing/data/000950Z.json";
const GZ_FILE: &str = "/home/marti/greenwashing/data/000950Z.json.gz";

const PREP_DIR: &str = "/home/marti/poc/datarust";

#[tokio::main]
async fn main() {
    //main_rust().expect("DuckDB failed");
    //main_polars().expect("Polars Failed");
    main_df().await.expect("Datafusion failed");
}
