use std::collections::HashSet;

use anyhow::bail;

pub fn main(input: String) -> anyhow::Result<()> {
    let directions: Vec<(&str, i32)> = input
        .lines()
        .flat_map(|line| {
            line.split_once(' ')
                .map(|(direction, distance)| (direction, distance.parse().unwrap_or_default()))
        })
        .collect();

    let mut rope = Rope::new(None);

    for (direction, distance) in directions {
        for _ in 0..distance {
            let head_position = &mut rope.head.current_position;
            let tail_position = &mut rope.tail.current_position;

            match direction {
                "U" => {
                    head_position.1 += 1;
                    // The head pulls the tail
                    if head_position.1.abs_diff(tail_position.1) > 1 {
                        tail_position.1 += 1;
                        // The tail will always be aligned with the head on the x axis
                        tail_position.0 = head_position.0;
                    }
                }
                "D" => {
                    head_position.1 -= 1;
                    // The head pulls the tail
                    if head_position.1.abs_diff(tail_position.1) > 1 {
                        tail_position.1 -= 1;
                        // The tail will always be aligned with the head on the x axis
                        tail_position.0 = head_position.0;
                    }
                }
                "L" => {
                    head_position.0 -= 1;
                    // The head pulls the tail
                    if head_position.0.abs_diff(tail_position.0) > 1 {
                        tail_position.0 -= 1;
                        // The tail will always be aligned with the head on the y axis
                        tail_position.1 = head_position.1;
                    }
                }
                "R" => {
                    head_position.0 += 1;
                    // The head pulls the tail
                    if head_position.0.abs_diff(tail_position.0) > 1 {
                        tail_position.0 += 1;
                        // The tail will always be aligned with the head on the y axis
                        tail_position.1 = head_position.1;
                    }
                }
                _ => bail!("{direction} is invalid"),
            }

            rope.head.position_history.insert(*head_position);
            rope.tail.position_history.insert(*tail_position);
        }
    }

    log::info!(
        "number of positions the tail visited at least once: {}",
        rope.tail.position_history.len()
    );

    Ok(())
}

/// A node in the rope (e.g. head, tail).
#[derive(Debug)]
struct Node {
    current_position: (i32, i32),
    position_history: HashSet<(i32, i32)>,
}

/// A rope with a head and tail.
#[derive(Debug)]
struct Rope {
    head: Node,
    tail: Node,
}

impl Rope {
    /// Default starting position of the rope's head and tail.
    const DEFAULT_START_POSITION: (i32, i32) = (0, 0);

    /// Creates a new rope.
    fn new(start_position: Option<(i32, i32)>) -> Rope {
        let start_position = start_position.unwrap_or(Self::DEFAULT_START_POSITION);
        Rope {
            head: Node {
                current_position: start_position,
                position_history: HashSet::from([start_position]),
            },
            tail: Node {
                current_position: start_position,
                position_history: HashSet::from([(start_position)]),
            },
        }
    }
}
