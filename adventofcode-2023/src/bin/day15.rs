use std::fs::read_to_string;

fn hash(s: &str) -> usize {
    let mut h = 0;
    for c in s.chars() {
        h = ((h + c as usize) * 17) % 256;
    }
    h
}

fn part1(input: &str) -> usize {
   input.trim().split(",").map(|s| hash(s)).sum()
}

fn part2(input: &str) -> usize {
    let steps = input.trim().split(",");
    let mut boxes = vec![vec![]; 256];
    for step in steps {
        if step.contains("=") {
            let mut parts = step.split("=");
            let label = parts.next().unwrap();
            let b = hash(&label);
            let focal_len = parts.next().unwrap().parse::<usize>().unwrap();
            if let Some(idx) = boxes[b].iter().position(|(l,_)| *l == label) {
                boxes[b][idx] = (label, focal_len);
            } else {
                boxes[b].push((label, focal_len));
            }
        } else {
            let label = &step[0..step.len()-1];
            let b = hash(label);
            if let Some(idx) = boxes[b].iter().position(|(l,_)| *l == label) {
                boxes[b].remove(idx);
            }
        }

    }
    boxes
        .iter()
        .enumerate()
        .map(|(i, b)| b
             .iter()
             .enumerate()
             .map(|(j, (_, f))| (i+1) * (j+1) * f)
             .sum::<usize>())
        .sum()
}

fn main() {
    let test_input = read_to_string("inputs/day15-test.txt").unwrap();
    let real_input = read_to_string("inputs/day15.txt").unwrap();
    println!("Part 1 test: {}", part1(&test_input));
    println!("Part 1: {}",      part1(&real_input));
    println!("Part 2 test: {}", part2(&test_input));
    println!("Part 2: {}",      part2(&real_input));
}
