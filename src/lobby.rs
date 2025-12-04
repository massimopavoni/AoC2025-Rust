use lexical_core::FromLexical;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn total_joltage_2_batteries(input: &str) -> u16 {
    // Find biggest joltage sum with 2 batteries per bank
    total_joltage::<2, u16>(input)
}

pub fn total_joltage_12_batteries(input: &str) -> u64 {
    // Find biggest joltage sum with 12 batteries per bank
    total_joltage::<12, u64>(input)
}

// ------------------------------------------------------------------------------------------------
// Functions

fn total_joltage<const BATTERIES_COUNT: usize, N>(input: &str) -> N
where
    N: FromLexical,
{
    let mut total = N::default();
    let mut stack = [0; BATTERIES_COUNT];
    let mut size;

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

        total += lexical_core::parse(&stack).expect("Expected valid digits");
    }

    total
}
