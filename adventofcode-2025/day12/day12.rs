use std::fs;

fn parse(fname: &str) -> (Vec<usize>, Vec<((usize, usize), Vec<usize>)>) {
    let s = fs::read_to_string(fname).unwrap();
    let blocks: Vec<&str> = s.split("\n\n").collect();
    let areas = blocks
        .iter()
        .take(blocks.len() - 1)
        .map(|b| b.chars().filter(|&c| c == '#').count())
        .collect();
    let qs = blocks
        .last()
        .unwrap()
        .split("\n")
        .filter_map(|l| {
            let x = l.trim();
            if x.is_empty() { None } else { Some(x) }
        })
        .map(|line| {
            let (l, r) = line.split_once(": ").unwrap();
            let (w, h) = l.split_once("x").unwrap();

            let counts = r.trim().split(" ").map(|x| x.parse().unwrap()).collect();

            (
                (
                    w.trim().parse::<usize>().unwrap(),
                    h.trim().parse::<usize>().unwrap(),
                ),
                counts,
            )
        })
        .collect();

    (areas, qs)
}

fn solve(presents: &Vec<usize>, regions: &Vec<((usize, usize), Vec<usize>)>) -> usize {
    let mut res = 0;
    for ((w, h), ps) in regions.iter() {
        let npresents: usize = ps.iter().sum();
        let conservative_area = (w / 3) * (h / 3);
        if conservative_area >= npresents {
            res += 1; //a presents fits into a 3x3 area
            continue;
        }
        let minspace: usize = ps.iter().enumerate().map(|(i, c)| presents[i] * c).sum();
        if w * h < minspace {
            continue; // it definitely can't fit
        }
        println!("need to check region: [{:?}]", ((w, h), ps));
    }
    res
}
fn main() {
    let (sp, sr) = parse("./sample.txt");
    let (ip, ir) = parse("./input.txt");
    println!("{} {}", solve(&sp, &sr), solve(&ip, &ir));
}
