pub fn main(input: String) -> anyhow::Result<()> {
    let rock_paper_scissors: Vec<(char, char)> = input
        .lines()
        .map(|s| {
            s.split_once(' ')
                .map(|(s1, s2)| (s1.parse().unwrap(), s2.parse().unwrap()))
                .unwrap()
        })
        .collect();

    // Part 1
    log::info!("part 1 total score: {}", part1(&rock_paper_scissors));

    // Part 2
    log::info!("part 2 total score: {}", part2(&rock_paper_scissors));

    Ok(())
}

fn part1(games: &[(char, char)]) -> u64 {
    games
        .iter()
        .map(|&(opponent_move, your_move)| {
            let shape_points = your_move as u64 - 'W' as u64; // X=1,Y=2,Z=3
            let outcome_points = match (opponent_move, your_move) {
                ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3, // draw
                ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6, // win
                _ => 0,                                    // loss
            };
            shape_points + outcome_points
        })
        .sum()
}

fn part2(games: &[(char, char)]) -> u64 {
    games
        .iter()
        .map(|&(opponent_move, desired_result)| {
            let outcome_points = 3 * (desired_result as u64 - 'X' as u64); // X=0,Y=3,Z=6
            let shape_points = match (opponent_move, desired_result) {
                ('A', 'Y') | ('B', 'X') | ('C', 'Z') => 1, // rock
                ('B', 'Y') | ('A', 'Z') | ('C', 'X') => 2, // paper
                _ => 3,                                    // scissors
            };
            outcome_points + shape_points
        })
        .sum()
}
