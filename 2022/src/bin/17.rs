use hashbrown::{HashMap, HashSet};
use std::cmp::{max, min};

const SHAPES: [&[(usize, usize)]; 5] = [
    &[(0, 0), (1, 0), (2, 0), (3, 0)],
    &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
    &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    &[(0, 0), (0, 1), (0, 2), (0, 3)],
    &[(0, 0), (1, 0), (0, 1), (1, 1)],
];

type Pos = (usize, usize);

struct Rock {
    shape: Vec<Pos>,
}

impl Rock {
    fn bounds(&self) -> (usize, usize, usize, usize) {
        self.shape.iter().fold((7, 0, usize::MAX, 0), |bounds, c| {
            (
                min(bounds.0, c.0),
                max(bounds.1, c.0),
                min(bounds.2, c.1),
                max(bounds.3, c.1),
            )
        })
    }

    fn move_left(&self, chamber: &HashSet<Pos>) -> (Vec<Pos>, bool) {
        let (xmin, _, _, _) = self.bounds();
        let can_move = xmin != 0
            && self
                .shape
                .iter()
                .all(|c| !chamber.contains(&(c.0 - 1, c.1)));
        match can_move {
            false => (self.shape.clone(), false),
            true => (self.shape.iter().map(|c| (c.0 - 1, c.1)).collect(), true),
        }
    }

    fn move_right(&self, chamber: &HashSet<Pos>) -> (Vec<Pos>, bool) {
        let (_, xmax, _, _) = self.bounds();
        let can_move = xmax != 6
            && self
                .shape
                .iter()
                .all(|c| !chamber.contains(&(c.0 + 1, c.1)));
        // chamber is 7 units wide
        match can_move {
            false => (self.shape.clone(), false),
            true => (self.shape.iter().map(|c| (c.0 + 1, c.1)).collect(), true),
        }
    }

    fn move_down(&self, chamber: &HashSet<Pos>) -> (Vec<Pos>, bool) {
        let (_, _, ymin, _) = self.bounds();
        let can_move = ymin != 0
            && self
                .shape
                .iter()
                .all(|c| !chamber.contains(&(c.0, c.1 - 1)));
        match can_move {
            false => (self.shape.clone(), false),
            true => (self.shape.iter().map(|c| (c.0, c.1 - 1)).collect(), true),
        }
    }
}

fn display(chamber: &HashSet<Pos>, ymax: usize) {
    (0..=ymax).rev().for_each(|y| {
        print!("\n");
        (0..7).for_each(|x| match chamber.contains(&(x, y)) {
            true => print!("#"),
            false => print!("."),
        })
    });
    print!("\n\n");
}

fn floor_relative_pattern(chamber: &HashSet<Pos>) -> [usize; 7] {
    let mut pattern = [0; 7];
    chamber.iter().for_each(|c| {
        pattern[c.0] = max(pattern[c.0], c.1);
    });
    let minimum = pattern.into_iter().min().unwrap();
    (0..7).for_each(|i| pattern[i] = pattern[i] - minimum);
    pattern
}

// seven units wide.
// Each rock appears so that its left edge is two units away from the left wall
// and its bottom edge is three units above the highest rock in the room (or the floor, if there isn't one).

fn part1(ins: &str) -> usize {
    let mut rocks: Vec<Vec<Pos>> = vec![];
    let mut chamber: HashSet<Pos> = HashSet::new();

    let (mut i, mut s, mut bottom) = (0, 0, 0);

    while i < 2022 {
        let mut rock = Rock {
            shape: SHAPES[i % SHAPES.len()]
                .iter()
                .map(|c| (c.0 + 2, c.1 + bottom + 3))
                .collect(),
        };
        // println!(">> shape {:?}", rock.shape);

        loop {
            match ins.chars().nth(s % ins.len()).unwrap() {
                '<' => {
                    let (new_shape, can_move) = rock.move_left(&chamber);
                    // println!("moving left {}", s);
                    if can_move {
                        rock.shape = new_shape;
                    }
                }
                '>' => {
                    // println!("moving right {}", s);
                    let (new_shape, can_move) = rock.move_right(&chamber);
                    if can_move {
                        rock.shape = new_shape;
                    }
                }
                _ => unreachable!(),
            }
            s += 1;

            let (new_shape, can_move) = rock.move_down(&chamber);
            if !can_move {
                let (_, _, _, ymax) = rock.bounds();
                bottom = max(ymax + 1, bottom);
                new_shape.iter().for_each(|c| {
                    chamber.insert(*c);
                });
                rocks.push(new_shape);
                i += 1;
                // println!("-- new rock. next sign {:?}", s);
                // display(&chamber, bottom);
                break;
            } else {
                rock.shape = new_shape;
            }
        }
    }

    // println!("{:?}", rocks);
    bottom
}

fn part2(ins: &str) -> usize {
    let mut chamber: HashSet<Pos> = HashSet::new();
    let mut state: HashMap<(usize, usize, [usize; 7]), (usize, usize)> = HashMap::new();

    let (mut i, mut s, mut bottom) = (0, 0, 0);
    let mut c = 0;
    let mut p = (0, 0, floor_relative_pattern(&chamber));
    let mut has_been_seen = false;
    while i < 1_000_000_000_000 {
        let rock_pos = i % SHAPES.len();
        let dir_pos = s % ins.len();

        let mut rock = Rock {
            shape: SHAPES[rock_pos]
                .iter()
                .map(|c| (c.0 + 2, c.1 + bottom + 3))
                .collect(),
        };
        // println!(">> shape {:?}", rock.shape);

        loop {
            match ins.chars().nth(dir_pos).unwrap() {
                '<' => {
                    let (new_shape, can_move) = rock.move_left(&chamber);
                    // println!("moving left {}", s);
                    if can_move {
                        rock.shape = new_shape;
                    }
                }
                '>' => {
                    // println!("moving right {}", s);
                    let (new_shape, can_move) = rock.move_right(&chamber);
                    if can_move {
                        rock.shape = new_shape;
                    }
                }
                _ => unreachable!(),
            }
            s += 1;

            let (new_shape, can_move) = rock.move_down(&chamber);
            if !can_move {
                let (_, _, _, ymax) = rock.bounds();
                bottom = max(ymax + 1, bottom);
                new_shape.iter().for_each(|c| {
                    chamber.insert(*c);
                });
                // rocks.push(new_shape);
                i += 1;
                // println!("-- new rock. next sign {:?}", s);
                // display(&chamber, bottom);
                break;
            } else {
                rock.shape = new_shape;
            }
        }

        let combo = (rock_pos, dir_pos, floor_relative_pattern(&chamber));
        if !has_been_seen {
            p = combo;
        }
        if state.contains_key(&combo) {
            has_been_seen = true;
            // if combo == p {
            println!(
                "ALREADY SEEN {} {} => {} ({}) - {} ({})",
                combo.0,
                combo.1,
                bottom,
                bottom - state[&combo].0,
                i,
                i - state[&combo].1,
            );
            c += 1;
            if c > 5 {
                break;
            }
            // }
        };
        state.insert(combo, (bottom, i));
    }

    bottom
}

#[aoc::main("test")]
fn main(input: &str) -> (usize, usize) {
    let p1 = part1(&input);
    let p2 = part2(&input);
    (p1, p2)
}
