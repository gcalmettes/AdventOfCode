use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashSet;

fn parse_grid(input: &str) -> HashSet<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(c, ch)| (ch == '#').then(|| (c, r)))
        })
        .collect::<HashSet<(usize, usize)>>()
}

fn parts(galaxies: &HashSet<(usize, usize)>) -> (usize, usize) {
    let (cols, rows) = galaxies
        .iter()
        .fold((HashSet::new(), HashSet::new()), |mut acc, g| {
            acc.0.insert(g.0);
            acc.1.insert(g.1);
            acc
        });

    let empty_cols = (0..*cols.iter().max().unwrap())
        .filter(|c| !cols.contains(c))
        .collect::<HashSet<usize>>();
    let empty_rows = (0..*rows.iter().max().unwrap())
        .filter(|r| !rows.contains(r))
        .collect::<HashSet<usize>>();

    galaxies
        .iter()
        .tuple_combinations()
        .fold((0, 0), |acc, (g1, g2)| {
            let (min_c, max_c) = (min(g1.0, g2.0), max(g1.0, g2.0));
            let (min_r, max_r) = (min(g1.1, g2.1), max(g1.1, g2.1));
            let empty_c_in_between = empty_cols
                .intersection(&HashSet::from_iter(min_c..max_c))
                .count();
            let empty_r_in_between = empty_rows
                .intersection(&HashSet::from_iter(min_r..max_r))
                .count();
            let steps = (max_r - min_r) + (max_c - min_c);
            let empty = empty_r_in_between + empty_c_in_between;

            (acc.0 + steps + empty, acc.1 + steps + empty * (1000000 - 1))
        })
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let galaxies = parse_grid(input);
    let (p1, p2) = parts(&galaxies);
    (p1, p2)
}
