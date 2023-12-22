use std::fs::read_to_string;
use std::collections::HashMap;
use std::collections::VecDeque;
use num::integer::lcm;

type Pulse = bool;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType {
    Forward,
    FlipFlop,
    Conj,
}

struct State<'a> {
    module_outputs: HashMap<&'a str, Vec<&'a str>>,
    module_types: HashMap<&'a str, ModuleType>,
    module_inputs: HashMap<&'a str, HashMap<&'a str, Pulse>>,
    module_values: HashMap<&'a str, Pulse>,
    pulses: Vec<(&'a str, &'a str, Pulse)>,
}

fn parse(input: &str) -> State {
    let mut s = State {
        module_outputs: HashMap::new(),
        module_types: HashMap::new(),
        module_inputs: HashMap::new(),
        module_values: HashMap::new(),
        pulses: Vec::new(),
    };
    for line in input.lines() {
        let (name, outputs) = line.split_once(" -> ").unwrap();
        let outputs: Vec<_> = outputs.split(", ").collect();
        let t = match name.as_bytes()[0] {
            b'%' => ModuleType::FlipFlop,
            b'&' => ModuleType::Conj,
            _ => ModuleType::Forward,
        };
        let name = name.trim_start_matches("%").trim_start_matches("&");
        s.module_types.insert(name, t);
        s.module_outputs.insert(name, outputs.clone());
        s.module_values.insert(name, false);
        for output in outputs {
            s.module_inputs.entry(output).or_insert(HashMap::new()).insert(name, false);
        }
    }
    s
}

fn iter(s: &mut State) {
    let mut queue = VecDeque::new();
    queue.push_back(("button","broadcaster", false));
    while let Some((from, to, pulse)) = queue.pop_front() {
        s.pulses.push((from, to, pulse));
        let outputs = s.module_outputs.get(to);
        if outputs.is_none() {
            continue;
        }
        let outputs = outputs.unwrap();
        let t = s.module_types.get(to).unwrap();
        match t {
            ModuleType::Forward => {
                for output in outputs {
                    queue.push_back((to, output, pulse));
                }
            },
            ModuleType::FlipFlop => {
                if !pulse {
                    let new_state = !s.module_values.get(to).unwrap();
                    s.module_values.insert(to, new_state);
                    for output in outputs {
                        queue.push_back((to, output, new_state));
                    }
                }
            },
            ModuleType::Conj => {
                s.module_inputs.get_mut(to).unwrap().insert(from, pulse);
                let all_high = s.module_inputs.get(to).unwrap().values().all(|&x| x);
                for output in outputs {
                    queue.push_back((to, output, !all_high));
                }
            },
        }
    }
}


fn part1(input: &str) -> usize {
    let mut s = parse(input);
    for _ in 0..1000 {
        iter(&mut s);
    }
    let mut low = 0;
    let mut high = 0;
    for (_, _, pulse) in s.pulses {
        if pulse {
            high += 1;
        } else {
            low += 1;
        }
    }
    low * high
}

fn part2(input: &str) -> usize {
    let mut s = parse(input);
    assert_eq!(s.module_inputs.get("rx").unwrap().len(), 1);
    let rx_input = s.module_inputs.get("rx").unwrap().iter().next().unwrap().0;
    let rx_depends = s.module_inputs.get(rx_input).unwrap().iter()
        .map(|(k, _)| k.clone()).collect::<Vec<_>>();

    let rx_input = rx_input.clone();
    let rx_depends = rx_depends.clone();
    let mut rx_depends_cycle_len = HashMap::new();
    let mut i = 0;
    loop {
        iter(&mut s);
        i += 1;

        for rx_d in &rx_depends {
            if rx_depends_cycle_len.contains_key(rx_d) {
                continue;
            }
            if s.pulses.iter().any(|(from, _, p)| from == rx_d && *p) {
                rx_depends_cycle_len.insert(rx_d, i);
            }
        }
        if rx_depends_cycle_len.len() == rx_depends.len() {
            break;
        }
        s.pulses.clear();
    }
    rx_depends_cycle_len
        .values().map(|x| *x as usize)
        .fold(1, |acc, x| lcm(acc, x))
}

fn main() {
    let test_input = read_to_string("inputs/day20-test.txt").unwrap();
    let real_input = read_to_string("inputs/day20.txt").unwrap();
    println!("Part 1 test: {}", part1(&test_input));
    println!("Part 1: {}",      part1(&real_input));
    println!("Part 2: {}",      part2(&real_input));
}

