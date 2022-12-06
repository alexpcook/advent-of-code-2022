use std::collections::HashSet;

pub fn main(input: String) -> anyhow::Result<()> {
    let chars: Vec<_> = input.chars().collect();

    // Part 1
    log::info!(
        "first start of packet marker: {}",
        first_distinct_chunk(&chars, 4).unwrap_or_default()
    );

    // Part 2
    log::info!(
        "first start of message marker: {}",
        first_distinct_chunk(&chars, 14).unwrap_or_default()
    );

    Ok(())
}

/// Returns the number of characters processed in `chars` to arrive at a chunk of length `n` of all
/// distinct characters. Returns `None` if no chunks of size `n` contain all distinct characters.
fn first_distinct_chunk(chars: &[char], n: usize) -> Option<usize> {
    let mut i = 0;
    let mut contains = HashSet::with_capacity(n);

    while let Some(chunk) = chars.get(i..i + n) {
        for c in chunk {
            contains.insert(c);
        }

        if contains.len() == n {
            return Some(i + n);
        }

        contains.clear();
        i += 1;
    }

    None
}
