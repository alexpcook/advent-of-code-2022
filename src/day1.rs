pub fn main(input: String) -> anyhow::Result<()> {
    const ELF_DELIMITER: &str = "\n\n";
    const ITEM_DELIMITER: char = '\n';

    // Last elf has a trailing newline
    let input = input.trim();

    let elves: Vec<_> = input
        .split(ELF_DELIMITER)
        .map(|elf_calories| {
            let items = elf_calories
                .split(ITEM_DELIMITER)
                .map(|item_calories| item_calories.parse().unwrap_or_default())
                .collect();

            Elf { items }
        })
        .collect();

    let mut calorie_totals: Vec<_> = elves.iter().map(|elf| elf.total()).collect();

    calorie_totals.sort();
    calorie_totals.reverse();

    // Part 1
    let most_calories: u32 = calorie_totals.iter().take(1).sum();
    log::info!("most calories carried: {most_calories}");

    // Part 2
    let top_3_calories: u32 = calorie_totals.iter().take(3).sum();
    log::info!("sum of top three calories carried: {top_3_calories}");

    Ok(())
}

/// Item that contains calories.
type Item = u32;

/// Elf that carries items.
struct Elf {
    items: Vec<Item>,
}

impl Elf {
    /// Returns the sum of calories carried by the elf.
    fn total(&self) -> u32 {
        self.items.iter().sum()
    }
}
