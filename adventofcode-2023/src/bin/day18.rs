use std::fs::read_to_string;

type Instruction = (Direction, isize);
type Position = (isize, isize);
type Grid = Vec<Vec<bool>>;

fn step(pos: &Position, dir: &Direction, steps: &isize) -> Position {
    match dir {
        Direction::Up    => (pos.0, pos.1 - steps),
        Direction::Down  => (pos.0, pos.1 + steps),
        Direction::Left  => (pos.0 - steps, pos.1),
        Direction::Right => (pos.0 + steps, pos.1),
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse(input: &str) -> (Vec<Instruction>, Vec<Instruction>) {
    input.trim().lines()
        .map(|line|{
            let mut parts = line.split_whitespace();
            let dir = match parts.next().unwrap() {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Invalid direction"),
            };
            let steps = parts.next().unwrap().parse().unwrap();

            let color = parts.next().unwrap();
            let steps2 = isize::from_str_radix(&color[2..color.len()-2], 16).unwrap();
            let dir2 = match color.as_bytes()[color.len()-2] {
                b'0' => Direction::Right,
                b'1' => Direction::Down,
                b'2' => Direction::Left,
                b'3' => Direction::Up,
                _ => panic!("Invalid direction"),
            };

            ((dir, steps), (dir2, steps2))
        }).unzip()
}

fn solve(instructions: &Vec<Instruction>) -> isize {
    let mut pos = (0, 0);
    let mut total = 2;
    let mut visited = Vec::new();

    for (dir, steps) in instructions {
        visited.push(pos);
        pos = step(&pos, dir, steps);
        total += steps;
    }
    visited.push(visited[0]);
    (visited.windows(2).map(|w| (w[0], w[1]))
        .map(|(f, t)|
             (f.0*t.1) - (f.1*t.0)).sum::<isize>() + total) / 2
}

fn part1(input: &str) -> isize {
    let (i, _) = parse(input);
    solve(&i)
}

fn part2(input: &str) -> isize {
    let (_, i) = parse(input);
    solve(&i)
}

fn main() {
    let test_input = read_to_string("inputs/day18-test.txt").unwrap();
    let real_input = read_to_string("inputs/day18.txt").unwrap();
    println!("Part 1 test: {}", part1(&test_input));
    println!("Part 1: {}",      part1(&real_input));
    println!("Part 2 test: {}", part2(&test_input));
    println!("Part 2: {}",      part2(&real_input));
}
