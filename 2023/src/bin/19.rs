use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Rule {
    left: char,
    cmp: char,
    num: usize,
    go_to: String,
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    default: String,
}

fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<HashMap<char, usize>>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows: HashMap<String, Workflow> = workflows
        .lines()
        .map(|line| {
            let (name, rules) = line.split_once("{").unwrap();
            let mut default: String = "".to_string();
            let rules = rules.trim_end_matches("}");
            let rules = rules.split(",");
            let rules = rules
                .filter_map(|block| {
                    let mut block = block.split(":");
                    let r = block.next().unwrap();
                    match block.next() {
                        Some(wf) => {
                            let mut ch = r.chars();
                            let left = ch.next().unwrap();
                            let cmp = ch.next().unwrap();
                            let num = ch.join("").parse::<usize>().unwrap();
                            Some(Rule {
                                left,
                                cmp,
                                num,
                                go_to: wf.to_string(),
                            })
                        }
                        None => {
                            default = r.to_string();
                            None
                        }
                    }
                })
                .collect();
            (name.to_string(), Workflow { rules, default })
        })
        .collect();

    let parts = parts
        .lines()
        .map(|line| {
            let line = line.trim_start_matches("{").trim_end_matches("}");
            let rating = line
                .split(",")
                .map(|r| {
                    let (left, val) = r.split_once("=").unwrap();
                    let val = val.parse::<usize>().unwrap();
                    (left.chars().next().unwrap(), val)
                })
                .collect();
            rating
        })
        .collect();
    (workflows, parts)
}

fn part1(workflows: &HashMap<String, Workflow>, parts: &Vec<HashMap<char, usize>>) -> usize {
    let mut current_wf = "in".to_string();
    let checks: usize =
        parts
            .iter()
            .filter_map(|part| {
                let keys = part.into_iter().map(|(k, _v)| *k).collect::<Vec<char>>();
                // do the workflow
                while (current_wf != "A".to_string()) && (current_wf != "R".to_string()) {
                    if let Some(wf) = workflows.get(&current_wf) {
                        let is_match =
                            wf.rules
                                .iter()
                                .find_map(|rule| match keys.contains(&rule.left) {
                                    true => {
                                        // test condition
                                        let value_to_check = part.get(&rule.left).unwrap();
                                        match rule.cmp {
                                            '>' => (value_to_check > &rule.num)
                                                .then(|| rule.go_to.clone()),
                                            '<' => (value_to_check < &rule.num)
                                                .then(|| rule.go_to.clone()),
                                            _ => unreachable!(),
                                        }
                                    }
                                    false => None,
                                });
                        if let Some(is_match) = is_match {
                            current_wf = is_match
                        } else {
                            current_wf = wf.default.clone()
                        }
                    } else {
                        // something bad happened
                        panic!("We shouldn't be here:\n{:?}", current_wf);
                    };
                }
                let score = match current_wf.as_str() {
                    "A" => {
                        let s: usize = part.into_iter().map(|(_k, v)| *v).sum();
                        Some(s)
                    }
                    "R" => None,
                    _ => unreachable!(),
                };
                current_wf = "in".to_string();
                score
            })
            .sum();
    checks
}

fn count_valid(
    workflows: &HashMap<String, Workflow>,
    current: String,
    mut x: Vec<usize>,
    mut m: Vec<usize>,
    mut s: Vec<usize>,
    mut a: Vec<usize>,
) -> usize {
    if current == "A".to_string() {
        return x.len() * m.len() * a.len() * s.len();
    }
    if current == "R".to_string() {
        return 0;
    }
    let mut total = 0;
    let workflow = workflows.get(&current).unwrap();
    for rule in workflow.rules.iter() {
        match rule.left {
            'x' => {
                let (valid_x, not_valid): (Vec<_>, Vec<_>) =
                    x.iter().partition(|v| match rule.cmp {
                        '>' => v > &&rule.num,
                        '<' => v < &&rule.num,
                        _ => unreachable!(),
                    });
                if !valid_x.is_empty() {
                    total += count_valid(
                        workflows,
                        rule.go_to.clone(),
                        valid_x,
                        m.clone(),
                        s.clone(),
                        a.clone(),
                    );
                }
                x = not_valid;
            }
            'm' => {
                let (valid_m, not_valid): (Vec<_>, Vec<_>) =
                    m.iter().partition(|v| match rule.cmp {
                        '>' => v > &&rule.num,
                        '<' => v < &&rule.num,
                        _ => unreachable!(),
                    });
                if !valid_m.is_empty() {
                    total += count_valid(
                        workflows,
                        rule.go_to.clone(),
                        x.clone(),
                        valid_m,
                        s.clone(),
                        a.clone(),
                    );
                }
                m = not_valid;
            }
            'a' => {
                let (valid_a, not_valid): (Vec<_>, Vec<_>) =
                    a.iter().partition(|v| match rule.cmp {
                        '>' => v > &&rule.num,
                        '<' => v < &&rule.num,
                        _ => unreachable!(),
                    });
                if !valid_a.is_empty() {
                    total += count_valid(
                        workflows,
                        rule.go_to.clone(),
                        x.clone(),
                        m.clone(),
                        s.clone(),
                        valid_a,
                    );
                }
                a = not_valid;
            }
            's' => {
                let (valid_s, not_valid): (Vec<_>, Vec<_>) =
                    s.iter().partition(|v| match rule.cmp {
                        '>' => v > &&rule.num,
                        '<' => v < &&rule.num,
                        _ => unreachable!(),
                    });
                if !valid_s.is_empty() {
                    total += count_valid(
                        workflows,
                        rule.go_to.clone(),
                        x.clone(),
                        m.clone(),
                        valid_s,
                        a.clone(),
                    );
                }
                s = not_valid;
            }
            _ => unreachable!(),
        }
    }
    total += count_valid(workflows, workflow.default.clone(), x, m, s, a);
    total
}

fn part2(workflows: &HashMap<String, Workflow>) -> usize {
    let range = (1..=4000).collect::<Vec<usize>>();
    let current = "in".to_string();
    let total = count_valid(
        workflows,
        current.clone(),
        range.clone(),
        range.clone(),
        range.clone(),
        range.clone(),
    );

    total
}
#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let (workflows, parts) = parse_input(input);
    let p1 = part1(&workflows, &parts);
    let p2 = part2(&workflows);
    (p1, p2)
}
