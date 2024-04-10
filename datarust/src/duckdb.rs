use duckdb::arrow::record_batch::RecordBatch;
use duckdb::arrow::util::pretty::print_batches;
use duckdb::{params, Connection, Result};

use crate::PREP_DIR;
use crate::RAW_DIRECTORY;

use std::time::{Duration, Instant};

pub fn main_rust() -> Result<()> {
    for f in [
        //raw_processing,
        query_num,
        query_num_aircrafts,
        query_aircraft_stats,
    ] {
        let start = Instant::now();
        f()?;
        let duration = start.elapsed();
        println!("Time elapsed for DuckDB: {:?}", duration);
    }

    Ok(())
}

fn raw_processing() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    let save_dir = format!("{PREP_DIR}/duckdb_pq");

    println!("Starting to read aircrafts");
    let query = format!(
        "
        PRAGMA enable_print_progress_bar;
        SET temp_directory = '/tmp/duckdb';
        INSTALL parquet;
        COPY (
        WITH raw AS (
            SELECT now AS ts, unnest(aircraft, max_depth :=2)
            FROM read_json_auto('{RAW_DIRECTORY}/*')
        )
        SELECT *, hex[:1] AS hex_start
        FROM raw
        --ORDER BY hex, ts -- Does not finish!
        --) TO '{save_dir}.parquet' (FORMAT PARQUET)
        ) TO '{save_dir}/'
        (FORMAT PARQUET, partition_by (hex_start), OVERWRITE_OR_IGNORE);
   "
    );

    println!("{}", &query);

    conn.execute_batch(&query)?;

    Ok(())
}

fn query_num() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    let save_dir = format!("{PREP_DIR}/duckdb_pq");
    let query = format!(
        "SELECT COUNT(*) FROM read_parquet('{save_dir}/*/*.parquet', hive_partitioning=1);"
    );
    let mut stmt = conn.prepare(&query)?;

    let rbs: Vec<RecordBatch> = stmt.query_arrow([])?.collect();
    print_batches(&rbs).unwrap();

    Ok(())
}
fn query_num_aircrafts() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    let save_dir = format!("{PREP_DIR}/duckdb_pq");
    let query = format!(
        "SELECT COUNT(distinct hex) FROM read_parquet('{save_dir}/*/*.parquet', hive_partitioning=1);"
    );
    let mut stmt = conn.prepare(&query)?;

    let rbs: Vec<RecordBatch> = stmt.query_arrow([])?.collect();
    print_batches(&rbs).unwrap();

    Ok(())
}

fn query_aircraft_stats() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    let save_dir = format!("{PREP_DIR}/duckdb_pq");
    let query = format!(
        "SELECT
        hex,
        MAX(IFNULL(TRY_CAST(alt_baro AS INTEGER), 0)) AS max_altitude
        , MAX(gs) AS max_ground_speed
        , MAX(emergency is not NULL and emergency != 'none') AS  had_emergency
      FROM read_parquet('{save_dir}/*/*.parquet', hive_partitioning=1)
      GROUP BY hex
      HAVING had_emergency
      LIMIT 20;"
    );
    let mut stmt = conn.prepare(&query)?;

    let rbs: Vec<RecordBatch> = stmt.query_arrow([])?.collect();
    print_batches(&rbs).unwrap();

    Ok(())
}
