use clap::Parser;
use log::LevelFilter;
use simple_logger::SimpleLogger;

use advent_of_code_2022::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .env()
        .init()?;

    let config = Config::parse();
    advent_of_code_2022::solution(config).await?;

    Ok(())
}
