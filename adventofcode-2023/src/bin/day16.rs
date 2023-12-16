use std::fs::read_to_string;

type Grid = Vec<Vec<char>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse(input: &str) -> Grid {
    input.trim().lines().map(|line| line.chars().collect()).collect()
}

fn energize(grid: &Grid, initial: (isize, isize, Direction)) -> usize {
    let mut beams_queue: Vec<(isize, isize, Direction)> = Vec::new();
    beams_queue.push(initial);

    let mut visited: Vec<Vec<Vec<Direction>>> = vec![vec![vec![]; grid[0].len()]; grid.len()];

    while !beams_queue.is_empty() {
        let (x, y, dir) = beams_queue.pop().unwrap();
        //println!("{} {} {:?} \t\t {:?}", x, y, dir, beams_queue);
        if x < 0 || y < 0 || y >= grid.len() as isize || x >= grid[y as usize].len() as isize {
            continue;
        }
        let c = grid[y as usize][x as usize];
        if visited[y as usize][x as usize].contains(&dir) {
            continue;
        }
        visited[y as usize][x as usize].push(dir);

        match c {
            '.' => beams_queue.push(match dir {
                Direction::Up    => (x, y - 1, Direction::Up),
                Direction::Down  => (x, y + 1, Direction::Down),
                Direction::Left  => (x - 1, y, Direction::Left),
                Direction::Right => (x + 1, y, Direction::Right),
            }),
            '\\' => beams_queue.push(match dir {
                Direction::Up    => (x - 1, y, Direction::Left),
                Direction::Down  => (x + 1, y, Direction::Right),
                Direction::Left  => (x, y - 1, Direction::Up),
                Direction::Right => (x, y + 1, Direction::Down),
            }),
            '/' => beams_queue.push(match dir {
                Direction::Up    => (x + 1, y, Direction::Right),
                Direction::Down  => (x - 1, y, Direction::Left),
                Direction::Left  => (x, y + 1, Direction::Down),
                Direction::Right => (x, y - 1, Direction::Up),
            }),
            '-' => {
                match dir {
                    Direction::Left => beams_queue.push((x - 1, y, Direction::Left)),
                    Direction::Right => beams_queue.push((x + 1, y, Direction::Right)),
                    Direction::Up | Direction::Down => {
                        beams_queue.push((x - 1, y, Direction::Left));
                        beams_queue.push((x + 1, y, Direction::Right));
                    },
                }
            },
            '|' => {
                match dir {
                    Direction::Up => beams_queue.push((x, y - 1, Direction::Up)),
                    Direction::Down => beams_queue.push((x, y + 1, Direction::Down)),
                    Direction::Left | Direction::Right => {
                        beams_queue.push((x, y - 1, Direction::Up));
                        beams_queue.push((x, y + 1, Direction::Down));
                    },
                }
            },
            _ => panic!("Unknown character: {}", c),
        };
    }
    visited.iter().map(|row| row.iter().filter(|v| !v.is_empty()).count()).sum()
}

fn part1(input: &str) -> usize {
    energize(&parse(input), (0, 0, Direction::Right))
}

fn part2(input: &str) -> usize {
    let g = parse(input);
    let mx = g[0].len() as isize - 1;
    let my = g.len() as isize - 1;
    let mut edge_tiles = vec![
        (0, 0, Direction::Right),
        (0, 0, Direction::Down),
        (mx, 0, Direction::Left),
        (mx, 0, Direction::Down),
        (0, my, Direction::Right),
        (0, my, Direction::Up),
        (mx, my, Direction::Left),
        (mx, my, Direction::Up),
    ];
    for i in 1..mx-1 {
        edge_tiles.push((i, 0, Direction::Down));
        edge_tiles.push((i, my, Direction::Up));
    }
    for i in 1..my-1 {
        edge_tiles.push((0, i, Direction::Right));
        edge_tiles.push((mx, i, Direction::Left));
    }

    edge_tiles.iter().map(|&initial| energize(&g, initial)).max().unwrap()
}

fn main() {
    let test_input = read_to_string("inputs/day16-test.txt").unwrap();
    let real_input = read_to_string("inputs/day16.txt").unwrap();
    println!("Part 1 test: {}", part1(&test_input));
    println!("Part 1: {}",      part1(&real_input));
    println!("Part 2 test: {}", part2(&test_input));
    println!("Part 2: {}",      part2(&real_input));
}
