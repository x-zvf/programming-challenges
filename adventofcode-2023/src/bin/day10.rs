use std::fs::read_to_string;
use colored::*;
use std::collections::HashSet;

fn edges(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let c = grid[y][x];
    let (x, y) = (x as isize, y as isize);

    let candidates = match c {
        '.' => vec![],
        '|' => vec![(x, y-1), (x, y+1)],
        '-' => vec![(x-1, y), (x+1, y)],
        'L' => vec![(x, y-1), (x+1, y)],
        'J' => vec![(x, y-1), (x-1, y)],
        '7' => vec![(x, y+1), (x-1, y)],
        'F' => vec![(x, y+1), (x+1, y)],
        'S' => panic!("Starting point"),
        _ => panic!("Unknown character: {}", c),
    };
    candidates.into_iter()
        .filter(|(x, y)| x >= &0 && y >= &0
                && *x < (grid[0].len() as isize) && *y < (grid.len() as isize))
        .map(|(x, y)| (x as usize, y as usize))
        .collect::<Vec<_>>()
}

fn parse(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let starting_point: (usize, usize) = grid.iter().enumerate().find_map(|(y, row)| {
        row.iter().enumerate().find_map(|(x, c)| {
            if *c == 'S' {
                Some((x, y))
            } else {
                None
            }
        })
    }).unwrap();
    (grid, starting_point)
}

fn get_loop(grid: &Vec<Vec<char>>, starting_point: (usize, usize)) -> Vec<(usize, usize)> {
    // DFS until we reach 'S' again
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    //one down from starting point, as S is always up or down
    // so manually taking the first step
    let mut current = (starting_point.0, starting_point.1 + 1);
    let mut path = vec![];

    visited[starting_point.1][starting_point.0] = true; // hacky
    path.push(current);

    while grid[current.1][current.0] != 'S' {
        visited[current.1][current.0] = true;
        let next = edges(&grid, current.0, current.1).into_iter()
            .filter(|(x, y)| !visited[*y][*x])
            .next()
            .unwrap_or(
                // last step, only S is left
                edges(&grid, current.0, current.1).into_iter()
                    .filter(|(x, y)| !visited[*y][*x] || grid[*y][*x] == 'S')
                    .next().unwrap());

        current = next;
        path.push(current);
    }
    path
}

fn print_points(width: usize, height: usize, points: &Vec<(usize, usize, ColoredString)>) {
    println!("{}"," ".repeat(width+2).on_white());
    let mut grid = vec![vec![" ".on_black(); width]; height];
    for (x, y, c) in points {
        grid[*y][*x] = c.clone();
    }
    for row in grid {
        print!("{}", " ".on_white());
        for c in row {
            print!("{}", c);
        }
        println!("{}", "|".on_white());

    }

    println!("{}","-".repeat(width+2).on_white());
}

fn print_path(width: usize, height: usize, path: &Vec<(usize, usize)>) {
    let mut points = path
        .windows(2)
        .map(|w| (w[0], w[1]))
        .map(|((xc,yc),(xn,yn))| {
            let c = match (xc as isize - xn as isize, yc as isize - yn as isize) {
                (0, -1) => "v".blue(),
                (0, 1) => "^".blue(),
                (-1, 0) => ">".blue(),
                (1, 0) => "<".blue(),
                _ => panic!("Unknown direction"),
            };
            (xc, yc, c)
        })
     .collect::<Vec<_>>();
    let last = path.last().unwrap();
    points.push((last.0, last.1, "S".red()));
    print_points(width, height, &points);
}

fn part1(input: &str) -> i64 {
    let (grid, starting_point) = parse(input);
    let path = get_loop(&grid, starting_point);
    path.len() as i64 / 2
}

fn part2(input: &str) -> i64 {
    let (grid, starting_point) = parse(input);
    let path = get_loop(&grid, starting_point);
    print_path(grid[0].len(), grid.len(), &path);

    let mut left_of_path: Vec<(isize, isize)> = vec![];
    let mut right_of_path = vec![];
    let mut current = path.last().unwrap().clone();
    current.1 += 1; // one down from starting point, as S is always up or down
    let mut direction = (0, 1);

    for (nx, ny) in path.iter().skip(1) {
        let c = grid[current.1][current.0];
        //println!("Current: {:?} Next: {:?} Char: {}", current, (nx, ny), c);
        let (cxi, cyi) = (current.0 as isize, current.1 as isize);
        match c {

            '|' =>{
                match direction {
                    (0, -1) => {
                        left_of_path.push((cxi-1, cyi));
                        right_of_path.push((cxi+1, cyi));
                    }
                    (0, 1) => {
                        right_of_path.push((cxi-1, cyi));
                        left_of_path.push((cxi+1, cyi));
                    }
                    _ => panic!("Unexpected direction: {:?}", direction),
                }
            },
            '-' =>{
                match direction {
                    (-1, 0) =>{
                        left_of_path.push((cxi, cyi+1));
                        right_of_path.push((cxi, cyi-1));
                    }
                    (1, 0) => {
                        right_of_path.push((cxi, cyi+1));
                        left_of_path.push((cxi, cyi-1));
                    }
                    _ => panic!("Unexpected direction: {:?}", direction),
                }
            },
            'L' =>{
                match direction {
                    (0, 1) => {
                        right_of_path.push((cxi - 1, cyi));
                        right_of_path.push((cxi, cyi + 1));
                    },
                    (-1, 0) => {
                        left_of_path.push((cxi, cyi + 1));
                        left_of_path.push((cxi - 1, cyi));
                    },
                    _ => panic!("Unexpected direction: {:?}", direction),
                }
            },
            'J' =>{
                match direction {
                    (0, 1) => {
                        left_of_path.push((cxi + 1, cyi));
                        left_of_path.push((cxi, cyi + 1));
                    },
                    (1, 0) => {
                        right_of_path.push((cxi, cyi + 1));
                        right_of_path.push((cxi + 1, cyi));
                    },
                    _ => panic!("Unexpected direction: {:?}", direction),
                }
            },
            '7' =>{
                match direction {
                    (0, -1) => {
                        right_of_path.push((cxi + 1, cyi));
                        right_of_path.push((cxi, cyi - 1));
                    },
                    (1, 0) => {
                        left_of_path.push((cxi, cyi - 1));
                        left_of_path.push((cxi + 1, cyi));
                    },
                    _ => panic!("Unexpected direction: {:?}", direction),
                }
            },
            'F' =>{
                match direction {
                    (0, -1) => {
                        left_of_path.push((cxi - 1, cyi));
                        left_of_path.push((cxi, cyi - 1));
                    },
                    (-1, 0) => {
                        right_of_path.push((cxi, cyi - 1));
                        right_of_path.push((cxi - 1, cyi));
                    },
                    _ => panic!("Unexpected direction: {:?}", direction),
                }
            },
            _ => panic!("Unexpected character: {}", c),

        };


        let (dx, dy) = (*nx as isize - cxi, *ny as isize - cyi);
        let new_direction = match (dx, dy) {
            (0, -1) => (0, -1),
            (0, 1) => (0, 1),
            (-1, 0) => (-1, 0),
            (1, 0) => (1, 0),
            _ => panic!("Unknown direction: {:?} {}", (dx, dy), c),
        };
        direction = new_direction;
        current = (*nx, *ny);
    }
    let filter_points = |points: &Vec<(isize,isize)>| {
        points.iter()
            .map(|(x, y)| (*x as isize, *y as isize))
            .filter(|(x, y)| *x >= 0 && *y >= 0
                    && *x < (grid[0].len() as isize) && *y < (grid.len() as isize))
            .filter(|p| !path.contains(&(p.0 as usize, p.1 as usize)))
            .collect::<Vec<_>>()
    };


    left_of_path = filter_points(&left_of_path);
    right_of_path = filter_points(&right_of_path);
    //print_points(grid[0].len(), grid.len(), &left_of_path.iter().map(|(x, y)| (*x as usize, *y as usize, "L".green())).collect::<Vec<_>>());
    //print_points(grid[0].len(), grid.len(), &right_of_path.iter().map(|(x, y)| (*x as usize, *y as usize, "R".red())).collect::<Vec<_>>());

    let flood_fill_from_point = |(x, y): (isize, isize)| {
//        println!("Flood fill from ({}, {})", x, y);
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        visited[y as usize][x as usize] = true;
        let mut queue = vec![(x, y)];
        while let Some((x, y)) = queue.pop() {
            if x == 0 || y == 0 || x == (grid[0].len() as isize - 1) || y == (grid.len() as isize - 1) {
                return HashSet::new(); // reached outside
            }
            //print!("@ ({}, {}) visiting neighbours =>", x, y);
            let neighbours = vec![(x, y-1), (x, y+1), (x-1, y), (x+1, y)].into_iter()
                .map(|(x, y)| {
                    //print!(" ({}, {}): visited={},grid={} |", x, y, visited[y as usize][x as usize], grid[y as usize][x as usize]);
                    (x, y)})
                .filter(|(x, y)|
                        (!visited[*y as usize][*x as usize])
                        && !path.contains(&(*x as usize, *y as usize)))
                .collect::<Vec<_>>();
            //println!();
            for (nx, ny) in neighbours {
                //println!("@ ({}, {}) visiting ({}, {})", x, y, nx, ny);
                visited[ny as usize][nx as usize] = true;
                queue.push((nx, ny));
            }
        }
        let filled_points_set = visited.into_iter()
            .enumerate()
            .flat_map(|(y, row)| row.into_iter().enumerate().map(move |(x, v)| (x, y, v)))
            .filter(|(_, _, v)| *v)
            .map(|(x, y, _)| (x, y))
            .collect::<HashSet<_>>();
        filled_points_set
    };


    let flood_all = |points: &Vec<(isize, isize)>| {
        let mut filled_points = HashSet::new();
        for (x, y) in points {
            let new = flood_fill_from_point((*x, *y));
            //println!("=====> Filled point {:?}: {:?}", (x, y), new);
            if new.is_empty() {
                return HashSet::new();
            }
            filled_points = filled_points.union(&new).map(|(x, y)| (*x, *y)).collect::<HashSet<_>>();
        }
        filled_points
    };
    //println!("Left: {:?}", left_of_path);
    //println!("Right: {:?}", right_of_path);
    let left_filled = flood_all(&left_of_path);
    //println!("=============== RIGHT ===============");
    let right_filled = flood_all(&right_of_path);

    //print_points(grid[0].len(), grid.len(), &left_filled.iter().map(|(x, y)| (*x as usize, *y as usize, "F".yellow())).collect::<Vec<_>>());
    //print_points(grid[0].len(), grid.len(), &right_filled.iter().map(|(x, y)| (*x as usize, *y as usize, "X".red())).collect::<Vec<_>>());

    if left_filled.is_empty() && right_filled.is_empty() {
        panic!("No solution found");
    }
    let inner = if left_filled.is_empty() {
        right_filled
    } else {
        left_filled
    };
    print_points(grid[0].len(), grid.len(), &inner.iter().map(|(x, y)| (*x as usize, *y as usize, "F".yellow())).collect::<Vec<_>>());

    inner.len() as i64
}


fn main() {
    let test_input = read_to_string("inputs/day10-test.txt").unwrap();
    let test_input2 = read_to_string("inputs/day10-test2.txt").unwrap();
    let test_input3 = read_to_string("inputs/day10-test3.txt").unwrap();
    let test_input4 = read_to_string("inputs/day10-test4.txt").unwrap();
    let real_input = read_to_string("inputs/day10.txt").unwrap();

    println!("Part 1 test: {}", part1(&test_input));
    println!("Part 1: {}", part1(&real_input));
    println!("Part 2 test 1: {}", part2(&test_input));
    println!("Part 2 test 2: {}", part2(&test_input2));
    println!("Part 2 test 3: {}", part2(&test_input3));
    println!("Part 2 test 4: {}", part2(&test_input4));
    println!("Part 2: {}", part2(&real_input));
}
