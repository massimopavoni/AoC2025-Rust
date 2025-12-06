use grid::Grid;
use itertools::Itertools;
use rustc_hash::FxHashSet;

use crate::random_utils::{
    FxHashWithCapacity, bytes_grid,
    pos::{GridPosGet, Pos},
};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn accessible_paper_rolls(input: &str) -> u16 {
    let mut grid = bytes_grid(input);

    // Count easily accessible paper rolls only
    easily_accessible_paper_rolls(0, &mut grid, |accessible, _, _| *accessible += 1)
}

pub fn all_accessible_paper_rolls(input: &str) -> u16 {
    let mut grid = bytes_grid(input);

    // Start with easily accessible paper rolls
    let mut accessible_rolls = easily_accessible_paper_rolls(
        FxHashSet::with_capacity(grid.rows() * grid.cols() / 2),
        &mut grid,
        |set, grid, position| {
            set.insert(position);
            *grid.pos_index_mut(position) = b'.';
        },
    );

    let mut accessible = 0;

    // Follow neighbor layers until no more rolls can be removed
    #[allow(clippy::cast_possible_truncation)]
    while !accessible_rolls.is_empty() {
        accessible += accessible_rolls.len() as u16;
        for neighbor in accessible_rolls
            .drain()
            .flat_map(|position| neighboring_rolls(position, &grid))
            .collect_vec()
        {
            if neighboring_rolls(neighbor, &grid).count() < 4 {
                accessible_rolls.insert(neighbor);
                *grid.pos_index_mut(neighbor) = b'.';
            }
        }
    }

    accessible
}

// ------------------------------------------------------------------------------------------------
// Functions

fn easily_accessible_paper_rolls<Init, Update>(
    mut init: Init,
    grid: &mut Grid<u8>,
    update: Update,
) -> Init
where
    Update: Fn(&mut Init, &mut Grid<u8>, Pos),
{
    for x in 0..grid.rows() {
        for y in 0..grid.cols() {
            let position = Pos::from((x, y));
            if grid[(x, y)] == b'@' && neighboring_rolls(position, grid).count() < 4 {
                update(&mut init, grid, position);
            }
        }
    }

    init
}

#[inline]
fn neighboring_rolls(position: Pos, grid: &Grid<u8>) -> impl Iterator<Item = Pos> {
    position
        .neighbors()
        .filter(|&pos| grid.pos_get(pos).is_some_and(|&b| b == b'@'))
}
