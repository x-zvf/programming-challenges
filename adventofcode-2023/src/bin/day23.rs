use std::fs::read_to_string;
use std::collections::{HashMap, HashSet};

type Position = (usize, usize);
type Grid = Vec<Vec<char>>;
type Graph = HashMap<Position, Vec<(Position, usize)>>;

fn parse(input: &str) -> Grid {
    input.trim().lines().map(|line| line.chars().collect()).collect()
}

fn neighbours(grid: &Grid, pos: &Position) -> Vec<Position> {
    let mut result = vec![];
    let (x, y) = *pos;
    match grid[pos.1][pos.0] {
        '#' => panic!("No neighbours for forrest"),
        '.' => {
            if x > 0 && grid[y][x-1] != '#' {
                result.push((x-1, y));
            }
            if y > 0 && grid[y-1][x] != '#' {
                result.push((x, y-1));
            }
            if x < grid[0].len()-1 && grid[y][x+1] != '#' {
                result.push((x+1, y));
            }
            if y < grid.len()-1 && grid[y+1][x] != '#' {
                result.push((x, y+1));
            }
        },
        '^' => {
            if y > 0 && grid[y-1][x] != '#' {
                result.push((x, y-1));
            }
        },
        '>' => {
            if x < grid[0].len()-1 && grid[y][x+1] != '#' {
                result.push((x+1, y));
            }
        },
        'v' => {
            if y < grid.len()-1 && grid[y+1][x] != '#' {
                result.push((x, y+1));
            }
        },
        '<' => {
            if x > 0 && grid[y][x-1] != '#' {
                result.push((x-1, y));
            }
        },
        _ => panic!("Unknown character"),
    }
    result
}

fn find_longest_path(graph: &Graph, from: &Position, to: &Position) -> usize {
    let mut stack = vec![];
    stack.push((*from, vec![*from], 0));
    let mut longest_path = (vec![], 0);
    while let Some((pos, path, distance)) = stack.pop() {
        for (neighbour, dst) in &graph[&pos] {
            if path.contains(neighbour) {
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(*neighbour);

            let new_distance = distance + dst;

            if neighbour == to {
                if new_distance > longest_path.1 {
                    longest_path = (new_path, new_distance);
                }
            } else {
                stack.push((*neighbour, new_path, new_distance));
            }
        }
    }
    longest_path.1
}

fn build_graph(g: &Grid, from: &Position) -> Graph {
    let mut adj: Graph = HashMap::new();

    let mut visited = HashSet::new();
    let mut stack = vec![];
    stack.push(*from);

    while let Some(pos) = stack.pop() {
        if visited.contains(&pos) {
            continue;
        }
        for n in neighbours(g, &pos) {
            let mut len = 1;
            let mut prev = pos;
            let mut cpos = n;
            loop {
                let nn = neighbours(g, &cpos);
                if nn.len() == 1 && nn[0] == prev {
                    break;
                }
                if nn.len() != 2 {
                    break;
                }
                let next = if nn[0] == prev { nn[1] } else { nn[0] };
                len += 1;
                prev = cpos;
                cpos = next;
            }
            adj.entry(pos).or_insert(vec![]).push((cpos, len));
            stack.push(cpos);
        }
        visited.insert(pos);
    }
    adj
}

fn solve(grid: &Grid) -> usize {
    let find_free = |row: &Vec<char>| row.iter().position(|&c| c == '.').unwrap();

    let start = (find_free(&grid[0]), 0);
    let end = (find_free(&grid[grid.len()-1]), grid.len() - 1);
    let graph = build_graph(&grid, &start);
    find_longest_path(&graph, &start, &end)

}

fn part1(input: &str) -> usize {
    let grid = parse(input);
    solve(&grid)
}
fn part2(input: &str) -> usize {
    let input = input.replace("^", ".").replace("v", ".").replace("<", ".").replace(">", ".");
    solve(&parse(&input))
}



fn main() {
    let test_input = read_to_string("inputs/day23-test.txt").unwrap();
    let real_input = read_to_string("inputs/day23.txt").unwrap();
    println!("Part 1 test: {}", part1(&test_input));
    println!("Part 1: {}",      part1(&real_input));
    println!("Part 2 test: {}", part2(&test_input));
    println!("Part 2: {}",      part2(&real_input));
}

