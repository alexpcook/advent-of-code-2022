pub fn main(input: String) -> anyhow::Result<()> {
    let mut visible_trees = 0;
    let mut highest_scenic_score = 0;

    let forest: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

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

            let current_height = forest.get(i).unwrap().get(j).unwrap();

            // Up
            if (0..i).all(|n| {
                let other_height = forest.get(n).unwrap().get(j).unwrap();
                other_height < current_height
            }) {
                visible_trees += 1;
                continue;
            }

            // Down
            if (i + 1..rows).all(|n| {
                let other_height = forest.get(n).unwrap().get(j).unwrap();
                other_height < current_height
            }) {
                visible_trees += 1;
                continue;
            }

            // Left
            if (0..j).all(|n| {
                let other_height = forest.get(i).unwrap().get(n).unwrap();
                other_height < current_height
            }) {
                visible_trees += 1;
                continue;
            }

            // Right
            if (j + 1..cols).all(|n| {
                let other_height = forest.get(i).unwrap().get(n).unwrap();
                other_height < current_height
            }) {
                visible_trees += 1;
                continue;
            }
        }
    }

    // Part 1
    log::info!("number of trees visible from outside the forest: {visible_trees}");

    // Part 2
    log::info!("highest scenic score in the forest: {highest_scenic_score}");

    Ok(())
}
