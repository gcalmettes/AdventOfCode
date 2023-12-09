use itertools::Itertools;

fn compute_deltas(series: &Vec<isize>) -> Vec<isize> {
    series
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect::<Vec<isize>>()
}

fn predict_history(series: &Vec<isize>, reverse: bool) -> isize {
    let mut layers = vec![series.to_vec()];
    let mut deltas = compute_deltas(&series);
    while deltas.iter().any(|d| d != &0) {
        layers.push(deltas.clone());
        deltas = compute_deltas(&deltas);
    }

    layers.iter().rev().fold(0, |acc, s| {
        match reverse {
            false => acc + s[s.len() - 1], // predict in future
            true => s[0] - acc,            // predict in past
        }
    })
}

#[aoc::main()]
fn main(input: &str) -> (isize, isize) {
    let series = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|d| d.parse::<isize>().ok().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<Vec<isize>>>();

    let p1 = series.iter().map(|s| predict_history(s, false)).sum();
    let p2 = series.iter().map(|s| predict_history(s, true)).sum();
    (p1, p2)
}
