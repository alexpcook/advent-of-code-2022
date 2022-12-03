pub fn main(input: String) -> anyhow::Result<()> {
    let common_item_priority_sum: u64 = input
        .lines()
        .map(|rucksack| {
            let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);
            first_compartment
                .chars()
                .find(|&c| second_compartment.contains(c))
                .map(get_priority)
                .unwrap_or_default()
        })
        .sum();

    // Part 1
    log::info!("part 1: {common_item_priority_sum}");

    let badge_priority_sum: u64 = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| {
            group
                .first()
                .and_then(|first_rucksack| {
                    first_rucksack
                        .chars()
                        .filter_map(|c| {
                            matches!(
                                (group.get(1), group.get(2)),
                                (Some(second_rucksack), Some(third_rucksack))
                                    if second_rucksack.contains(c) && third_rucksack.contains(c)
                            )
                            .then(|| get_priority(c))
                        })
                        .last()
                })
                .unwrap_or_default()
        })
        .sum();

    // Part 2
    log::info!("badge priority sum: {badge_priority_sum}");

    Ok(())
}

/// Type alias for the rucksack items.
type Item = char;

/// Gets the priority of an item.
fn get_priority(item: Item) -> u64 {
    if item.is_ascii_lowercase() {
        item as u64 - 'a' as u64 + 1
    } else {
        item as u64 - 'A' as u64 + 27
    }
}
