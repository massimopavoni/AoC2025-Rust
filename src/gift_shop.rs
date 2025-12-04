use itertools::Itertools;
use rustc_hash::FxHashSet;

use crate::random_utils::{FxHashWithCapacity, parse_number};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn invalid_ids_sum(input: &str) -> u64 {
    // Sum product ids invalid because (.+)\1
    sum_product_ids(input, false)
}

pub fn more_invalid_ids_sum(input: &str) -> u64 {
    // Sum product ids invalid because (.+)\1+
    sum_product_ids(input, true)
}

// ------------------------------------------------------------------------------------------------
// Functions

fn sum_product_ids(input: &str, more_repeats: bool) -> u64 {
    #[inline]
    const fn digits_len(n: u64) -> usize {
        if n < 10 { 1 } else { n.ilog10() as usize + 1 }
    }

    const POWERS_OF_10: [u64; 12] = [
        1,
        10,
        100,
        1_000,
        10_000,
        100_000,
        1_000_000,
        10_000_000,
        100_000_000,
        1_000_000_000,
        10_000_000_000,
        100_000_000_000,
    ];

    let mut seen = FxHashSet::with_capacity(256);

    input
        .trim()
        .split(&['-', ','])
        .tuples()
        .map(|(start, end)| {
            let (start, end) = (parse_number::<u64>(start), parse_number::<u64>(end));
            let mut sum = 0;
            seen.clear();

            for digits_len in digits_len(start)..=digits_len(end) {
                // Entire digits class outside range
                if POWERS_OF_10[digits_len] - 1 < start || POWERS_OF_10[digits_len - 1] > end {
                    continue;
                }

                for sequence_len in 1..digits_len {
                    // Must divide evenly
                    if digits_len % sequence_len != 0 {
                        continue;
                    }

                    let repetitions = digits_len / sequence_len;

                    if !more_repeats && repetitions != 2 || repetitions < 2 {
                        continue;
                    }

                    // Repetition multiplier
                    let base = POWERS_OF_10[sequence_len];
                    let mut repeat_multiplier = 0u64;
                    for _ in 0..repetitions {
                        repeat_multiplier = repeat_multiplier * base + 1;
                    }

                    // Sequence range within [start, end]
                    let sequence_min = start
                        .div_ceil(repeat_multiplier)
                        .max(POWERS_OF_10[sequence_len - 1]);
                    let sequence_max =
                        (end / repeat_multiplier).min(POWERS_OF_10[sequence_len] - 1);

                    if sequence_min > sequence_max {
                        continue;
                    }

                    // Repeated-sequence numbers
                    for sequence in sequence_min..=sequence_max {
                        let mut value = 0u64;
                        for _ in 0..repetitions {
                            value = value * base + sequence;
                        }

                        // Avoid duplicates
                        if seen.insert(value) {
                            sum += value;
                        }
                    }
                }
            }

            sum
        })
        .sum()
}
