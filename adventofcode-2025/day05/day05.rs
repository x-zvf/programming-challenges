use std::env::args;
use std::fs;
use std::ops::RangeInclusive as RA;

fn merge_ranges(mut rs: Vec<RA<usize>>) -> Vec<RA<usize>> {
    rs.sort_by_key(|x| x.start().clone());

    let mut i = 0;
    loop {
        if i >= rs.len() - 1 {
            break;
        }
        let c = &rs[i];
        let next = &rs[i + 1];
        if c.end() < next.start() {
            i += 1;
            continue;
        }
        if c.end() >= next.start() {
            let ne = *std::cmp::max(next.end(), c.end());
            rs[i] = *c.start()..=ne;
            rs.remove(i + 1);
        }
    }

    rs
}

fn load(fname: &str) -> (Vec<RA<usize>>, Vec<usize>) {
    let input = fs::read_to_string(fname).unwrap();
    let (fresh_s, ing_s) = input.split_once("\n\n").unwrap();

    (
        fresh_s
            .split("\n")
            .map(|line| {
                let (l, r) = line.trim().split_once("-").unwrap();
                l.parse().unwrap()..=r.parse().unwrap()
            })
            .collect(),
        ing_s
            .split("\n")
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.parse().unwrap())
            .collect(),
    )
}

fn main() {
    let (fresh, ings) = load(&args().last().unwrap());

    let nfresh_ings = ings
        .iter()
        .filter(|ing| fresh.iter().filter(|r| r.contains(ing)).next().is_some())
        .count();

    println!("part1: {nfresh_ings}");

    let total: usize = merge_ranges(fresh)
        .iter()
        .map(|r| r.end() - r.start() + 1)
        .sum();

    println!("part2: {total}");
}
