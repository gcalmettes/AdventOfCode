use itertools::Itertools;

fn get_sorted_elves(input: &str) -> Vec<usize> {
    input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|elf| {
            elf.lines()
                .map(|l| l.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .sorted()
        .rev()
        .collect::<Vec<usize>>()
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let elves = get_sorted_elves(input);
    let p1 = elves[0];
    let p2 = elves[0..3].iter().sum::<usize>();
    (p1, p2)
}
