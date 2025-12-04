struct Grid {
    cells: Vec<Vec<bool>>,
    w: usize,
    h: usize,
}

impl Grid {
    fn parse(s: &str) -> Self {
        let cs: Vec<Vec<bool>> = s
            .trim()
            .split("\n")
            .map(|line| line.chars().map(|c| c == '@').collect::<Vec<bool>>())
            .filter(|line| !line.is_empty())
            .collect();
        Grid {
            h: cs.len(),
            w: cs[0].len(),
            cells: cs,
        }
    }
    fn is_paper(&self, x: usize, y: usize) -> bool {
        self.cells[y][x]
    }
    fn is_accessible(&self, x: usize, y: usize) -> bool {
        self.is_paper(x, y) && self.neighbours_count(x, y) < 4
    }
    fn neighbours_count(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        let a = y as i64;
        let b = x as i64;
        for i in (a - 1)..=(a + 1) {
            if i < 0 || i >= self.h as i64 {
                continue;
            }
            for j in (b - 1)..=(b + 1) {
                if j < 0 || j >= self.w as i64 || (j == x as i64 && i == y as i64) {
                    continue;
                }
                if self.cells[i as usize][j as usize] {
                    count += 1
                }
            }
        }
        count
    }
    fn part1(&self) -> usize {
        let mut res = 0;
        for i in 0..self.h {
            for j in 0..self.w {
                if self.is_accessible(j, i) {
                    res += 1
                }
            }
        }
        res
    }
    fn part2(&mut self) -> usize {
        let mut removed = 0;
        let mut prev_removed = 0;
        loop {
            for i in 0..self.h {
                for j in 0..self.w {
                    if self.is_accessible(j, i) {
                        removed += 1;
                        self.cells[i][j] = false;
                    }
                }
            }
            if removed == prev_removed {
                break;
            }
            prev_removed = removed;
        }

        removed
    }
}

fn main() {
    let mut grid: Grid =
        Grid::parse(&std::fs::read_to_string(std::env::args().last().unwrap()).unwrap());
    println!("{} {}", grid.part1(), grid.part2());
}
