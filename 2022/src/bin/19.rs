use hashbrown::HashMap;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Currency {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

// The value is the currency it produces
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Robot {
    Ore(Currency),
    Clay(Currency),
    Obsidian(Currency),
    Geode(Currency),
}

type Blueprint = HashMap<Robot, HashMap<Currency, u64>>;

fn parse_input(input: &str) -> Vec<Blueprint> {
    let re = Regex::new(r"Each (\w+) robot costs (\d+) (\w+)[.|\s](?:and (\d+) (\w+))?").unwrap();
    let mut blueprints: Vec<Blueprint> = vec![];
    input.lines().for_each(|line| {
        let mut blueprint: Blueprint = HashMap::new();
        re.captures_iter(&line).for_each(|c| {
            let robot = c.get(1).unwrap().as_str();
            let mut price: HashMap<Currency, u64> = HashMap::new();
            c.iter().skip(2).tuples().for_each(|(v1, v2)| {
                if let Some(q) = v1 {
                    let quantity = q.as_str().parse::<u64>().unwrap();

                    if let Some(n) = v2 {
                        match n.as_str() {
                            "ore" => price.insert(Currency::Ore, quantity),
                            "clay" => price.insert(Currency::Clay, quantity),
                            "obsidian" => price.insert(Currency::Obsidian, quantity),
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
                "ore" => Robot::Ore(Currency::Ore),
                "clay" => Robot::Clay(Currency::Clay),
                "obsidian" => Robot::Obsidian(Currency::Obsidian),
                "geode" => Robot::Geode(Currency::Geode),
                _ => unreachable!(),
            };
            blueprint.insert(robot, price);
        });
        blueprints.push(blueprint);
    });
    blueprints
}

fn need_more(time_left: u64, current_stock: u64, current_robots: u64, max: u64) -> bool {
    !(current_robots >= max || time_left * current_robots + current_stock > time_left * max)
}

fn part1(blueprints: &Vec<Blueprint>) -> usize {
    println!("robots: {:?}", blueprints);

    let mut time = 24;

    // we start with 1 Ore robot in our pack
    let mut pack: HashMap<Robot, u64> = HashMap::from_iter([
        (Robot::Ore(Currency::Ore), 1),
        (Robot::Clay(Currency::Clay), 0),
        (Robot::Obsidian(Currency::Obsidian), 0),
        (Robot::Geode(Currency::Geode), 0),
    ]);
    let mut pending: HashMap<Robot, u64> = HashMap::from_iter([
        (Robot::Ore(Currency::Ore), 0),
        (Robot::Clay(Currency::Clay), 0),
        (Robot::Obsidian(Currency::Obsidian), 0),
        (Robot::Geode(Currency::Geode), 0),
    ]);
    // and no money ... yet !
    let mut money: HashMap<Currency, u64> = HashMap::from_iter([
        (Currency::Ore, 0),
        (Currency::Clay, 0),
        (Currency::Obsidian, 0),
        (Currency::Geode, 0),
    ]);

    let blueprint = &blueprints[0];

    // The robot factory can only produce 1 robot per turn,
    // so no need to produce more currency than the max that can
    // be used.
    let mut max_needed_currency = HashMap::new();
    blueprint.values().for_each(|price| {
        price.into_iter().for_each(|(currency, &quantity)| {
            let state_currency_max = max_needed_currency.entry(*currency).or_insert(quantity);
            if price[currency] > *state_currency_max {
                *max_needed_currency.get_mut(currency).unwrap() = price[currency];
            };
        });
    });

    while time > 0 {
        // how many of each currency are we producing per turn ?

        // let mut currency_produced: HashMap<Currency, u64> = HashMap::new();
        // pack.iter().for_each(|(robot, quantity)| {
        //     match robot {
        //         Robot::Ore(c) => {
        //             currency_produced.insert(*c, *quantity);
        //         }
        //         Robot::Clay(c) => {
        //             currency_produced.insert(*c, *quantity);
        //         }
        //         Robot::Obsidian(c) => {
        //             currency_produced.insert(*c, *quantity);
        //         }
        //         _ => (),
        //     };
        // });

        // println!("PER ROUND: {:?}", currency_produced);

        // can we create new robots, in the order of importance ?
        [
            Robot::Geode(Currency::Geode),
            Robot::Obsidian(Currency::Obsidian),
            Robot::Clay(Currency::Clay),
            Robot::Ore(Currency::Ore),
        ]
        .iter()
        .for_each(|robot| {
            let need_more = match robot {
                Robot::Geode(_) => true,
                Robot::Obsidian(c) => {
                    need_more(time, money[c], pack[robot], max_needed_currency[c])
                }
                Robot::Clay(c) => {
                    // we wanrt at least 2 ore robots
                    pack[&Robot::Ore(Currency::Ore)] >= 2
                        && need_more(time, money[c], pack[robot], max_needed_currency[c])
                }
                Robot::Ore(c) => need_more(time, money[c], pack[robot], max_needed_currency[c]),
            };
            // let need_more = match robot {
            //     Robot::Geode => true,
            //     Robot::Obsidian(c) => currency_produced[c] < max_needed_currency[c],
            //     Robot::Clay(c) => currency_produced[c] < max_needed_currency[c],
            //     Robot::Ore(c) => currency_produced[c] < max_needed_currency[c],
            // };

            // if let Some(c) = match robot {
            //     Robot::Geode => None,
            //     Robot::Obsidian(c) => Some(c),
            //     Robot::Clay(c) => Some(c),
            //     Robot::Ore(c) => Some(c),
            // } {
            //     println!(
            //         "   {} >> {:?} ({}) {} ({}) {}",
            //         time,
            //         robot,
            //         pack[robot],
            //         need_more,
            //         money[c],
            //         !(pack[robot] >= max_needed_currency[c]
            //             || time * pack[robot] + money[c] > time * max_needed_currency[c])
            //     );
            // };

            // !(current_robots >= max || time_left * current_robots + current_stock > time_left * max)
            if need_more {
                // check if we can create it
                let price = &blueprint[robot];
                let can_create = price
                    .iter()
                    .all(|(currency, quantity)| money[currency] >= *quantity);
                if can_create {
                    println!("-- creating a {:?}", robot);
                    *pending.get_mut(robot).unwrap() += 1;
                    // pay the price
                    price.iter().for_each(|(currency, quantity)| {
                        *money.get_mut(currency).unwrap() -= quantity;
                    });
                }
            }
        });

        let in_pack = pack
            .iter()
            .filter(|(_robot, quantity)| quantity > &&0)
            .collect::<Vec<(&Robot, &u64)>>();

        // add production of this round to money pot
        in_pack.iter().for_each(|(robot, quantity)| {
            let currency = match robot {
                Robot::Ore(c) => c,
                Robot::Clay(c) => c,
                Robot::Obsidian(c) => c,
                Robot::Geode(c) => c,
            };
            *money.get_mut(currency).unwrap() += **quantity;
        });

        // add pending robot to pack
        pending.clone().into_iter().for_each(|(robot, quantity)| {
            *pack.get_mut(&robot).unwrap() += quantity;
            *pending.get_mut(&robot).unwrap() = 0;
        });

        time -= 1;
    }
    println!("MONEY: {:?}", money);
    println!("PACK: {:?}", pack);
    println!("MAX NEEDED: {:?}", max_needed_currency);
    0
}

// fn part2(blueprints: &Vec<Robot>) -> usize {
//     0
// }

#[aoc::main("test")]
fn main(input: &str) -> (usize, usize) {
    let blueprints = parse_input(input);
    let p1 = part1(&blueprints);
    // let p2 = part2(&blueprints);
    // (p1, p2)
    (p1, 0)
}
