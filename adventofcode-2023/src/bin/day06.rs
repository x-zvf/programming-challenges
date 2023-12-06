use std::fs::read_to_string;

fn ways((t,d): (&u64,&u64)) -> u64 {
    //Note: brute force is actually fast enough for part 2. but MAAAAATH!!!
    /*
    let correct = (1..*t).map(|c| {
        let distance = (t - c) * c;
        distance
    }).filter(|dst| (dst > &d)).count() as u64; */

    let t = *t as f64;
    let d = *d as f64;

    let c1: f64 = (t + (t*t - (4.0 * d)).sqrt()) / 2.0;
    let c2: f64 = (t - (t*t - (4.0 * d)).sqrt()) / 2.0;

    let start = c1.min(c2).floor() as u64;
    let end = c1.max(c2).ceil() as u64;
    end - start - 1
}

fn part1(input: &str) -> u64 {
    let td = input.lines().map(|line| {
        line.split(" ").filter(|s| !s.is_empty())
            .skip(1)
            .map(|num| num.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let times = &td[0];
    let dsts = &td[1];
    times
        .iter()
        .zip(dsts.iter())
        .map(ways)
        .product()
}

fn part2(input: &str) -> u64 {
    let nums = 
        input.lines()
        .map(|line|
             line.chars()
             .filter(|c| c.is_digit(10))
             .collect::<String>()
             .parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    ways((&nums[0],&nums[1]))
}


fn main() {
    let test_input = read_to_string("inputs/day06-test.txt").unwrap();
    let real_input = read_to_string("inputs/day06.txt").unwrap();

    println!("Part 1 test: {}", part1(&test_input));
    println!("Part 1: {}", part1(&real_input));
    println!("Part 2 test: {}", part2(&test_input));
    println!("Part 2: {}", part2(&real_input));
}
