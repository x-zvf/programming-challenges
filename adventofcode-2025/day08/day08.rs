#![feature(exact_length_collection)]
use petgraph::algo;
use petgraph::data::FromElements;
use petgraph::dot::Config;
use petgraph::graph::UnGraph;
use std::collections::BTreeMap;
use std::env;
use std::fs;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn dist2(&self, other: &Self) -> isize {
        (self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z)
    }
    fn from_str(line: &str) -> Self {
        if let Some(arr) = line.trim().split(",").collect_array::<3>() {
            return Self {
                x: arr[0].parse().unwrap(),
                y: arr[1].parse().unwrap(),
                z: arr[2].parse().unwrap(),
            };
        }
        panic!("failed to parse line {line}")
    }
}

fn parse(file: &str) -> (Vec<Point>, BTreeMap<isize, (u32, u32)>) {
    let f = fs::read_to_string(file).unwrap();

    let points: Vec<Point> = f
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| Point::from_str(l))
        .collect();

    let mut edges = BTreeMap::<isize, (u32, u32)>::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let a = &points[i];
            let b = &points[j];
            let d = a.dist2(b);
            edges.insert(d, (i as u32, j as u32));
        }
    }
    (points, edges)
}

fn part1(edges: BTreeMap<isize, (u32, u32)>, take_edges: usize, take_sccs: usize) -> usize {
    let graph = UnGraph::<u32, ()>::from_edges(edges.values().take(take_edges));
    let mut sccs: Vec<usize> = petgraph::algo::kosaraju_scc(&graph)
        .iter()
        .map(|x| x.len())
        .collect();
    sccs.sort_unstable();
    sccs.iter().rev().take(take_sccs).product()
}
fn part2(nodes: Vec<Point>, mut edges: BTreeMap<isize, (u32, u32)>) -> usize {
    let mut graph = UnGraph::<u32, ()>::new_undirected();
    for i in 0..nodes.len() {
        graph.add_node(i as u32);
    }

    loop {
        let e = edges.pop_first().unwrap().1;
        graph.add_edge(e.0.into(), e.1.into(), ());
        if algo::connected_components(&graph) == 1 {
            let a = nodes[e.0 as usize].x;
            let b = nodes[e.1 as usize].x;
            return (b * a) as usize;
        }
    }
}

fn main() {
    let (s_n, s_e) = parse("./sample.txt");
    let (i_n, i_e) = parse("./input.txt");
    let sample_p1 = part1(s_e.clone(), 10, 3);
    let input_p1 = part1(i_e.clone(), 1000, 3);
    println!("part1: {sample_p1} {input_p1}");

    let sample_p2 = part2(s_n, s_e);
    let input_p2 = part2(i_n, i_e);
    println!("part1: {sample_p2} {input_p2}");
}
