use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct Valve<'a> {
    flow_rate: u64,
    exit_valves: Vec<&'a str>,
}

fn parse_input(input: &str) -> HashMap<&str, Valve> {
    let re_valves =
        Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (.+)")
            .unwrap();
    re_valves
        .captures_iter(&input)
        .fold(HashMap::new(), |mut valves, c| {
            let name = c.get(1).map_or("", |m| m.as_str());
            let flow_rate = c.get(2).map_or(0, |m| m.as_str().parse::<u64>().unwrap());
            let exit_valves = c.get(3).map_or("", |m| m.as_str());
            let exit_valves = exit_valves.split(", ").collect::<Vec<&str>>();
            valves.insert(
                name,
                Valve {
                    flow_rate,
                    exit_valves,
                },
            );
            valves
        })
}

/// Time needed to move to and activate a non-zero flow valve from a given starting valve.
fn get_times_to_open_valves_from<'a>(
    valves: &'a HashMap<&str, Valve>,
    non_zero_valves: &HashSet<&str>,
    start: &'a str,
) -> HashMap<&'a str, u64> {
    let mut queue: VecDeque<(u64, &str)> = VecDeque::new();
    let mut visited: HashSet<&str> = HashSet::new();

    let mut times: HashMap<&str, u64> = HashMap::new();
    queue.push_back((0, start));
    visited.insert(start);

    while !queue.is_empty() {
        let (time, valve) = queue.pop_front().unwrap();
        if non_zero_valves.contains(&valve) {
            times.insert(valve, time + 1);
        }
        // add exit valves to queue
        valves[&valve].exit_valves.iter().for_each(|id| {
            if !visited.contains(id) {
                visited.insert(id);
                queue.push_back((time + 1, id));
            }
        });
    }
    times
}

/// All times to go to a valve with non-zero flow rate and opening it.
fn get_opening_times_map<'a>(
    valves: &'a HashMap<&'a str, Valve>, // ) -> HashMap<String, HashMap<String, u64>> {
) -> HashMap<&'a str, HashMap<&'a str, u64>> {
    let non_zero_valves = valves
        .iter()
        .filter(|(id, v)| *id == &"AA" || v.flow_rate > 0)
        .map(|(id, _)| *id)
        .collect::<HashSet<&str>>();

    let mut path_times: HashMap<&str, HashMap<&str, u64>> = HashMap::new();
    // // Find the activation times for other valid valves for each valid valve
    for valve in non_zero_valves.iter() {
        let downstream_path_opening_times =
            get_times_to_open_valves_from(valves, &non_zero_valves, valve);
        path_times.insert(valve, downstream_path_opening_times);
    }
    path_times
}

/// Paths we can take in the allowed time when starting from a given valve.
fn all_downstream_paths<'a>(
    start_valve: &'a str,
    valve_activation_times: &HashMap<&'a str, HashMap<&'a str, u64>>,
    max_minutes: u64,
) -> Vec<Vec<&'a str>> {
    let mut possible_paths: Vec<Vec<&str>> = vec![];
    downstream_paths_recursive(
        vec![start_valve],
        max_minutes,
        &mut possible_paths,
        valve_activation_times,
    );
    possible_paths
}

fn downstream_paths_recursive<'b>(
    current_path: Vec<&'b str>,
    max_minutes: u64,
    possible_paths: &mut Vec<Vec<&'b str>>,
    path_times: &HashMap<&'b str, HashMap<&'b str, u64>>,
) {
    let current_valve = current_path.last().unwrap();
    for next_valve in path_times.keys() {
        // Look up the activation time
        let activation_time = *path_times
            .get(current_valve)
            .unwrap()
            .get(next_valve)
            .unwrap();
        // Check if the next valve represents a valid move
        if current_path.contains(next_valve) || activation_time >= max_minutes {
            continue;
        }
        // Form the new path
        let mut new_path = current_path.clone();
        new_path.push(next_valve.clone());
        // Keep finding new paths
        downstream_paths_recursive(
            new_path,
            max_minutes - activation_time,
            possible_paths,
            path_times,
        );
    }
    possible_paths.push(current_path);
}

fn part1(valves: &HashMap<&str, Valve>) -> u64 {
    const TOTAL_MINUTES: u64 = 30;
    let path_times = get_opening_times_map(&valves);

    let all_paths = all_downstream_paths("AA", &path_times, TOTAL_MINUTES);

    all_paths
        .iter()
        .map(|path| {
            let mut remaining_time = TOTAL_MINUTES;
            let mut pressure_per_minute = 0;
            let mut total_pressure_released = 0;
            // We do not move to first valve, so no need to count 1min to go there
            for i in 1..path.len() {
                let time_to_open_valve =
                    path_times.get(&path[i - 1]).unwrap().get(&path[i]).unwrap();
                // Sum up pressure released while travelling to and opening valve
                total_pressure_released += pressure_per_minute * time_to_open_valve;
                // Add new valve's flow rate to the pressure released per minute
                pressure_per_minute += valves.get(&path[i]).unwrap().flow_rate;
                // We used up the time to open valve
                remaining_time -= time_to_open_valve;
            }
            // Use up the remaining time to release pressure
            total_pressure_released += pressure_per_minute * remaining_time;
            total_pressure_released
        })
        .max()
        .unwrap()
}

/// Possible paths the elephant can choose for a given path we take
fn elephant_paths<'e>(
    start_valve: &'e str,
    you_path: &Vec<&'e str>,
    path_times: &HashMap<&'e str, HashMap<&'e str, u64>>,
    max_minutes: u64,
) -> Vec<Vec<&'e str>> {
    let mut elephant_paths: Vec<Vec<&str>> = vec![];
    elephant_paths_recursive(
        you_path,
        vec![start_valve],
        max_minutes,
        &mut elephant_paths,
        path_times,
    );
    elephant_paths
}

fn elephant_paths_recursive<'e>(
    you_path: &Vec<&'e str>,
    current_path: Vec<&'e str>,
    max_minutes: u64,
    possible_paths: &mut Vec<Vec<&'e str>>,
    path_times: &HashMap<&'e str, HashMap<&'e str, u64>>,
) {
    // Get reference to the current valve - the last valve of the current path
    let current_valve = current_path.last().unwrap();
    for next_valve in path_times.keys() {
        // Get the activation time for the next valve from the current valve
        let time_to_open_valve = *path_times
            .get(current_valve)
            .unwrap()
            .get(next_valve)
            .unwrap();

        if current_path.contains(next_valve)
            || you_path.contains(next_valve)
            || time_to_open_valve >= max_minutes
        {
            continue;
        }

        let mut new_path = current_path.clone();
        new_path.push(next_valve.clone());
        elephant_paths_recursive(
            you_path,
            new_path,
            max_minutes - time_to_open_valve,
            possible_paths,
            path_times,
        );
    }
    // No more possible moves from the current path so add the current path to the possible paths
    possible_paths.push(current_path);
}

fn part2(valves: &HashMap<&str, Valve>) -> u64 {
    const TOTAL_MINUTES: u64 = 26;
    let path_times = get_opening_times_map(&valves);

    let all_paths = all_downstream_paths("AA", &path_times, TOTAL_MINUTES);

    all_paths
        .iter()
        .flat_map(|path| {
            let mut remaining_time = TOTAL_MINUTES;
            let mut pressure_per_minute = 0;
            let mut you_pressure_released = 0;
            // Idem than part 1, pressure release by you
            for i in 1..path.len() {
                let time_to_open_valve =
                    path_times.get(&path[i - 1]).unwrap().get(&path[i]).unwrap();
                you_pressure_released += pressure_per_minute * time_to_open_valve;
                pressure_per_minute += valves.get(&path[i]).unwrap().flow_rate;
                remaining_time -= time_to_open_valve;
            }
            you_pressure_released += pressure_per_minute * remaining_time;

            let elephant_paths = elephant_paths("AA", path, &path_times, TOTAL_MINUTES);

            // For each pressure we release, add all the possible pressure release by the elephant
            // based on the possible elephant paths.
            // Same code than for us basically.
            elephant_paths
                .iter()
                .map(|elephant_path| {
                    let mut remaining_time = TOTAL_MINUTES;
                    let mut pressure_per_minute = 0;
                    let mut elephant_pressure_released = 0;
                    for i in 1..elephant_path.len() {
                        let time_to_open_valve = path_times
                            .get(&elephant_path[i - 1])
                            .unwrap()
                            .get(&elephant_path[i])
                            .unwrap();
                        elephant_pressure_released += pressure_per_minute * time_to_open_valve;
                        pressure_per_minute += valves.get(&elephant_path[i]).unwrap().flow_rate;
                        remaining_time -= time_to_open_valve;
                    }
                    elephant_pressure_released += pressure_per_minute * remaining_time;
                    you_pressure_released + elephant_pressure_released
                })
                .collect::<Vec<u64>>()
        })
        .max()
        .unwrap()
}

#[aoc::main()]
fn main(input: &str) -> (u64, u64) {
    let valves = parse_input(&input);
    let p1 = part1(&valves);
    let p2 = part2(&valves);
    (p1, p2)
}
