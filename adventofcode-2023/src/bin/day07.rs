use std::fs::read_to_string;
use std::collections::HashMap;

fn card_value(card: char) -> u32 {
    match card {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => card.to_digit(10).unwrap(),
    }
}

fn card_value_2(card: char) -> u32 {
    match card {
        'T' => 10,
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => card.to_digit(10).unwrap(),
    }
}

static KIND_FACTOR: u64 = 10_00_00_00_00_00;

fn eval_cards(hand: &Vec<u32>) -> (u64, HashMap<&u32, u32>) {
    let mut score: u64 = 0;
    for (i, card) in hand.iter().enumerate() {
        score += *card as u64 * 100u64.pow((5 - i - 1) as u32);
    }

    let card_counts = hand.iter().fold(HashMap::new(), |mut acc, card| {
        let count = acc.entry(card).or_insert(0);
        *count += 1;
        acc
    });
    
    (score, card_counts)
}

fn score_hand_1(hand: &Vec<u32>) -> u64 {
    let (score, card_counts) = eval_cards(hand);
    // five of a kind
    if card_counts.values().any(|&count| count == 5) {
        return 7 * KIND_FACTOR + score;
    }
    // four of a kind
    if card_counts.values().any(|&count| count == 4) {
        return 6 * KIND_FACTOR + score;
    }
    // full house
    if card_counts.values().any(|&count| count == 3) && card_counts.values().any(|&count| count == 2) {
        return 5 * KIND_FACTOR + score;
    }
    // three of a kind
    if card_counts.values().any(|&count| count == 3) {
        return 4 * KIND_FACTOR + score;
    }
    // two pair
    if card_counts.values().filter(|&&count| count == 2).count() == 2 {
        return 3 * KIND_FACTOR + score;
    }
    // one pair
    if card_counts.values().any(|&count| count == 2) {
        return 2 * KIND_FACTOR + score;
    }
    // high card
    1 * KIND_FACTOR + score

}

fn score_hand_2(hand: &Vec<u32>) -> u64 {
    let (score,card_counts) = eval_cards(hand);

    let card_counts_no_jokers = card_counts.iter().filter(|(&&card, _)| card != 1).collect::<HashMap<_,_>>();
    let jokers = card_counts.get(&&1).unwrap_or(&0);

    // five of a kind
    if *jokers == 5 || card_counts_no_jokers.values().any(|&count| count + jokers == 5) {
        return 7 * KIND_FACTOR + score;
    }
    // four of a kind
    if card_counts_no_jokers.values().any(|&count| count + jokers == 4) {
        return 6 * KIND_FACTOR + score;
    }

    // full house
    assert!(*jokers <= 2);
    if *jokers == 0 {
        if card_counts_no_jokers.values().any(|&&count| count == 3) && card_counts_no_jokers.values().any(|&&count| count == 2) {
            return 5 * KIND_FACTOR + score;
        }
    } else if *jokers == 1 {
        if card_counts_no_jokers.values().any(|&&count| count == 3) || card_counts_no_jokers.values().filter(|&&&count| count == 2).count() == 2 {
            return 5 * KIND_FACTOR + score;
        }
    } else if *jokers == 2 {
        if card_counts_no_jokers.values().any(|&&count| count == 2) {
            return 5 * KIND_FACTOR + score;
        }
    } 

    // three of a kind
    if card_counts_no_jokers.values().any(|&count| count + jokers == 3) {
        return 4 * KIND_FACTOR + score;
    }
    // two pair
    assert!(*jokers <= 1);
    if *jokers == 0 {
        if card_counts_no_jokers.values().filter(|&&&count| count == 2).count() == 2 {
            return 3 * KIND_FACTOR + score;
        }
    } else if *jokers == 1 {
        if card_counts_no_jokers.values().filter(|&&&count| count == 2).count() == 1 {
            return 3 * KIND_FACTOR + score;
        }
    }
    // one pair
    if card_counts_no_jokers.values().any(|&count| count + jokers == 2) {
        return 2 * KIND_FACTOR + score;
    }
    // high card
    assert!(*jokers == 0);
    1 * KIND_FACTOR + score
}

fn run(input: &str, score_fn: fn(&Vec<u32>) -> u64, card_val_fn: fn(char) -> u32) -> u64 {
    let mut scored_cards = input.lines().map(|line| {
        let parts = line.split(" ").collect::<Vec<_>>();
        let hand = parts[0].to_string();
        let bid = parts[1].parse::<u32>().unwrap();
        (hand, bid)
    }).map(|(hand, bid)| {
        let values = hand.chars().map(|c| card_val_fn(c)).collect::<Vec<_>>(); 
        let score = score_fn(&values);
        (score, bid)
    }).collect::<Vec<_>>();
    scored_cards.sort_by(|a, b| a.0.cmp(&b.0)); // reverse order

    scored_cards.iter().enumerate().map(|(i, (_, bid))| {
        (i+1) as u64 * *bid as u64
    }).sum()
}

fn part1(input: &str) -> u64 {
    run(input, score_hand_1, card_value)
}

fn part2(input: &str) -> u64 {
    run(input, score_hand_2, card_value_2)
}


fn main() {
    let test_input = read_to_string("inputs/day07-test.txt").unwrap();
    let real_input = read_to_string("inputs/day07.txt").unwrap();

    println!("Part 1 test: {}", part1(&test_input));
    println!("Part 1: {}", part1(&real_input));
    println!("Part 2 test: {}", part2(&test_input));
    println!("Part 2: {}", part2(&real_input));
}
