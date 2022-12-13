use std::fmt;

pub fn main(input: String) -> anyhow::Result<()> {
    let elevations: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| match u8::try_from(c) {
                    Ok(elevation) => Some(elevation),
                    Err(e) => {
                        log::error!("failed to convert {c} to u8: {e}");
                        None
                    }
                })
                .collect()
        })
        .collect();

    let mut map = Map {
        elevations,
        start_position: (0, 0),
        end_position: (0, 0),
    };

    let e = u8::try_from('E')?;
    let s = u8::try_from('S')?;

    for (x, row) in map.elevations.iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            if *col == e || *col == s {
                map.end_position = (x, y);
            }
        }
    }

    println!("{map}");

    Ok(())
}

/// The map of the terrain.
#[derive(Debug)]
struct Map {
    /// The start position S.
    start_position: (usize, usize),
    /// The end position E.
    end_position: (usize, usize),
    /// The elevations of each square on the map.
    elevations: Vec<Vec<u8>>,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.elevations {
            for col in row {
                write!(f, "{}", char::from(*col))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
