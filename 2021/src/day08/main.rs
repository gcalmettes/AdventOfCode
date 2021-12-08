use std::path::Path;
use std::fs;
use std::collections::HashMap;


fn part1(input: &str) -> usize {
    let unique = vec![2, 4, 3, 7];

    let count: usize = input
        .lines()
        .map(|entry| {
            let parts: Vec<&str> = entry.split(" | ").collect();
            let output: &str = parts[1];
            let lengths: Vec<usize> = output.split_whitespace()
                .map(|s| s.len())
                .collect();
            lengths
        })
        .flatten()
        .filter(|v| unique.contains(v))
        .count();
    count
}

fn find_len(patterns: Vec<&str>, len_to_match: usize) -> &str {
    patterns.iter()
        .find(|p| p.len() == len_to_match)
        .unwrap()
}

fn sort_chars(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));
    String::from_iter(chars)
}

fn concat_digits(digits: Vec<&usize>) -> usize {
    digits.iter().fold(0, |acc, d| acc * 10 + *d )
}

fn build_map(patterns: Vec<&str>) -> HashMap<String, usize> {
    let mut numbers = HashMap::new();
    
    let one = find_len(patterns.to_vec(), 2);
    let four = find_len(patterns.to_vec(), 4);

    patterns.iter()
        .for_each(|p| {
            match p.len() {
                // unique lengths
                2 => numbers.insert(sort_chars(p), 1),
                3 => numbers.insert(sort_chars(p), 7),
                4 => numbers.insert(sort_chars(p), 4),
                7 => numbers.insert(sort_chars(p), 8),
                // deduce others from length
                // and number of intersections with 1 and 4
                len => match (
                    len,
                    p.chars().filter(|&c| one.contains(c)).count(),
                    p.chars().filter(|&c| four.contains(c)).count(),
                ){
                    (5, 1, 3) => numbers.insert(sort_chars(p), 5),
                    (5, 2, 3) => numbers.insert(sort_chars(p), 3),
                    (5, _, 2) => numbers.insert(sort_chars(p), 2),
                    (6, 1, _) => numbers.insert(sort_chars(p), 6),
                    (6, _, 3) => numbers.insert(sort_chars(p), 0),
                    (6, _, 4) => numbers.insert(sort_chars(p), 9),
                    _ => unreachable!(),
                },
            };
        });
    numbers
}

fn part2(input: &str) -> usize {

    let patterns: Vec<Vec<&str>> = input
        .lines()
        .filter_map(|entry| entry.split_once(" | ")
                .map(|s| s.0)
                .map(|s| s.split_whitespace().collect::<Vec<_>>()))
        .collect();

    let outputs: Vec<Vec<&str>> = input
        .lines()
        .filter_map(|entry| entry.split_once(" | ")
                .map(|s| s.1)
                .map(|s| s.split_whitespace().collect::<Vec<_>>()))
        .collect();

    patterns.iter()
        .zip(outputs.iter())
        .map(|(p, o)| {
            // known lengths
            let hashmap = build_map(p.to_vec());
            let outputs:Vec<&usize> = o.iter()
                .filter_map(|p| {
                    let pattern = sort_chars(p);
                    hashmap.get(&pattern)
                })
                .collect();

            let concatenated_output = concat_digits(outputs.to_vec()); 

            // println!("{:?} {}", outputs, number);
            concatenated_output
        })
        .sum()
}

fn main() {
    let path = Path::new("./inputs/day08.txt");
    let input = fs::read_to_string(path).expect("Unable to read file");

    let p1 = part1(&input);
    println!("{}", p1);
    let p2 = part2(&input);
    println!("{}", p2);
}
