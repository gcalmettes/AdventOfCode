use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    fn from_string(s: &str) -> Dir {
        match s {
            "U" => Dir::North,
            "D" => Dir::South,
            "R" => Dir::East,
            "L" => Dir::West,
            _ => unreachable!(),
        }
    }

    fn next(&self, pos: (isize, isize)) -> (isize, isize) {
        match self {
            Dir::North => (pos.0, pos.1 - 1),
            Dir::South => (pos.0, pos.1 + 1),
            Dir::West => (pos.0 - 1, pos.1),
            Dir::East => (pos.0 + 1, pos.1),
        }
    }
}

fn parse_dig_plan(input: &str) -> Vec<(Dir, isize, &str)> {
    input
        .lines()
        .map(|line| {
            let mut ins = line.splitn(3, " ");
            let dir = ins.next().map(|d| Dir::from_string(d)).unwrap();
            let n = ins
                .next()
                .map(|d| d.parse::<isize>().ok().unwrap())
                .unwrap();
            let color = ins
                .next()
                .map(|c| c.trim_end_matches(")").trim_start_matches("("))
                .unwrap();
            (dir, n, color)
        })
        .collect()
}

fn part1(plan: &Vec<(Dir, isize, &str)>) -> usize {
    let mut current = (0, 0);
    let mut dig: HashSet<(isize, isize)> = HashSet::from_iter([current.clone()]);

    // follow dig plan
    plan.iter().for_each(|(dir, n, _)| {
        (0..*n).for_each(|_ins| {
            current = dir.next(current);
            dig.insert(current);
        });
    });

    let (cmin, cmax, rmin, rmax) = dig.iter().fold(
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
        |state, pos| {
            (
                pos.0.min(state.0),
                pos.0.max(state.1),
                pos.1.min(state.2),
                pos.1.max(state.3),
            )
        },
    );

    // flood fill
    let mut outside: HashSet<(isize, isize)> = HashSet::new();
    // start with a point we know is outside
    let mut stack = vec![(cmin - 1, rmin - 1)];

    while let Some(pos) = stack.pop() {
        if outside.contains(&pos) {
            continue;
        }
        outside.insert(pos);
        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .filter_map(|(dc, dr)| {
                let new_pos = (pos.0 + dc, pos.1 + dr);
                // make sure the position is in imposed boundaries and not a boundary point
                (new_pos.0 >= cmin - 1
                    && new_pos.0 <= cmax + 1
                    && new_pos.1 >= rmin - 1
                    && new_pos.1 <= rmax + 1
                    && !dig.contains(&new_pos))
                .then(|| new_pos)
            });
        stack.extend(neighbors);
    }

    let all_filled = ((rmax - rmin + 3) * (cmax - cmin + 3)) as usize;

    all_filled - outside.len()
}

fn part2(plan: &Vec<(Dir, isize, &str)>) -> usize {
    let boundaries = plan
        .iter()
        .fold(vec![(0, 0)], |mut polygon, (_dir, _n, color)| {
            let color = color.trim_start_matches("#");
            let n = color
                .chars()
                .enumerate()
                .filter_map(|(i, c)| (i < 5).then(|| c))
                .collect::<String>();
            let n = isize::from_str_radix(&n, 16).unwrap();
            let last = polygon[polygon.len() - 1];
            let new_pos = match color.chars().last().unwrap() {
                '0' => (last.0 + n, last.1), //Dir::East,
                '1' => (last.0, last.1 + n), //Dir::South,
                '2' => (last.0 - n, last.1), //Dir::West,
                '3' => (last.0, last.1 - n), //Dir::North,
                _ => unreachable!(),
            };
            polygon.push(new_pos);
            polygon
        });
    shoelace_and_pick(&boundaries)
}

fn shoelace_and_pick(polygon: &Vec<(isize, isize)>) -> usize {
    let mut area: isize = 0;
    let mut perimeter: isize = 0;

    // Shoelace formula
    // https://en.wikipedia.org/wiki/Shoelace_formula
    for i in 0..(polygon.len() - 1) {
        let mut window = polygon[i..=(i + 1)].iter();
        let first = window.next().unwrap();
        let second = window.next().unwrap();
        area += (first.0 * second.1) - (first.1 * second.0);
        perimeter += f64::abs(f64::sqrt(
            ((second.0 - first.0) * (second.0 - first.0)) as f64
                + ((second.1 - first.1) * (second.1 - first.1)) as f64,
        )) as isize;
    }
    let area: isize = isize::abs(area) / 2;

    // Pick's theorem
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    let interior: isize = area - (perimeter / 2) + 1;

    (interior + perimeter) as usize
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let dig_plan = parse_dig_plan(input);
    let p1 = part1(&dig_plan);
    let p2 = part2(&dig_plan);

    (p1, p2)
}
