use std::fs::read_to_string;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

type Grid = Vec<Vec<u8>>;
type Position = (i32, i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Nil,
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn turns(&self) -> Vec<Direction> {
        match self {
            Direction::Nil => vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right],
            Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
        }
    }
    fn index(&self) -> usize {
        match self {
            Direction::Nil => 0,
            Direction::Up => 1,
            Direction::Down => 2,
            Direction::Left => 3,
            Direction::Right => 4,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Pfdata {
    pos: Position,
    dir: Direction,
    value: i32,
}
impl Ord for Pfdata {
    fn cmp(&self, other: &Pfdata) -> Ordering {
        other.value.cmp(&self.value)
    }
}
impl PartialOrd for Pfdata {
    fn partial_cmp(&self, other: &Pfdata) -> Option<Ordering> {
        other.value.partial_cmp(&self.value)
    }
}

fn step(p: Position, dir: Direction) -> Position {
    match dir {
        Direction::Nil   => p,
        Direction::Up    => (p.0, p.1 - 1),
        Direction::Down  => (p.0, p.1 + 1),
        Direction::Left  => (p.0 - 1, p.1),
        Direction::Right => (p.0 + 1, p.1),
    }
}

fn parse(input: &str) -> Grid {
    input.trim().lines().map(|line| line.chars()
                             .map(|c| c as u8 - '0' as u8)
                             .collect()).collect()
}

fn find_path(g: &Grid, from: Position, to: Position, min_count: u8, max_count: u8) -> i32 {
    let mut heap = BinaryHeap::new();
    heap.push(Pfdata { pos: from, dir: Direction::Nil, value: 0 });

    let mut visited = vec![vec![vec![-1i32; 5]; g[0].len()]; g.len()];

    while let Some(Pfdata { pos, dir, value }) = heap.pop() {
        let v = visited[pos.1 as usize][pos.0 as usize][dir.index()];
        if v != -1 && v <= value {
            continue;
        }
        if pos == to {
            return value;
        }
        visited[pos.1 as usize][pos.0 as usize][dir.index()] = value;
        for &d in dir.turns().iter() {
            let mut new_pos = pos;
            let mut new_value = value;
            for steps in 1..=max_count {
                new_pos = step(new_pos, d);
                if new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 >= g[0].len() as i32 || new_pos.1 >= g.len() as i32 {
                    break;
                }
                new_value += g[new_pos.1 as usize][new_pos.0 as usize] as i32;
                if steps >= min_count {
                    heap.push(Pfdata { pos: new_pos, dir: d, value: new_value });
                }
            }
        }
    }
    panic!("No path found")
}
fn solve(input: &str, min_count: u8, max_count: u8) -> i32 {
    let g = parse(input);
    find_path(&g, (0, 0), (g[0].len() as i32 - 1, g.len() as i32 - 1), min_count, max_count)
}

fn part1(input: &str) -> i32 {
    solve(input, 1, 3)
}

fn part2(input: &str) -> i32 {
    solve(input, 4, 10)
}


fn main() {
    let test_input = read_to_string("inputs/day17-test.txt").unwrap();
    let real_input = read_to_string("inputs/day17.txt").unwrap();
    println!("Part 1 test: {}", part1(&test_input));
    println!("Part 1: {}",      part1(&real_input));
    println!("Part 2 test: {}", part2(&test_input));
    println!("Part 2: {}",      part2(&real_input));
}

