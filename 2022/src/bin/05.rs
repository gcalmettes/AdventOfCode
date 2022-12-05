use regex::Regex;

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Vec<usize>>) {
    // let's try the new "if let" syntax
    let (crates, ins) = if let Some((crates, ins)) = input.split_once("\n\n") {
        (crates, ins)
    } else {
        panic!("Could not split input.")
    };
    // other syntax using unwrap
    // let (crates, ins) = input.split_once("\n\n").unwrap();

    // crates
    let n_stacks = (crates.split("\n").nth(0).unwrap().len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![vec![]; n_stacks];
    let re_stack = Regex::new(r"\[([A-Z])\]").unwrap();
    crates.lines().for_each(|line| {
        re_stack.captures_iter(&line).for_each(|c| {
            let pos = c.get(1).unwrap().start();
            let stack_number = (pos - 1) / 4;
            let letter = c.get(1).map_or("", |m| m.as_str());
            stacks[stack_number].push(letter.chars().next().unwrap());
        })
    });
    // reverse piles
    for i in 0..n_stacks {
        stacks[i].reverse();
    }

    // instructions
    let re_ins = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let steps = re_ins
        .captures_iter(&ins)
        .map(|c| {
            c.iter()
                .filter_map(|m| m.unwrap().as_str().parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    (stacks, steps)
}

fn part1(mut stacks: Vec<Vec<char>>, steps: &Vec<Vec<usize>>) -> String {
    for s in steps.iter() {
        let n = s[0];
        let from = s[1];
        let to = s[2];
        for _ in 0..n {
            let item = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(item);
        }
    }
    stacks.iter().map(|c| c[c.len() - 1]).collect::<String>()
}

fn part2(mut stacks: Vec<Vec<char>>, steps: &Vec<Vec<usize>>) -> String {
    for s in steps.iter() {
        let n = s[0];
        let from = s[1];
        let to = s[2];
        // increase size of receiving stack
        let new_size = stacks[to - 1].len() + n;
        stacks[to - 1].resize(new_size, '.');
        for i in 0..n {
            let item = stacks[from - 1].pop().unwrap();
            stacks[to - 1][new_size - 1 - i] = item;
        }
    }
    stacks.iter().map(|c| c[c.len() - 1]).collect::<String>()
}

#[aoc::main()]
fn main(input: &str) -> (String, String) {
    let (stacks, steps) = parse_input(input);
    let p1 = part1(stacks.clone(), &steps);
    let p2 = part2(stacks, &steps);
    (p1, p2)
}
