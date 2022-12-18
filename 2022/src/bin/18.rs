use hashbrown::HashSet;
use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn sided_by(&self, other: &Cube) -> bool {
        let same = [self.x - other.x, self.y - other.y, self.z - other.z];
        same.iter().filter(|v| **v == 0).count() == 2 && same.iter().sum::<i32>().abs() == 1
    }

    fn adjacent_cubes(&self) -> Vec<Cube> {
        [
            (0, 0, 1),
            (0, 0, -1),
            (0, 1, 0),
            (0, -1, 0),
            (1, 0, 0),
            (-1, 0, 0),
        ]
        .iter()
        .map(|(dx, dy, dz)| Cube {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        })
        .collect::<Vec<Cube>>()
    }
}

fn parse_input(input: &str) -> Vec<Cube> {
    input
        .lines()
        .map(|line| {
            let pos = line
                .split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            Cube {
                x: pos[0],
                y: pos[1],
                z: pos[2],
            }
        })
        .collect()
}

fn part1(cubes: &Vec<Cube>) -> usize {
    cubes
        .into_iter()
        .map(|cube| {
            // how many faces not occupied by sided cube ?
            6 - cubes.into_iter().filter(|c| cube.sided_by(c)).count()
        })
        .sum()
}

fn part2(cubes: Vec<Cube>) -> usize {
    // We will explore a volume containing the drops, start from the exterior and check
    // how many external cubes are in contact with cubes from the input.

    let maxs = cubes.iter().fold((i32::MIN, i32::MIN, i32::MIN), |m, c| {
        (max(m.0, c.x), max(m.1, c.y), max(m.2, c.z))
    });
    let mins = cubes.iter().fold((i32::MAX, i32::MAX, i32::MAX), |m, c| {
        (min(m.0, c.x), min(m.1, c.y), min(m.2, c.z))
    });

    // the limit of our playground, min/max expanded by 1 on each direction
    // so we encompass all the drops and outside volume is in one piece
    let playground = vec![
        (mins.0 - 1..=maxs.0 + 1).collect::<Vec<i32>>(),
        (mins.1 - 1..=maxs.1 + 1).collect::<Vec<i32>>(),
        (mins.2 - 1..=maxs.2 + 1).collect::<Vec<i32>>(),
    ];
    let playground = playground
        .iter()
        .multi_cartesian_product()
        .map(|pos| Cube {
            x: *pos[0],
            y: *pos[1],
            z: *pos[2],
        })
        .collect::<HashSet<Cube>>();

    // keep track of our progress
    let mut seen: HashSet<Cube> = HashSet::new();

    // cubes to check
    // we want to make sure we start from the exterior, so let's start with a cube at the mins
    let mut q: VecDeque<Cube> = VecDeque::from([Cube {
        x: mins.0,
        y: mins.1,
        z: mins.2,
    }]);

    let mut outside_surface = 0;

    while let Some(c) = q.pop_front() {
        if !seen.contains(&c) {
            seen.insert(c.clone());
            for adj_cube in c
                .adjacent_cubes()
                .into_iter()
                // make sure we don't go beyond our playground
                .filter(|v| playground.contains(v))
            {
                if cubes.contains(&adj_cube) {
                    // we found a cube adjacent to the outside of our drops
                    outside_surface += 1;
                } else {
                    // unknown neighbors, let's check it
                    q.push_back(adj_cube)
                }
            }
        }
    }
    outside_surface
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    let p1 = part1(&data);
    let p2 = part2(data.clone());
    (p1, p2)
}
