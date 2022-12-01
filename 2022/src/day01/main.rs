use itertools::Itertools;
use std::fs;

fn get_sorted_elves() -> Vec<usize> {
    let input = fs::read_to_string("./inputs/01.in")
        .expect("file not found");

    input.split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|elf| elf.lines()
            .map(|l| l.parse::<usize>().unwrap())
            .sum::<usize>()
        )
        .sorted()
        .rev()
        .collect::<Vec<usize>>()
}

fn main() {
    let elves = get_sorted_elves();
    let p1 = elves[0];
    let p2 = &elves[0..3].iter().sum::<usize>();
    println!("part 1: {:?}", p1);
    println!("part 2: {:?}", p2);
}
