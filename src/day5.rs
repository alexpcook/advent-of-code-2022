pub fn main(input: String) -> anyhow::Result<()> {
    let (initial_stack, instructions) = input.split_once("\n\n").unwrap();

    let mut stacks = Stacks::<9>::default();

    let rows: Vec<_> = initial_stack.split('\n').collect();

    for row in rows {
        for (i, a_crate) in row.chars().skip(1).step_by(4).enumerate() {
            println!("{i} {a_crate}");
            if a_crate.is_ascii_uppercase() {
                stacks[i].push(a_crate);
            }
        }
    }

    stacks.iter_mut().for_each(|s| s.reverse());

    let instructions: Vec<_> = instructions
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .filter_map(|i| Instruction::try_from(i).ok())
        .collect();

    for instruction in instructions {
        let mut temp = vec![];
        for _ in 0..instruction.quantity {
            let moved = stacks[instruction.start - 1].pop().unwrap();
            temp.push(moved);
        }
        temp.reverse();
        for moved in temp {
            stacks[instruction.end - 1].push(moved);
        }
    }

    for stack in stacks {
        print!("{}", stack.get(stack.len() - 1).unwrap());
    }

    println!();

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
            .filter_map(|s| {
                let s = s.trim();
                (!s.is_empty()).then_some(s)
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
