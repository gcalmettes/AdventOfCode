use std::fs;

#[derive(Debug, PartialEq, Copy, Clone)]
enum RPS {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

impl RPS {
    fn value(&self) -> isize {
        match self {
            RPS::ROCK => 1,
            RPS::PAPER => 2,
            RPS::SCISSORS => 3,
        }
    }
}

#[derive(Debug)]
struct Round(RPS, RPS);

impl Round {
    fn is_draw(&self) -> bool {
        self.0 == self.1
    }
    fn is_win(&self) -> bool {
        match self.1.value() - (self.0.value() % 3) {
            1 => true,
            _ => false,
        }
    }
    fn score(&self) -> isize {
        let outcome_points = match (self.is_draw(), self.is_win()) {
            (false, true) => 6, // win
            (true, false) => 3, // draw
            _ => 0,             // loss
        };
        self.1.value() + outcome_points
    }
}

fn get_sign(c: char) -> RPS {
    match c {
        'A' | 'X' => RPS::ROCK,
        'B' | 'Y' => RPS::PAPER,
        'C' | 'Z' => RPS::SCISSORS,
        _ => unreachable!(),
    }
}

fn get_needed_shape_for_outcome(opponent: RPS, outcome: char) -> RPS {
    match (opponent, outcome) {
        (RPS::ROCK, 'Z') => RPS::PAPER,
        (RPS::ROCK, 'X') => RPS::SCISSORS,
        (RPS::PAPER, 'Z') => RPS::SCISSORS,
        (RPS::PAPER, 'X') => RPS::ROCK,
        (RPS::SCISSORS, 'Z') => RPS::ROCK,
        (RPS::SCISSORS, 'X') => RPS::PAPER,
        _ => opponent, // all draws
    }
}

fn part1() -> isize {
    let input = fs::read_to_string("./inputs/02.in").expect("file not found");

    let score = input
        .lines()
        .map(|round| {
            let g = round.chars().collect::<Vec<char>>();
            let r = Round(get_sign(g[0]), get_sign(g[2]));
            r.score()
        })
        .sum();
    score
}

fn part2() -> isize {
    let input = fs::read_to_string("./inputs/02.in").expect("file not found");

    let score = input
        .lines()
        .map(|round| {
            let g = round.chars().collect::<Vec<char>>();
            let r = Round(
                get_sign(g[0]),
                get_needed_shape_for_outcome(get_sign(g[0]), g[2]),
            );
            r.score()
        })
        .sum();
    score
}

fn main() {
    let p1 = part1();
    let p2 = part2();
    println!("part 1: {:?}", p1);
    println!("part 2: {:?}", p2);
}
