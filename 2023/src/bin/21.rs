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

fn save_steps(start: &Pos, max_step: usize, rocks: &HashSet<Pos>, shape: &(isize, isize)) {
    let mut csv = String::from("");
    for i in 0..=max_step {
        println!("{i}");
        let res = walk(&start, i, &rocks, &shape);
        csv += &format!("{i},{res}\n");
    }
    std::fs::write("img/21.csv", csv).unwrap();
}

fn parse_input(input: &str) -> (Pos, HashSet<Pos>, (isize, isize)) {
    input.lines().enumerate().fold(
        (Pos(0, 0), HashSet::new(), (0, 0)),
        |(mut start, mut pos, (mut cmax, mut rmax)), (r, line)| {
            line.chars().enumerate().for_each(|(c, ch)| {
                cmax = cmax.max((c + 1) as isize);
                rmax = rmax.max((r + 1) as isize);
                match ch {
                    '#' => {
                        pos.insert(Pos(c as isize, r as isize));
                    }
                    'S' => {
                        start = Pos(c as isize, r as isize);
                    }
                    _ => {}
                }
            });
            (start, pos, (cmax, rmax))
        },
    )
}

fn walk(start: &Pos, target: usize, rocks: &HashSet<Pos>, shape: &(isize, isize)) -> usize {
    let mut queue = VecDeque::from_iter([(0, start.clone())]);
    let mut seen = HashSet::<Pos>::new();
    let mut tiles = HashSet::<Pos>::new();

    while let Some((i, pos)) = queue.pop_front() {
        if i > target {
            break;
        }
        if i <= target && i % 2 == (target % 2) {
            tiles.insert(pos.clone());
        }
        if seen.contains(&pos) {
            continue;
        }
        seen.insert(pos.clone());
        pos.neighbors()
            .iter()
            .filter(|p| !rocks.contains(&Pos(p.0.rem_euclid(shape.0), p.1.rem_euclid(shape.1))))
            .for_each(|p| {
                queue.push_back((i + 1, p.clone()));
            });
    }
    tiles.len()
}

fn solve_p2(start: &Pos, target: usize, rocks: &HashSet<Pos>, shape: &(isize, isize)) -> usize {
    // Cycle, 131, size of the grid
    // Note: 26501365 // 131 == 202300
    //       26501365 % 131 == 65
    let s = 65;
    let cycle = 131;
    let steps = (0..3).map(|n| s + n * cycle).collect::<Vec<_>>();
    let tiles_reached = steps
        .iter()
        .map(|n| (*n, walk(&start, *n, &rocks, &shape)))
        .collect::<Vec<_>>();

    // 3 points formula to solve the quadratic formula
    // We solve P(x) = a*x^2 + b*x + c for different x
    //
    // P(0) = a*(65 + 0*131)² + b*(65 + 0*131) + c
    // P(1) = a*(65 + 1*131)² + b*(65 + 1*131) + c
    // P(2) = a*(65 + 2*131)² + b*(65 + 2*131) + c
    //
    // c = P(0)
    // we substitute in the others and we find
    // b = (4*P(1) -3*P(0) - P(2)) / 2
    // a = P(1) - P(0) - b
    let c = tiles_reached[0].1;
    let b = (4 * tiles_reached[1].1 - 3 * tiles_reached[0].1 - tiles_reached[2].1) / 2;
    let a = tiles_reached[1].1 - tiles_reached[0].1 - b;

    // number of whole tile lenghts
    // this is 202300
    let x = (target - s) / cycle;

    // polynomial deg 2
    a * x.pow(2) + b * x + c
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let (start, rocks, shape) = parse_input(input);
    let p1 = walk(&start, 64, &rocks, &shape);
    let p2 = solve_p2(&start, 26501365, &rocks, &shape);
    save_steps(&start, 1000, &rocks, &shape);
    (p1, p2)
}
