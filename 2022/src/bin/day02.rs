use std::fs;

#[derive(Debug)]
struct Round(char, char);

impl Round {
    fn is_draw(&self) -> bool {
        self.0 == self.1
    }
    fn is_win(&self) -> bool {
        match (self.0, self.1) {
            ('S', 'R') | ('P', 'S') | ('R', 'P') => true,
            _ => false,
        }
    }
    fn score(&self) -> u32 {
        let shape_points = get_shape_points(self.1);
        let outcome_points = match (self.is_draw(), self.is_win()) {
            (false, true) => 6, // win
            (true, false) => 3, // draw
            _ => 0,             // loss
        };
        shape_points + outcome_points
    }
}

fn get_sign(c: char) -> char {
    match c {
        'A' | 'X' => 'R', // rock
        'B' | 'Y' => 'P', // paper
        'C' | 'Z' => 'S', // siscor
        _ => '.',
    }
}

fn get_shape_points(play: char) -> u32 {
    match play {
        'R' => 1,
        'P' => 2,
        'S' => 3,
        _ => 0,
    }
}

fn get_needed_shape_for_outcome(oponent: char, outcome: char) -> char {
    match (oponent, outcome) {
        ('R', 'Z') => 'P',
        ('R', 'X') => 'S',
        ('P', 'Z') => 'S',
        ('P', 'X') => 'R',
        ('S', 'Z') => 'R',
        ('S', 'X') => 'P',
        _ => oponent, // all draws
    }
}

fn part1() -> u32 {
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

fn part2() -> u32 {
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
