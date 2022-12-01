use std::{fs::File, io, path::Path};

use clap::Parser;
use log::LevelFilter;
use simple_logger::SimpleLogger;

/// The default day to execute if CLI argument is not given.
const DEFAULT_DAY: u8 = 1;

/// The directory from which to read challenge input files.
const INPUT_FILE_DIRECTORY: &str = "input";

/// CLI for running Advent of Code challenges.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct AdventOfCode {
    #[arg(short, long)]
    day: Option<u8>,
}

/// Reads challenge input files.
fn read_input_file(path: String) -> Result<String, io::Error> {
    log::info!("input file path: {path}");

    let path = Path::new(&path);
    let file = File::open(path)?;

    let content = io::read_to_string(file)?;
    log::debug!("input file content: {content}");

    Ok(content)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .env()
        .init()?;

    let aoc = AdventOfCode::parse();
    let day = match aoc.day.unwrap_or(DEFAULT_DAY) {
        0 => {
            log::error!("`day` argument cannot be 0, using 1 instead");
            1
        }
        x => x,
    };

    log::info!("running challenge from day {day}");

    let input_filepath = format!("{INPUT_FILE_DIRECTORY}/day_{day}.txt");
    let input = match read_input_file(input_filepath) {
        Ok(s) => s,
        Err(e) => {
            log::error!("failed to read input file: {e}");
            return Err(Box::new(e));
        }
    };

    Ok(())
}
