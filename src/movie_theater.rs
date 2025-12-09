use std::iter::once;

use itertools::Itertools;

use crate::random_utils::parse_numbers;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn largest_rectangle_area(input: &str) -> u64 {
    // Simply find largest rectangle
    red_tiles_coordinates(input)
        .iter()
        .tuple_combinations()
        .map(|(&(x1, y1), &(x2, y2))| (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1))
        .max()
        .expect("Expected largest rectangle area")
}

pub fn largest_red_green_rectangle_area(input: &str) -> u64 {
    type Rect = ((u64, u64), (u64, u64));

    #[inline]
    fn coordinates_ordering(iter: impl Iterator<Item = Rect>) -> impl Iterator<Item = Rect> {
        iter.map(|((x1, y1), (x2, y2))| ((x1.min(x2), y1.min(y2)), (x1.max(x2), y1.max(y2))))
    }

    let red_tiles = red_tiles_coordinates(input);
    // Precompute green segments between red tiles
    let green_segments = coordinates_ordering(
        red_tiles
            .iter()
            .chain(once(&red_tiles[0]))
            .copied()
            .tuple_windows::<Rect>(),
    )
    .collect_vec();

    // Find largest rectangle without green segments intersections
    coordinates_ordering(red_tiles.into_iter().tuple_combinations()).fold(
        0,
        |max_area, ((x1, y1), (x2, y2))| {
            let area = (x2 - x1 + 1) * (y2 - y1 + 1);

            if area > max_area
                && !green_segments
                    .iter()
                    .any(|&((sx1, sy1), (sx2, sy2))| sx1 < x2 && sy1 < y2 && sx2 > x1 && sy2 > y1)
            {
                area
            } else {
                max_area
            }
        },
    )
}

// ------------------------------------------------------------------------------------------------
// Parsers

#[inline]
fn red_tiles_coordinates(input: &str) -> Vec<(u64, u64)> {
    parse_numbers(input).tuples().collect()
}
