use std::fs::read_to_string;

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

fn combine_tables(tables: &Vec<Table>) -> Table {
    //let mut combined_mappings: Vec<Mapping> = Vec::new();

    let mut descision_points: Vec<u64> = tables.iter().flat_map(|table| {
        table.mappings.iter()
            .map(|m| vec![m.from, m.from + m.len])
            .flatten()
            .collect::<Vec<_>>()
    }).collect();

    /*
    let mut descision_points: Vec<u64> = first.mappings.iter()
        .map(|m| vec![m.from, m.from + m.len])
        .flatten()
        .chain(second.mappings.iter()
            .map(|m| vec![m.from, m.from + m.len])
            .flatten()
        )
        .collect(); */
    descision_points.sort();
    descision_points.dedup();
    //println!("Descision points: {:?}", descision_points);

    let new_map = descision_points.windows(2)
        .flat_map(|w| {
             let (a,b) = (w[0], w[1]);
             let r = tables.iter().fold(a, |x, table| table.translate(x));

             if a == r {
                 vec![]
             } else {
                 vec![Mapping {
                     to: r,
                     from: a,
                     len: b-a,
                 }]
             }
        }).collect::<Vec<_>>();
    Table::new(new_map)
}

fn part1(input: &str) -> u64 {
    let (seeds,tables) = parse_input(input);
    let combined_table = combine_tables(&tables);
    seeds.iter()
        .map(|seed| combined_table.translate(*seed))
        .min().unwrap()
}

fn part2(input: &str) -> u64 {
    let (seeds,tables) = parse_input(input);
    let combined_table = combine_tables(&tables);
    println!("Seeds: {:?}", seeds);
    println!("Combined table: {:?}", combined_table);


    println!(" 82 -> {}", combined_table.translate(82));
    let mut possible_points = seeds.windows(2).flat_map(|w| {
        let (start,len) = (w[0], w[1]);
        let mut points = Vec::new();
        for mapp in combined_table.mappings.iter() {
            // if seed and mapping overlap
            if start + len >= mapp.from && start < mapp.from + mapp.len {
                // before
                if start < mapp.from {
                    points.push(mapp.from);
                } else {
                    points.push(start);
                }
            } 
        }
        points
    }).collect::<Vec<_>>();
    
    println!("Possible points: {:?}", possible_points);

    0
}

fn test_tables(tables: Vec<Table>) {
    let combined_table = combine_tables(&tables);
    for i in 0..100 {
        let r = tables.iter().fold(i, |x, table| table.translate(x));
        print!("{}, ", r);
    }
    println!("");
    for i in 0..100 {
        print!("{}, ", combined_table.translate(i));
    }
    println!("");
}

fn main() {
    let test_input = read_to_string("inputs/day05-test.txt").unwrap();
    let real_input = read_to_string("inputs/day05.txt").unwrap();

    println!("Part 1 test: {}", part1(&test_input));
    println!("Part 1: {}", part1(&real_input));
    println!("Part 2 test: {}", part2_brute_force(&test_input));
    println!("Part 2: {}", part2_brute_force(&real_input));
}
