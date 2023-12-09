use std::fs::read_to_string;

fn solve(input: &str, reducer: fn(i64, &Vec<i64>) -> i64) -> i64 {
    input
        .lines()
        .map(|line| line.split_whitespace()
             .map(|x| x.parse::<i64>().unwrap())
             .collect::<Vec<_>>())
        .map(|history| {
            let mut deltas = vec![history];
            while !deltas.last().map(|ds| ds.iter().all(|&x| x == 0)).unwrap() {
                deltas.push(
                    deltas.last().unwrap().windows(2)
                    .map(|delta| delta[1] - delta[0])
                    .collect::<Vec<_>>());
            }
            deltas.iter().rev().fold(0, |acc, ds| reducer(acc, ds))
        }).sum()
}

fn part1(input: &str) -> i64 {
    solve(input, |acc, ds| acc + ds.last().unwrap())
}

fn part2(input: &str) -> i64 {
    solve(input, |acc, ds| ds.first().unwrap() - acc)
}


fn main() {
    let test_input = read_to_string("inputs/day09-test.txt").unwrap();
    let real_input = read_to_string("inputs/day09.txt").unwrap();

    println!("Part 1 test: {}", part1(&test_input));
    println!("Part 1: {}", part1(&real_input));
    println!("Part 2 test: {}", part2(&test_input));
    println!("Part 2: {}", part2(&real_input));
}
