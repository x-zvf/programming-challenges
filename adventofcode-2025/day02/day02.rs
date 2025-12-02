use std::collections::HashSet;
use std::env;
use std::fs;

struct Range {
    min: i64,
    max: i64,
}

fn ndigits(x: i64) -> i64 {
    ((x as f64).log10().floor() as i64) + 1
}

fn power(b: i64, p: i64) -> i64 {
    if p == 0 {
        return 1;
    } else if p == 1 {
        return b;
    } else if p % 2 == 0 {
        return power(b * b, p / 2);
    } else {
        return b * power(b, p - 1);
    }
}

fn nines(n: i64) -> i64 {
    let mut sum = 0;
    for _ in 0..n {
        sum *= 10;
        sum += 9;
    }
    sum
}

fn palindrome_minmax(x: i64) -> (i64, i64) {
    let nd = ndigits(x);
    let hl = nd / 2;
    let dm = power(10, hl);
    let tn = x / dm;
    let bm = x % dm;

    if nd % 2 == 1 {
        (dm, nines(hl))
    } else if tn < bm {
        (tn + 1, tn)
    } else if tn == bm {
        (tn, tn)
    } else {
        (tn, tn - 1)
    }
}

fn palindrome_of(x: i64) -> i64 {
    power(10, ndigits(x)) * x + x
}

impl Range {
    fn from_str(s: &str) -> Self {
        let xs: Vec<i64> = s
            .split("-")
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect();
        Range {
            min: xs[0],
            max: xs[1],
        }
    }
    // yes, I known these aren't palindromes, but I've accidentally called
    // them that everywhere
    fn sum_double_palindromes(&self) -> i64 {
        let (x, _) = palindrome_minmax(self.min);
        let (_, y) = palindrome_minmax(self.max);
        (x..=y).map(palindrome_of).sum()
    }
    fn sum_all_palindromes(&self) -> i64 {
        let (_, mn) = palindrome_minmax(self.max);
        let mut ps: HashSet<i64> = Default::default();
        for x in 1..=mn {
            let mut v = x;
            let mult = power(10, ndigits(x));
            loop {
                v = mult * v + x;
                if v > self.max {
                    break;
                }
                if v >= self.min {
                    ps.insert(v);
                }
            }
        }
        ps.iter().sum::<i64>()
    }
}

fn main() {
    let fpath = env::args().last().unwrap();
    let input: Vec<Range> = fs::read_to_string(fpath)
        .unwrap()
        .split(",")
        .map(|s| Range::from_str(s))
        .collect();

    let part1: i64 = input.iter().map(|x| x.sum_double_palindromes()).sum();
    println!("{part1}");
    let part2: i64 = input.iter().map(|x| x.sum_all_palindromes()).sum();
    println!("{part2}");
}
