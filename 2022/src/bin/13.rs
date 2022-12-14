use itertools::Itertools;
use serde_json::{Number, Value};
use std::cmp::{max, Ordering};

fn parse_input(input: &str) -> Vec<Value> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .collect::<Vec<Value>>()
}

fn cmp(p1: &Value, p2: &Value) -> Ordering {
    match (p1, p2) {
        // comparison direct of same types
        (Value::Number(a), Value::Number(b)) => a.as_u64().unwrap().cmp(&b.as_u64().unwrap()),
        (Value::Array(a), Value::Array(b)) => {
            let m = max(a.len(), b.len());
            for i in 0..m {
                match (a.get(i), b.get(i)) {
                    // a array ran out of items first
                    (None, _) => return Ordering::Less,
                    // b array ran out of items first
                    (_, None) => return Ordering::Greater,
                    (Some(x), Some(y)) => match cmp(x, y) {
                        Ordering::Equal => (), // do nothing
                        other_ordering => return other_ordering,
                    },
                }
            }
            // the items were all equals inside array
            return Ordering::Equal;
        }
        // mixed types
        (Value::Array(_), Value::Number(_)) => {
            // need to cast number as list
            let p2_vec = vec![p2.clone()];
            cmp(p1, &Value::Array(p2_vec))
        }
        (Value::Number(_), Value::Array(_)) => {
            // need to cast number as list
            let p1_vec = vec![p1.clone()];
            cmp(&Value::Array(p1_vec), p2)
        }
        _ => unreachable!(),
    }
}

fn part1(data: &Vec<Value>) -> usize {
    data.iter()
        .tuples()
        .enumerate()
        .map(|(i, (p1, p2))| (i + 1, cmp(p1, p2)))
        .filter(|(_i, ordering)| ordering != &Ordering::Greater)
        .map(|(i, _ordering)| i)
        .sum()
}

fn part2(data: &mut Vec<Value>) -> usize {
    let new_packet_1 = Value::Array(vec![Value::Array(vec![serde_json::to_value(2).unwrap()])]);
    let new_packet_2 = Value::Array(vec![Value::Array(vec![serde_json::to_value(6).unwrap()])]);
    let extra = vec![new_packet_1, new_packet_2];
    data.extend(extra.clone());
    data.iter()
        .sorted_by(|a, b| cmp(a, b))
        .enumerate()
        .filter(|(_i, v)| extra.contains(v))
        .map(|(i, _)| i + 1)
        .product()
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let mut data = parse_input(input);
    let p1 = part1(&data);
    let p2 = part2(&mut data);
    (p1, p2)
}
