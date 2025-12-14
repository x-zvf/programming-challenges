use petgraph::algo::all_simple_paths;
use petgraph::graph::DiGraph;
use petgraph::visit::EdgeRef;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::fs;

fn parse(fname: &str) -> (Vec<String>, DiGraph<u32, ()>) {
    let mut nodes: Vec<String> = vec![];
    let mut find_or_insert = |name: &str| -> u32 {
        if let Some(p) = nodes.iter().position(|x| x == name) {
            return p as u32;
        }
        nodes.push(name.into());
        (nodes.len() - 1) as u32
    };
    let mut edges: Vec<(u32, u32)> = vec![];
    fs::read_to_string(fname)
        .unwrap()
        .split("\n")
        .filter_map(|l| {
            let x = l.trim();
            if x.is_empty() { None } else { Some(x) }
        })
        .for_each(|l| {
            let (src, tss) = l.split_once(":").unwrap();
            let srci = find_or_insert(src);
            tss.trim().split(" ").for_each(|t| {
                let ti = find_or_insert(t);
                edges.push((srci, ti));
            });
        });
    let g = DiGraph::from_edges(&edges);
    (nodes, g)
}
fn part1(nodes: &Vec<String>, graph: &DiGraph<u32, ()>) -> usize {
    let start = nodes.iter().position(|x| x == "you").unwrap() as u32;
    let end = nodes.iter().position(|x| x == "out").unwrap() as u32;
    all_simple_paths::<Vec<_>, _, RandomState>(&graph, start.into(), end.into(), 0, None).count()
}

fn part2(nodes: &Vec<String>, graph: &DiGraph<u32, ()>) -> usize {
    let to_ni = |n: &str| -> u32 { nodes.iter().position(|x| x == n).unwrap() as u32 };
    let start = to_ni("svr");
    let end = to_ni("out");
    let fft = to_ni("fft");
    let dac = to_ni("dac");

    fn npaths(memo: &mut Vec<usize>, graph: &DiGraph<u32, ()>, f: u32, t: u32) -> usize {
        if f == t {
            return 1;
        }
        if memo[f as usize] != usize::MAX {
            return memo[f as usize];
        }
        let res = graph
            .edges(f.into())
            .map(|e| npaths(memo, graph, e.target().index() as u32, t))
            .sum();
        memo[f as usize] = res;
        res
    }
    let np = |f: u32, t: u32| -> usize {
        let mut memo = vec![usize::MAX; nodes.len()];
        npaths(&mut memo, graph, f, t)
    };
    let s2fft = np(start, fft);
    let s2dac = np(start, dac);
    let fft2dac = np(fft, dac);
    let dac2fft = np(dac, fft);
    let dac2end = np(dac, end);
    let fft2end = np(fft, end);

    (s2fft * fft2dac * dac2end) + (s2dac * dac2fft * fft2end)
}
fn main() {
    let (s_n, s_g) = parse("./sample.txt");
    let (s2_n, s2_g) = parse("./sample2.txt");
    let (i_n, i_g) = parse("./input.txt");
    println!("part1 sample: {}", part1(&s_n, &s_g));
    println!("part1 input: {}", part1(&i_n, &i_g));

    println!("part2 sample: {}", part2(&s2_n, &s2_g));
    println!("part2 input: {}", part2(&i_n, &i_g));
}
