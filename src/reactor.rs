use std::array::from_fn;

use pathfinding::directed::count_paths::count_paths;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn you_to_out_paths(input: &str) -> usize {
    // Count paths between you and out
    graph_count_paths(
        &devices_graph(input),
        node_to_index(b"you"),
        node_to_index(b"out"),
    )
}

pub fn svr_to_out_through_dac_fft_paths(input: &str) -> usize {
    let graph = devices_graph(input);

    let (svr, dac, fft, out) = (
        node_to_index(b"svr"),
        node_to_index(b"dac"),
        node_to_index(b"fft"),
        node_to_index(b"out"),
    );

    // Count paths betwen svr and out that pass through both dac and fft
    let dac_to_fft = graph_count_paths(&graph, dac, fft);

    if dac_to_fft == 0 {
        graph_count_paths(&graph, svr, fft)
            * graph_count_paths(&graph, fft, dac)
            * graph_count_paths(&graph, dac, out)
    } else {
        graph_count_paths(&graph, svr, dac) * dac_to_fft * graph_count_paths(&graph, fft, out)
    }
}

// ------------------------------------------------------------------------------------------------
// Functions

#[inline]
fn graph_count_paths(graph: &[Vec<usize>; 17576], start: usize, end: usize) -> usize {
    count_paths(
        start,
        |&node| graph[node].iter().copied(),
        |&node| node == end,
    )
}

// ------------------------------------------------------------------------------------------------
// Parsers

#[inline]
const fn node_to_index(node: &[u8]) -> usize {
    676 * (node[0] - b'a') as usize + 26 * (node[1] - b'a') as usize + (node[2] - b'a') as usize
}

#[allow(clippy::large_stack_frames)]
fn devices_graph(input: &str) -> [Vec<usize>; 17576] {
    let mut graph = from_fn(|_| Vec::with_capacity(20));

    for line in input.lines() {
        let mut nodes = line.split_ascii_whitespace();
        let start = node_to_index(nodes.next().expect("Expected graph node").as_bytes());

        for node in nodes {
            graph[start].push(node_to_index(node.as_bytes()));
        }
    }

    graph
}
