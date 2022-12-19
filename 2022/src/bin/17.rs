use hashbrown::HashMap;
use std::{
    collections::{hash_map::DefaultHasher, BTreeSet},
    hash::Hasher,
};

type Pos = (i32, i32);
type Rock = Vec<Pos>;

struct Chamber {
    rocks: Vec<BTreeSet<usize>>,
    current_rock: Option<Rock>,
}

impl Chamber {
    fn new() -> Self {
        Self {
            rocks: vec![BTreeSet::new(); 7],
            current_rock: None,
        }
    }

    fn move_current_rock(&mut self, (dx, dy): Pos) {
        self.current_rock = self.current_rock.take().map(|rock| {
            rock.iter()
                .map(|(x, y)| (x + dx, y + dy))
                .collect::<Vec<Pos>>()
        });
    }

    fn insert_rock(&mut self, rock: Rock) {
        self.current_rock = Some(rock);
        self.move_current_rock((2, 0)); // 2 units from the right.
        self.move_current_rock((0, self.height() as i32 + 3)); // 3 units above the highest rock.
    }

    fn run(&mut self, op: char) {
        let offset = match op {
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => unreachable!(),
        };

        if self.can_move(offset) {
            self.move_current_rock(offset);
        }

        if !self.can_move((0, -1)) {
            self.solidify();
        } else {
            self.move_current_rock((0, -1))
        }
    }

    fn height(&self) -> usize {
        self.rocks
            .iter()
            .map(|s| s.last().map(|h| h + 1).unwrap_or(0))
            .max()
            .unwrap_or(0)
    }

    fn floor(&self) -> usize {
        self.rocks
            .iter()
            .map(|s| s.last().map(|h| h + 1).unwrap_or(0))
            .min()
            .unwrap_or(0)
    }

    fn can_move(&self, (dx, dy): Pos) -> bool {
        self.current_rock
            .as_ref()
            .map(|shape| shape.iter().all(|(x, y)| self.is_empty((x + dx, y + dy))))
            .unwrap_or(false)
    }

    fn solidify(&mut self) {
        if let Some(rock) = self.current_rock.take() {
            for (x, y) in rock.into_iter() {
                self.rocks[x as usize].insert(y as usize);
            }
        }
    }

    fn is_empty(&self, (x, y): Pos) -> bool {
        self.in_bounds((x, y)) && !self.rocks[x as usize].contains(&(y as usize))
    }

    fn in_bounds(&self, (x, y): Pos) -> bool {
        (0..7).contains(&x) && y >= 0
    }

    #[allow(unused)]
    fn draw(&self) {
        println!("{:?}", self.rocks);
        let lines = (0..self.height() + 7).rev().map(|y| {
            self.rocks
                .iter()
                .enumerate()
                .map(|(x, col)| {
                    if col.contains(&y) {
                        '#'
                    } else {
                        match &self.current_rock {
                            Some(shape) if shape.contains(&(x as i32, y as i32)) => '@',
                            _ => '.',
                        }
                    }
                })
                .collect::<String>()
        });
        println!("\n\n\n");
        for line in lines {
            println!("{}", line)
        }
    }
}

fn simulate(input: &str, mut stop_at: usize) -> usize {
    let rocks = [
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],         // h line
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], // cross
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], // L
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],         // v line
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],         // square
    ];

    let mut chamber = Chamber::new();

    // our cyclers !
    let mut input = input.chars().enumerate().cycle();
    let rocks = rocks.iter().enumerate().cycle().enumerate();

    // cache has a hash as key and (iteration number, chamber height) as value
    let mut cache: HashMap<u64, (usize, usize)> = HashMap::new();
    let mut computed_height_offset = 0;
    for (iterations, (rock_idx, rock)) in rocks {
        if iterations == stop_at {
            break;
        }

        // New rock is in play !
        chamber.insert_rock(rock.to_vec());
        let mut dir_idx = 0;
        while chamber.current_rock.is_some() {
            let (i, dir) = input.next().expect("no end as it's a cycle");
            chamber.run(dir);
            dir_idx = i;
        }

        // Now let's hash and store the height to find any state repetitions
        let mut hasher = DefaultHasher::new();
        hasher.write_usize(dir_idx); // If we are in the same move cycle
        hasher.write_usize(rock_idx); // and in the same shape cycle
        let floor = chamber.floor();
        for col in &chamber.rocks {
            hasher.write_usize(col.last().unwrap_or(&0) + 1 - floor); // and the floor looks the same
        }
        let hash = hasher.finish();

        let height = chamber.height();

        // if we found a similar item in our cache, let's calculate the cycle length and
        // and artificially run the simulation n times
        if let Some((last_iteration, last_height)) = cache.get(&hash) {
            let delta_height = height - last_height;
            let iterations_cycle = iterations - last_iteration;

            // Note: iterations starts at zero
            let repeats = (stop_at - (iterations + 1)) / iterations_cycle; //

            // make sure we need to run less more iterations than the cycle size
            if repeats > 0 {
                // we artificially decrease the number of iterations at which to stop
                // since the iterations number comes from the iterator
                stop_at -= repeats * iterations_cycle;
                computed_height_offset += repeats * delta_height;
            }
        } else {
            cache.insert(hash, (iterations, height));
        }
    }

    chamber.height() + computed_height_offset
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let p1 = simulate(&input, 2022);
    let p2 = simulate(&input, 1_000_000_000_000);
    (p1, p2)
}
