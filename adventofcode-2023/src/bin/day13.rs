use std::fs::read_to_string;
type Grid = Vec<Vec<char>>;

fn parse(input: &str) -> Vec<Grid> {
    input.split("\n\n").map(|s| {
        s.lines().map(|l| l.chars().collect()).collect()
    }).collect()
}
fn symmetry_axes(grid: &Grid) -> (Vec<usize>, Vec<usize>) {
    let transposed: Grid = (0..grid[0].len()).map(|i| {
        grid.iter().map(|row| row[i]).collect()
    }).collect();
    let find_symmetry_axis = |g: &Grid| {
        (0..g.len()-1)
            .filter(|&i| (0..=i.min(g.len() - i - 2))
                    .all(|r| g[i - r] == g[i + r + 1]))
            .map(|i| i+1)
            .collect()
    };
    (find_symmetry_axis(&transposed), find_symmetry_axis(grid))
}

fn summarize((v, h): (&Vec<usize>, &Vec<usize>), not: usize) -> usize {
    v.iter().map(|x| *x)
        .chain(h.iter().map(|i| i * 100))
        .filter(|i| *i != not)
        .sum()
}

fn part1(input: &str) -> usize {
    parse(input).iter()
        .map(symmetry_axes)
        .map(|(v, h)| summarize((&v, &h), 0))
        .sum()
}

fn part2(input: &str) -> usize {
    let mut grids = parse(input);
    let smudged_grid = |grid: &mut Grid| {
        let (ov, oh) = symmetry_axes(grid);
        let os = summarize((&ov, &oh), 0);
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                let oc = grid[y][x];
                grid[y][x] = if oc == '#' { '.' } else { '#' };
                let nes = symmetry_axes(grid);
                let ns = summarize((&nes.0, &nes.1), os);
                if ns != 0 {
                    return ns;
                }
                grid[y][x] = oc;
            }
        }
        0
    };
    grids.iter_mut().map(|g| smudged_grid(g)).sum()

}


fn main() {
    let test_input = read_to_string("inputs/day13-test.txt").unwrap();
    let real_input = read_to_string("inputs/day13.txt").unwrap();
    println!("Part 1 test: {}", part1(&test_input));
    println!("Part 1: {}",      part1(&real_input));
    println!("Part 2 test: {}", part2(&test_input));
    println!("Part 2: {}",      part2(&real_input));
}
