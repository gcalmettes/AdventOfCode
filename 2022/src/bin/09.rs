use hashbrown::HashSet;

#[derive(Debug)]
enum Move {
    Right(u32),
    Left(u32),
    Up(u32),
    Down(u32),
}

type Coord = (i32, i32);

struct Rope {
    head_pos: Coord,
    tail_pos: Box<[Coord]>,
    tail_visited: HashSet<Coord>,
}

impl Rope {
    fn compute_delta(&self, a: Coord, b: Coord) -> (i32, i32) {
        (b.0 - a.0, b.1 - a.1)
    }

    fn update_tail(&mut self, multi: bool) {
        self.tail_pos
            .clone()
            .iter()
            .enumerate()
            .for_each(|(i, pos)| {
                let delta = match i {
                    0 => self.compute_delta(self.tail_pos[i], self.head_pos),
                    _ => self.compute_delta(*pos, self.tail_pos[i - 1]),
                };

                match delta {
                    (2, dy) => {
                        self.tail_pos[i].0 += 1;
                        match dy {
                            1 | 2 => self.tail_pos[i].1 += 1,
                            -1 | -2 => self.tail_pos[i].1 -= 1,
                            _ => (),
                        }
                    }
                    (-2, dy) => {
                        self.tail_pos[i].0 -= 1;
                        match dy {
                            1 | 2 => self.tail_pos[i].1 += 1,
                            -1 | -2 => self.tail_pos[i].1 -= 1,
                            _ => (),
                        }
                    }
                    (dx, 2) => {
                        self.tail_pos[i].1 += 1;
                        match dx {
                            1 => self.tail_pos[i].0 += 1,
                            -1 => self.tail_pos[i].0 -= 1,
                            _ => (),
                        }
                    }
                    (dx, -2) => {
                        self.tail_pos[i].1 -= 1;
                        match dx {
                            1 => self.tail_pos[i].0 += 1,
                            -1 => self.tail_pos[i].0 -= 1,
                            _ => (),
                        }
                    }
                    (_, _) => (),
                }
                match (multi, i) {
                    (false, _) => {
                        self.tail_visited.insert(self.tail_pos[0]);
                    }
                    (true, 8) => {
                        self.tail_visited.insert(self.tail_pos[i]);
                    }
                    (_, _) => (),
                }
            });
    }

    fn step(&mut self, m: &Move, multi: bool) {
        match m {
            Move::Right(v) => (0..*v).for_each(|_| {
                self.head_pos.0 += 1;
                self.update_tail(multi);
            }),
            Move::Left(v) => (0..*v).for_each(|_| {
                self.head_pos.0 -= 1;
                self.update_tail(multi);
            }),
            Move::Up(v) => (0..*v).for_each(|_| {
                self.head_pos.1 += 1;
                self.update_tail(multi);
            }),
            Move::Down(v) => (0..*v).for_each(|_| {
                self.head_pos.1 -= 1;
                self.update_tail(multi);
            }),
        };
    }
}

fn parse_input(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| match line.split_once(" ") {
            Some(("R", v)) => Move::Right(v.parse::<u32>().unwrap()),
            Some(("L", v)) => Move::Left(v.parse::<u32>().unwrap()),
            Some(("U", v)) => Move::Up(v.parse::<u32>().unwrap()),
            Some(("D", v)) => Move::Down(v.parse::<u32>().unwrap()),
            _ => unreachable!(),
        })
        .collect::<Vec<Move>>()
}

fn part1(data: &Vec<Move>) -> usize {
    let mut rope = Rope {
        head_pos: (0, 0),
        tail_pos: Box::new([(0, 0)]),
        tail_visited: HashSet::from([(0, 0)]),
    };
    data.iter().for_each(|m| {
        rope.step(m, false);
    });
    rope.tail_visited.len()
}

fn part2(data: &Vec<Move>) -> usize {
    let mut rope = Rope {
        head_pos: (0, 0),
        tail_pos: Box::new([(0, 0); 9]),
        tail_visited: HashSet::from([(0, 0)]),
    };
    data.iter().for_each(|m| {
        rope.step(m, true);
    });
    rope.tail_visited.len()
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    let p1 = part1(&data);
    let p2 = part2(&data);
    (p1, p2)
}
