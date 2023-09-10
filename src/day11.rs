use std::collections::{HashMap, VecDeque};

use anyhow::{anyhow, bail};

/// The number of rounds to simulate.
const ROUNDS: usize = 20;

/// The number by which to divide your worry level.
const WORRY_LEVEL_DIVISOR: u32 = 3;

/// The number of monkeys expected in the input.
const WANT_MONKEYS: usize = 4;

pub fn main(input: String) -> anyhow::Result<()> {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").flat_map(Monkey::try_from).collect();

    let n = monkeys.len();
    if n != WANT_MONKEYS {
        bail!("want {WANT_MONKEYS} monkeys, got {n}");
    }

    let mut throw_items: HashMap<usize, VecDeque<u32>> = HashMap::with_capacity(8);

    for round in 0..ROUNDS {
        log::info!("starting round {}", round + 1);

        for (m, monkey) in monkeys.iter_mut().enumerate() {
            if let Some(queue) = throw_items.get_mut(&m) {
                while let Some(mut worry_level) = queue.pop_front() {
                    let next_worry_level =
                        (monkey.operation)(worry_level) / WORRY_LEVEL_DIVISOR % monkey.test_divisor;
                    for i in 0..1000 {
                        if (monkey.operation)(i) / WORRY_LEVEL_DIVISOR % monkey.test_divisor
                            == next_worry_level
                        {
                            worry_level = i;
                            break;
                        }
                    }
                    monkey.items.push(worry_level);
                }
            }

            for item in &monkey.items {
                // assert_eq!(
                //     (monkey.operation)(*item) / WORRY_LEVEL_DIVISOR % monkey.test_divisor,
                //     (monkey.modulo_operation)(*item, monkey.test_divisor) / WORRY_LEVEL_DIVISOR
                //         % monkey.test_divisor,
                //     "monkey {m} assertion failed: item={item}, WORRY_LEVEL_DIVISOR={WORRY_LEVEL_DIVISOR}, test_divisor={}", monkey.test_divisor
                // );

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
        .iter()
        .take(2)
        .fold(1, |acc, monkey| acc * monkey.inspected);

    for (m, monkey) in monkeys.iter().enumerate() {
        log::info!("monkey {m} inspected {} items", monkey.inspected);
    }
    log::info!("level of monkey business: {monkey_business}");

    Ok(())
}

/// Finds the result of modular division.
fn mod_divide(a: u32, b: u32, m: u32) -> Option<u32> {
    mod_inverse(b, m).map(|inv| inv * (a % m) % m)
}

/// Finds the modular inverse.
fn mod_inverse(b: u32, m: u32) -> Option<u32> {
    let (mut x, mut y) = (0, 0);
    let gcd = gcd(b, m, &mut x, &mut y);

    if gcd != 1 {
        None
    } else {
        Some((x as u32 % m + m) % m)
    }
}

/// Finds the GCD.
fn gcd(a: u32, b: u32, x: &mut i32, y: &mut i32) -> u32 {
    if a == 0 {
        (*x, *y) = (0, 1);
        return b;
    }

    let (mut x1, mut y1) = (0, 0);
    let gcd = gcd(b % a, a, &mut x1, &mut y1);

    *x = y1 - (b as i32 / a as i32) * x1;
    *y = x1;

    gcd
}

/// A monkey that took your items.
#[derive(Debug)]
struct Monkey {
    /// Items with a worry level.
    items: Vec<u32>,
    /// Operation to calculate the new worry level
    operation: fn(u32) -> u32,
    /// Modulo operation to calculate the new worry level
    modulo_operation: fn(u32, u32) -> u32,
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
            // test input
            0 => |i| i * 19,
            1 => |i| i + 6,
            2 => |i| i * i,
            3 => |i| i + 3,

            // real input
            // 0 => |i| i * 13,
            // 1 => |i| i + 2,
            // 2 => |i| i + 1,
            // 3 => |i| i + 8,
            // 4 => |i| i * i,
            // 5 => |i| i + 4,
            // 6 => |i| i * 17,
            // 7 => |i| i + 5,
            _ => bail!("failed to get operation for monkey"),
        };

        let modulo_operation = match n {
            // test input
            0 => |i, d| (i % d) * (19 % d),
            1 => |i, d| (i % d) + (6 % d),
            2 => |i, d| (i % d) * (i % d),
            3 => |i, d| (i % d) + (3 % d),

            // real input
            // 0 => |i, d| (i % d) * (13 % d),
            // 1 => |i, d| (i % d) + (2 % d),
            // 2 => |i, d| (i % d) + (1 % d),
            // 3 => |i, d| (i % d) + (8 % d),
            // 4 => |i, d| (i % d) * (i % d),
            // 5 => |i, d| (i % d) + (4 % d),
            // 6 => |i, d| (i % d) * (17 % d),
            // 7 => |i, d| (i % d) + (5 % d),
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
            modulo_operation,
            test_divisor,
            true_monkey,
            false_monkey,
            inspected: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_divide() {
        let (a, b, m) = (8, 3, 5);
        assert_eq!(mod_divide(a, b, m), Some(1));
    }
}
