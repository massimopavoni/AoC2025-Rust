use crate::random_utils::parse_number_bytes;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn total_joltage_2_batteries(input: &str) -> u64 {
    total_joltage::<2>(input)
}

pub fn total_joltage_12_batteries(input: &str) -> u64 {
    total_joltage::<12>(input)
}

// ------------------------------------------------------------------------------------------------
// Functions

fn total_joltage<const BATTERIES_COUNT: usize>(input: &str) -> u64 {
    let mut total = 0;
    let mut stack = [0; BATTERIES_COUNT];
    let mut size;

    // Find biggest joltage sum by using digits stack
    for bank in input.lines() {
        size = 0;

        for (i, &digit) in bank.as_bytes().iter().enumerate() {
            while size > 0 && bank.len() - i + size > BATTERIES_COUNT && stack[size - 1] < digit {
                size -= 1;
            }

            if size < BATTERIES_COUNT {
                stack[size] = digit;
                size += 1;
            }
        }

        total += parse_number_bytes::<u64>(&stack);
    }

    total
}
