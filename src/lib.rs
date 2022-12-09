use std::{
    fs::File,
    io::{self, ErrorKind, Write},
    path::Path,
    time::Duration,
};

use anyhow::bail;
use clap::Parser;
use reqwest::Url;

/// CLI for running Advent of Code challenges.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// The day of the solution to run.
    #[arg(short, long)]
    day: Option<u8>,

    /// A session cookie to allow pulling input for the day from the website.
    #[arg(short, long)]
    session: Option<String>,
}

/// Day 1 solution.
pub mod day1;
/// Day 2 solution.
pub mod day2;
/// Day 3 solution.
pub mod day3;
/// Day 4 solution.
pub mod day4;
/// Day 5 solution.
pub mod day5;
/// Day 6 solution.
pub mod day6;
/// Day 7 solution.
pub mod day7;
/// Day 8 solution.
pub mod day8;
/// Day 9 solution.
pub mod day9;

/// Runs a solution from the given configuration.
pub async fn solution(config: Config) -> anyhow::Result<()> {
    const DEFAULT_DAY: u8 = 1;

    let day = match config.day.unwrap_or(DEFAULT_DAY) {
        0 => {
            log::error!("--day argument cannot be 0, using 1 instead");
            1
        }
        x => x,
    };

    log::info!("running solution for day {day}");

    let input = match input(day, config.session).await {
        Ok(s) => s,
        Err(e) => bail!("failed to get input for day {day}: {e}"),
    };

    match day {
        1 => day1::main(input),
        2 => day2::main(input),
        3 => day3::main(input),
        4 => day4::main(input),
        5 => day5::main(input),
        6 => day6::main(input),
        7 => day7::main(input),
        8 => day8::main(input),
        9 => day9::main(input),
        d => bail!("solution for day {d} does not exist"),
    }
}

/// Gets the input for a challenge.
async fn input(day: u8, session: Option<String>) -> anyhow::Result<String> {
    let filepath = format!("input/day_{day}.txt");
    let filepath = Path::new(&filepath);

    let input = match File::open(filepath) {
        Ok(f) => {
            log::debug!("getting input from file {}", filepath.to_string_lossy());

            io::read_to_string(f)?
        }
        Err(e) => {
            let ErrorKind::NotFound = e.kind() else {
                bail!(
                    "failed to process input file {}: {e}",
                    filepath.to_string_lossy()
                );
            };

            let Some(session) = session else {
                bail!(
                    "failed to find input file {} and no session cookie provided to get input from website",
                    filepath.to_string_lossy()
                );
            };

            log::debug!("getting input from website");

            const TIMEOUT: Duration = Duration::from_secs(2);
            let client = reqwest::Client::builder().timeout(TIMEOUT).build()?;
            let url = format!("https://adventofcode.com/2022/day/{day}/input");

            let response = client
                .get(Url::parse(&url)?)
                .header("cookie", format!("session={session}"))
                .send()
                .await?;

            let Ok(response) = response.error_for_status() else {
                bail!("got non-200 status code getting input from website");
            };

            let input = response.text().await?;

            File::create(filepath)?.write_all(input.as_bytes())?;

            input
        }
    };

    Ok(input)
}
