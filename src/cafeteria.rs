use itertools::Itertools;

use crate::random_utils::{parse_number, parse_numbers_whitespace};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn available_fresh_ingredients(input: &str) -> usize {
    let (ranges, ingredients) = ranges_and_ingredients(input);

    // Just filter available ingredients
    ingredients
        .filter(|id| ranges.iter().any(|(start, end)| start <= id && id <= end))
        .count()
}

pub fn all_possible_fresh_ingredients(input: &str) -> u64 {
    let (mut ranges, _) = ranges_and_ingredients(input);

    // Merge ranges and measure fresh IDs space
    ranges.sort_unstable();

    let mut count = 0;
    let (mut current_start, mut current_end) = ranges[0];

    for (start, end) in ranges.into_iter().skip(1) {
        if start <= current_end {
            if current_end < end {
                current_end = end;
            }
        } else {
            count += current_end - current_start + 1;
            (current_start, current_end) = (start, end);
        }
    }

    count + current_end - current_start + 1
}

// ------------------------------------------------------------------------------------------------
// Parsers

fn ranges_and_ingredients(input: &str) -> (Vec<(u64, u64)>, impl Iterator<Item = u64>) {
    let (ranges, ingredients) = input.split_once("\n\n").expect("Expected two sections");

    (
        ranges
            .split(&['-', '\n'])
            .map(parse_number)
            .tuples()
            .collect(),
        parse_numbers_whitespace(ingredients),
    )
}
