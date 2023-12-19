use std::fs::read_to_string;
use std::collections::HashMap;
use regex::Regex;
use core::ops::Range;

type Part = [isize; 4];

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Comparison {
    Always,
    LessThan,
    GreaterThan
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Condition {
    idx: usize,
    value: isize,
    comparison: Comparison,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Workflow {
    name: String,
    steps: Vec<(Condition, Action)>
}
impl Workflow {
    fn process(&self, part: &Part) -> &Action {
        for (condition, action) in self.steps.iter() {
            let val = part[condition.idx];
            let result = match condition.comparison {
                Comparison::Always => true,
                Comparison::LessThan => val < condition.value,
                Comparison::GreaterThan => val > condition.value,
            };
            if result {
                return action;
            }
        }
        panic!("No action found for {:?}", part);
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Action {
    Accept,
    Reject,
    Send(String),
}

fn stoidx(s: &str) -> usize {
    match s {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!("Unknown key {}", s),
    }
}

fn parse(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let worflow_re = Regex::new(r"^(\w+)\{(.*)\}$").unwrap();
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows = workflows.lines().map(|line| {
        //example: px{a<2006:qkq,m>2090:A,rfg}
        let (name, steps) = (worflow_re.captures(line).unwrap()[1].to_string(), worflow_re.captures(line).unwrap()[2].to_string());
        let steps = steps.split(",").map(|step| {
            if step.contains(":") {
                let (condition, action) = step.split_once(":").unwrap();
                let idx = stoidx(&condition[0..1]);
                let comparison = match &condition[1..2] {
                    "<" => Comparison::LessThan,
                    ">" => Comparison::GreaterThan,
                    _ => panic!("Unknown comparison {}", condition),
                };
                let value = condition[2..].parse().unwrap();
                let action = match action {
                    "A" => Action::Accept,
                    "R" => Action::Reject,
                    _ => Action::Send(action.to_string()),
                };
                (Condition { idx, value, comparison }, action)
            } else {
                let action = match step {
                    "A" => Action::Accept,
                    "R" => Action::Reject,
                    _ => Action::Send(step.to_string()),
                };
                (Condition { idx: 0, value: 0, comparison: Comparison::Always }, action)
            }
        }).collect(); 
        (name.to_string(), Workflow { name, steps })

    }).collect::<HashMap<String,Workflow>>();
    let parts = parts.lines().map(|line| {
        let mut part = Part::default();
        for mapping in line[1..line.len()-1].split(",") {
            let value = mapping[2..].parse().unwrap();
            let idx = stoidx(&mapping[0..1]);
            part[idx] = value;
        }
        part
    }).collect();

    (workflows, parts)
}

fn accept(workflows: &HashMap<String, Workflow>, part: &Part) -> bool {
    let mut workflow = workflows.get("in").unwrap();
    loop {
        let action = workflow.process(part);
        match action {
            Action::Accept => { return true; },
            Action::Reject => { return false; },
            Action::Send(name) => { workflow = workflows.get(name).unwrap(); },
        }
    }
}

fn part1(input: &str) -> isize {
    let (workflows, parts) = parse(input);
    parts.iter().filter(|part| accept(&workflows, part)).map(|part| part.iter().sum::<isize>()).sum()
}

fn eval_range_recur(workflows: &HashMap<String, Workflow>, action: &Action, ranges: &[Range<isize>; 4], depth: usize) -> isize {
    match action {
        Action::Accept => {
            ranges.iter().map(|r| r.end - r.start + 1).product()
        },
        Action::Reject => { 0 },
        Action::Send(name) => { 
            let mut ranges = ranges.clone();
            let wf = workflows.get(name).unwrap();

            let mut p = 0;
            for step in wf.steps.iter() {
                let mut new_ranges = ranges.clone();
                p += match step.0 {
                    Condition { comparison: Comparison::Always, .. } => {
                        eval_range_recur(workflows, &step.1, &new_ranges, depth + 1)
                    },
                    Condition { idx, value, comparison: Comparison::LessThan } => {
                        new_ranges[idx].end = value - 1;
                        ranges[idx].start = value;
                        eval_range_recur(workflows, &step.1, &new_ranges, depth + 1)
                    },
                    Condition { idx, value, comparison: Comparison::GreaterThan } => {
                        let mut new_ranges = ranges.clone();
                        ranges[idx].end = value;
                        new_ranges[idx].start = value + 1;
                        eval_range_recur(workflows, &step.1, &new_ranges, depth + 1)
                    },
                }
            }
            p
        },
    }
}

fn part2(input: &str) -> isize {
    let (workflows, _) = parse(input);
    let ranges = [(1..4000), (1..4000), (1..4000), (1..4000)];
    eval_range_recur(&workflows, &Action::Send("in".to_string()), &ranges, 0)
}


fn main() {
    let test_input = read_to_string("inputs/day19-test.txt").unwrap();
    let real_input = read_to_string("inputs/day19.txt").unwrap();
    println!("Part 1 test: {}", part1(&test_input));
    println!("Part 1: {}",      part1(&real_input));
    println!("Part 2 test: {}", part2(&test_input));
    println!("Part 2: {}",      part2(&real_input));
}

