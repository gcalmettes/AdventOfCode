use hashbrown::HashMap;
use regex::Regex;

type Pos = (isize, isize);

#[derive(Debug)]
enum Tile {
    Wall,
    Empty,
}

enum Ins {
    Forward(u16),
    Turn(Box<dyn Fn(Pos) -> Pos>),
}

fn boundaries(board: &HashMap<Pos, Tile>) -> (isize, isize, isize, isize) {
    board.keys().fold(
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
        |state, pos| {
            let mut new_state = state;
            if pos.0 < state.0 {
                new_state.0 = pos.0;
            };
            if pos.0 > state.1 {
                new_state.1 = pos.0;
            };
            if pos.1 < state.2 {
                new_state.2 = pos.1;
            };
            if pos.1 > state.3 {
                new_state.3 = pos.1;
            };
            new_state
        },
    )
}

fn draw(board: &HashMap<Pos, Tile>) {
    let (min_x, max_x, min_y, max_y) = boundaries(board);
    (min_y..=max_y).for_each(|y| {
        (min_x..=max_x).for_each(|x| {
            if board.contains_key(&(x, y)) {
                match board[&(x, y)] {
                    Tile::Empty => print!("."),
                    Tile::Wall => print!("#"),
                }
            } else {
                print!(" ")
            };
        });
        print!("\n");
    });
}

fn clockwise(dir: Pos) -> Pos {
    (-dir.1, dir.0)
}

fn anticlockwise(dir: Pos) -> Pos {
    (dir.1, -dir.0)
}

fn parse_input(input: &str) -> (HashMap<Pos, Tile>, Vec<Ins>) {
    let mut input = input.split("\n\n");
    let map = input.next().unwrap();
    let ins = input.next().unwrap();

    let mut board: HashMap<Pos, Tile> = HashMap::new();
    map.lines().enumerate().for_each(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| c != &' ')
            .for_each(|(x, c)| {
                let tile = match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    _ => unreachable!(),
                };
                board.insert((x as isize, y as isize), tile);
            });
    });

    let re = Regex::new(r"([\d]+|[RL])").unwrap();

    let ins = re
        .captures_iter(&ins)
        .map(|c| {
            let c = c.get(1).map(|m| m.as_str()).unwrap();
            match c.parse::<u16>() {
                Ok(n) => Ins::Forward(n),
                _ => match c {
                    "R" => Ins::Turn(Box::new(clockwise)),
                    "L" => Ins::Turn(Box::new(anticlockwise)),
                    _ => unreachable!(),
                },
            }
        })
        .collect::<Vec<_>>();

    (board, ins)
}

fn get_start(board: &HashMap<Pos, Tile>) -> Pos {
    let (_, _, min_y, _) = boundaries(board);
    let x_start = board
        .iter()
        .filter(|(pos, _)| pos.1 == min_y)
        .map(|(pos, _)| pos.0)
        .min()
        .unwrap();
    (x_start as isize, min_y as isize)
}

fn get_opposite_tile(board: &HashMap<Pos, Tile>, current_pos: Pos, step: Pos) -> Pos {
    match step {
        (0, v) => {
            // same x
            let x = current_pos.0;
            let y = board
                .iter()
                .filter(|(pos, _)| pos.0 == x)
                .map(|(pos, _)| pos.1);
            let y = if v == 1 {
                // we go south so go to min
                y.min().unwrap()
            } else {
                // we go north so go to max
                y.max().unwrap()
            };
            (x, y)
        }
        (v, 0) => {
            // same x
            let y = current_pos.1;
            let x = board
                .iter()
                .filter(|(pos, _)| pos.1 == y)
                .map(|(pos, _)| pos.0);
            let x = if v == 1 {
                // we go east, so go to min
                x.min().unwrap()
            } else {
                // we go west so go to max
                x.max().unwrap()
            };
            (x, y)
        }
        _ => unreachable!(),
    }
}

fn part1(board: &HashMap<Pos, Tile>, ins: &Vec<Ins>) -> isize {
    let mut ins = ins.iter();
    let start_pos = get_start(board);

    let mut step: (isize, isize) = (1, 0);
    let mut current_pos = start_pos;

    loop {
        match ins.next() {
            Some(Ins::Forward(x)) => {
                for _ in 0..*x {
                    let next_pos = (current_pos.0 + step.0, current_pos.1 + step.1);
                    match board.get(&next_pos) {
                        Some(Tile::Empty) => {
                            // move to tile
                            current_pos = next_pos;
                        }
                        Some(Tile::Wall) => {
                            // cannot go forward, skip to next instruction
                            break;
                        }
                        None => {
                            // need to wrap around
                            let opposite_pos = get_opposite_tile(board, current_pos, step);
                            match board.get(&opposite_pos) {
                                Some(Tile::Wall) => {
                                    // cannot go forward, skip to next instruction
                                    break;
                                }
                                _ => (),
                            }
                            current_pos = opposite_pos;
                        }
                    }
                }
            }

            Some(Ins::Turn(f)) => {
                step = f(step);
            }
            None => break,
        }
    }
    let row = current_pos.1 + 1;
    let col = current_pos.0 + 1;
    let facing = match step {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => unreachable!(),
    };
    1000 * row + 4 * col + facing
}

fn get_wrapped_cube_tile(current_pos: Pos, step: Pos) -> (Pos, Pos) {
    const FACE_SIZE: isize = 50;
    let current_face = match (current_pos.0 / FACE_SIZE, current_pos.1 / FACE_SIZE) {
        (1, 0) => 1,
        (2, 0) => 2,
        (1, 1) => 3,
        (0, 2) => 4,
        (1, 2) => 5,
        (0, 3) => 6,
        _ => panic!("Unexpected position: {:?}", current_pos),
    };

    let mut new_pos = current_pos;
    let mut new_step = step;
    new_pos.0 += step.0;
    new_pos.1 += step.1;
    match current_face {
        1 => match step {
            (1, 0) => {}
            (0, 1) => {}
            (-1, 0) => {
                new_step = (1, 0);
                new_pos.0 = 0;
                new_pos.1 = FACE_SIZE * 3 - current_pos.1 - 1;
            }
            (0, -1) => {
                new_step = (1, 0);
                new_pos.0 = 0;
                new_pos.1 = current_pos.0 + FACE_SIZE * 2;
            }
            _ => panic!("Unexpected direction: {:?}", step),
        },
        2 => match step {
            (1, 0) => {
                new_step = (-1, 0);
                new_pos.0 = current_pos.0 - FACE_SIZE;
                new_pos.1 = FACE_SIZE * 3 - current_pos.1 - 1;
            }
            (0, 1) => {
                new_step = (-1, 0);
                new_pos.0 = FACE_SIZE * 2 - 1;
                new_pos.1 = current_pos.0 - FACE_SIZE;
            }
            (-1, 0) => {}
            (0, -1) => {
                new_step = (0, -1);
                new_pos.0 = current_pos.0 - FACE_SIZE * 2;
                new_pos.1 = FACE_SIZE * 4 - 1;
            }
            _ => panic!("Unexpected direction: {:?}", step),
        },
        3 => match step {
            (1, 0) => {
                new_step = (0, -1);
                new_pos.0 = current_pos.1 + FACE_SIZE;
                new_pos.1 = FACE_SIZE - 1;
            }
            (0, 1) => {}
            (-1, 0) => {
                new_step = (0, 1);
                new_pos.0 = current_pos.1 - FACE_SIZE;
                new_pos.1 = FACE_SIZE * 2;
            }
            (0, -1) => {}
            _ => panic!("Unexpected direction: {:?}", step),
        },
        4 => match step {
            (1, 0) => {}
            (0, 1) => {}
            (-1, 0) => {
                new_step = (1, 0);
                new_pos.0 = FACE_SIZE;
                new_pos.1 = 3 * FACE_SIZE - current_pos.1 - 1;
            }
            (0, -1) => {
                new_step = (1, 0);
                new_pos.0 = FACE_SIZE;
                new_pos.1 = current_pos.0 + FACE_SIZE;
            }
            _ => panic!("Unexpected direction: {:?}", step),
        },
        5 => match step {
            (1, 0) => {
                new_step = (-1, 0);
                new_pos.0 = FACE_SIZE * 3 - 1;
                new_pos.1 = 3 * FACE_SIZE - current_pos.1 - 1;
            }
            (0, 1) => {
                new_step = (-1, 0);
                new_pos.0 = FACE_SIZE - 1;
                new_pos.1 = current_pos.0 + FACE_SIZE * 2;
            }
            (-1, 0) => {}
            (0, -1) => {}
            _ => panic!("Unexpected direction: {:?}", step),
        },
        6 => match step {
            (1, 0) => {
                new_step = (0, -1);
                new_pos.0 = current_pos.1 - 2 * FACE_SIZE;
                new_pos.1 = FACE_SIZE * 3 - 1;
            }
            (0, 1) => {
                new_step = (0, 1);
                new_pos.0 = current_pos.0 + FACE_SIZE * 2;
                new_pos.1 = 0;
            }
            (-1, 0) => {
                new_step = (0, 1);
                new_pos.0 = current_pos.1 - FACE_SIZE * 2;
                new_pos.1 = 0;
            }
            (0, -1) => {}
            _ => panic!("Unexpected direction: {:?}", step),
        },
        _ => panic!("Unexpected face: {}", current_face),
    }

    (new_pos, new_step)
}

fn part2(board: &HashMap<Pos, Tile>, ins: &Vec<Ins>) -> isize {
    let mut ins = ins.iter();
    let start_pos = get_start(board);

    let mut step: (isize, isize) = (1, 0);
    let mut current_pos = start_pos;

    loop {
        match ins.next() {
            Some(Ins::Forward(x)) => {
                for _ in 0..*x {
                    let next_pos = (current_pos.0 + step.0, current_pos.1 + step.1);
                    match board.get(&next_pos) {
                        Some(Tile::Empty) => {
                            // move to tile
                            current_pos = next_pos;
                        }
                        Some(Tile::Wall) => {
                            // cannot go forward, skip to next instruction
                            break;
                        }
                        None => {
                            // need to wrap around
                            let (opposite_pos, new_step) = get_wrapped_cube_tile(current_pos, step);
                            match board.get(&opposite_pos) {
                                Some(Tile::Wall) => {
                                    // cannot go forward, skip to next instruction
                                    break;
                                }
                                _ => (),
                            }
                            current_pos = opposite_pos;
                            step = new_step;
                        }
                    }
                }
            }

            Some(Ins::Turn(f)) => {
                step = f(step);
            }
            None => break,
        }
    }
    let row = current_pos.1 + 1;
    let col = current_pos.0 + 1;
    let facing = match step {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => unreachable!(),
    };
    1000 * row + 4 * col + facing
}

#[aoc::main()]
fn main(input: &str) -> (isize, isize) {
    let (board, ins) = parse_input(input);
    let p1 = part1(&board, &ins);
    let p2 = part2(&board, &ins);
    (p1, p2)
}
