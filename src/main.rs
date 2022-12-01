use clap::Parser;
use log::LevelFilter;
use simple_logger::SimpleLogger;

/// The default day to execute if CLI argument is not given.
const DEFAULT_DAY: u8 = 1;

/// CLI for running Advent of Code challenges.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct AdventOfCode {
    #[arg(short, long)]
    day: Option<u8>,
}

fn main() -> anyhow::Result<()> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .env()
        .init()?;

    let aoc = AdventOfCode::parse();
    let day = match aoc.day.unwrap_or(DEFAULT_DAY) {
        0 => {
            log::error!("--day argument cannot be 0, using 1 instead");
            1
        }
        x => x,
    };

    log::info!("running solution for day {day}");

    advent_of_code_2022::solution(day)?;

    Ok(())
}
