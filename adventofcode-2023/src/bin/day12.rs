use std::fs::read_to_string;
use itertools::Itertools;

fn parse(input: &str) -> Vec<(Vec<char>, Vec<usize>)> {
    input.lines().map(|line| {
        let mut parts = line.split(" ");
        let left = parts.next().unwrap();
        let right = parts.next().unwrap();
        let conditions = left.chars().collect::<Vec<char>>();
        let groups = right.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        (conditions, groups)
    }).collect()
}

fn solve(rows:  Vec<(Vec<char>, Vec<usize>)>) -> isize {
    let mut total = 0;
    for (conditions, damaged_groups) in rows {
        let x = conditions.len(); // positions
        let y = damaged_groups.len(); // groups
        let z = x; // counts taken
        
        let mut dp = vec![vec![vec![0; x+1]; y+1]; z+1];
        dp[x][y][0] = 1; //end
        dp[x][y-1][damaged_groups[y-1]] = 1; // last damage
        for i in (0..x).rev() {
            for j in 0..y {
                for k in 0..=damaged_groups[j] {
                    if conditions[i] == '.' || conditions[i] == '?' {
                        if k == 0 {
                            dp[i][j][k] += dp[i+1][j][0];
                        } else if j < y && damaged_groups[j] == k {
                            dp[i][j][k] += dp[i+1][j + 1][0];
                        }
                    }
                    if conditions[i] == '#' || conditions[i] == '?' {
                        dp[i][j][k] += dp[i+1][j][k+1];
                    }

                }
            }
            if conditions[i] != '#' {
                dp[i][y][0] += dp[i+1][y][0];
            }
        }
        let res = dp[0][0][0];
        total += res;
    }
    total
}

fn part1(input: &str) -> isize {
    solve(parse(input))
}

fn part2(input: &str) -> isize {
    solve(parse(input)
          .into_iter()
          .map(|(conditions, damaged_groups)| {
              let filler = vec!['?'];
              let expanded_c: Vec<_> = [&conditions;5]
                  .into_iter()
                  .intersperse(&filler)
                  .flatten()
                  .map(|x| *x)
                  .collect();
              let expanded_d = damaged_groups.repeat(5); 
              (expanded_c, expanded_d)
          })
          .collect())
}

fn main() {
    let test_input = read_to_string("inputs/day12-test.txt").unwrap();
    let real_input = read_to_string("inputs/day12.txt").unwrap();

    println!("Part 1 test: {}", part1(&test_input));
    println!("Part 1: {}",      part1(&real_input));
    println!("Part 2 test: {}", part2(&test_input));
    println!("Part 2: {}",      part2(&real_input));
}
