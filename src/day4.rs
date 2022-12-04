pub fn main(input: String) -> anyhow::Result<()> {
    let pairs: Vec<((u32, u32), (u32, u32))> = input
        .lines()
        .map(|line| {
            line.split_once(',')
                .and_then(|(first_range, second_range)| {
                    first_range
                        .split_once('-')
                        .and_then(|(start, end)| start.parse().ok().zip(end.parse().ok()))
                        .zip(
                            second_range
                                .split_once('-')
                                .and_then(|(start, end)| start.parse().ok().zip(end.parse().ok())),
                        )
                })
                .unwrap_or_default()
        })
        .collect();

    // Part 1
    let fully_contained_count = pairs
        .clone()
        .into_iter()
        .filter(one_range_is_fully_contained)
        .count();
    log::info!("full contained count: {fully_contained_count}");

    // Part 2
    let any_overlap_count = pairs.into_iter().filter(ranges_have_any_overlap).count();
    log::info!("any overlap count: {any_overlap_count}");

    Ok(())
}

/// Is one range fully contained within the other?
fn one_range_is_fully_contained(ranges: &((u32, u32), (u32, u32))) -> bool {
    let lhs = ranges.0;
    let rhs = ranges.1;
    (lhs.0 <= rhs.0 && lhs.1 >= rhs.1) || (rhs.0 <= lhs.0 && rhs.1 >= lhs.1)
}

/// Do the ranges overlap at all?
fn ranges_have_any_overlap(ranges: &((u32, u32), (u32, u32))) -> bool {
    let lhs = ranges.0;
    let rhs = ranges.1;
    (rhs.0 <= lhs.0 || lhs.1 >= rhs.0) && lhs.0 <= rhs.1
}
