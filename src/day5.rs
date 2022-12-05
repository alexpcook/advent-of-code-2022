pub fn main(input: String) -> anyhow::Result<()> {
    let (initial_stack, instructions) = input.split_once("\n\n").unwrap();

    let mut stacks = Stacks::default();

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
        .map(|i| Instruction::new(i.to_string()))
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

    todo!();
}

type Crate = char;

type Stacks = [Vec<Crate>; 9];

struct Instruction {
    quantity: usize,
    start: usize,
    end: usize,
}

impl Instruction {
    fn new(raw: String) -> Instruction {
        let parts: Vec<_> = raw.split_whitespace().map(|s| s.trim()).collect();

        let quantity: usize = parts.get(1).unwrap().parse().unwrap();
        let start: usize = parts.get(3).unwrap().parse().unwrap();
        let end: usize = parts.get(5).unwrap().parse().unwrap();

        Instruction {
            quantity,
            start,
            end,
        }
    }
}
