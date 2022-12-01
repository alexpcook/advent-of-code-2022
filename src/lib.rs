use std::{fs::File, io, path::Path};

use anyhow::bail;

pub mod day1;

/// Runs a solution from a given `day`.
pub fn solution(day: u8) -> anyhow::Result<()> {
    let input_filepath = format!("input/day_{day}.txt");
    let input = match read_input_file(input_filepath) {
        Ok(s) => s,
        Err(e) => {
            bail!("failed to read input file for day {day}: {e}");
        }
    };

    match day {
        1 => day1::main(input),
        d => bail!("solution for day {d} does not exist"),
    }
}

/// Reads challenge input files.
fn read_input_file(path: String) -> Result<String, io::Error> {
    log::debug!("input file path: {path}");

    let path = Path::new(&path);
    let file = File::open(path)?;

    let content = io::read_to_string(file)?;
    log::debug!("input file content: {content}");

    Ok(content)
}
