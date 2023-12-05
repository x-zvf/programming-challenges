use std::fs::read_to_string;
use std::cmp::min;
use std::cmp::max;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Mapping {
    to: u64,
    from: u64,
    len: u64,
}
impl Mapping{
    fn contains(&self, x: u64) -> bool {
        self.from <= x && x < self.from + self.len
    }
    fn apply(&self, x: u64) -> u64 {
//        println!("apply {:?} to {}", self, x);
        assert!(self.contains(x));
        let delta: i64 = x as i64 - self.from as i64;
        (self.to as i64 + delta) as u64
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Table {
    mappings: Vec<Mapping>,
}

impl Table {
    fn new(mappings: Vec<Mapping>) -> Self {
        Self {
            mappings,
        }
    }

    fn translate(&self, x: u64) -> u64 {
        for mapping in &self.mappings {
            if mapping.contains(x as u64) {
                return mapping.apply(x);
            }
        }
        x
    }

    fn translate_ranges(&self, mut inp: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
        let mut result = vec![];
        for mapping in &self.mappings {
            let map_end = mapping.from + mapping.len;

            let mut new_r: Vec<(u64,u64)> = Vec::new();
            while !inp.is_empty() {
                let (start,end) = inp.pop().unwrap();

                let pre = (start,min(end, mapping.from));
                let intersect = (max(start, mapping.from), min(end, map_end));
                let post = (max(start, map_end), end);

                if pre.0 < pre.1 {
                    new_r.push(pre);
                }
                if intersect.0 < intersect.1 {
                    result.push((intersect.0 - mapping.from + mapping.to, intersect.1 - mapping.from + mapping.to));
                }
                if post.0 < post.1 {
                    new_r.push(post);
                }
            }
            inp = new_r;

        }
        result.extend(inp);
        result
    }
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<Table>) {
    let mut parts = input.split("\n\n")
        .filter(|l| !l.is_empty())
        .map(|line| {
            String::from(line)
        });
    let seeds = parts.next().unwrap()
        .split(" ").skip(1)
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let tables = parts
        .map(|table| {
            let t = table.split("\n").skip(1)
                .filter(|l| !l.is_empty())
                .map(|line| {
                    let mut ns = line.split(" ")
                        .filter(|n| !n.is_empty())
                        .map(|n| n.parse::<u64>().unwrap());

                    Mapping {
                        to: ns.next().unwrap(),
                        from: ns.next().unwrap(),
                        len: ns.next().unwrap(),
                    }
                })
                .collect::<Vec<_>>();
            Table::new(t)
        })
        .collect::<Vec<Table>>();
    (seeds, tables)
}
fn part1(input: &str) -> u64 {
    let (seeds,tables) = parse_input(input);
    seeds.iter()
        .map(|seed|
             tables.iter()
                .fold(*seed, |x, table| table.translate(x)))
        .min().unwrap()
}

fn part2(input: &str) -> u64 {
    let (seeds,tables) = parse_input(input);

    let seed_ranges = seeds.chunks(2).map(|w| (w[0], w[1])).collect::<Vec<(u64,u64)>>();
    //println!("seed_ranges: {:?}", seed_ranges);

    let mut mins = vec![];
    for (start,len) in seed_ranges {
        let mut rs = vec![(start, start+len)];
        for table in &tables {
            //println!("table: {:?}: rs: {:?}", table, rs);
            rs = table.translate_ranges(rs);
        }
        mins.push(rs.iter().map(|(s,_)| s).min().unwrap().clone());
    }
   mins.iter().min().unwrap().clone()
}


fn main() {
    let test_input = read_to_string("inputs/day05-test.txt").unwrap();
    let real_input = read_to_string("inputs/day05.txt").unwrap();

    println!("Part 1 test: {}", part1(&test_input));
    println!("Part 1: {}", part1(&real_input));
    println!("Part 2 test: {}", part2(&test_input));
    println!("Part 2: {}", part2(&real_input));
}
