use regex::Regex;
use std::fs;

fn parse_input() -> Vec<Vec<usize>> {
    let input = fs::read_to_string("./inputs/04.in").expect("file not found");
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    re.captures_iter(&input)
        .map(|c| {
            c.iter()
                .filter_map(|m| m.unwrap().as_str().parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

fn overlap(p: Vec<usize>) -> usize {
    match (
        p[0] <= p[2] && p[2] <= p[1],
        p[0] <= p[3] && p[3] <= p[1],
        p[2] <= p[0] && p[0] <= p[3],
        p[2] <= p[1] && p[1] <= p[3],
    ) {
        (true, ..) => 1,
        (_, true, ..) => 1,
        (_, _, true, ..) => 1,
        (.., true) => 1,
        _ => 0,
    }
}

fn overlap_full(p: Vec<usize>) -> usize {
    match (p[0] <= p[2] && p[1] >= p[3], p[2] <= p[0] && p[3] >= p[1]) {
        (true, _) => 1,
        (_, true) => 1,
        _ => 0,
    }
}

fn part1() -> usize {
    let pairs = parse_input();
    pairs.iter().map(|p| overlap_full(p.to_vec())).sum()
}

fn part2() -> usize {
    let pairs = parse_input();
    pairs.iter().map(|p| overlap(p.to_vec())).sum()
}

fn main() {
    let p1 = part1();
    let p2 = part2();
    println!("part 1: {:?}", p1);
    println!("part 2: {:?}", p2);
}
