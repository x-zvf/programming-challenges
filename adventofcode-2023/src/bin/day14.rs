use std::fs::read_to_string;
type Grid = Vec<Vec<char>>;
use itertools::Itertools;
use std::collections::HashMap;

fn parse(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Debug, Clone, PartialEq)]
enum Dir {North, East, South, West}

fn tilt_grid(grid: &Grid, dir: Dir) -> Grid {
    let transpose = |grid: &Grid| {
        (0..grid[0].len()).map(|i| {
            grid.iter().map(|row| row[i]).collect::<Vec<_>>()
        }).collect::<Vec<_>>()
    };
    let og = if dir == Dir::North || dir == Dir::South {
        transpose(grid)
    } else {
        grid.clone()
    };

    let mut new_grid = vec![];
    for row in og {
        let groups = row.split(|&c| c == '#')
            .map(|group| {
                let si = group.iter().sorted();
                if dir == Dir::North || dir == Dir::West {
                    si.rev().collect::<Vec<_>>()
                } else {
                    si.collect::<Vec<_>>()
                }
            })
            .intersperse(vec![&'#'])
            .flatten()
            .map(|&c| c)
            .collect::<Vec<_>>();
        new_grid.push(groups);
    }
    if dir == Dir::North || dir == Dir::South {
        transpose(&new_grid)
    } else {
        new_grid
    }
}

fn tilt_cycle(grid: &Grid) -> Grid {
    tilt_grid(&tilt_grid(&tilt_grid(&tilt_grid(grid, Dir::North), Dir::West), Dir::South), Dir::East)
}

fn load(grid: &Grid) -> usize {
    let mut load  = 0;
    for (y, row) in grid.iter().enumerate() {
        for c in row {
            if *c == 'O' {
                load += grid.len() - y;
            }
        }
    }
    load
}

fn part1(input: &str) -> usize {
    load(&tilt_grid(&parse(input), Dir::North))
}

fn part2(input: &str) -> usize {
    let mut g = parse(input);
    let mut cache: Vec<Grid> = vec![];

    while !cache.contains(&g) {
        let new_grid = tilt_cycle(&g);
        cache.push(g);
        g = new_grid;
    }
    let cycle_start = cache.iter().position(|x| x == &g).unwrap();
    let cycle_len = cache.len() - cycle_start;
    let iter = 1_000_000_000;
    let idx = (iter - cycle_start) % cycle_len;

    load(&cache[cycle_start + idx])
}

fn main() {
    let test_input = read_to_string("inputs/day14-test.txt").unwrap();
    let real_input = read_to_string("inputs/day14.txt").unwrap();
    println!("Part 1 test: {}", part1(&test_input));
    println!("Part 1: {}",      part1(&real_input));
    println!("Part 2 test: {}", part2(&test_input));
    println!("Part 2: {}",      part2(&real_input));
}
