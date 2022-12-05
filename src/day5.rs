pub fn main(input: String) -> anyhow::Result<()> {
    let (raw_initial_stack, raw_instructions) = input.split_once("\n\n").unwrap();

    // There are nine stacks in this particular case
    let mut stacks = Stacks::<9>::default();

    let raw_initial_stack_rows: Vec<_> = raw_initial_stack
        .split('\n')
        .filter_map(|row| {
            let row = row.trim();
            // Skip the row with the stack numbers
            (!row.is_empty() && !row.contains('1')).then_some(row)
        })
        // Crates toward the bottom of each stack need to be added to the vector first
        .rev()
        .collect();

    for raw_initial_stack_row in raw_initial_stack_rows {
        // `crate` is a Rust keyword, so use `_crate` instead
        // Skip the first `[` character in each row and increment by four after that
        for (i, _crate) in raw_initial_stack_row.chars().skip(1).step_by(4).enumerate() {
            // Should always be true with valid input, but check just in case
            if _crate.is_ascii_uppercase() {
                stacks[i].push(_crate);
            }
        }
    }

    let instructions: Vec<_> = raw_instructions
        .split('\n')
        .filter_map(|s| {
            let s = s.trim();
            if s.is_empty() {
                None
            } else {
                Instruction::try_from(s).ok()
            }
        })
        .collect();

    // Part 1
    // We need a copy of the stacks for each part because we are mutating
    let mut stacks_for_part1 = stacks.clone();

    for instruction in &instructions {
        for _ in 0..instruction.quantity {
            let moved_crate = stacks_for_part1[instruction.start - 1]
                .pop()
                .unwrap_or_default();

            stacks_for_part1[instruction.end - 1].push(moved_crate);
        }
    }

    let top_crates_for_part1 = stacks_for_part1
        .iter()
        .fold(String::new(), |mut top, stack| {
            top.push(*stack.last().unwrap_or(&char::default()));
            top
        });

    println!("top crates for part 1: {top_crates_for_part1}");

    // Part 2
    let mut stacks_for_part2 = stacks;

    for instruction in &instructions {
        // Need a temporary holding place for moved crates in each instruction
        let mut temp = Vec::with_capacity(instruction.quantity);

        for _ in 0..instruction.quantity {
            let moved_crate = stacks_for_part2[instruction.start - 1]
                .pop()
                .unwrap_or_default();

            temp.push(moved_crate);
        }

        // Moving multiple crates at once means order is reversed from part 1
        temp.reverse();

        for moved_crate in temp {
            stacks_for_part2[instruction.end - 1].push(moved_crate);
        }
    }

    let top_crates_for_part2 = stacks_for_part2
        .iter()
        .fold(String::new(), |mut top, stack| {
            top.push(*stack.last().unwrap_or(&char::default()));
            top
        });

    println!("top crates for part 2: {top_crates_for_part2}");

    Ok(())
}

/// A crate contains an ASCII uppercase letter.
type Crate = char;

/// There are `N` stacks of `Crate`s.
type Stacks<const N: usize> = [Vec<Crate>; N];

/// An instruction moves `quantity` crates from `start` stack to `end` stack.
struct Instruction {
    quantity: usize,
    start: usize,
    end: usize,
}

impl TryFrom<&str> for Instruction {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<_> = value
            .split_whitespace()
            .filter_map(|part| {
                let part = part.trim();
                (!part.is_empty()).then_some(part)
            })
            .collect();

        Ok(Instruction {
            quantity: parts
                .get(1)
                .ok_or("failed to find quantity")?
                .parse()
                .map_err(|_| "failed to parse quantity")?,
            start: parts
                .get(3)
                .ok_or("failed to find start")?
                .parse()
                .map_err(|_| "failed to parse start")?,
            end: parts
                .get(5)
                .ok_or("failed to find end")?
                .parse()
                .map_err(|_| "failed to parse end")?,
        })
    }
}
