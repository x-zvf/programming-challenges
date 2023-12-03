use std::fs::read_to_string;
use regex::Regex;
use std::collections::HashSet;

fn neighbours(y: i32, start: i32, end: i32, maxy: i32, maxx: i32) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    neighbours.push((y, start - 1));
    neighbours.push((y, end + 1));
    for x in start-1..=end+1 {
        neighbours.push((y - 1, x));
        neighbours.push((y + 1, x));
    }
    neighbours.into_iter()
        .filter(|(y, x)| {
            *y >= 0 && *x >= 0 && *y < maxy && *x < maxx
        })
    .map(|(y,x)| (y as usize, x as usize))
        .collect::<Vec<(usize, usize)>>()
}

fn positions(re: &Regex, lines: &Vec<String>) -> Vec<(usize, usize, usize, String)> {
    lines.iter().enumerate().flat_map(|(y, line)| 
        re.captures_iter(line).map(move |capture| {
            let cap = capture.get(1).unwrap();
            let start = cap.start();
            let end = cap.end() - 1;
            (y, start, end, String::from(cap.as_str()))
        })).collect()
}

fn part_numbers(lines: &Vec<String>) -> Vec<(u32, u32, u32, u32)> {
    let num_re = Regex::new(r"(\d+)").unwrap();
    positions(&num_re, lines)
        .into_iter()
        .map(|(y, start, end, num)|
             (y as u32, start as u32, end as u32, num.parse::<u32>().unwrap())
            )
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> u32 {
    let lines = input.split("\n").map(String::from).collect::<Vec<String>>();
    let mat = lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    part_numbers(&lines)
        .into_iter()
        .filter(|(y, start, end, _)| {
            let neighbours = neighbours(*y as i32, *start as i32, *end as i32,
                                        mat.len() as i32, mat[0].len() as i32);
            for (y, x) in neighbours {
                if mat[y as usize].len() <= x as usize {
                    continue; // empty lines
                }
                let c = mat[y][x];
                if !c.is_digit(10) && c != '.'{
                    return true;
                }
            }
            return false;
        }).map(|(_, _, _, num)| {
                num
            }).sum()
}

fn part2(input: &str) -> u32 {
    let lines = input.split("\n").map(String::from).collect::<Vec<String>>();
    let gear_re = Regex::new(r"(\*)").unwrap();

    let part_numbers = part_numbers(&lines);

    positions(&gear_re, &lines)
        .into_iter()
        .map(|(y, start, end, _)| {
        let neighbours = neighbours(y as i32, start as i32, end as i32,
                                    lines.len() as i32, lines[0].len() as i32);
        let mut neighbour_parts: HashSet<_> = HashSet::new();

        for (y, x) in neighbours {
            for part in part_numbers.iter() {
                let (yp, start, end, _) = part;
                if y as u32 == *yp && x as u32 >= *start && x as u32 <= *end {
                    neighbour_parts.insert(part);
                }
            }
        }
        if neighbour_parts.len() == 2 {
            neighbour_parts.iter().map(|(_, _, _, num)| num).product::<u32>()
        } else {0}
        }).sum()

}

fn main() {
    let test_input = read_to_string("inputs/day03-test.txt").unwrap();
    let real_input = read_to_string("inputs/day03.txt").unwrap();

    println!("Part 1 test: {}", part1(&test_input));
    println!("Part 1: {}", part1(&real_input));
    println!("Part 2 test: {}", part2(&test_input));
    println!("Part 2: {}", part2(&real_input));
}
