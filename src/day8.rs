pub fn main(input: String) -> anyhow::Result<()> {
    let forest: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    // Part 1
    let mut visible_trees = 0;

    let rows = forest.len();

    for (i, row) in forest.iter().enumerate() {
        let cols = row.len();

        if i == 0 || i == rows - 1 {
            visible_trees += cols;
            continue;
        }

        for (j, tree) in row.iter().enumerate() {
            if j == 0 || j == cols - 1 {
                visible_trees += 1;
                continue;
            }

            // Up
            if (0..i).all(|n| {
                let other_height = forest.get(n).unwrap().get(j).unwrap();
                other_height < tree
            }) {
                visible_trees += 1;
                continue;
            }

            // Down
            if (i + 1..rows).all(|n| {
                let other_height = forest.get(n).unwrap().get(j).unwrap();
                other_height < tree
            }) {
                visible_trees += 1;
                continue;
            }

            // Left
            if (0..j).all(|n| {
                let other_height = forest.get(i).unwrap().get(n).unwrap();
                other_height < tree
            }) {
                visible_trees += 1;
                continue;
            }

            // Right
            if (j + 1..cols).all(|n| {
                let other_height = forest.get(i).unwrap().get(n).unwrap();
                other_height < tree
            }) {
                visible_trees += 1;
                continue;
            }
        }
    }

    log::info!("number of trees visible from outside the forest: {visible_trees}");

    // Part 2
    let mut highest_scenic_score = 0;

    let rows = forest.len();

    for (i, row) in forest.iter().enumerate() {
        let cols = row.len();

        for (j, tree) in row.iter().enumerate() {
            // Up
            let up_score = if i == 0 {
                0
            } else {
                match (0..i).position(|n| {
                    let other_height = forest.get(i - n - 1).unwrap().get(j).unwrap();
                    other_height >= tree
                }) {
                    Some(blocking_tree_iter_pos) => blocking_tree_iter_pos + 1,
                    None => i,
                }
            };

            // Down
            let down_score = if i == rows - 1 {
                0
            } else {
                match (i + 1..rows).position(|n| {
                    let other_height = forest.get(n).unwrap().get(j).unwrap();
                    other_height >= tree
                }) {
                    Some(blocking_tree_iter_pos) => blocking_tree_iter_pos + 1,
                    None => rows - i - 1,
                }
            };

            // Left
            let left_score = if j == 0 {
                0
            } else {
                match (0..j).position(|n| {
                    let other_height = forest.get(i).unwrap().get(j - n - 1).unwrap();
                    other_height >= tree
                }) {
                    Some(blocking_tree_iter_pos) => blocking_tree_iter_pos + 1,
                    None => j,
                }
            };

            // Right
            let right_score = if j == cols - 1 {
                0
            } else {
                match (j + 1..cols).position(|n| {
                    let other_height = forest.get(i).unwrap().get(n).unwrap();
                    other_height >= tree
                }) {
                    Some(blocking_tree_iter_pos) => blocking_tree_iter_pos + 1,
                    None => cols - j - 1,
                }
            };

            let scenic_score = up_score * down_score * left_score * right_score;

            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
            }
        }
    }

    log::info!("highest scenic score in the forest: {highest_scenic_score}");

    Ok(())
}
