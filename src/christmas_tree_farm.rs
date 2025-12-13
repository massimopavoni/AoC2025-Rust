use itertools::Itertools;

use crate::random_utils::{parse_number, parse_numbers};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn tree_regions_that_fit_all_presents(input: &str) -> usize {
    let (presents, tree_regions) = input.rsplit_once("\n\n").expect("Expected two sections");
    let presents = presents
        .split("\n\n")
        .map(|present| {
            let (p, shape) = present.split_once(":\n").expect("Expected shape info");

            (
                parse_number::<usize>(p),
                shape.bytes().filter(|&b| b == b'#').count(),
            )
        })
        .collect_vec();

    let mut definitely_solvable = 0;

    // Check how many definitely solvable packing problems there are
    // (turns out they are all trivially solvable)
    for mut region_info in tree_regions.lines().map(parse_numbers::<usize>) {
        let (width, length) = (
            region_info.next().expect("Expected region width"),
            region_info.next().expect("Expected region length"),
        );

        let (total_presents, total_space) = region_info
            .zip(presents.iter())
            .fold((0, 0), |(tp, ts), (count, &(_, units))| {
                (tp + count, ts + count * units)
            });

        if width / 3 * (length / 3) >= total_presents {
            // Enough 3x3 boxes
            definitely_solvable += 1;
        } else if width * length >= total_space {
            // Problem needs proper packing solution
            unreachable!("Eric did us dirty!");
        }
        // Otherwise, completely unsolvable
    }

    definitely_solvable
}
