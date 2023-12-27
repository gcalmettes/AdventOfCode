use itertools::Itertools;
use num::integer::lcm;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq)]
enum ModuleType {
    FlipFlop,
    Conjunction,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum FlipFlop {
    Off,
    On,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Pulse {
    High,
    Low,
}

impl ModuleType {
    fn from_str(s: &str) -> Option<ModuleType> {
        match s {
            "%" => Some(ModuleType::FlipFlop),
            "&" => Some(ModuleType::Conjunction),
            _ => None,
        }
    }
}

/// broadcaster targets, modules
fn parse_modules(input: &str) -> (Vec<&str>, HashMap<&str, (ModuleType, Vec<&str>)>) {
    input.lines().fold(
        (vec![], HashMap::new()),
        |(mut broadcast_targets, mut modules), line| {
            let (mut name, children) = line.split_once(" -> ").unwrap();
            let destination = children.split(", ").collect::<Vec<&str>>();

            if let Some(mod_type) = ModuleType::from_str(name.get(0..1).unwrap()) {
                name = name.get(1..).unwrap();
                modules.insert(name, (mod_type, destination));
            } else {
                broadcast_targets = destination
            };
            (broadcast_targets, modules)
        },
    )
}

fn generate_dot(broadcast_targets: &Vec<&str>, modules: &HashMap<&str, (ModuleType, Vec<&str>)>) {
    let mut dot = String::from("digraph G {\n");
    dot += &format!("  {{ rank = sink; rx; }};\n");
    dot += &format!("  button -> brodcaster;\n");
    for t in broadcast_targets.iter().sorted() {
        dot += &format!("  brodcaster -> {};\n", t);
    }
    for (name, (mod_type, destinations)) in modules {
        for dst in destinations.iter().sorted() {
            dot += &format!(
                "  {} -> {}{};\n",
                name,
                dst,
                if mod_type == &ModuleType::Conjunction {
                    " [style=dotted]"
                } else {
                    ""
                }
            );
        }
    }
    dot += "}";
    // Run the following to visualize the graph:
    //   dot -Tsvg img/20.dot > img/20.svg
    std::fs::write("img/20.dot", dot).unwrap();
}

fn simulate(targets: &Vec<&str>, modules: &HashMap<&str, (ModuleType, Vec<&str>)>) -> usize {
    // get list of inputs for each conjunction module and initialize them
    let (mut conj_inputs, mut flipflop_state) = modules.iter().fold(
        (HashMap::new(), HashMap::new()),
        |(mut conj, mut flipflop), (name, (mod_type, _))| {
            match mod_type {
                &ModuleType::Conjunction => {
                    conj.insert(name, HashMap::<&str, Pulse>::new());
                }
                &ModuleType::FlipFlop => {
                    flipflop.insert(name, FlipFlop::Off);
                }
            }
            (conj, flipflop)
        },
    );
    for module in modules {
        let (name, (_mod_type, destinations)) = module;
        for dest in destinations {
            if conj_inputs.contains_key(dest) {
                conj_inputs.get_mut(dest).unwrap().insert(name, Pulse::Low);
            }
        }
    }

    // simulation
    let mut pulse_low = 0;
    let mut pulse_high = 0;

    for _ in 0..1000 {
        // we press the button
        pulse_low += 1;

        let mut queue =
            VecDeque::from_iter(targets.iter().map(|name| ("brodcaster", Pulse::Low, *name)));

        while let Some((src, pulse, dest)) = queue.pop_front() {
            // println!("{:?} {:?} {:?}", src, pulse, dest);

            if pulse == Pulse::High {
                pulse_high += 1;
            } else {
                pulse_low += 1;
            }
            if let Some((mod_type, destinations)) = modules.get(dest) {
                let pulse_to_send = match mod_type {
                    &ModuleType::Conjunction => {
                        // update pulse memory for the input
                        let inputs = conj_inputs.get_mut(&dest).unwrap();
                        *inputs.get_mut(src).unwrap() = pulse;
                        match inputs.iter().all(|(_, p)| p == &Pulse::High) {
                            true => Some(Pulse::Low),
                            false => Some(Pulse::High),
                        }
                    }
                    &ModuleType::FlipFlop => {
                        let state = flipflop_state.get_mut(&dest).unwrap();
                        match (pulse, state.clone()) {
                            (Pulse::Low, FlipFlop::Off) => {
                                *state = FlipFlop::On;
                                Some(Pulse::High)
                            }
                            (Pulse::Low, FlipFlop::On) => {
                                *state = FlipFlop::Off;
                                Some(Pulse::Low)
                            }
                            (Pulse::High, _) => None,
                        }
                    }
                };
                if let Some(pulse_to_send) = pulse_to_send {
                    for new_dest in destinations {
                        queue.push_back((dest, pulse_to_send, new_dest));
                    }
                }
            } else {
                continue;
            }
        }
    }
    pulse_low * pulse_high
}

fn find_cycles(
    targets: &Vec<&str>,
    modules: &HashMap<&str, (ModuleType, Vec<&str>)>,
) -> Vec<usize> {
    // get list of inputs for each conjunction module and initialize them
    let (mut conj_inputs, mut flipflop_state) = modules.iter().fold(
        (HashMap::new(), HashMap::new()),
        |(mut conj, mut flipflop), (name, (mod_type, _))| {
            match mod_type {
                &ModuleType::Conjunction => {
                    conj.insert(name, HashMap::<&str, Pulse>::new());
                }
                &ModuleType::FlipFlop => {
                    flipflop.insert(name, FlipFlop::Off);
                }
            }
            (conj, flipflop)
        },
    );
    let mut to_rx = "tbd";
    for module in modules {
        let (name, (_mod_type, destinations)) = module;
        // find input for rx
        if destinations.contains(&&"rx") {
            to_rx = name;
        }
        for dest in destinations {
            if conj_inputs.contains_key(dest) {
                conj_inputs.get_mut(dest).unwrap().insert(name, Pulse::Low);
            }
        }
    }

    let to_be_synchronized = conj_inputs
        .get(&&to_rx)
        .unwrap()
        .iter()
        .map(|(k, _)| *k)
        .collect::<Vec<&str>>();

    let mut cycles: HashMap<&str, Vec<usize>> = HashMap::new();
    let mut cycle = 0;

    while (cycles.len() < to_be_synchronized.len()) | cycles.iter().any(|(_k, v)| v.len() != 2) {
        cycle += 1;

        let mut queue =
            VecDeque::from_iter(targets.iter().map(|name| ("brodcaster", Pulse::Low, *name)));

        while let Some((src, pulse, dest)) = queue.pop_front() {
            if to_be_synchronized.contains(&&src) && pulse == Pulse::High {
                let module_cycles = cycles.entry(src).or_insert(vec![]);
                if module_cycles.len() < 2 {
                    module_cycles.push(cycle)
                }
                if cycles.len() == to_be_synchronized.len()
                    && cycles.iter().all(|(_k, v)| v.len() == 2)
                {
                    return cycles.values().map(|v| v[1] - v[0]).collect();
                }
            }

            if let Some((mod_type, destinations)) = modules.get(dest) {
                let pulse_to_send = match mod_type {
                    &ModuleType::Conjunction => {
                        // update pulse memory for the input
                        let inputs = conj_inputs.get_mut(&dest).unwrap();
                        *inputs.get_mut(src).unwrap() = pulse;
                        match inputs.iter().all(|(_, p)| p == &Pulse::High) {
                            true => Some(Pulse::Low),
                            false => Some(Pulse::High),
                        }
                    }
                    &ModuleType::FlipFlop => {
                        let state = flipflop_state.get_mut(&dest).unwrap();
                        match (pulse, state.clone()) {
                            (Pulse::Low, FlipFlop::Off) => {
                                *state = FlipFlop::On;
                                Some(Pulse::High)
                            }
                            (Pulse::Low, FlipFlop::On) => {
                                *state = FlipFlop::Off;
                                Some(Pulse::Low)
                            }
                            (Pulse::High, _) => None,
                        }
                    }
                };
                if let Some(pulse_to_send) = pulse_to_send {
                    for new_dest in destinations {
                        queue.push_back((dest, pulse_to_send, new_dest));
                    }
                }
            } else {
                continue;
            }
        }
    }
    unreachable!()
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let (broadcast_targets, modules) = parse_modules(input);
    let p1 = simulate(&broadcast_targets, &modules);
    // generate graph to understand what's going on
    // generate_dot(&broadcast_targets, &modules);
    let cycles = find_cycles(&broadcast_targets, &modules);
    let p2 = cycles.into_iter().fold(1, |acc, c| lcm(c, acc));
    (p1, p2)
}
