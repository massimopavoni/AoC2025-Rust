use std::{array::from_fn, env::args, fmt::Display, sync::LazyLock, time::Instant};

use include_dir::{Dir, include_dir};
use itertools::Itertools;
use rustc_hash::FxHashMap;

mod random_utils;

mod cafeteria;
mod gift_shop;
mod laboratories;
mod lobby;
mod movie_theater;
mod playground;
mod printing_department;
mod secret_entrance;
mod trash_compactor;

use cafeteria::{all_possible_fresh_ingredients, available_fresh_ingredients};
use gift_shop::{invalid_ids_sum, more_invalid_ids_sum};
use laboratories::{tachyon_beam_splits, tachyon_particle_timelines};
use lobby::{total_joltage_2_batteries, total_joltage_12_batteries};
use movie_theater::{largest_rectangle_area, largest_red_green_rectangle_area};
use playground::{largest_3_circuits, minimum_spanning_circuit_last_cable};
use printing_department::{accessible_paper_rolls, all_accessible_paper_rolls};
use secret_entrance::{door_password, door_password_click_method};
use trash_compactor::{problem_answers_sum, transposed_problem_answers_sum};

// ------------------------------------------------------------------------------------------------
// Globals

static SELECTED_PUZZLES: LazyLock<[bool; 25]> = LazyLock::new(|| {
    let args = args().collect_vec();

    if args.len() == 1 {
        [true; 25]
    } else {
        from_fn(|day| args.contains(&(day + 1).to_string()))
    }
});

// ------------------------------------------------------------------------------------------------
// Resources

static RESOURCES_DIR: Dir = include_dir!("src/resources");

macro_rules! get_resource {
    ($file:expr) => {
        RESOURCES_DIR
            .get_file($file)
            .expect("Resource not found")
            .contents_utf8()
            .expect("Resource is not UTF-8")
    };
}

static PUZZLE_ANSWERS: LazyLock<FxHashMap<&str, [&str; 2]>> = LazyLock::new(|| {
    get_resource!("PuzzleAnswers.out")
        .lines()
        .map(|line| {
            let parts = line.split_ascii_whitespace().collect_vec();

            (parts[0], [parts[1], parts[2]])
        })
        .collect()
});

// ------------------------------------------------------------------------------------------------
// Functions

#[inline]
fn pretty_solution<R>(puzzle: &str, part: usize, solution: fn(&str) -> R, input: &str, answer: &str)
where
    R: Display + PartialEq,
{
    let now = Instant::now();
    let solution = solution(input);
    let microseconds = now.elapsed().as_micros();

    assert!(
        solution.to_string() == answer,
        "Wrong solution for {puzzle} part {part}: expected {answer}, but got {solution}"
    );

    println!("{part} -> {answer} ({microseconds}μs)");
}

macro_rules! pretty_solution_2 {
    ($day:literal, $puzzle: literal, $solution1:ident $(,$solution2:ident)?) => {
        if SELECTED_PUZZLES[$day - 1] {
            println!("Day {}: {}", $day, $puzzle);

            let input = get_resource!($puzzle.to_string() + ".in");
            let answers = PUZZLE_ANSWERS.get($puzzle).expect("Puzzle answer not found");

            pretty_solution($puzzle, 1, $solution1, input, answers[0]);

            $(pretty_solution($puzzle, 2, $solution2, input, answers[1]);)?

            println!();
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Exports

#[allow(clippy::too_many_lines)]
pub fn main() {
    println!("AoC 2025 - Rust\n");

    pretty_solution_2!(
        1,
        "SecretEntrance",
        door_password,
        door_password_click_method
    );

    pretty_solution_2!(2, "GiftShop", invalid_ids_sum, more_invalid_ids_sum);

    pretty_solution_2!(
        3,
        "Lobby",
        total_joltage_2_batteries,
        total_joltage_12_batteries
    );

    pretty_solution_2!(
        4,
        "PrintingDepartment",
        accessible_paper_rolls,
        all_accessible_paper_rolls
    );

    pretty_solution_2!(
        5,
        "Cafeteria",
        available_fresh_ingredients,
        all_possible_fresh_ingredients
    );

    pretty_solution_2!(
        6,
        "TrashCompactor",
        problem_answers_sum,
        transposed_problem_answers_sum
    );

    pretty_solution_2!(
        7,
        "Laboratories",
        tachyon_beam_splits,
        tachyon_particle_timelines
    );

    pretty_solution_2!(
        8,
        "Playground",
        largest_3_circuits,
        minimum_spanning_circuit_last_cable
    );

    pretty_solution_2!(
        9,
        "MovieTheater",
        largest_rectangle_area,
        largest_red_green_rectangle_area
    );
}
