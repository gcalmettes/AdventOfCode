use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn next_forward(&self, pos: (isize, isize)) -> ((isize, isize), Dir) {
        match self {
            Dir::North => ((pos.0, pos.1 - 1), Dir::North),
            Dir::East => ((pos.0 + 1, pos.1), Dir::East),
            Dir::South => ((pos.0, pos.1 + 1), Dir::South),
            Dir::West => ((pos.0 - 1, pos.1), Dir::West),
        }
    }
    fn next_right(&self, pos: (isize, isize)) -> ((isize, isize), Dir) {
        match self {
            Dir::North => ((pos.0 + 1, pos.1), Dir::East),
            Dir::East => ((pos.0, pos.1 + 1), Dir::South),
            Dir::South => ((pos.0 - 1, pos.1), Dir::West),
            Dir::West => ((pos.0, pos.1 - 1), Dir::North),
        }
    }
    fn next_left(&self, pos: (isize, isize)) -> ((isize, isize), Dir) {
        match self {
            Dir::North => ((pos.0 - 1, pos.1), Dir::West),
            Dir::East => ((pos.0, pos.1 - 1), Dir::North),
            Dir::South => ((pos.0 + 1, pos.1), Dir::East),
            Dir::West => ((pos.0, pos.1 + 1), Dir::South),
        }
    }
}

fn parse_input(input: &str) -> HashMap<(isize, isize), char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(move |(c, ch)| ((c as isize, r as isize), ch))
        })
        .collect()
}

fn count_energized(start: ((isize, isize), Dir), grid: &HashMap<(isize, isize), char>) -> usize {
    let mut stack: Vec<((isize, isize), Dir)> = Vec::new();
    let mut seen: HashSet<((isize, isize), Dir)> = HashSet::new();

    // we start at top-left corner, going East
    stack.push(start);

    while let Some((pos, dir)) = stack.pop() {
        if seen.contains(&(pos, dir)) {
            continue;
        }
        let symbol = grid.get(&pos).map(|ch| {
            // if tile in grid, we energize it
            seen.insert((pos, dir));
            ch
        });
        match (symbol, dir) {
            (Some('.'), d) => {
                // continue in same dir
                stack.push(d.next_forward(pos));
            }
            (Some('|'), d) => {
                // if aligned continue, else split
                match d {
                    Dir::North | Dir::South => {
                        stack.push(d.next_forward(pos));
                    }
                    Dir::West | Dir::East => {
                        stack.push(d.next_right(pos));
                        stack.push(d.next_left(pos));
                    }
                }
            }
            (Some('-'), d) => {
                // if aligned continue, else split
                match d {
                    Dir::West | Dir::East => {
                        stack.push(d.next_forward(pos));
                    }
                    Dir::North | Dir::South => {
                        stack.push(d.next_right(pos));
                        stack.push(d.next_left(pos));
                    }
                }
            }
            (Some('/'), d) => {
                // reflect
                match d {
                    Dir::North | Dir::South => {
                        stack.push(d.next_right(pos));
                    }
                    Dir::West | Dir::East => {
                        stack.push(d.next_left(pos));
                    }
                }
            }
            (Some('\\'), d) => {
                // reflect
                match d {
                    Dir::North | Dir::South => {
                        stack.push(d.next_left(pos));
                    }
                    Dir::West | Dir::East => {
                        stack.push(d.next_right(pos));
                    }
                }
            }
            _ => {}
        }
    }

    seen.iter().map(|(pos, _dir)| pos).unique().count()
}

fn get_max(grid: &HashMap<(isize, isize), char>) -> usize {
    let (cmax, rmax) = grid
        .iter()
        .fold((0, 0), |acc, (pos, _)| (pos.0.max(acc.0), pos.1.max(acc.1)));

    (0..=cmax)
        .map(|c| count_energized(((c, 0), Dir::South), &grid))
        .chain((0..=cmax).map(|c| count_energized(((c, rmax), Dir::North), &grid)))
        .chain((0..=rmax).map(|r| count_energized(((0, r), Dir::East), &grid)))
        .chain((0..=rmax).map(|r| count_energized(((cmax, r), Dir::West), &grid)))
        .max()
        .unwrap()
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let grid = parse_input(input);

    let p1 = count_energized(((0, 0), Dir::East), &grid);
    let p2 = get_max(&grid);

    (p1, p2)
}
