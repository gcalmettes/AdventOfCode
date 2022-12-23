use hashbrown::HashSet;
use itertools::Itertools;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Elf(i64, i64);

impl Elf {
    fn relocate(&self, map: &HashSet<Elf>, dir_seq: &[(i64, i64); 4]) -> Option<Elf> {
        for (dx, dy) in dir_seq {
            let to_check: Vec<Elf> = match (dx, dy) {
                (0, y) => (-1..=1).map(|v| Elf(self.0 + v, self.1 + y)).collect(),
                (x, 0) => (-1..=1).map(|v| Elf(self.0 + x, self.1 + v)).collect(),
                _ => unreachable!(),
            };
            if to_check.iter().all(|elf| !map.contains(elf)) {
                return Some(Elf(self.0 + dx, self.1 + dy));
            }
        }
        None
    }

    fn can_stay_put(&self, map: &HashSet<Elf>) -> bool {
        ((self.0 - 1)..=(self.0 + 1))
            .cartesian_product((self.1 - 1)..=(self.1 + 1))
            .filter(|(x, y)| (x, y) != (&self.0, &self.1))
            .filter(|(x, y)| !map.contains(&Elf(*x, *y)))
            .count()
            == 8
    }
}

fn boundaries(elves: &HashSet<Elf>) -> (i64, i64, i64, i64) {
    elves
        .iter()
        .fold((i64::MAX, i64::MIN, i64::MAX, i64::MIN), |state, elf| {
            let mut new_state = state;
            if elf.0 < state.0 {
                new_state.0 = elf.0;
            };
            if elf.0 > state.1 {
                new_state.1 = elf.0;
            };
            if elf.1 < state.2 {
                new_state.2 = elf.1;
            };
            if elf.1 > state.3 {
                new_state.3 = elf.1;
            };
            new_state
        })
}

fn show(elves: &HashSet<Elf>) {
    let (min_x, max_x, min_y, max_y) = boundaries(elves);
    ((min_y - 1)..=(max_y + 1)).for_each(|y| {
        print!("\n");
        ((min_x - 1)..=(max_x) + 1).for_each(|x| {
            if elves.contains(&Elf(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        })
    });
    print!("\n");
}

fn count_empty(elves: &HashSet<Elf>) -> usize {
    let (min_x, max_x, min_y, max_y) = boundaries(elves);
    let count = (min_y..=max_y)
        .flat_map(|y| {
            (min_x..=max_x)
                .map(|x| if elves.contains(&Elf(x, y)) { 0 } else { 1 })
                .collect::<Vec<_>>()
        })
        .sum();
    count
}

fn parse_input(input: &str) -> HashSet<Elf> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| c == &'#')
                .map(|(x, _)| Elf(x as i64, y as i64))
                .collect::<Vec<Elf>>()
        })
        .collect::<HashSet<Elf>>()
}

fn simulate(elves: &HashSet<Elf>, stop_at: Option<usize>, debug: bool) -> (HashSet<Elf>, usize) {
    let mut dirs = [
        [(0, -1), (0, 1), (-1, 0), (1, 0)],
        [(0, 1), (-1, 0), (1, 0), (0, -1)],
        [(-1, 0), (1, 0), (0, -1), (0, 1)],
        [(1, 0), (0, -1), (0, 1), (-1, 0)],
    ]
    .iter()
    .cycle();

    if debug {
        println!("==== Starting positions");
        show(&elves);
    };

    let mut elves = elves.clone();

    let mut round = 1;
    loop {
        let dir_seq = dirs.next().expect("dirs are infinite");
        // println!("==== NEW ROUND {:?}", dir_seq);
        let proposed_moves = elves
            .iter()
            .map(|elf| {
                if !elf.can_stay_put(&elves) {
                    let new_pos = elf.relocate(&elves, dir_seq);
                    return match new_pos {
                        Some(moved_elf) => (moved_elf, *elf),
                        None => (*elf, *elf),
                    };
                }
                (*elf, *elf)
            })
            .into_group_map()
            .into_iter()
            .flat_map(|(new_pos, candidates)| match candidates.len() == 1 {
                true => vec![new_pos],
                false => candidates.to_vec(),
            })
            .collect::<HashSet<Elf>>();

        elves = proposed_moves;
        if debug {
            show(&elves);
        };

        match stop_at {
            Some(r) => {
                if round == r {
                    break;
                }
            }
            None => (),
        }
        if elves.iter().all(|elf| elf.can_stay_put(&elves)) {
            break;
        }
        round += 1;
    }
    (elves, round)
}

fn part1(elves: &HashSet<Elf>, debug: bool) -> usize {
    let (elves, _) = simulate(elves, Some(10), debug);
    count_empty(&elves)
}

fn part2(elves: &HashSet<Elf>, debug: bool) -> usize {
    let (_, round) = simulate(elves, None, debug);
    round
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    let p1 = part1(&data, false);
    let p2 = part2(&data, false);
    (p1, p2)
}
