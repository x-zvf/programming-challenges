use std::fs::read_to_string;
use std::collections::HashMap;
use num::integer::lcm;

#[derive(Debug,Clone,Hash)]
enum Dir {
    Left,
    Right,
}

fn parse(input: &str) -> (Vec<Dir>, HashMap<String,(String,String)>) {
    let (path_str, map_str) = input.split_once("\n\n").unwrap();
    let path = path_str.trim().chars().map(|c| match c {
        'L' => Dir::Left,
        'R' => Dir::Right,
        _ => panic!("Invalid direction"),
    }).collect::<Vec<_>>();

    let mut map = HashMap::new();
    let map_line_re = regex::Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    for line in map_str.lines() {
        let caps = map_line_re.captures(line).unwrap();
        let from = caps.get(1).unwrap().as_str();
        let left = caps.get(2).unwrap().as_str();
        let right = caps.get(3).unwrap().as_str();
        map.insert(from.to_string(), (left.to_string(), right.to_string()));
        
    }
    (path, map)
}

fn follow_path_until_z(path: &[Dir], map: &HashMap<String,(String,String)>, from: &str) -> u64 {
    let mut current = from;
    let mut i = 0;
    while current.chars().nth(2) != Some('Z') {
        let (left, right) = map.get(current).unwrap();
        current = match path[i % path.len()] {
            Dir::Left => left,
            Dir::Right => right,
        };
        i += 1;
    }
    i as u64
}


fn part1(input: &str) -> u64 {
    let (path, map) = parse(input);
    let steps = follow_path_until_z(&path, &map, "AAA");
    steps
}

fn part2(input: &str) -> u64 {
    let (path, map) = parse(input);
    let steps_till_z = map.iter()
        .filter(|(point, _)| point.chars().nth(2) == Some('A'))
        .map(|(point, _)| point.clone())
        .map(|point| follow_path_until_z(&path, &map, &point));
    let mut result = 1; // coundn't use reduce, because references....
    for step in steps_till_z {
        result = lcm(result, step);
    }
    result
}


fn main() {
    let test_input1 = read_to_string("inputs/day08-test1.txt").unwrap();
    let test_input2 = read_to_string("inputs/day08-test2.txt").unwrap();
    let real_input = read_to_string("inputs/day08.txt").unwrap();

    println!("Part 1 test2: {}", part1(&test_input2));
    println!("Part 1: {}", part1(&real_input));
    println!("Part 2 test: {}", part2(&test_input1));
    println!("Part 2: {}", part2(&real_input));
}
