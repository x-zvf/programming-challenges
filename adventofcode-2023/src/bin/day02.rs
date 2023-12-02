use std::fs::read_to_string;
use std::collections::HashMap;

fn part1(input: &str, limit: &HashMap<&str, u32>) -> u32 {
    input.split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts = line.split(":").collect::<Vec<&str>>();
            let id = parts.first().unwrap().split(" ").last().unwrap().parse::<u32>().unwrap();
            let subsets = parts.last().unwrap().split(";");
            for subset in subsets {
                let cubes = subset.split(",");
                for cube in cubes {
                    let x = cube.trim().split(" ").collect::<Vec<&str>>();
                    let amount = x.first().unwrap() .parse::<u32>().unwrap();
                    let color = x.last().unwrap();
                    if amount > *limit.get(color).unwrap() {
                        return 0;
                    }
                }
            }
            id
        }).sum()
}

fn part2(input: &str) -> u32 {
    input.split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts = line.split(":").collect::<Vec<&str>>();
            let subsets = parts.last().unwrap().split(";");
            let mut min_amounts: HashMap<String, u32> = HashMap::new();
            for subset in subsets {
                let cubes = subset.split(",");
                for cube in cubes {
                    let x = cube.trim().split(" ").collect::<Vec<&str>>();
                    let amount = x.first().unwrap() .parse::<u32>().unwrap();
                    let color = x.last().unwrap().to_string();
                    if min_amounts.contains_key(&color) {
                        if amount > *min_amounts.get(&color).unwrap() {
                            min_amounts.insert(color, amount);
                        }
                    } else {
                        min_amounts.insert(color, amount);
                    }
                }
            }
            min_amounts.values().fold(1, |acc, x| acc * x)
        }).sum()
}

fn main() {
    let test_input = read_to_string("inputs/day02-test.txt").unwrap();
    let real_input = read_to_string("inputs/day02.txt").unwrap();
    let limits: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    println!("Part 1 test: {}", part1(&test_input, &limits));
    println!("Part 1: {}", part1(&real_input, &limits));
    println!("Part 2 test: {}", part2(&test_input));
    println!("Part 2: {}", part2(&real_input));
}
