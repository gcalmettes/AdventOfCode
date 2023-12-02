use std::cmp::max;

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    blue: usize,
    red: usize,
    green: usize,
}

impl Round {
    fn compute_power(&self) -> usize {
        self.blue * self.red * self.green
    }
}

impl Game {
    fn from_string(line: &str) -> Self {
        let (game, cubes) = line.split_once(": ").unwrap();
        let id = game
            .split(" ")
            .last()
            .and_then(|d| d.parse::<usize>().ok())
            .unwrap();
        let rounds = cubes
            .split("; ")
            .map(|rounds| {
                let mut r = Round {
                    blue: 0,
                    red: 0,
                    green: 0,
                };
                rounds.split(", ").for_each(|round| {
                    let (n, color) = round.split_once(" ").unwrap();
                    match color {
                        "red" => r.red = n.parse::<usize>().unwrap(),
                        "blue" => r.blue = n.parse::<usize>().unwrap(),
                        "green" => r.green = n.parse::<usize>().unwrap(),
                        _ => {
                            unreachable!()
                        }
                    };
                });
                r
            })
            .collect::<Vec<Round>>();
        Game { id, rounds }
    }

    fn is_possible(&self) -> bool {
        self.rounds
            .iter()
            .map(|r| (r.red <= 12 && r.green <= 13 && r.blue <= 14) as usize)
            .sum::<usize>()
            == self.rounds.len()
    }

    fn min_cubes_needed(&self) -> Round {
        self.rounds.iter().fold(
            Round {
                blue: 0,
                red: 0,
                green: 0,
            },
            |mut acc, r| {
                acc.blue = max(acc.blue, r.blue);
                acc.green = max(acc.green, r.green);
                acc.red = max(acc.red, r.red);
                acc
            },
        )
    }
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let p1 = input
        .split("\n")
        .map(|line| Game::from_string(line))
        .filter(|g| g.is_possible())
        .map(|g| g.id)
        .sum::<usize>();

    let p2 = input
        .split("\n")
        .map(|line| Game::from_string(line).min_cubes_needed().compute_power())
        .sum::<usize>();

    (p1, p2)
}
