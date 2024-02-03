use std::error::Error;

use chrono::prelude::*;
use clap::Parser;
use polars::df;
use polars::prelude::*;

use pqinspect::run;
use pqinspect::Cli;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    dbg!(&cli);

    run(&cli)?;

    Ok(())
}
