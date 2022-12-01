pub fn main(input: String) -> anyhow::Result<()> {
    let elves_calories: Vec<_> = input.split("\n\n").collect();

    let mut elves = Vec::with_capacity(elves_calories.len());

    for elf_calories in elves_calories {
        let elf_calories = elf_calories.trim();
        log::debug!("elf calories: {elf_calories}");

        let items_calories: Vec<_> = elf_calories.split('\n').collect();
        log::debug!("items calories: {items_calories:?}");

        let mut elf = Elf {
            items: Vec::with_capacity(items_calories.len()),
        };

        for item_calories in items_calories {
            let item: Item = item_calories.parse()?;
            log::debug!("item calories: {item_calories}");

            elf.items.push(item);
        }

        elves.push(elf);
    }

    let mut calorie_sums: Vec<_> = elves.into_iter().map(|elf| elf.total()).collect();

    calorie_sums.sort();
    calorie_sums.reverse();

    // Part 1
    let most_calories: u32 = calorie_sums.iter().take(1).sum();
    log::info!("most calories carried: {most_calories}");

    // Part 2
    let top_3_calories: u32 = calorie_sums.iter().take(3).sum();
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
