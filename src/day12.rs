use std::{collections::HashSet, fmt};

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

    let mut paths: Vec<u32> = vec![];
    let mut previous_positions = HashSet::new();

    climb(
        map.start_position,
        &mut previous_positions,
        &map,
        0,
        &mut paths,
    );

    paths.sort_unstable();
    paths.reverse();

    log::info!(
        "part 1, shortest path from S to E is {} steps",
        paths.first().unwrap_or(&0)
    );

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

fn climb(
    current_position: (usize, usize),
    previous_positions: &mut HashSet<(usize, usize)>,
    map: &Map,
    step: u32,
    paths: &mut Vec<u32>,
) {
    if step > 500 {
        return;
    }

    let current_elevation = map
        .elevations
        .get(current_position.0)
        .unwrap()
        .get(current_position.1)
        .unwrap();

    for square in adjacent_squares(current_position, previous_positions, map) {
        previous_positions.clear();

        // Add this solution to the vector of solutions
        if square == map.end_position {
            paths.push(step + 1);
            continue;
        }

        let square_elevation = map.elevations.get(square.0).unwrap().get(square.1).unwrap();

        // For squares that are not the starting position, this square is too high to be a valid
        // next step
        if current_position != map.start_position && *square_elevation > *current_elevation + 1 {
            continue;
        }

        climb(square, previous_positions, map, step + 1, paths);
    }
}

fn adjacent_squares(
    current_position: (usize, usize),
    previous_positions: &HashSet<(usize, usize)>,
    map: &Map,
) -> Vec<(usize, usize)> {
    let mut adjacent_squares = Vec::with_capacity(4);

    // Up
    if current_position.0 > 0 {
        let up = (current_position.0 - 1, current_position.1);
        if !previous_positions.contains(&up) {
            adjacent_squares.push(up);
        }
    }

    // Down
    if current_position.0 < map.elevations.len() - 1 {
        let down = (current_position.0 + 1, current_position.1);
        if !previous_positions.contains(&down) {
            adjacent_squares.push(down);
        }
    }

    // Left
    if current_position.1 > 0 {
        let left = (current_position.0, current_position.1 - 1);
        if !previous_positions.contains(&left) {
            adjacent_squares.push(left);
        }
    }

    // Right
    if current_position.1 < map.elevations[0].len() - 1 {
        let right = (current_position.0, current_position.1 + 1);
        if !previous_positions.contains(&right) {
            adjacent_squares.push(right);
        }
    }

    adjacent_squares
}
