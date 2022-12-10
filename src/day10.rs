use anyhow::bail;

pub fn main(input: String) -> anyhow::Result<()> {
    let mut cpu = Cpu::new();

    let instructions: Vec<_> = input
        .lines()
        .filter_map(|line| Instruction::try_from(line).ok())
        .collect();

    for instruction in instructions {
        match instruction {
            Instruction::Noop => cpu.cycle(None),
            Instruction::Addx(x) => {
                for inc_x in [None, Some(x)] {
                    cpu.cycle(inc_x);
                }
            }
        }
    }

    // Part 1
    log::info!("part 1, sum of signal strengths: {}", cpu.state());

    Ok(())
}

/// A CPU instruction.
#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl TryFrom<&str> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<_> = value.split_whitespace().collect();

        let instruction = match parts.first() {
            None => bail!("failed to get instruction"),
            Some(instruction) => *instruction,
        };

        let instruction = match instruction {
            "noop" => Instruction::Noop,
            "addx" => match parts.get(1) {
                None => bail!("failed to get x increment value"),
                Some(x) => {
                    let x = x.parse()?;
                    Instruction::Addx(x)
                }
            },
            instruction => bail!("unknown instruction {instruction}"),
        };

        Ok(instruction)
    }
}

/// The communication system CPU.
#[derive(Debug)]
struct Cpu {
    /// The CPU cycle number.
    cycle: u64,
    /// The value of register X.
    x: i64,
    /// Keeps track of the 20th, 60th, 100th, etc. signal strengths.
    state: i64,
}

impl Cpu {
    /// The special signal strength cycles to measure.
    const MAGIC_CYCLES: [u64; 6] = [20, 60, 100, 140, 180, 220];

    /// Constructs a new CPU.
    fn new() -> Cpu {
        Cpu {
            cycle: 0,
            x: 1,
            state: 0,
        }
    }

    /// Performs one cycle of the CPU.
    fn cycle(&mut self, increment_x: Option<i64>) {
        self.cycle += 1;
        if Self::MAGIC_CYCLES.contains(&self.cycle) {
            self.state += self.cycle as i64 * self.x;
        }
        if let Some(x) = increment_x {
            self.x += x;
        }
    }

    /// Retrieves the final state calculation of the CPU.
    fn state(&self) -> i64 {
        self.state
    }
}
