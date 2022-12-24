use hashbrown::HashSet;
use itertools::Itertools;

type Pos = (usize, usize);

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
enum Blizzard {
    Left,
    Right,
    Up,
    Down,
}

impl Blizzard {
    fn next_pos(&self, pos: &Pos, bounds: &Pos) -> Pos {
        match self {
            Blizzard::Left => {
                if pos.0 > 1 {
                    (pos.0 - 1, pos.1)
                } else {
                    (bounds.0 - 2, pos.1)
                }
            }
            Blizzard::Right => {
                if pos.0 < bounds.0 - 2 {
                    (pos.0 + 1, pos.1)
                } else {
                    (1, pos.1)
                }
            }
            Blizzard::Up => {
                if pos.1 > 1 {
                    (pos.0, pos.1 - 1)
                } else {
                    (pos.0, bounds.1 - 2)
                }
            }
            Blizzard::Down => {
                if pos.1 < bounds.1 - 2 {
                    (pos.0, pos.1 + 1)
                } else {
                    (pos.0, 1)
                }
            }
        }
    }
}

fn parse_input(input: &str) -> (HashSet<(Pos, Blizzard)>, Pos, Pos) {
    let mut blizzards = HashSet::new();
    let (mut start, mut exit) = (0, 0);
    let y_max = input.lines().count();
    let x_max = input.lines().next().unwrap().chars().count();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            '<' => {
                blizzards.insert(((x, y), Blizzard::Left));
            }
            '>' => {
                blizzards.insert(((x, y), Blizzard::Right));
            }
            '^' => {
                blizzards.insert(((x, y), Blizzard::Up));
            }
            'v' => {
                blizzards.insert(((x, y), Blizzard::Down));
            }
            v => {
                if y == 0 && v == '.' {
                    start = x;
                };
                if y == (y_max - 1) && v == '.' {
                    exit = x;
                }
            }
        })
    });
    (blizzards, (x_max, y_max), (start, exit))
}

fn get_possible_moves(pos: &Pos, bounds: &Pos, target_spot: &Pos) -> Vec<Pos> {
    let (x_max, y_max) = bounds;

    // staying put is a possibility
    let mut neighbors = vec![(pos.0, pos.1)];

    // exception exit point
    if pos.0 == target_spot.0 {
        let sided = match target_spot.1 > pos.1 {
            true => target_spot.1 - pos.1 == 1,
            false => pos.1 - target_spot.1 == 1,
        };
        if sided {
            neighbors.push((target_spot.0, target_spot.1));
        }
    }

    if pos.1 > 0 && pos.1 != y_max - 1 {
        if pos.0 > 1 {
            neighbors.push((pos.0 - 1, pos.1));
        };
        if pos.0 < x_max - 2 {
            neighbors.push((pos.0 + 1, pos.1));
        };
    }
    if pos.1 > 1 {
        neighbors.push((pos.0, pos.1 - 1));
    };
    if pos.1 < y_max - 2 {
        neighbors.push((pos.0, pos.1 + 1));
    };
    neighbors
}

fn draw(board: &HashSet<(Pos, Blizzard)>, bounds: &Pos, current: &Vec<Pos>) {
    let grouped = board
        .clone()
        .into_iter()
        .into_group_map()
        .into_iter()
        .map(|(pos, blizzards)| match blizzards.len() {
            1 => match blizzards[0] {
                Blizzard::Left => (pos, "<".to_string()),
                Blizzard::Right => (pos, ">".to_string()),
                Blizzard::Up => (pos, "^".to_string()),
                Blizzard::Down => (pos, "v".to_string()),
            },
            _ => {
                let num = blizzards.len().to_string();
                // let num = &num.as_str();

                (pos, num)
            }
        })
        .collect::<HashSet<(Pos, String)>>();

    let grouped_pos = grouped
        .iter()
        .map(|(pos, _)| pos)
        .collect::<HashSet<&Pos>>();

    (0..bounds.1).for_each(|y| {
        (0..bounds.0).for_each(|x| {
            if current.contains(&&(x, y)) {
                print!("o")
            } else {
                if y == 0 || y == bounds.1 - 1 || x == 0 || x == bounds.0 - 1 {
                    print!("#")
                } else {
                    if grouped_pos.contains(&(x, y)) {
                        let sign = &grouped.iter().find(|(p, _)| p == &(x, y)).unwrap().1;
                        print!("{sign}")
                    } else {
                        print!(".")
                    };
                };
            }
        });

        print!("\n");
    });
}

fn run(blizzards: &HashSet<(Pos, Blizzard)>, bounds: &Pos) -> HashSet<(Pos, Blizzard)> {
    blizzards
        .iter()
        .map(|(pos, blizzard)| (blizzard.next_pos(pos, bounds), *blizzard))
        .collect::<HashSet<_>>()
}

fn part1(blizzards: &mut HashSet<(Pos, Blizzard)>, bounds: &Pos, setup: &Pos) -> usize {
    let (x_start, x_exit) = setup;
    let current_pos = (*x_start, 0);
    let mut current = vec![current_pos];
    let exit_spot = (*x_exit, bounds.1 - 1);

    let mut time = 0;

    loop {
        *blizzards = run(blizzards, bounds);
        let blizzards_positions = blizzards
            .iter()
            .map(|(pos, _)| pos)
            .collect::<HashSet<&Pos>>();

        let neighbors = current
            .iter()
            .map(|p| get_possible_moves(p, bounds, &exit_spot))
            .collect::<Vec<Vec<Pos>>>();

        let possible = neighbors
            .into_iter()
            .flat_map(|x| x)
            .unique()
            .filter(|p| !blizzards_positions.contains(p))
            .collect::<Vec<Pos>>();

        current = possible;
        time += 1;
        if current.contains(&&exit_spot) {
            break;
        }
    }
    time
}

fn part2(blizzards: &mut HashSet<(Pos, Blizzard)>, bounds: &Pos, setup: &Pos) -> usize {
    let (x_start, x_exit) = setup;
    let start_spot = (*x_start, 0);
    let exit_spot = (*x_exit, bounds.1 - 1);

    let mut current: Vec<Pos> = vec![];

    let mut time = 0;

    [
        start_spot, exit_spot, exit_spot, start_spot, start_spot, exit_spot,
    ]
    .into_iter()
    .tuples()
    .for_each(|(s, exit)| {
        current = vec![s];

        loop {
            *blizzards = run(blizzards, bounds);
            let blizzards_positions = blizzards
                .iter()
                .map(|(pos, _)| pos)
                .collect::<HashSet<&Pos>>();

            let neighbors = current
                .iter()
                .map(|p| get_possible_moves(p, bounds, &exit))
                .collect::<Vec<Vec<Pos>>>();

            let possible = neighbors
                .into_iter()
                .flat_map(|x| x)
                .unique()
                .filter(|p| !blizzards_positions.contains(p))
                .collect::<Vec<Pos>>();

            current = possible;

            time += 1;

            // println!("======= Time {time}");
            // draw(blizzards, bounds, &current);

            if current.contains(&&exit) {
                break;
            }
        }
    });
    time
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let (blizzards, bounds, setup) = parse_input(input);
    let p1 = part1(&mut blizzards.clone(), &bounds, &setup);
    let p2 = part2(&mut blizzards.clone(), &bounds, &setup);
    (p1, p2)
}
