use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use std::cmp::{max, min};

#[derive(Debug, Clone)]
enum Material {
    Wall(char),
    Sand(char),
}

type Pos = (i32, i32);

fn parse_input(input: &str) -> HashMap<Pos, Material> {
    let mut map = HashMap::new();
    let walls = input
        .lines()
        .flat_map(|line| {
            let abc = line
                .split(" -> ")
                .scan((-1, -1), |state, pos| {
                    let (x, y) = if let Some((x, y)) = pos
                        .split(",")
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect_tuple()
                    {
                        (x, y)
                    } else {
                        todo!()
                    };

                    // not first iteration
                    let coords = if state.0 > 0 {
                        match x - state.0 {
                            // same x, so y are different
                            0 => (min(state.1, y)..=max(state.1, y))
                                .map(|ny| (x, ny))
                                .collect::<Vec<(i32, i32)>>(),
                            // x are different
                            _ => (min(state.0, x)..=max(state.0, x))
                                .map(|nx| (nx, y))
                                .collect::<Vec<(i32, i32)>>(),
                        }
                    } else {
                        vec![]
                    };
                    *state = (x, y);
                    Some(coords)
                })
                .flat_map(|x| x)
                .collect::<Vec<(i32, i32)>>();
            abc
        })
        .unique()
        .collect::<Vec<(i32, i32)>>();

    walls.into_iter().for_each(|pos| {
        map.insert(pos, Material::Wall('█'));
    });
    map
}

fn display(map: &HashMap<Pos, Material>) -> String {
    let xmin = map.keys().map(|(a, _)| a).min().unwrap();
    let xmax = map.keys().map(|(a, _)| a).max().unwrap();
    let ymin = map.keys().map(|(_, b)| b).min().unwrap();
    let ymax = map.keys().map(|(_, b)| b).max().unwrap();

    let mut d = "".to_string();

    (*ymin..=*ymax).for_each(|y| {
        d.push('\n');
        (*xmin..=*xmax).for_each(|x| match map.get(&(x, y)) {
            Some(Material::Wall(v)) => d.push(*v),
            Some(Material::Sand(v)) => d.push(*v),
            _ => d.push(' '),
        })
    });
    d
}

fn get_boundaries(map: &mut HashMap<Pos, Material>) -> (i32, i32, i32, i32) {
    let xmin = map.keys().map(|(a, _)| a).min().unwrap();
    let xmax = map.keys().map(|(a, _)| a).max().unwrap();
    let ymin = map.keys().map(|(_, b)| b).min().unwrap();
    let ymax = map.keys().map(|(_, b)| b).max().unwrap();
    (*xmin, *xmax, *ymin, *ymax)
}

fn simulate(map: &mut HashMap<Pos, Material>, floor: bool, debug: bool) -> usize {
    let mut current_sand = (500, 0);
    let mut sand_path: Vec<Pos> = vec![];
    let (_, _, _, ymax) = get_boundaries(map);

    let mut count = 1;
    loop {
        // possible moves
        let dir = [(0, 1), (-1, 1), (1, 1)];
        if let Some(sand) = dir
            .iter()
            .map(|d| (current_sand.0 + d.0, current_sand.1 + d.1))
            .find(|p| !map.contains_key(p))
        {
            sand_path.push(current_sand);
            current_sand = sand;
        } else {
            map.insert(current_sand, Material::Sand('o'));
            current_sand = sand_path.pop().unwrap();
            count += 1;
        }

        if floor {
            // last layer before virtual floor
            if current_sand.1 == ymax + 1 {
                map.insert(current_sand, Material::Sand('o'));
                current_sand = (500, 0);
                count += 1;
            };
            match (
                current_sand == (500, 0),
                map.get(&(500, 1)),
                map.get(&(499, 1)),
                map.get(&(501, 1)),
            ) {
                (
                    true,
                    Some(Material::Sand(_)),
                    Some(Material::Sand(_)),
                    Some(Material::Sand(_)),
                ) => {
                    if debug {
                        println!("{}", display(map));
                    };
                    break;
                }
                _ => (),
            }
        } else {
            if current_sand.1 > ymax {
                if debug {
                    println!("{}", display(map));
                }
                break;
            };
        }
    }
    count - 1
}

fn simulate_p2(map: &mut HashMap<Pos, Material>) -> usize {
    let (_, _, _, ymax) = get_boundaries(map);
    // floor
    let ymax = ymax + 2;
    let mut all_sand: HashSet<(i32, i32)> = HashSet::new();
    // first row, tip of the triangle, only one possible x
    all_sand.insert((500, 0));
    let (xmin, xmax) = (500, 500);

    let (_, _, count) = (1..ymax).fold(
        (all_sand, (xmin, xmax), 1),
        |(state_sand, (row_xmin, row_xmax), count), y| {
            // we build a triangle, the base extend on both sides by 1
            let (row_xmin, row_xmax) = (row_xmin - 1, row_xmax + 1);
            let state_sand: HashSet<(i32, i32)> = (row_xmin..=row_xmax)
                // only add sand if the spot is not already occupied by a wall already,
                // or if this is a spot in which a sand grain could have actually occupied
                .filter(|x| {
                    !map.contains_key(&(*x, y)) // no wall on position
                    // we know it's a possible location for sand
                    && (
                    state_sand.contains(&(*x-1, y-1)) // sand grain present above-left
                    || state_sand.contains(&(*x+1, y-1)) // sand grain present above-right
                    || state_sand.contains(&(*x, y-1)) // sand grain present above
                    )
                })
                .map(|x| (x, y))
                .collect();
            let count = count + state_sand.len();
            (state_sand, (row_xmin, row_xmax), count)
        },
    );
    count
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    let p1 = simulate(&mut data.clone(), false, false);
    // let p2 = simulate(&mut data.clone(), true, false);
    let p2 = simulate_p2(&mut data.clone());
    (p1, p2)
}
