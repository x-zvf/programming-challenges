use std::fs::read_to_string;
use itertools::Itertools;
use std::process::Command;

type V3 = (isize, isize, isize);
type V2 = (isize, isize);

type SnowFlake = (V3, V3);
type Bounds = (V2, V2);


fn parse(input: &str) -> Vec<SnowFlake> {
    let parse_v3 = |s: &str| -> V3 {
        let mut parts = s.split(",");
        let x = parts.next().unwrap().trim().parse().unwrap();
        let y = parts.next().unwrap().trim().parse().unwrap();
        let z = parts.next().unwrap().trim().parse().unwrap();
        (x, y, z)
    };
    input.lines().map(|line| {
        let mut parts = line.split(" @ ");
        let pos = parse_v3(parts.next().unwrap());
        let vel = parse_v3(parts.next().unwrap());
        (pos, vel)
    }).collect()
}
fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.0001
}

fn intersect_in(a: &SnowFlake, b: &SnowFlake, bounds: &Bounds) -> bool {
    let m_1 = (a.1.1 as f64) / (a.1.0 as f64);
    let m_2 = (b.1.1 as f64) / (b.1.0 as f64);
    let b_1 = a.0.1 as f64 - m_1 * a.0.0 as f64;
    let b_2 = b.0.1 as f64 - m_2 * b.0.0 as f64;
    if close(m_1, m_2) {
        if close(b_1, b_2) {
            return true; // id
        } else {
            return false; //parallel
        }
    }

    let cx = (b_2 - b_1) / (m_1 - m_2);
    let cy = m_1 * cx + b_1;

    if (cx > a.0.0 as f64) != (a.1.0 > 0) || (cx > b.0.0 as f64) != (b.1.0 > 0) {
        return false;
    }

    cx >= bounds.0.0 as f64 && cx <= bounds.1.0 as f64
        && cy >= bounds.0.1 as f64 && cy <= bounds.1.1 as f64
}

fn part1(input: &str, bounds: &Bounds) -> usize {
    let snowflakes = parse(input);
    snowflakes.iter().combinations(2).filter(|pair| {
        intersect_in(pair[0], pair[1], bounds)
    }).count()
}

fn part2(input: &str) -> isize {
    let snowflakes = parse(input);
    let mut expr = "px + py + pz /. Solve[{".to_string();
    for i in 0..3 {
        let (ps, vs) = (vec![snowflakes[i].0.0, snowflakes[i].0.1, snowflakes[i].0.2],
                        vec![snowflakes[i].1.0, snowflakes[i].1.1, snowflakes[i].1.2]);
        for (j,name) in ["x","y","z"].iter().enumerate() {
            expr.push_str(format!("{} + t{} * {} == p{} + t{} * v{}", ps[j], i, vs[j], name, i, name).as_str());
            if !(i == 2 && j == 2) {
                expr.push_str(", ");
            }
        }
    }
    expr.push_str("},{px,py,pz,vx,vy,vz,t0,t1,t2}]");
    let output = Command::new("wolframscript")
        .arg("-code").arg(expr).output().unwrap().stdout;
    let output = String::from_utf8(output).unwrap();
    let output = output[1..output.len()-2].trim().to_string();
    output.parse().unwrap()
}

fn main() {
    let test_input = read_to_string("inputs/day24-test.txt").unwrap();
    let real_input = read_to_string("inputs/day24.txt").unwrap();

    println!("Part 1 test: {}", part1(&test_input, &((7, 7), (27, 27)) ));
    println!("Part 1: {}",      part1(&real_input,
                                      &((200000000000000,200000000000000),
                                      (400000000000000,400000000000000) ) ));

    println!("Part 2 test: {}", part2(&test_input));
    println!("Part 2: {}",      part2(&real_input));
}
