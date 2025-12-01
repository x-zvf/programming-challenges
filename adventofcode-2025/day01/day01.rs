use std::env;
use std::fs::read_to_string;
use std::io;

#[derive(Debug)]
enum Rotation {
    Left(i16),
    Right(i16),
}

impl Rotation {
    fn rotate(&self, sv: i16) -> (i16, i16) {
        let nv = match self {
            Self::Left(x) => sv - *x,
            Self::Right(x) => sv + *x,
        }
        .rem_euclid(100);

        let full_rotations = match self {
            Self::Left(x) => x / 100,
            Self::Right(x) => x / 100,
        };
        let signdelta = match self {
            Self::Left(_) if nv > sv && sv != 0 => 1,
            Self::Right(_) if nv < sv => 1,
            _ if sv != 0 && nv == 0 => 1,
            _ => 0,
        };
        let zc = full_rotations + signdelta;
        (nv, zc)
    }
    fn from_str(s: &str) -> Rotation {
        let v: i16 = s[1..].parse().unwrap();
        if s[0..1] == *"L" {
            Rotation::Left(v)
        } else {
            Rotation::Right(v)
        }
    }
    fn parse(file: &str) -> io::Result<Vec<Self>> {
        Ok(read_to_string(file)?.lines().map(Self::from_str).collect())
    }
}

fn solve(file: &str) -> (i16, i16) {
    let (_, part1, part2) =
        Rotation::parse(file)
            .unwrap()
            .iter()
            .fold((50, 0, 0), |(st, nz, zc), r| {
                let (nst, cross) = r.rotate(st);
                let nnz = if nst == 0 { nz + 1 } else { nz };
                (nst, nnz, cross + zc)
            });
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
        let (nv, cr) = x.rotate(50);
        assert_eq!(nv, 50);
        assert_eq!(cr, 10);
    }
}
