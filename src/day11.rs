use std::collections::{HashMap, VecDeque};

use anyhow::{anyhow, bail};

/// The number of rounds to simulate.
const ROUNDS: usize = 20;

/// The number by which to divide your worry level.
const WORRY_LEVEL_DIVISOR: u32 = 3;

pub fn main(input: String) -> anyhow::Result<()> {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").flat_map(Monkey::try_from).collect();

    let n = monkeys.len();
    if n != 8 {
        bail!("want 8 monkeys, got {n}");
    }

    let mut throw_items: HashMap<usize, VecDeque<u32>> = HashMap::with_capacity(8);

    for round in 0..ROUNDS {
        log::info!("starting round {round}");

        for (m, monkey) in monkeys.iter_mut().enumerate() {
            if let Some(queue) = throw_items.get_mut(&m) {
                while let Some(worry_level) = queue.pop_front() {
                    let mut adjusted_worry_level = worry_level % monkey.test_divisor;

                    while (monkey.operation)(worry_level) / WORRY_LEVEL_DIVISOR
                        != (monkey.operation)(adjusted_worry_level) / WORRY_LEVEL_DIVISOR
                    {
                        adjusted_worry_level += monkey.test_divisor;
                    }

                    assert_eq!(
                        (monkey.operation)(worry_level) / WORRY_LEVEL_DIVISOR % monkey.test_divisor,
                        (monkey.operation)(adjusted_worry_level) / WORRY_LEVEL_DIVISOR
                            % monkey.test_divisor,
                    "monkey {m}, worry level {worry_level}, adjusted worry_level {adjusted_worry_level}");

                    monkey.items.push(adjusted_worry_level);
                }
            }

            for item in &monkey.items {
                let worry_level = (monkey.operation)(*item) / WORRY_LEVEL_DIVISOR;

                let to_monkey = if worry_level % monkey.test_divisor == 0 {
                    monkey.true_monkey
                } else {
                    monkey.false_monkey
                };

                throw_items
                    .entry(to_monkey)
                    .and_modify(|queue| queue.push_back(worry_level))
                    .or_insert_with(|| VecDeque::from([worry_level]));
            }

            monkey.inspected += monkey.items.len() as u64;
            monkey.items.clear();
        }
    }

    monkeys.sort_unstable_by(|monkey1, monkey2| {
        monkey2.inspected.partial_cmp(&monkey1.inspected).unwrap()
    });

    let monkey_business = monkeys
        .into_iter()
        .take(2)
        .fold(1, |acc, monkey| acc * monkey.inspected);

    log::info!("level of monkey business: {monkey_business}");

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
    /// The number of items inspected by the monkey.
    inspected: u64,
}

impl TryFrom<&str> for Monkey {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lines: Vec<_> = value.lines().collect();

        let n: usize = lines
            .first()
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
            inspected: 0,
        })
    }
}
