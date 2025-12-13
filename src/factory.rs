use itertools::Itertools;
use microlp::{ComparisonOp, OptimizationDirection, Problem};
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::random_utils::parse_numbers;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn lights_fewest_button_presses(input: &str) -> usize {
    lights_buttons_and_joltages(input)
        .map(|(target, buttons, _)| {
            for set in buttons.iter().powerset().skip(1) {
                let mut lights = [false; 10];

                for &button in &set {
                    for &i in button {
                        lights[i] = !lights[i];
                    }
                }

                if lights == target {
                    return set.len();
                }
            }

            unreachable!("Expected powerset member to solve lights");
        })
        .sum()
}

pub fn joltage_fewest_button_presses(input: &str) -> f64 {
    lights_buttons_and_joltages(input)
        .par_bridge()
        .map(|(_, buttons, target)| {
            let mut problem = Problem::new(OptimizationDirection::Minimize);

            let variables = buttons
                .iter()
                .map(|_| problem.add_integer_var(1.0, (0, i32::MAX)))
                .collect_vec();

            for (i, &joltage) in target.iter().enumerate() {
                problem.add_constraint(
                    variables
                        .iter()
                        .enumerate()
                        .filter_map(|(v, &x)| buttons[v].contains(&i).then_some((x, 1.0))),
                    ComparisonOp::Eq,
                    joltage.into(),
                );
            }

            problem
                .solve()
                .expect("Expected ILP problem solution")
                .objective()
        })
        .sum()
}

// ------------------------------------------------------------------------------------------------
// Parsers

fn lights_buttons_and_joltages(
    input: &str,
) -> impl Iterator<Item = ([bool; 10], Vec<Vec<usize>>, [u16; 10])> {
    input.lines().map(|machine| {
        let mut parts = machine.split_ascii_whitespace();
        let (mut indicator_lights_diagram, mut joltage_requirements) = ([false; 10], [0; 10]);

        indicator_lights_diagram
            .iter_mut()
            .zip(parts.next().expect("Expected lights").bytes().skip(1))
            .for_each(|(l, b)| {
                if b == b'#' {
                    *l = true;
                }
            });

        joltage_requirements
            .iter_mut()
            .zip(parse_numbers(parts.next_back().expect("Expected joltage")))
            .for_each(|(j, n)| *j = n);

        (
            indicator_lights_diagram,
            parts
                .map(|button| parse_numbers(button).collect())
                .collect(),
            joltage_requirements,
        )
    })
}
