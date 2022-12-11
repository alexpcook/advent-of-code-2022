use anyhow::{anyhow, bail};

pub fn main(input: String) -> anyhow::Result<()> {
    let monkeys: Vec<Monkey> = input
        .split("\n\n")
        .flat_map(|lines| Monkey::try_from(lines))
        .collect();

    println!("{monkeys:?}");

    Ok(())
}

/// A monkey that took your items.
#[derive(Debug)]
struct Monkey {
    /// Items with a worry level.
    items: Vec<u32>,
    /// Operation to calculate the new worry level
    operation: fn(u32) -> u32,
    /// How the monkey decides where to throw the item next.
    test_divisor: u32,
    /// Which monkey gets the item if the test is true.
    true_monkey: usize,
    /// Which monkey gets the item if the test is false.
    false_monkey: usize,
}

impl TryFrom<&str> for Monkey {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lines: Vec<_> = value.lines().collect();

        let n: usize = lines
            .get(0)
            .and_then(|line| {
                line.chars()
                    .find(|c| c.is_ascii_digit())
                    .and_then(|c| c.to_digit(10))
            })
            .ok_or_else(|| anyhow!("failed to get number for monkey"))?
            as usize;

        let items: Vec<u32> = lines
            .get(1)
            .and_then(|line| {
                line.split_once(": ")
                    .and_then(|(_, nums)| nums.split(", ").map(|n| n.parse().ok()).collect())
            })
            .ok_or_else(|| anyhow!("failed to get items for monkey"))?;

        let operation = match n {
            0 => |i| i * 13,
            1 => |i| i + 2,
            2 => |i| i + 1,
            3 => |i| i + 8,
            4 => |i| i * i,
            5 => |i| i + 4,
            6 => |i| i * 17,
            7 => |i| i + 5,
            _ => bail!("failed to get operation for monkey"),
        };

        let test_divisor: u32 = lines
            .get(3)
            .and_then(|line| {
                line.split_once("divisible by ")
                    .and_then(|(_, divisor)| divisor.parse().ok())
            })
            .ok_or_else(|| anyhow!("failed to get test divisor for monkey"))?;

        let true_monkey: usize = lines
            .get(4)
            .and_then(|line| {
                line.split_once("true: throw to monkey ")
                    .and_then(|(_, n)| n.parse().ok())
            })
            .ok_or_else(|| anyhow!("failed to get true monkey for monkey"))?;

        let false_monkey: usize = lines
            .get(5)
            .and_then(|line| {
                line.split_once("false: throw to monkey ")
                    .and_then(|(_, n)| n.parse().ok())
            })
            .ok_or_else(|| anyhow!("failed to get false monkey for monkey"))?;

        Ok(Monkey {
            items,
            operation,
            test_divisor,
            true_monkey,
            false_monkey,
        })
    }
}
