use good_lp::{
    Expression, IntoAffineExpression, Solution, SolverModel, default_solver, variable, variables,
};
use itertools::Itertools;
use std::fs;

struct Machine {
    target_state: u32,
    buttons: Vec<u32>,
    joltages: Vec<u32>,
}
impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let (target, rest) = value.split_once("]").unwrap();
        let target_state: u32 = target
            .chars()
            .filter(|&c| c == '#' || c == '.')
            .enumerate()
            .map(|(pos, c)| match c {
                '#' => 1 << pos,
                '.' => 0,
                _ => unreachable!(),
            })
            .sum();

        let (button_s, joltages_s) = rest.split_once("{").unwrap();

        let buttons: Vec<u32> = button_s
            .split(")")
            .map(|bs| {
                let nums: Vec<Option<u32>> = bs
                    .split(",")
                    .map(|is| {
                        is.chars()
                            .filter(|&c| c >= '0' && c <= '9')
                            .collect::<String>()
                            .parse::<u32>()
                            .ok()
                    })
                    .collect();
                nums.iter()
                    .map(|n| match n {
                        Some(x) => 1 << x,
                        None => 0,
                    })
                    .sum()
            })
            .collect();
        let joltages: Vec<u32> = joltages_s
            .split(",")
            .map(|js| {
                js.chars()
                    .filter(|&c| c >= '0' && c <= '9')
                    .collect::<String>()
                    .parse::<u32>()
                    .ok()
            })
            .filter_map(|j| j)
            .collect();
        Machine {
            target_state,
            buttons,
            joltages,
        }
    }
}

impl Machine {
    fn part1(&self) -> usize {
        let solution = &self
            .buttons
            .iter()
            .powerset()
            .filter(|ps| {
                self.target_state ^ ps.iter().fold(0, |x: u32, y: &&u32| (x ^ **y) as u32) == 0
            })
            .next()
            .unwrap();
        solution.len()
    }
    fn part2(&self) -> usize {
        let mut vs = variables!();

        let presses: Vec<_> = (0..self.buttons.len())
            .map(|_| vs.add(variable().integer().min(0)))
            .collect();
        let mut problem = vs
            .minimise(presses.iter().sum::<Expression>())
            .using(default_solver);
        problem.set_parameter("logLevel", "0");

        let mut exs = vec![0.into_expression(); self.joltages.len()];
        for (bi, btn) in self.buttons.iter().enumerate() {
            for i in 0..self.joltages.len() {
                if (btn & (1 << i)) == 0 {
                    continue;
                }
                exs[i] += presses[bi];
            }
        }
        for i in 0..self.joltages.len() {
            problem.add_constraint(exs[i].clone().eq(self.joltages[i]));
        }
        let s = problem.solve().unwrap();
        presses.iter().map(|v| s.value(*v) as usize).sum()
    }
}

fn read(fname: &str) -> Vec<Machine> {
    fs::read_to_string(fname)
        .unwrap()
        .split("\n")
        .filter_map(|x| {
            let y = x.trim();
            if y.is_empty() { None } else { Some(y) }
        })
        .map(Machine::from)
        .collect()
}
fn part1(ms: &Vec<Machine>) -> usize {
    ms.iter().map(|m| m.part1()).sum()
}
fn part2(ms: &Vec<Machine>) -> usize {
    ms.iter().map(|m| m.part2()).sum()
}

fn main() {
    let sample = read("./sample.txt");
    let input = read("./input.txt");
    println!("part1: {} {}", part1(&sample), part1(&input));
    println!("part2: {} {}", part2(&sample), part2(&input));
}
