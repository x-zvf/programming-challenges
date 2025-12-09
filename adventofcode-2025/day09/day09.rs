use std::fs;

fn parse(fname: &str) -> Vec<(usize, usize)> {
    fs::read_to_string(fname)
        .unwrap()
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}
fn part1(ps: &Vec<(usize, usize)>) -> usize {
    let mut p1 = 0;
    for i in 0..ps.len() {
        for j in (i + 1)..ps.len() {
            let area = (ps[i].0.abs_diff(ps[j].0) + 1) * (ps[i].1.abs_diff(ps[j].1) + 1);
            p1 = p1.max(area);
        }
    }
    p1
}

fn part2(ps: &Vec<(usize, usize)>) -> usize {
    let lines: Vec<((usize, usize), (usize, usize))> = (0..ps.len())
        .map(|i| (ps[i], ps[(i + 1) % ps.len()]))
        .collect();
    let inside = |xs: usize, ys: usize, xl: usize, yl: usize| -> bool {
        for &((p1x, p1y), (p2x, p2y)) in &lines {
            let (pxs, pys, pxl, pyl) = (p1x.min(p2x), p1y.min(p2y), p1x.max(p2x), p1y.max(p2y));
            // check for crossings
            if (p1x == p2x && (xs < p1x && xl > p1x) && (ys.max(pys) < yl.min(pyl)))
                || (p1y == p2y && (ys < p1y && yl > p1y) && (xs.max(pxs) < xl.min(pxl)))
            {
                return false;
            }
        }
        // raycast center (0.5 for odd)
        let (cx, cy) = ((xs + xl) as f64 / 2.0, (ys + yl) as f64 / 2.0);
        let mut cross = 0;
        for &((p1x, p1y), (_, p2y)) in &lines {
            if p1y == p2y {
                continue;
            }
            if p1x as f64 > cx && cy > p1y.min(p2y) as f64 && cy < p1y.max(p2y) as f64 {
                cross += 1;
            }
        }
        cross % 2 == 1
    };
    let mut p2 = 0;
    for i in 0..ps.len() {
        let (x1, y1) = ps[i];
        for j in (i + 1)..ps.len() {
            let (x2, y2) = ps[j];
            //assumption: the largest rect is not going to be a 1*x line
            if x1 == x2 || y1 == y2 {
                continue;
            }
            let (xs, ys, xl, yl) = (x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2));

            let area = (xl - xs + 1) * (yl - ys + 1);
            if area > p2 && inside(xs, ys, xl, yl) {
                p2 = area;
            }
        }
    }
    p2
}

fn main() {
    let sample = parse("./sample.txt");
    let input = parse("./input.txt");
    println!("part1: {} {}", part1(&sample), part1(&input));
    println!("part2: {} {}", part2(&sample), part2(&input));
}
