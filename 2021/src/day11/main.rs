use std::path::Path;
use std::fs;
use std::collections::{HashMap, HashSet};
use ansi_term::Colour;
use ansi_term::{ANSIString, ANSIStrings};

fn parse_input(content: &str) -> HashMap<(usize, usize), usize> {

    let mut map = HashMap::new();
    
    const RADIX: u32 = 10;

    content
        .lines()
        .enumerate()
        .for_each(|(y, line)| line.chars()
             .enumerate()
             .for_each(|(x, v)| {
                    let value = v.to_digit(RADIX).unwrap() as usize;
                    map.insert((x, y), value);
             }));
    map
}

fn show(m: &HashMap<(usize, usize), usize>, highlight: usize) {
    let mut lines: Vec<ANSIString> = Vec::new();
    for y in 0..10 {
        for x in 0..10 {
            match m.get(&(x, y)) {
                Some(i) => {
                    if *i == highlight {
                        lines.push(Colour::Red.paint(i.to_string()))
                    } else {
                        lines.push(Colour::White.paint(i.to_string()))
                    }
                },
                _ => ()
            }
        }
        lines.push(Colour::White.paint("\n"))
    }
    println!("{}", ANSIStrings(&lines));
}

fn get_neighbors(coords: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    neighbors.push((coords.0, coords.1 + 1));
    neighbors.push((coords.0 + 1, coords.1));
    neighbors.push((coords.0 + 1, coords.1 + 1));
    // prevent panic overflow for unsigned
    match (coords.0 != 0, coords.1 != 0) {
        (false, true) => {
            neighbors.push((coords.0, coords.1 - 1));
            neighbors.push((coords.0 + 1, coords.1 - 1));
        },
        (true, false) => {
            neighbors.push((coords.0 - 1, coords.1));
            neighbors.push((coords.0 - 1, coords.1 + 1));
        },
        (true, true) => {
            neighbors.push((coords.0, coords.1 - 1));
            neighbors.push((coords.0 + 1, coords.1 - 1));
            neighbors.push((coords.0 - 1, coords.1));
            neighbors.push((coords.0 - 1, coords.1 + 1));
            neighbors.push((coords.0 - 1, coords.1 - 1));
        },
        (_, _) => (),
    }
    neighbors
}

fn cascade_flashes(m: &mut HashMap<(usize, usize), usize>, flashed: &mut HashSet<(usize, usize)>, new_flashing: HashSet<(usize, usize)>) {

    // update flashed
    new_flashing.iter()
        .for_each(|c| {flashed.insert(*c);});

    // store the flashes due to neighbors flashing
    let mut to_flash = HashSet::new();

    new_flashing.iter()
        .for_each(|c| {
            let neighbors = get_neighbors(*c);
            neighbors.iter()
                .for_each(|n| { 
                    if !flashed.contains(n) && !to_flash.contains(n) {
                        let value = m.get(n);
                        match value {
                            Some(value) => {
                                let new_value = value+1;
                                m.insert(*n, new_value);
                                match new_value > 9 {
                                    true => {
                                        to_flash.insert(*n);
                                        flashed.insert(*n);
                                    },
                                    false => (),
                                }
                            },
                            None => (),
                        }
                    }
                });
        });

    if to_flash.len() > 0 {
        cascade_flashes(m, flashed, to_flash);
    }

}

fn run_step(m: &mut HashMap<(usize, usize), usize>) -> usize {
    
    const LEN_X: usize = 10;
    const LEN_Y: usize = 10;
    // Increase all by 1
    for y in 0..LEN_Y {
        for x in 0..LEN_X {
            match m.get(&(x, y)) {
                Some(v) => {
                    let new = v+1;
                    m.insert((x, y), new);
                },
                _ => (),
            }
        }
    }

    // Get flashing octopuses
    let mut new_flashing = HashSet::new();
    m.into_iter()
        .filter(|(_, v)| **v > 9)
        .for_each(|(c, _)| {new_flashing.insert(*c);});
    
    // println!(">> {:?}", flashing);
    let mut flashed = HashSet::new();
    cascade_flashes(m, &mut flashed, new_flashing);

    // Post flash cleanup
    let mut n_flash: usize = 0;
    for y in 0..LEN_Y {
        for x in 0..LEN_X {
            match m.get(&(x, y)) {
                Some(v) => {
                    if *v > 9 {
                        m.insert((x, y), 0);
                        n_flash += 1;
                    }
                },
                _ => (),
            }
        }
    }
    n_flash
}

fn part1(input: &str) -> usize {
    let mut octopuses = parse_input(input);
    
    const N_STEP: usize = 100;

    let mut count:usize = 0;
    for _step  in 0..N_STEP {
        count += run_step(&mut octopuses);
    }
    count
}

fn part2(input: &str) -> usize {
    let mut octopuses = parse_input(input);
    
    let mut step: usize = 0;
    let mut stop = false;
    while !stop {
        let count = run_step(&mut octopuses);
        stop = count == 100;
        step += 1;
        // show(&octopuses, 0);
    }
    step
}

fn main() {
    let path = Path::new("./inputs/day11.txt");
    let input = fs::read_to_string(path).expect("Unable to read file");

    let p1 = part1(&input);
    println!("{}", p1);
    let p2 = part2(&input);
    println!("{}", p2);
}
