use hashbrown::HashMap;
use regex::Regex;

#[derive(Debug)]
struct Valve<'a> {
    flow_rate: u32,
    tunnels: Vec<&'a str>,
}

fn parse_input(input: &str) -> HashMap<&str, Valve> {
    let re_valves =
        Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (.+)")
            .unwrap();
    // let mut valves: HashMap<&str, Valve> = HashMap::new();
    re_valves
        .captures_iter(&input)
        .fold(HashMap::new(), |mut valves, c| {
            let name = c.get(1).map_or("", |m| m.as_str());
            let flow_rate = c.get(2).map_or(0, |m| m.as_str().parse::<u32>().unwrap());
            let tunnels = c.get(3).map_or("", |m| m.as_str());
            let tunnels = tunnels.split(", ").collect::<Vec<&str>>();
            valves.insert(name, Valve { flow_rate, tunnels });
            valves
        })
    // .collect::<Vec<Valve>>()
}

fn part1(data: &HashMap<&str, Valve>) -> usize {
    println!("data: {:?}", data);
    0
}

fn part2(data: &HashMap<&str, Valve>) -> usize {
    0
}

#[aoc::main("test")]
fn main(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    let p1 = part1(&data);
    let p2 = part2(&data);
    (p1, p2)
}
