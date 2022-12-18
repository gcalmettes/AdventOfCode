#[derive(Debug, PartialEq)]
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

fn part2(data: &Vec<Cube>) -> usize {
    0
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    let p1 = part1(&data);
    let p2 = part2(&data);
    (p1, p2)
}
