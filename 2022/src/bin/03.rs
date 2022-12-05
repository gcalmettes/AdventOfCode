use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
struct Ruscksack {
    c1: HashSet<char>,
    c2: HashSet<char>,
}

impl Ruscksack {
    fn get_common_item(&self) -> char {
        let common = self.c1.intersection(&self.c2).collect::<Vec<&char>>();
        *common[0]
    }
}

impl FromStr for Ruscksack {
    type Err = std::string::ParseError;

    fn from_str(items_list: &str) -> Result<Self, Self::Err> {
        let items = items_list.chars().collect::<Vec<char>>();
        let len = items.len();
        let c1 = HashSet::from_iter(items[0..len / 2].iter().cloned());
        let c2 = HashSet::from_iter(items[len / 2..].iter().cloned());
        Ok(Ruscksack { c1, c2 })
    }
}

fn get_alphabet() -> String {
    // construct abc...xyzABC...XYZ
    let mut alphabet = ('a'..='z').into_iter().collect::<String>();
    alphabet.push_str(&('A'..='Z').into_iter().collect::<String>());
    alphabet
}

fn part1(input: &str, alphabet: &str) -> usize {
    input
        .lines()
        .map(|l| Ruscksack::from_str(l).unwrap())
        .map(|r| r.get_common_item())
        .map(|i| alphabet.find(i).unwrap() + 1)
        .sum()
}

fn part2(input: &str, alphabet: &str) -> usize {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let mut group = chunk.map(|ch| HashSet::from_iter(ch.chars()));

            let mut s: HashSet<char> = group.next().unwrap();
            for elf in group {
                s = s.intersection(&elf).copied().collect();
            }
            let common = s.iter().next().unwrap();
            alphabet.find(*common).unwrap() + 1
        })
        .sum::<usize>()
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let alphabet = get_alphabet();
    let p1 = part1(input, &alphabet);
    let p2 = part2(input, &alphabet);
    (p1, p2)
}
