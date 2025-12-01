use crate::random_utils::parse_number;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn door_password(input: &str) -> i16 {
    // Count 0s
    fold_dial_rotations(input, |dial| i16::from(dial % 100 == 0))
}

pub fn door_password_click_method(input: &str) -> i16 {
    // Count 0-clicks
    fold_dial_rotations(input, |dial| dial / 100)
}

// ------------------------------------------------------------------------------------------------
// Functions

fn fold_dial_rotations<Trans>(input: &str, password_transform: Trans) -> i16
where
    Trans: Fn(i16) -> i16,
{
    input
        .lines()
        .fold(
            (50, 0, b'R'),
            |(mut dial, mut password, mut right_or_left), rotation| {
                if rotation.as_bytes()[0] != right_or_left {
                    dial = (100 - dial) % 100;
                    right_or_left = rotation.as_bytes()[0];
                }

                dial += parse_number::<i16>(&rotation[1..]);
                password += password_transform(dial);
                dial %= 100;

                (dial, password, right_or_left)
            },
        )
        .1
}
