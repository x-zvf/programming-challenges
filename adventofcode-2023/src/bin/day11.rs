use std::fs::read_to_string;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<(isize, isize)> {
    let grid = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>(); 
    grid.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().filter_map(move |(x, &c)| {
            if c == '#' {
                Some((x as isize, y as isize))
            } else {
                None
            }
        })
    }).collect::<Vec<_>>()
}

fn expanded(positions: &Vec<(isize, isize)>, n: isize) -> Vec<(isize, isize)> {
    let n = n - 1;
    let occupied_rows = positions.iter().map(|(_, y)| *y).unique().collect::<Vec<_>>();
    let occupied_cols = positions.iter().map(|(x, _)| *x).unique().collect::<Vec<_>>();
    positions
        .iter()
        .map(|(x, y)| {
            let e_rows = (0..*y).filter(|&r| !occupied_rows.contains(&r)).count() as isize;
            let e_cols = (0..*x).filter(|&c| !occupied_cols.contains(&c)).count() as isize;
            (x + e_cols * n, y + e_rows * n)
        }
    ).collect::<Vec<_>>()
}

fn solve(input: &str, n: isize) -> isize {
    expanded(&parse_input(input), n)
        .iter()
        .combinations(2)
        .map(|v| {
            (v[0].0 - v[1].0).abs() + (v[0].1 - v[1].1).abs()
        }).sum()
}

fn main() {
    let test_input = read_to_string("inputs/day11-test.txt").unwrap();
    let real_input = read_to_string("inputs/day11.txt").unwrap();

    println!("Part 1 test: {}", solve(&test_input, 2));
    println!("Part 1: {}",      solve(&real_input, 2));
    println!("Part 2 test: {}", solve(&test_input, 100));
    println!("Part 2: {}",      solve(&real_input, 1_000_000));
}
