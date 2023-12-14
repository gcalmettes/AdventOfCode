use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Mirror {
    pattern: HashMap<(usize, usize), char>,
}

impl Mirror {
    fn get_col(&self, n: usize) -> Vec<char> {
        self.pattern
            .iter()
            .filter(|((c, _r), _v)| c == &n)
            .sorted_unstable_by_key(|((_c, r), _v)| r)
            .map(|(_, v)| *v)
            .collect()
    }

    fn get_row(&self, n: usize) -> Vec<char> {
        self.pattern
            .iter()
            .filter(|((_c, r), _v)| r == &n)
            .sorted_unstable_by_key(|((c, _r), _v)| c)
            .map(|(_, v)| *v)
            .collect()
    }

    fn shape(&self) -> (usize, usize) {
        self.pattern.iter().fold((0, 0), |shape, (pos, _)| {
            (pos.0.max(shape.0), pos.1.max(shape.1))
        })
    }

    fn compute_symmetry_score(&self, allowed_diffs: usize) -> usize {
        let (cmax, rmax) = self.shape();

        // // check fold along vertical line
        // let v = (1..=cmax).find_map(|idx| {
        //     (0..idx.min(cmax + 1 - idx))
        //         .all(|c| self.get_col(idx - 1 - c) == self.get_col(idx + c))
        //         .then(|| idx)
        // });
        // // check fold along horizontal line
        // let h = (1..=rmax).find_map(|idx| {
        //     (0..idx.min(rmax + 1 - idx))
        //         .all(|c| self.get_row(idx - 1 - c) == self.get_row(idx + c))
        //         .then(|| idx * 100)
        // });

        // check fold along vertical line
        let v = (1..=cmax).find_map(|idx| {
            let diff: usize = (0..idx.min(cmax + 1 - idx))
                .map(|c| {
                    self.get_col(idx - 1 - c)
                        .iter()
                        .zip(self.get_col(idx + c).iter())
                        .map(|(a, b)| (a != b) as usize)
                        .sum::<usize>()
                })
                .sum();
            (diff == allowed_diffs).then(|| idx)
        });
        let h = (1..=rmax).find_map(|idx| {
            let diff: usize = (0..idx.min(rmax + 1 - idx))
                .map(|c| {
                    self.get_row(idx - 1 - c)
                        .iter()
                        .zip(self.get_row(idx + c).iter())
                        .map(|(a, b)| (a != b) as usize)
                        .sum::<usize>()
                })
                .sum();
            (diff == allowed_diffs).then(|| idx * 100)
        });

        v.or(h).unwrap()
    }
}

// fn parse_input(input: &str) -> Vec<HashMap<(usize, usize), char>> {
fn parse_input(input: &str) -> Vec<Mirror> {
    input
        .split("\n\n")
        .map(|block| {
            let pattern = block
                .lines()
                .enumerate()
                .flat_map(|(r, line)| line.chars().enumerate().map(move |(c, ch)| ((c, r), ch)))
                .collect();
            Mirror { pattern }
        })
        .collect()
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let mirrors = parse_input(input);
    let p1 = mirrors.iter().map(|m| m.compute_symmetry_score(0)).sum();
    let p2 = mirrors.iter().map(|m| m.compute_symmetry_score(1)).sum();
    // let p2 = 0;
    (p1, p2)
}
