use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Rotation {
    Left(u16),
    Right(u16),
}

impl Rotation {
    fn rotate(&self, start: u16) -> (u16, u16) {
        let s = start as i16;
        let n = match self {
            Self::Left(x) => s - *x as i16,
            Self::Right(x) => s + *x as i16,
        };
        let nrots = match self {
            Self::Left(x) => x / 100,
            Self::Right(x) => x / 100,
        };

        let nv = n.rem_euclid(100);
        let cr = nrots
            + if let Self::Left(x) = self
                && s != 0
                && (x % 100) > start
            {
                1
            } else if let Self::Right(x) = self
                && (start + x % 100) > 100
            {
                1
            } else if s == 0 {
                1
            } else {
                0
            };

        (cr as u16, nv as u16)
    }
    fn parse(file: &str) -> io::Result<Vec<Self>> {
        println!("file: {file}");
        let mut res: Vec<Self> = vec![];
        for line in io::BufReader::new(File::open(file)?).lines() {
            let l = line.unwrap();
            let v: u16 = l[1..].parse().unwrap();
            res.push(if l[0..1] == *"L" {
                Rotation::Left(v)
            } else {
                Rotation::Right(v)
            });
        }
        Ok(res)
    }
}

fn solve(file: &str) -> (u16, u16) {
    let rs = Rotation::parse(file).unwrap();
    let mut st = 50;
    let mut part1 = 0;
    let mut part2 = 0;
    for r in rs.iter() {
        let (cross, nst) = r.rotate(st);
        st = nst;
        part2 += cross;
        if st == 0 {
            part1 += 1;
        }
    }
    (part1, part2)
}

fn main() {
    let mut args = env::args();
    if args.len() < 2 {
        println!("usage: ./day01 <file>");
        return;
    }
    let rs = solve(&args.nth(1).unwrap());
    println!("{rs:?}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let r = solve("./sample.txt");
        assert_eq!(r, (3, 6))
    }
    #[test]
    fn realsol() {
        let r = solve("./input.txt");
        assert_eq!(r, (1059, 6305))
    }

    #[test]
    fn rlarge() {
        let x = Rotation::Right(1000);
        let (cr, nv) = x.rotate(50);
        assert_eq!(cr, 10);
        assert_eq!(nv, 50);
    }
}
