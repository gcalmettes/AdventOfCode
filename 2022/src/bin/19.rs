use hashbrown::HashMap;
use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::collections::VecDeque;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Mineral {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

// The value is the mineral it produces
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Robot {
    Ore(Mineral),
    Clay(Mineral),
    Obsidian(Mineral),
    Geode(Mineral),
}

type Blueprint = HashMap<Robot, HashMap<Mineral, u16>>;
type Pack = HashMap<Robot, u16>;
type Stock = HashMap<Mineral, u16>;

fn parse_input(input: &str) -> Vec<(u16, Blueprint)> {
    let re = Regex::new(r"Each (\w+) robot costs (\d+) (\w+)[.|\s](?:and (\d+) (\w+))?").unwrap();
    let mut blueprints: Vec<(u16, Blueprint)> = vec![];
    input.lines().for_each(|line| {
        let mut blueprint: Blueprint = HashMap::new();
        let (id_part, _) = line.split_once(": ").unwrap();
        let (_, id) = id_part.split_once(" ").unwrap();
        let id = id.parse::<u16>().unwrap();

        re.captures_iter(&line).for_each(|c| {
            let robot = c.get(1).unwrap().as_str();
            let mut price: HashMap<Mineral, u16> = HashMap::new();
            c.iter().skip(2).tuples().for_each(|(v1, v2)| {
                if let Some(q) = v1 {
                    let quantity = q.as_str().parse::<u16>().unwrap();

                    if let Some(n) = v2 {
                        match n.as_str() {
                            "ore" => price.insert(Mineral::Ore, quantity),
                            "clay" => price.insert(Mineral::Clay, quantity),
                            "obsidian" => price.insert(Mineral::Obsidian, quantity),
                            _ => unreachable!(),
                        };
                    } else {
                        panic!("Could not parse the data {:?} {:?}", v1, v2);
                    };
                } else {
                    ();
                };
            });

            let robot = match robot {
                "ore" => Robot::Ore(Mineral::Ore),
                "clay" => Robot::Clay(Mineral::Clay),
                "obsidian" => Robot::Obsidian(Mineral::Obsidian),
                "geode" => Robot::Geode(Mineral::Geode),
                _ => unreachable!(),
            };
            blueprint.insert(robot, price);
        });
        blueprints.push((id, blueprint));
    });
    blueprints
}

fn need_more(time_left: u16, current_stock: u16, current_production: u16, max: u16) -> bool {
    !(current_production >= max || time_left * current_production + current_stock > time_left * max)
}

fn get_best_geode_for(blueprint: &Blueprint, time: u16) -> u16 {
    let mut to_check: VecDeque<(Pack, Pack, Stock, u16)> = VecDeque::new();

    // we start with 1 Ore robot in our pack
    let starting_pack: Pack = HashMap::from_iter([
        (Robot::Ore(Mineral::Ore), 1),
        (Robot::Clay(Mineral::Clay), 0),
        (Robot::Obsidian(Mineral::Obsidian), 0),
        (Robot::Geode(Mineral::Geode), 0),
    ]);
    let starting_pending: HashMap<Robot, u16> = HashMap::from_iter([
        (Robot::Ore(Mineral::Ore), 0),
        (Robot::Clay(Mineral::Clay), 0),
        (Robot::Obsidian(Mineral::Obsidian), 0),
        (Robot::Geode(Mineral::Geode), 0),
    ]);
    // and no mineral ... yet !
    let starting_minerals: Stock = HashMap::from_iter([
        (Mineral::Ore, 0),
        (Mineral::Clay, 0),
        (Mineral::Obsidian, 0),
        (Mineral::Geode, 0),
    ]);

    // add starting state
    to_check.push_back((starting_pack, starting_pending, starting_minerals, time));

    // The robot factory can only produce 1 robot per turn,
    // so no need to produce more minerals than the max that can
    // be used.
    let mut max_needed_minerals = HashMap::new();
    blueprint.values().for_each(|cost| {
        cost.into_iter().for_each(|(mineral, &quantity)| {
            let state_minerals_max = max_needed_minerals.entry(*mineral).or_insert(quantity);
            if cost[mineral] > *state_minerals_max {
                *max_needed_minerals.get_mut(mineral).unwrap() = cost[mineral];
            };
        });
    });

    let mut best_geodes = 0;
    while let Some((pack, pending, minerals, time)) = to_check.pop_front() {
        if time == 0 {
            continue;
        };
        // Continue only if it is potentially a path equally performing than the current best state
        // or if a similar state is not already present in the paths to continue.
        // Because we compare before the production of the turn, we check the best minus 1.
        if minerals[&Mineral::Geode]
            + pack[&Robot::Geode(Mineral::Geode)] * 2 * (24 - 1 - (24 - time))
            < best_geodes
            || to_check.contains(&(pack.clone(), pending.clone(), minerals.clone(), time))
        {
            continue;
        }
        // if minerals[&Mineral::Geode] < best_geodes.max(1) - 1
        //     || to_check.contains(&(pack.clone(), pending.clone(), minerals.clone(), time))
        // {
        //     continue;
        // }

        // Each turn we can do 5 actions.
        // Creating one type of robot or do nothing.
        // By order or importance
        [
            Some(Robot::Geode(Mineral::Geode)),
            Some(Robot::Obsidian(Mineral::Obsidian)),
            Some(Robot::Clay(Mineral::Clay)),
            Some(Robot::Ore(Mineral::Ore)),
            None,
        ]
        .iter()
        .for_each(|robot| {
            if let Some(robot) = robot {
                let need_more = match robot {
                    Robot::Geode(_) => true,
                    Robot::Obsidian(c) => {
                        need_more(time, minerals[c], pack[robot], max_needed_minerals[c])
                    }
                    Robot::Clay(c) => {
                        need_more(time, minerals[c], pack[robot], max_needed_minerals[c])
                    }
                    Robot::Ore(c) => {
                        need_more(time, minerals[c], pack[robot], max_needed_minerals[c])
                    }
                };

                // If we can produce a robot, produce it
                let new_state = if need_more {
                    // check if we can create it
                    let cost = &blueprint[robot];
                    let can_create = cost
                        .iter()
                        .all(|(mineral, quantity)| minerals[mineral] >= *quantity);

                    if can_create {
                        let (mut pending, mut minerals) = (pending.clone(), minerals.clone());
                        *pending.get_mut(robot).unwrap() += 1;
                        // pay the price
                        cost.iter().for_each(|(mineral, quantity)| {
                            *minerals.get_mut(mineral).unwrap() -= quantity;
                        });
                        Some((pack.clone(), pending, minerals, time))
                    } else {
                        None
                    }
                } else {
                    None
                };

                match new_state {
                    Some((mut pack, mut pending, mut minerals, time)) => {
                        // add production of this round to mineral stock
                        pack.iter().for_each(|(robot, quantity)| {
                            let mineral = match robot {
                                Robot::Ore(c) => c,
                                Robot::Clay(c) => c,
                                Robot::Obsidian(c) => c,
                                Robot::Geode(c) => c,
                            };
                            *minerals.get_mut(mineral).unwrap() += *quantity;
                        });

                        best_geodes = best_geodes.max(minerals[&Mineral::Geode]);

                        // add pending robot to pack
                        pending.clone().into_iter().for_each(|(robot, quantity)| {
                            *pack.get_mut(&robot).unwrap() += quantity;
                            *pending.get_mut(&robot).unwrap() = 0;
                        });

                        // Potentially update current best, now that minerals production has
                        // occured.
                        best_geodes = best_geodes.max(minerals[&Mineral::Geode]);
                        // Continue with this path only if it is at least equal to the current best state
                        // or if a similar state is not already present in the paths to continue
                        if !(minerals[&Mineral::Geode] < best_geodes
                            || to_check.contains(&(
                                pack.clone(),
                                pending.clone(),
                                minerals.clone(),
                                time,
                            )))
                        {
                            to_check.push_back((pack, pending, minerals, time - 1));
                        };
                    }
                    None => (),
                }
            } else {
                // We don't do anything in this round except producing minerals.
                let (mut pack, mut pending, mut minerals, time) =
                    (pack.clone(), pending.clone(), minerals.clone(), time);

                // add production of this round to mineral stock
                pack.iter().for_each(|(robot, quantity)| {
                    let mineral = match robot {
                        Robot::Ore(c) => c,
                        Robot::Clay(c) => c,
                        Robot::Obsidian(c) => c,
                        Robot::Geode(c) => c,
                    };
                    *minerals.get_mut(mineral).unwrap() += *quantity;
                });

                // add pending robot to pack
                pending.clone().into_iter().for_each(|(robot, quantity)| {
                    *pack.get_mut(&robot).unwrap() += quantity;
                    *pending.get_mut(&robot).unwrap() = 0;
                });

                // Potentially update current best, now that minerals production has
                // occured.
                best_geodes = best_geodes.max(minerals[&Mineral::Geode]);
                // Continue with this path only if it is at least equal to the current best state
                // or if a similar state is not already present in the paths to continue
                if !(minerals[&Mineral::Geode] < best_geodes
                    || to_check.contains(&(pack.clone(), pending.clone(), minerals.clone(), time)))
                {
                    to_check.push_back((pack, pending, minerals, time - 1));
                };
            }
            // println!("Queue length: {}", to_check.len());
        });
    }
    best_geodes
}

fn part1(blueprints: &Vec<(u16, Blueprint)>) -> u16 {
    const STARTING_TIME: u16 = 24;
    blueprints
        .par_iter()
        .map(|(id, blueprint)| {
            let geodes = get_best_geode_for(blueprint, STARTING_TIME);
            println!("{} >> {}", id, geodes * id);
            geodes * id
        })
        .sum()
}

fn part2(blueprints: &Vec<(u16, Blueprint)>) -> u16 {
    const STARTING_TIME: u16 = 32;
    blueprints
        .par_iter()
        .take(3)
        .map(|(id, blueprint)| {
            let geodes = get_best_geode_for(blueprint, STARTING_TIME);
            println!("{} >> {}", id, geodes * id);
            geodes * id
        })
        .product()
}

#[aoc::main()]
fn main(input: &str) -> (u16, u16) {
    let blueprints = parse_input(input);
    // let p1 = part1(&blueprints);
    let p1 = 0;
    let p2 = part2(&blueprints);
    (p1, p2)
}

//3080
