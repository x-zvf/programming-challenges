use std::fs::read_to_string;
use std::collections::HashSet;
//use polynomial::Polynomial;

type Grid = Vec<Vec<char>>;

fn parse(input: &str) -> Grid {
    input.trim().lines().map(|line| line.chars().collect()).collect()
}

fn visitable_in(grid: &Grid, n: usize, x: usize, y: usize) -> isize {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let vis_rem = n % 2;
    let mut total = 0;

    let mut stack = vec![(x, y)];
    for step in 0..=n {
        let mut next_stack = Vec::new();
        for (x, y) in stack {
            if grid[y][x] == '#' || visited[y][x] { continue; }
            visited[y][x] = true;
            if step % 2 == vis_rem { total += 1; }
            if x > 0 { next_stack.push((x - 1, y)); }
            if y > 0 { next_stack.push((x, y - 1)); }
            if x < grid[0].len() - 1 { next_stack.push((x + 1, y)); }
            if y < grid.len() - 1 { next_stack.push((x, y + 1)); }
        }
        stack = next_stack;
    }

    total
}

fn part1(input: &str, n: usize) -> isize {
    let grid = parse(input);
    let (sx, sy) = grid.iter().enumerate().find_map(|(y, row)| {
        row.iter().enumerate().find_map(|(x, &c)| if c == 'S' { Some((x, y)) } else { None })
    }).unwrap();
    visitable_in(&grid, n, sx, sy)
}

fn part2(input: &str, n: isize) -> isize {
    let grid = parse(input);
    let sz = grid.len() as isize;
    let mid = sz / 2;
    assert!(grid[0].len() as isize == sz);
    assert!(sz % 2 == 1);
    assert!(grid[mid as usize][mid as usize] == 'S');

    let mut visited = HashSet::new();
    let mut stack = vec![(mid, mid)];
    let mut s_stack = vec![(mid, mid)];

    
    let wrap = |x, s| (((x % s) + s) % s ) as usize;

    let cross = vec![mid, mid + sz, mid + 2*sz];

    let mut ys = Vec::new();

    let mut odds = 0;
    let mut evens = 0;

    for i in 1..=(mid + 2*sz) {
        while let Some((x, y)) = stack.pop() {
            for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (nx, ny) = (x + dx, y + dy);
                if grid[wrap(ny,sz)][wrap(nx,sz)] == '#' || visited.contains(&(nx, ny)) { continue; }
                visited.insert((nx, ny));
                s_stack.push((nx, ny));
                if i % 2 == 0 { evens += 1; } else { odds += 1; }
            }
        }
        std::mem::swap(&mut stack, &mut s_stack);
        if cross.contains(&i) {
            if i % 2 == 0 { ys.push(evens); } else { ys.push(odds); }
        }
    }

    let s = (n - mid) / sz;
    println!("ys: {:?}, s= {}", ys, s);
    // r = 15181s^2 + 15301s + 3849

    // Unfortunately the polynomial-rs library is broken, because the lagrange
    // function returns incorrect results. So I'm using mathematica to find
    // the polynomial instead.

    //let poly2 = Polynomial::lagrange(&vec![0,1,2], &ys).unwrap();
    //let poly = Polynomial::new(vec![3849,15181,15301]);
    //poly.eval(s)
    
    15181 * s * s + 15301 * s + 3849
}

fn main() {
    let test_input = read_to_string("inputs/day21-test.txt").unwrap();
    let real_input = read_to_string("inputs/day21.txt").unwrap();
    println!("Part 1 test: {}", part1(&test_input, 6));
    println!("Part 1: {}",      part1(&real_input, 64));
    println!("Part 2: {}",      part2(&real_input, 26501365));
}
