use std::path::Path;
use std::fs;

fn parse_input(content: &str) -> Vec<i32> {

    let crabs = content
        .lines()
        .flat_map(|entry| entry.split(",")
                .map(|i| i.parse().unwrap())
                .collect::<Vec<i32>>())
        .collect();
       crabs
}

fn part1(input: &str) -> usize {

    let crabs = parse_input(input);

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    let minfuel: Option<i32> = (min..=max)
        .map(|x| crabs.iter().map(|c| (c-x).abs()).sum())
        .min();

    match minfuel {
        Some(min) => return (min as i32).try_into().unwrap(),
        None      => return (-1 as i32).try_into().unwrap(),
    }
}


// move cost
// +1 =>  1 = 1 * (1+1) / 2
// +2 =>  3 = 2 * (2+1) / 2
// +3 =>  6 = 3 * (3+1) / 2
// +4 => 10 = 4 * (4+1) / 2
// +n =>  x = n * (n+1) / 2
fn cost(n: i32) -> i32 {
  n * (n + 1) / 2
}

fn part2(input: &str) -> usize {

    let crabs = parse_input(input);

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    let minfuel: Option<i32> = (min..=max)
        .map(|x| crabs.iter().map(|c| cost((c-x).abs())).sum())
        .min();

    match minfuel {
        Some(min) => return (min as i32).try_into().unwrap(),
        None      => return (-1 as i32).try_into().unwrap(),
    }
}


fn main() {
    let path = Path::new("./inputs/day07.txt");
    let input = fs::read_to_string(path).expect("Unable to read file");

    let p1 = part1(&input);
    println!("{}", p1);
    let p2 = part2(&input);
    println!("{}", p2);
}
