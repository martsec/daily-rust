use polars::io::mmap::MmapBytesReader;
use polars::lazy::dsl::all;
use polars::prelude::*;
use polars::sql::SQLContext;
use polars_core::POOL;
use rayon::prelude::*;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Cursor;
use std::io::Read;
use std::os::unix::process;

use crate::PREP_DIR;
use crate::RAW_DIRECTORY;

use std::time::Instant;

pub fn main_polars() -> Result<(), PolarsError> {
    for f in [raw_processing] {
        let start = Instant::now();
        f()?;
        let duration = start.elapsed();
        println!("Time elapsed for Polars: {:?}", duration);
    }

    Ok(())
}

fn raw_processing() -> Result<(), PolarsError> {
    //let raw = LazyJsonLineReader::new(&RAW_DIRECTORY).finish()?;
    // TODO looks like polars does not support to read from compressed
    // jsons, despite activating the `decompress` feature.
    // It's just for CSV
    //
    let files = fs::read_dir(&RAW_DIRECTORY)?;
    let files: Vec<String> = files
        .into_iter()
        .take(500)
        .filter_map(|e| {
            e.ok().and_then(|f| {
                let path = f.path().into_os_string();
                path.to_str().map(String::from)
            })
        })
        .collect();

    let frames = POOL.install(|| {
        files
            .into_par_iter()
            //.take(5)
            .map(|f| process_single_file(&f))
            .collect::<Result<Vec<LazyFrame>, PolarsError>>()
    })?;

    let raw = concat(
        frames,
        UnionArgs {
            to_supertypes: true,
            ..UnionArgs::default()
        },
    )?;
    //    .map(|f| process_single_file(&f.unwrap().path().file_name().unwrap().to_str().unwrap()));

    //println!("{}", raw.describe_plan());
    //println!("{}", raw.describe_optimized_plan()?);
    println!("{}", raw.clone().collect()?);
    //let mut ctx = SQLContext::new();
    //

    let mut to_write = raw.with_streaming(true).collect()?;
    let parquet_file_name = format!("{PREP_DIR}/polars.parquet");
    let mut file = std::fs::File::create(parquet_file_name).unwrap();
    ParquetWriter::new(&mut file).finish(&mut to_write).unwrap();

    Ok(())
}

fn process_single_file(path: &str) -> Result<LazyFrame, PolarsError> {
    let mut file = File::open(path).unwrap();
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf);
    let dec = decompress(&buf).ok_or(PolarsError::Io(std::io::Error::other(
        "Error uncompressing file",
    )))?;
    let mut cursor = Cursor::new(dec);
    let raw = JsonReader::new(&mut cursor)
        .finish()
        .unwrap()
        .lazy()
        .explode(["aircraft"])
        // Need to force this. Seems a bug
        // https://github.com/pola-rs/polars/issues/6060
        .select([col("now"), col("aircraft")])
        .unnest(["aircraft"]);

    // Need to sort them since unnest returns random col order and messes with the concat
    let mut col_names: Vec<String> = raw
        .schema()?
        .iter_names()
        .map(|n| n.to_string())
        .collect::<Vec<String>>();
    col_names.sort();

    let cols: Vec<Expr> = col_names.iter().map(|c| col(c)).collect();

    let res = raw.select(cols);

    Ok(res)
}

// magic numbers
const GZIP: [u8; 2] = [31, 139];

/// From https://github.com/pola-rs/polars/blob/dcee934bd1942f774339f44ca6ebad4aca6ea9db/crates/polars-io/src/csv/utils.rs#L528
fn decompress(bytes: &[u8]) -> Option<Vec<u8>> {
    if bytes.starts_with(&GZIP) {
        let mut decoder = flate2::read::MultiGzDecoder::new(bytes);
        decompress_impl(&mut decoder)
    } else {
        None
    }
}

fn decompress_impl<R: Read>(decoder: &mut R) -> Option<Vec<u8>> {
    Some({
        let mut out = vec![];
        let _ = decoder.read_to_end(&mut out).ok()?;
        out
    })
}
