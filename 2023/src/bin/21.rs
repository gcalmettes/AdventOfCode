use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Pos(isize, isize);

impl Pos {
    fn neighbors(&self) -> Vec<Pos> {
        vec![
            Pos(self.0 - 1, self.1),
            Pos(self.0 + 1, self.1),
            Pos(self.0, self.1 - 1),
            Pos(self.0, self.1 + 1),
        ]
    }
}

fn display(tiles: &HashSet<Pos>, rocks: &HashSet<Pos>) {
    let (xmin, xmax, ymin, ymax) = tiles.union(rocks).fold(
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
        |(xmin, xmax, ymin, ymax), p| (p.0.min(xmin), p.0.max(xmax), p.1.min(ymin), p.1.max(ymax)),
    );

    for y in (ymin - 1)..=(ymax + 1) {
        for x in (xmin - 1)..=(xmax + 1) {
            match (tiles.get(&Pos(x, y)), rocks.get(&Pos(x, y))) {
                (Some(_p), None) => print!("0"),
                (None, Some(_p)) => print!("#"),
                (None, None) => print!("."),
                (_, _) => unreachable!(),
            }
        }
        print!("\n");
    }
}

fn parse_input(input: &str) -> (Pos, HashSet<Pos>) {
    input.lines().enumerate().fold(
        (Pos(0, 0), HashSet::new()),
        |(mut start, mut pos), (r, line)| {
            line.chars().enumerate().for_each(|(c, ch)| match ch {
                '#' => {
                    pos.insert(Pos(c as isize, r as isize));
                }
                'S' => {
                    start = Pos(c as isize, r as isize);
                }
                _ => {}
            });
            (start, pos)
        },
    )
}

fn walk(start: &Pos, step: usize, rocks: &HashSet<Pos>) -> usize {
    let mut queue = VecDeque::from_iter([(0, start.clone())]);
    let mut seen = HashSet::<Pos>::new();
    let mut tiles = HashSet::<Pos>::new();

    while let Some((i, pos)) = queue.pop_front() {
        if i > step {
            break;
        }
        if i <= 64 && i % 2 == 0 {
            tiles.insert(pos.clone());
        }
        if seen.contains(&pos) {
            continue;
        }
        seen.insert(pos.clone());
        pos.neighbors()
            .iter()
            .filter(|p| !rocks.contains(&p))
            .for_each(|p| {
                queue.push_back((i + 1, p.clone()));
            });
    }
    tiles.len()
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let (start, rocks) = parse_input(input);
    let p1 = walk(&start, 64, &rocks);
    let p2 = 0;
    (p1, p2)
}
