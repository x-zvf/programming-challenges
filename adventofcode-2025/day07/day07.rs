use std::collections::HashMap;
use std::env;
use std::fs;

type Grid = Vec<Vec<Cell>>;
type MEMO = HashMap<(usize, usize), usize>;

#[derive(Eq, Debug, Clone, Copy, PartialEq)]
enum Cell {
    Free,
    Start,
    Splitter,
    Beam,
}
impl Cell {
    fn from(c: &char) -> Self {
        match c {
            '.' => Cell::Free,
            'S' => Cell::Start,
            '^' => Cell::Splitter,
            '|' => panic!("input shouldn't contain Beams"),
            _ => panic!("bad input"),
        }
    }
}

fn part1(mut grid: Grid) -> usize {
    let mut splitcount = 0;
    let setbeam = |grid: &mut Grid, y: isize, x: isize| -> bool {
        if y >= grid.len() as isize || x < 0 || x >= grid[y as usize].len() as isize {
            return false;
        }
        let x: usize = x as usize;
        let y: usize = y as usize;
        if grid[y][x] == Cell::Splitter || grid[y][x] == Cell::Start {
            println!("Attempting to place Beam upon Splitter/Start at y, x = {y}, {x}");
            panic!("not handled");
        }
        if grid[y][x] == Cell::Beam {
            return false;
        }

        grid[y][x] = Cell::Beam;
        return true;
    };
    for i in 1..grid.len() {
        for j in 0..grid[i].len() {
            let oc = grid[i - 1][j];
            let cc = grid[i][j];
            if cc == Cell::Beam {
                continue;
            }
            if oc == Cell::Free {
                continue;
            }
            if oc == Cell::Splitter && cc == Cell::Free {
                continue;
            }
            if oc == Cell::Start && cc == Cell::Free {
                grid[i][j] = Cell::Beam;
                continue;
            }
            if (oc == Cell::Start || oc == Cell::Beam) && cc == Cell::Splitter {
                let a = setbeam(&mut grid, i as isize, j as isize - 1);
                let b = setbeam(&mut grid, i as isize, j as isize + 1);
                if a || b {
                    splitcount += 1;
                }
                continue;
            }
            if oc == Cell::Beam && cc == Cell::Free {
                setbeam(&mut grid, i as isize, j as isize);
                continue;
            }
        }
    }
    splitcount
}

fn main() {
    let fname = env::args().last().unwrap();
    let grid: Vec<Vec<Cell>> = fs::read_to_string(fname)
        .unwrap()
        .split("\n")
        .map(|line| line.chars().map(|c| Cell::from(&c)).collect())
        .collect();
    let p1 = part1(grid.clone());
    println!("part1: {p1}");

    let mut memo = std::collections::HashMap::<(usize, usize), usize>::new();

    fn npaths(memo: &mut MEMO, grid: &Grid, y: usize, x: usize) -> usize {
        if y >= grid.len() || grid[y].len() == 0 {
            return 1;
        }
        if let Some(n) = memo.get(&(y, x)) {
            return *n;
        }
        let c = grid[y][x];
        let mut sum = 0;
        if c == Cell::Splitter {
            if x > 0 {
                sum += npaths(memo, grid, y, x - 1);
            }
            if x < grid[y].len() - 1 {
                sum += npaths(memo, grid, y, x + 1);
            }
        } else {
            sum = npaths(memo, grid, y + 1, x);
        }
        memo.insert((y, x), sum);
        return sum;
    }

    let mut sx = usize::max_value();
    for i in 0..grid[0].len() {
        if grid[0][i] == Cell::Start {
            sx = i;
        }
    }

    let p2 = npaths(&mut memo, &grid, 0, sx);
    println!("part2: {p2}");
}
