use std::iter::once;

use itertools::Itertools;

use crate::random_utils::{bytes_grid, parse_number_bytes, parse_numbers};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn problem_answers_sum(input: &str) -> u64 {
    let mut number_lines = Vec::with_capacity(4);
    let mut sum = 0;

    // Simply sum lines (iterator shenanigans)
    for line in input.lines().map(str::trim) {
        if line.starts_with(|c: char| c.is_ascii_digit()) {
            number_lines.push(parse_numbers::<u64>(line));
        } else {
            for operator in line.split_ascii_whitespace() {
                let numbers = number_lines
                    .iter_mut()
                    .filter_map(std::iter::Iterator::next);

                sum += match operator {
                    "+" => numbers.sum::<u64>(),
                    "*" => numbers.product(),
                    _ => unreachable!("Unknown arithmetic operator"),
                };
            }
        }
    }

    sum
}

pub fn transposed_problem_answers_sum(input: &str) -> u64 {
    // Use bytes grid to transpose input
    let mut grid = bytes_grid(input);
    grid.transpose();

    let (mut operands, mut operator) = (Vec::with_capacity(4), b'0');
    let mut sum = 0;

    // More iterator shenanigans on columns
    for line in grid
        .iter_rows()
        .map(|line| {
            line.filter(|&&b| !b.is_ascii_whitespace())
                .copied()
                .collect_vec()
        })
        .chain(once(vec![]))
    {
        if line.is_empty() {
            sum += match operator {
                b'+' => operands.iter().sum::<u64>(),
                b'*' => operands.iter().product(),
                _ => unreachable!("Unknown arithmetic operator"),
            };

            operands.clear();
        } else if operands.is_empty() {
            operands.push(parse_number_bytes(&line[..line.len() - 1]));
            operator = line[line.len() - 1];
        } else {
            operands.push(parse_number_bytes(&line));
        }
    }

    sum
}
