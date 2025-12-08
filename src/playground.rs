use std::cmp::Reverse;

use itertools::Itertools;
use rayon::slice::ParallelSliceMut;

use crate::random_utils::{graph::Dsu, parse_numbers};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn largest_3_circuits(input: &str) -> usize {
    let (mut edges, nodes) = edges_and_nodes(input);

    let mut dsu = Dsu::new(nodes.len());

    // Connect first 1000 junction box pairs
    for (n1, n2, _) in edges
        .select_nth_unstable_by_key(1000, |&(_, _, distance)| distance)
        .0
    {
        dsu.union(*n1, *n2);
    }

    // Multiply 3 largest circuits sizes
    dsu.size
        .select_nth_unstable_by_key(3, |&size| Reverse(size))
        .0
        .iter()
        .product()
}

pub fn minimum_spanning_circuit_last_cable(input: &str) -> u64 {
    let (mut edges, nodes) = edges_and_nodes(input);

    // Sort all edges
    edges.par_sort_unstable_by_key(|&(_, _, distance)| distance);

    let mut dsu = Dsu::new(nodes.len());

    // Find edge connecting last 2 components
    for &(n1, n2, _) in &edges {
        if dsu.union(n1, n2) && dsu.components == 1 {
            return nodes[n1].0 * nodes[n2].0;
        }
    }

    unreachable!("Invalid graph without MST");
}

// ------------------------------------------------------------------------------------------------
// Parsers

type Edge = (usize, usize, u64);
type Node = (u64, u64, u64);

fn edges_and_nodes(input: &str) -> (Vec<Edge>, Vec<Node>) {
    #[inline]
    const fn euclidean_distance(
        (x1, y1, z1): (u64, u64, u64),
        (x2, y2, z2): (u64, u64, u64),
    ) -> u64 {
        (x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2)
    }

    let nodes = parse_numbers(input).tuples().collect_vec();

    (
        (0..nodes.len())
            .tuple_combinations()
            .map(|(n1, n2)| (n1, n2, euclidean_distance(nodes[n1], nodes[n2])))
            .collect(),
        nodes,
    )
}
