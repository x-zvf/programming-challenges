use std::fs::read_to_string;
use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<u32> {
    input.split("\n")
        .filter(|l| !l.is_empty())
        .map(|line| {
            let (_, all_cards) = line.split_once(":").unwrap();
            let (winning, mine) = all_cards.split_once("|").unwrap();

            let parse_cards = |cards: &str|
                cards.split(" ")
                .filter(|n| !n.is_empty())
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();

            let winning = parse_cards(winning);
            let mine = parse_cards(mine);

            winning.intersection(&mine).count() as u32
        }).collect()
}

fn part1(input: &str) -> u32 {
    parse_input(input).iter()
        .map(|x|
             if x > &0 {
                 2u32.pow((x - 1) as u32)
             } else {
                 0
             }
            ).sum()
}

fn part2(input: &str) -> u32 {
    let evals = parse_input(input);
    let mut ncards: Vec<u32> = vec![1; evals.len()];
    for (id, x) in evals.into_iter().enumerate() {
        let u_id = id as u32;
        for c in u_id+1 ..= u_id+x {
            ncards[c as usize] += ncards[id];
        }
    }
    ncards.iter().sum()
}


fn main() {
    let test_input = read_to_string("inputs/day04-test.txt").unwrap();
    let real_input = read_to_string("inputs/day04.txt").unwrap();

    println!("Part 1 test: {}", part1(&test_input));
    println!("Part 1: {}", part1(&real_input));
    println!("Part 2 test: {}", part2(&test_input));
    println!("Part 2: {}", part2(&real_input));
}
