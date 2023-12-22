use std::fs::read_to_string;
use std::collections::HashMap;
use priority_queue::PriorityQueue;

type Pos = (isize, isize, isize);
type Brick = (Pos, Pos);

fn parse(input: &str) -> Vec<Brick> {
    let parse_coords = |s: &str| {
        let mut r = s.split(',').map(|s| s.parse::<isize>().unwrap());
        (r.next().unwrap(), r.next().unwrap(), r.next().unwrap())
    };
    let mut res: Vec<Brick> = input
        .lines()
        .map(|line| {
            let mut parts = line.split('~');
            (parse_coords(parts.next().unwrap()), parse_coords(parts.next().unwrap()))
        })
    .map(|((x1,y1,z1),(x2,y2,z2))|
         ((x1.min(x2), y1.min(y2), z1.min(z2)),
         (x1.max(x2), y1.max(y2), z1.max(z2))))
        .collect();
    res.sort_by_key(|&((_,_,z1),_)| z1);
    res
}

fn intersects(a: &Brick, b: &Brick) -> bool {
    let (x1,y1,z1) = a.0;
    let (x2,y2,z2) = a.1;
    let (x3,y3,z3) = b.0;
    let (x4,y4,z4) = b.1;
    z1 <= z4 && z2 >= z3 && x1 <= x4 && x2 >= x3 && y1 <= y4 && y2 >= y3 
}

fn settle(bricks: &Vec<Brick>) -> Vec<Brick> {
    let mut res: Vec<Brick> = vec![];
    let mut max_z = 1;
    for brick in bricks {
        let safe_desc = 0.min(brick.0.2 - max_z);

        let mut valid = true;
        let mut brick = *brick;
        brick.0.2 -= safe_desc;
        brick.1.2 -= safe_desc;
        loop {
            if brick.0.2 == 0 {
                break;
            }
            let mut i = res.len() as isize - 1;
            while i >= 0 {
                if res[i as usize].1.2 < brick.0.2 {
                    i -= 1;
                    continue;
                }
                if intersects(&brick, &res[i as usize]) {
                    valid = false;
                    break;
                }
                i -= 1;
            }
            if !valid {
                break;
            }
            brick.0.2 -= 1;
            brick.1.2 -= 1;
        }
        brick.0.2 += 1;
        brick.1.2 += 1;
        max_z = max_z.max(brick.1.2);
        res.push(brick);
    }
    res
}

fn brick_supports(brick: &Brick, other: &Brick) -> bool {
    //check z
    if brick.1.2 + 1 != other.0.2 {
        return false;
    }
    //check x,y overlap
    let (x1,y1,_) = brick.0;
    let (x2,y2,_) = brick.1;
    let (x3,y3,_) = other.0;
    let (x4,y4,_) = other.1;

    x1 <= x4 && x2 >= x3 && y1 <= y4 && y2 >= y3
}

fn prepare(input: &str) -> (Vec<Brick>, HashMap<usize,Vec<usize>>, HashMap<usize,Vec<usize>>) {
    let bricks = parse(input);
    let bricks = settle(&bricks);

    let mut supports: HashMap<usize,Vec<usize>> = HashMap::new();
    let mut supported_by: HashMap<usize,Vec<usize>> = HashMap::new();
    for (i, brick) in bricks.iter().enumerate() {
        for (j, other) in bricks.iter().enumerate() {
            if i == j {
                continue;
            }
            if brick_supports(brick, other) {
                supports.entry(i).or_insert(vec![]).push(j);
                supported_by.entry(j).or_insert(vec![]).push(i);
            }
        }
    }
    (bricks, supports, supported_by)
}

fn part1(input: &str) -> isize {
    let (bricks, supports, supported_by) = prepare(input);
    let e_vec = vec![];
    let res = bricks
        .iter().enumerate()
        .filter(|&(i,_)|
                supports.get(&i)
                .unwrap_or(&e_vec).iter()
                .all(|v| supported_by.get(v).unwrap().len() > 1))
        .count() as isize;
    res
}

fn part2(input: &str) -> usize {
    let (bricks, supports, supported_by) = prepare(input);
    (0..bricks.len()).rev().map(|i| {
        if !supports.contains_key(&i) {
            return 0;
        }

        let mut falls = vec![];
        let mut pq = PriorityQueue::new();
        pq.push(i, bricks[i].1.2);

        while let Some((idx, _)) = pq.pop() {
            falls.push(idx);
            for sup in supports.get(&idx).unwrap_or(&vec![]) {
                if supported_by.get(sup).unwrap().iter().all(|v| falls.contains(v)) {
                    pq.push(*sup, bricks[*sup].1.2);
                }
            }
        }
        falls.len() - 1
    }).sum()
}

fn main() {
    let test_input = read_to_string("inputs/day22-test.txt").unwrap();
    let real_input = read_to_string("inputs/day22.txt").unwrap();

    println!("Part 1 test: {}", part1(&test_input));
    println!("Part 1: {}",      part1(&real_input));
    println!("Part 2 test: {}", part2(&test_input));
    println!("Part 2: {}",      part2(&real_input));
}
