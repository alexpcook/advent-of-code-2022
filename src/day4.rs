pub fn main(input: String) -> anyhow::Result<()> {
    let pairs: Vec<((u32, u32), (u32, u32))> = input
        .lines()
        .map(|l| {
            let (first, second) = l.split_once(',').unwrap();
            let (start1, end1) = first.split_once('-').unwrap();
            let (start2, end2) = second.split_once('-').unwrap();
            (
                (start1.parse().unwrap(), end1.parse().unwrap()),
                (start2.parse().unwrap(), end2.parse().unwrap()),
            )
        })
        .collect();

    let mut fully_contained_count = 0;
    let mut any_overlap_count = 0;

    for ((start1, end1), (start2, end2)) in pairs {
        if (start1 <= start2 && end1 >= end2) || (start2 <= start1 && end2 >= end1) {
            fully_contained_count += 1;
        }

        if (end1 >= start2 && start1 <= start2) || (end2 >= start1 && start2 <= start1) {
            any_overlap_count += 1;
        }
    }

    // Part 1
    log::info!("full contained count: {fully_contained_count}");

    // Part 2
    log::info!("any overlap: {any_overlap_count}");

    Ok(())
}
