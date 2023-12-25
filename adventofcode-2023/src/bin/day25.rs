use std::fs::read_to_string;
use std::collections::HashMap;
use rustworkx_core::petgraph::graph::UnGraph;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::Result;

fn parse(input: &str) -> UnGraph<u32, ()> {
    let mut str_to_idx: HashMap<String,u32> = HashMap::new();
    let mut max_idx = 0;
    let mut to_idx = |s: &str| -> u32 {
        if let Some(&idx) = str_to_idx.get(s) {
            idx
        } else {
            let idx = max_idx;
            max_idx += 1;
            str_to_idx.insert(s.to_string(), idx);
            idx
        }
    };
    let edges: Vec<(u32,u32)> = input.lines().flat_map(|line| {
        let mut parts = line.split(": ");
        let from = to_idx(parts.next().unwrap());
        parts.next().unwrap().split(" ")
            .map(|s| (from, to_idx(s)))
            .collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    UnGraph::from_edges(edges)
}


fn part1(input: &str) -> usize {
    let graph = parse(input);
    let mcres: Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&graph, |_| Ok(1));
    let (_, partition) = mcres.unwrap().unwrap();
    let p1 = partition.len();
    let p2 = graph.node_count() - p1;
    p1 * p2
}

fn main() {
    let test_input = read_to_string("inputs/day25-test.txt").unwrap();
    let real_input = read_to_string("inputs/day25.txt").unwrap();
    println!("Part 1 test: {}", part1(&test_input));
    println!("Part 1: {}",      part1(&real_input));
    //println!("Part 2 test: {}", part2(&test_input));
    //println!("Part 2: {}",      part2(&real_input));
}
