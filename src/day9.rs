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

    for (direction, distance) in &directions {
        for _ in 0..*distance {
            let head_position = &mut rope.head.current_position;
            let tail_position = &mut rope.tail.current_position;

            match *direction {
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
        "part 1 - number of positions the tail visited at least once: {}",
        rope.tail.position_history.len()
    );

    let mut rope2 = Rope2::new(10);

    for (direction, distance) in &directions {
        for _ in 0..*distance {
            match *direction {
                "U" => {
                    for i in 0..rope2.nodes.len() {
                        if i == 0 {
                            let head_node = rope2.nodes.get_mut(i).unwrap();
                            head_node.current_position.1 += 1;
                            head_node
                                .position_history
                                .insert(head_node.current_position);
                        } else {
                            let prior_node_position =
                                rope2.nodes.get(i - 1).unwrap().current_position;
                            let current_node = rope2.nodes.get_mut(i).unwrap();
                            if more_than_one_apart(
                                prior_node_position,
                                current_node.current_position,
                            ) {
                                if prior_node_position.0 - current_node.current_position.0 > 1 {
                                    current_node.current_position.0 += 1;
                                } else if current_node.current_position.0 - prior_node_position.0
                                    > 1
                                {
                                    current_node.current_position.0 -= 1;
                                } else if current_node
                                    .current_position
                                    .0
                                    .abs_diff(prior_node_position.0)
                                    == 1
                                {
                                    current_node.current_position.0 = prior_node_position.0;
                                }
                                if prior_node_position.1 - current_node.current_position.1 > 1 {
                                    current_node.current_position.1 += 1;
                                } else if current_node.current_position.1 - prior_node_position.1
                                    > 1
                                {
                                    current_node.current_position.1 -= 1;
                                } else if current_node
                                    .current_position
                                    .1
                                    .abs_diff(prior_node_position.1)
                                    == 1
                                {
                                    current_node.current_position.1 = prior_node_position.1;
                                }

                                current_node
                                    .position_history
                                    .insert(current_node.current_position);
                            } else {
                                break;
                            }
                        }
                    }
                }
                "D" => {
                    for i in 0..rope2.nodes.len() {
                        if i == 0 {
                            let head_node = rope2.nodes.get_mut(i).unwrap();
                            head_node.current_position.1 -= 1;
                            head_node
                                .position_history
                                .insert(head_node.current_position);
                        } else {
                            let prior_node_position =
                                rope2.nodes.get(i - 1).unwrap().current_position;
                            let current_node = rope2.nodes.get_mut(i).unwrap();
                            if more_than_one_apart(
                                prior_node_position,
                                current_node.current_position,
                            ) {
                                if prior_node_position.0 - current_node.current_position.0 > 1 {
                                    current_node.current_position.0 += 1;
                                } else if current_node.current_position.0 - prior_node_position.0
                                    > 1
                                {
                                    current_node.current_position.0 -= 1;
                                } else if current_node
                                    .current_position
                                    .0
                                    .abs_diff(prior_node_position.0)
                                    == 1
                                {
                                    current_node.current_position.0 = prior_node_position.0;
                                }
                                if prior_node_position.1 - current_node.current_position.1 > 1 {
                                    current_node.current_position.1 += 1;
                                } else if current_node.current_position.1 - prior_node_position.1
                                    > 1
                                {
                                    current_node.current_position.1 -= 1;
                                } else if current_node
                                    .current_position
                                    .1
                                    .abs_diff(prior_node_position.1)
                                    == 1
                                {
                                    current_node.current_position.1 = prior_node_position.1;
                                }

                                current_node
                                    .position_history
                                    .insert(current_node.current_position);
                            } else {
                                break;
                            }
                        }
                    }
                }
                "L" => {
                    for i in 0..rope2.nodes.len() {
                        if i == 0 {
                            let head_node = rope2.nodes.get_mut(i).unwrap();
                            head_node.current_position.0 -= 1;
                            head_node
                                .position_history
                                .insert(head_node.current_position);
                        } else {
                            let prior_node_position =
                                rope2.nodes.get(i - 1).unwrap().current_position;
                            let current_node = rope2.nodes.get_mut(i).unwrap();
                            if more_than_one_apart(
                                prior_node_position,
                                current_node.current_position,
                            ) {
                                if prior_node_position.0 - current_node.current_position.0 > 1 {
                                    current_node.current_position.0 += 1;
                                } else if current_node.current_position.0 - prior_node_position.0
                                    > 1
                                {
                                    current_node.current_position.0 -= 1;
                                } else if current_node
                                    .current_position
                                    .0
                                    .abs_diff(prior_node_position.0)
                                    == 1
                                {
                                    current_node.current_position.0 = prior_node_position.0;
                                }

                                if prior_node_position.1 - current_node.current_position.1 > 1 {
                                    current_node.current_position.1 += 1;
                                } else if current_node.current_position.1 - prior_node_position.1
                                    > 1
                                {
                                    current_node.current_position.1 -= 1;
                                } else if current_node
                                    .current_position
                                    .1
                                    .abs_diff(prior_node_position.1)
                                    == 1
                                {
                                    current_node.current_position.1 = prior_node_position.1;
                                }
                                current_node
                                    .position_history
                                    .insert(current_node.current_position);
                            } else {
                                break;
                            }
                        }
                    }
                }
                "R" => {
                    for i in 0..rope2.nodes.len() {
                        if i == 0 {
                            let head_node = rope2.nodes.get_mut(i).unwrap();
                            head_node.current_position.0 += 1;
                            head_node
                                .position_history
                                .insert(head_node.current_position);
                        } else {
                            let prior_node_position =
                                rope2.nodes.get(i - 1).unwrap().current_position;
                            let current_node = rope2.nodes.get_mut(i).unwrap();
                            if more_than_one_apart(
                                prior_node_position,
                                current_node.current_position,
                            ) {
                                if prior_node_position.0 - current_node.current_position.0 > 1 {
                                    current_node.current_position.0 += 1;
                                } else if current_node.current_position.0 - prior_node_position.0
                                    > 1
                                {
                                    current_node.current_position.0 -= 1;
                                } else if current_node
                                    .current_position
                                    .0
                                    .abs_diff(prior_node_position.0)
                                    == 1
                                {
                                    current_node.current_position.0 = prior_node_position.0;
                                }

                                if prior_node_position.1 - current_node.current_position.1 > 1 {
                                    current_node.current_position.1 += 1;
                                } else if current_node.current_position.1 - prior_node_position.1
                                    > 1
                                {
                                    current_node.current_position.1 -= 1;
                                } else if current_node
                                    .current_position
                                    .1
                                    .abs_diff(prior_node_position.1)
                                    == 1
                                {
                                    current_node.current_position.1 = prior_node_position.1;
                                }

                                current_node
                                    .position_history
                                    .insert(current_node.current_position);
                            } else {
                                break;
                            }
                        }
                    }
                }
                _ => bail!("{direction} is invalid"),
            }
        }
    }

    log::info!(
        "part2 - number of positions the tail visited at least once: {}",
        rope2.nodes.last().unwrap().position_history.len()
    );

    Ok(())
}

/// A node in the rope (e.g. head, tail).
#[derive(Debug, Clone)]
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

/// A rope with an arbitrary number of nodes.
#[derive(Debug)]
struct Rope2 {
    nodes: Vec<Node>,
}

impl Rope2 {
    /// Constructs a rope with `n` nodes.
    fn new(n: usize) -> Rope2 {
        Rope2 {
            nodes: vec![
                Node {
                    current_position: (0, 0),
                    position_history: HashSet::from([(0, 0)]),
                };
                n
            ],
        }
    }
}

fn more_than_one_apart(rhs: (i32, i32), lhs: (i32, i32)) -> bool {
    rhs.0.abs_diff(lhs.0) > 1 || rhs.1.abs_diff(lhs.1) > 1
}
