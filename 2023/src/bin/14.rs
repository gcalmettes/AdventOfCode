use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

// (x, y, fixed)
fn parse_rocks(input: &str) -> HashSet<(usize, usize, bool)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                'O' => Some((x, y, false)),
                '#' => Some((x, y, true)),
                _ => None,
            })
        })
        .collect::<HashSet<(usize, usize, bool)>>()
}

fn part1(rocks: &HashSet<(usize, usize, bool)>) -> usize {
    let max_load = rocks.iter().map(|(_x, y, _fixed)| y).max().unwrap() + 1;

    let width = rocks.iter().map(|(x, _y, _fixed)| x).max().unwrap();

    (0..=*width)
        .map(|c| {
            rocks
                .iter()
                .filter(|(x, _y, _fixed)| x == &c)
                .sorted_unstable_by_key(|(_x, y, _fixed)| y)
                .fold((0, -1), |(score, occupied), (_x, y, fixed)| {
                    let p = match fixed {
                        // load doesn't change but occupied position is now the fixed
                        true => (score, *y as isize),
                        // load increases and occupied position is updated
                        false => {
                            let s = (occupied + 1) as usize;
                            (score + max_load - s, occupied + 1)
                        }
                    };
                    p
                })
                .0
        })
        .sum()
}

fn compute_load(rocks: &HashSet<(usize, usize, bool)>) -> usize {
    let max_load = rocks.iter().map(|(_x, y, _fixed)| y).max().unwrap() + 1;

    rocks
        .iter()
        .filter(|(_x, _y, fixed)| !fixed)
        .map(|(_x, y, _fixed)| max_load - y)
        .sum()
}

fn cycle(rocks: &HashSet<(usize, usize, bool)>, n: usize) -> usize {
    let (width, height) = rocks
        .iter()
        .fold((0, 0), |acc, (x, y, _fixed)| (acc.0.max(*x), acc.1.max(*y)));

    let mut state = rocks.clone();

    let mut seen = HashMap::new();
    for i in 1..=n {
        // North
        state = (0..=width)
            .flat_map(|c| {
                state
                    .iter()
                    .filter(|(x, _y, _fixed)| x == &c)
                    .sorted_unstable_by_key(|(_x, y, _fixed)| y)
                    .scan(-1, move |occupied, (_x, y, fixed)| {
                        match fixed {
                            // load doesn't change but occupied position is now the fixed
                            true => {
                                *occupied = *y as isize;
                                Some((c, *y, true))
                            }
                            // load increases and occupied position is updated
                            false => {
                                *occupied += 1;
                                Some((c, *occupied as usize, false))
                            }
                        }
                    })
            })
            .collect::<HashSet<(usize, usize, bool)>>();

        // West
        state = (0..=height)
            .flat_map(|r| {
                state
                    .iter()
                    .filter(|(_x, y, _fixed)| y == &r)
                    .sorted_unstable_by_key(|(x, _y, _fixed)| x)
                    .scan(-1, move |occupied, (x, _y, fixed)| {
                        match fixed {
                            // load doesn't change but occupied position is now the fixed
                            true => {
                                *occupied = *x as isize;
                                Some((*x, r, true))
                            }
                            // load increases and occupied position is updated
                            false => {
                                *occupied += 1;
                                Some((*occupied as usize, r, false))
                            }
                        }
                    })
            })
            .collect::<HashSet<(usize, usize, bool)>>();

        // South
        state = (0..=width)
            .flat_map(|c| {
                state
                    .iter()
                    .filter(|(x, _y, _fixed)| x == &c)
                    .sorted_unstable_by_key(|(_x, y, _fixed)| Reverse(y))
                    .scan(height + 1, move |occupied, (_x, y, fixed)| {
                        match fixed {
                            // load doesn't change but occupied position is now the fixed
                            true => {
                                *occupied = *y;
                                Some((c, *y, true))
                            }
                            // load increases and occupied position is updated
                            false => {
                                *occupied -= 1;
                                Some((c, *occupied, false))
                            }
                        }
                    })
            })
            .collect::<HashSet<(usize, usize, bool)>>();

        // East
        state = (0..=height)
            .flat_map(|r| {
                state
                    .iter()
                    .filter(|(_x, y, _fixed)| y == &r)
                    .sorted_unstable_by_key(|(x, _y, _fixed)| Reverse(x))
                    .scan(width + 1, move |occupied, (x, _y, fixed)| {
                        match fixed {
                            // load doesn't change but occupied position is now the fixed
                            true => {
                                *occupied = *x;
                                Some((*x, r, true))
                            }
                            // load increases and occupied position is updated
                            false => {
                                *occupied -= 1;
                                Some((*occupied, r, false))
                            }
                        }
                    })
            })
            .collect::<HashSet<(usize, usize, bool)>>();

        if let Some(seen_at) = seen.insert(
            Vec::from_iter(
                state
                    .iter()
                    .map(|(x, y, f)| format!("{x}-{y}-{f}"))
                    .sorted(),
            ),
            i,
        ) {
            if (n - i) % (i - seen_at) == 0 {
                break;
            }
        }
    }

    compute_load(&state)
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let rocks = parse_rocks(input);

    let p1 = part1(&rocks);
    let p2 = cycle(&rocks, 1000000000);
    (p1, p2)
}
