use itertools::Itertools;
use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let (ins, rest) = input.split_once("\n\n").unwrap();
    let ins = ins.chars().collect::<Vec<char>>();

    (
        ins,
        rest.lines()
            .map(|line| {
                let (k, mapping) = line.split_once(" = (").unwrap();
                let (left, right) = mapping.split_once(", ").unwrap();
                let right = right.trim_end_matches(")");

                (k, (left, right))
            })
            .collect::<HashMap<_, _>>(),
    )
}

fn part1(ins: &Vec<char>, m: &HashMap<&str, (&str, &str)>) -> usize {
    let mut cur = "AAA";
    let mut steps: usize = 0;
    let mut ins = ins.iter().cycle();

    while cur != "ZZZ" {
        let (left, right) = m.get(cur).unwrap();
        cur = match ins.next() {
            Some('R') => right,
            Some('L') => left,
            _ => unreachable!(), // it's a cycle
        };
        steps += 1;
    }
    steps
}

fn part2(ins: &Vec<char>, m: &HashMap<&str, (&str, &str)>) -> usize {
    let curs = m
        .iter()
        .filter_map(|(k, _v)| k.ends_with("A").then(|| *k))
        .collect::<Vec<&str>>();

    let cycles = curs
        .iter()
        .map(|mut cur| {
            let mut ins = ins.clone().into_iter().cycle();
            let mut steps: usize = 0;

            while !cur.ends_with("Z") {
                let (left, right) = m.get(cur).unwrap();
                cur = match ins.next() {
                    Some('R') => right,
                    Some('L') => left,
                    _ => unreachable!(), // it's a cycle
                };
                steps += 1;
            }
            steps
        })
        .collect::<Vec<usize>>();

    cycles.iter().fold(1, |acc, n| lcm(acc, *n))
}

// lcm and gcd functions taken from https://www.hackertouch.com/least-common-multiple-in-rust.html
fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let (ins, m) = parse_input(input);

    let p1 = part1(&ins, &m);
    let p2 = part2(&ins, &m);
    (p1, p2)
}
