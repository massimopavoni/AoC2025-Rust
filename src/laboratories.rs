use grid::Grid;
use rustc_hash::FxHashMap;

use crate::random_utils::{
    FxHashWithCapacity, bytes_grid,
    pos::{GridPosGet, Pos},
};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn tachyon_beam_splits(input: &str) -> u16 {
    let grid = bytes_grid(input);

    let mut beams_y = vec![false; grid.cols()];
    beams_y[grid
        .iter_row(0)
        .position(|&b| b == b'S')
        .expect("Expected beam entrance")] = true;
    let mut splits = 0;

    // Simply count splits
    for y in grid
        .indexed_into_iter()
        .filter_map(|((_, y), b)| (b == b'^').then_some(y))
    {
        if beams_y[y] {
            splits += 1;
            beams_y[y] = false;
            beams_y[y - 1] = true;
            beams_y[y + 1] = true;
        }
    }

    splits
}

pub fn tachyon_particle_timelines(input: &str) -> u64 {
    const LEFT: Pos = Pos::new(2, -1);
    const RIGHT: Pos = Pos::new(2, 1);
    const DOWN: Pos = Pos::new(2, 0);

    // Use memoization to count all possible timelines
    fn follow_particle(
        grid: &Grid<u8>,
        seen: &mut FxHashMap<Pos, u64>,
        position: Pos,
        timelines: u64,
    ) -> u64 {
        if let Some(count) = seen.get(&position) {
            return timelines + count;
        }

        if let Some(&byte) = grid.pos_get(position) {
            let count = if byte == b'^' {
                follow_particle(grid, seen, position + LEFT, timelines)
                    + follow_particle(grid, seen, position + RIGHT, timelines)
            } else {
                follow_particle(grid, seen, position + DOWN, timelines)
            };

            seen.insert(position, count);
            count
        } else {
            timelines + 1
        }
    }

    let grid = bytes_grid(input);

    follow_particle(
        &grid,
        &mut FxHashMap::with_capacity(64),
        Pos::from((
            2,
            grid.iter_row(0)
                .position(|&b| b == b'S')
                .expect("Expected beam entrance"),
        )),
        0,
    )
}
