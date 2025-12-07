use crate::random_utils::bytes_grid;

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
    let grid = bytes_grid(input);
    let mut timelines = vec![1; grid.cols()];

    // Sum all possible timelines in reverse
    for row in grid.iter_rows().skip(1).rev().skip(1).step_by(2) {
        for (y, &byte) in row.enumerate() {
            if byte == b'^' {
                timelines[y] = timelines[y - 1] + timelines[y + 1];
            }
        }
    }

    timelines[grid
        .iter_row(0)
        .position(|&b| b == b'S')
        .expect("Expected beam entrance")]
}
