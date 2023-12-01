use std::fs::read_to_string;
extern crate regex;
use regex::Regex;

fn part1(input: &str) -> u32 {
    input.lines()
        .map(String::from)
        .map(|line| {
            let nums = line.chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();
            let res = (10 * nums.first().unwrap()) + nums.last().unwrap();
            res
        })
    .sum()
}
/* Stupid fucking regex lib does not support overlapping matches => "eightwo" => 2 not 8
fn part2(input: &str) -> u32 {
    let valid_digits = Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    input.lines()
        .map(|line| {
            let digits = valid_digits.find_iter(line)
                .map(|m| m.as_str())
                .map(|s| match s {
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    _ => s.parse::<u32>().unwrap()
                })
            .collect::<Vec<u32>>();
            let res = if digits.len() >= 2 {
                (10 * digits.first().unwrap()) + digits.last().unwrap()
            } else {
                //println!("less than 2 digits: {} {:?}",line, digits);
                digits.first().unwrap().clone()
            };
            println!("line: {} digits: {:?} => {}", line, digits, res);
            res
        })
    .sum()
}
*/

fn part2(input: &str) -> u32 {
    let first_digits = Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let last_digits_rev = Regex::new("[1-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();

    input.lines()
        .map(|line| {
            let first_digit = first_digits.find(line).unwrap().as_str();
            let last_digit = last_digits_rev.find(&line.chars().rev().collect::<String>())
                .unwrap().as_str().chars().rev().collect::<String>();
            let to_int = |d| match d {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => d.parse::<u32>().unwrap()
            };
            (10 * to_int(&first_digit)) + to_int(&last_digit)
        })
    .sum()
}

fn main() {
    let test_input = read_to_string("inputs/day01-test.txt")
        .expect("Failed to read file");
    let real_input = read_to_string("inputs/day01.txt")
        .expect("Failed to read file");

    let test_input_2 = read_to_string("inputs/day01-test2.txt")
        .expect("Failed to read file");
    println!("Part 1 test: {}", part1(&test_input));
    println!("Part 1: {}", part1(&real_input));
    println!("Part 2 test: {}", part2(&test_input_2));
    println!("Part 2: {}", part2(&real_input));
}
