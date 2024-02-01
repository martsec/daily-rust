use std::error::Error;

use clap::{Parser, Subcommand};

use polars::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    operation: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Show { file: String },
    Schema { file: String },
}

pub fn run(options: &Cli) -> Result<(), Box<dyn Error>> {
    match &options.operation {
        Command::Show { file } => show(&file),
        Command::Schema { file } => schema(&file),
    }
}

fn show(fname: &str) -> Result<(), Box<dyn std::error::Error>> {
    let lf: LazyFrame = LazyFrame::scan_parquet(fname, Default::default())?;
    println!("{}", lf.collect()?);

    Ok(())
}

fn schema(fname: &str) -> Result<(), Box<dyn std::error::Error>> {
    let lf: LazyFrame = LazyFrame::scan_parquet(fname, Default::default())?;
    for col in lf.schema().into_iter() {
        println!("{:?}", col);
    }
    Ok(())
}
