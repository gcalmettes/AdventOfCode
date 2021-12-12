use std::path::Path;
use std::fs;
use std::collections::{HashMap, HashSet};

fn parse_input(content: &str) -> HashMap<&str, Vec<&str>> {
    let mut data: HashMap<&str, Vec<&str>> = HashMap::new();
    content
        .lines()
        .for_each(|entry| {
            let parts = entry.split("-").collect::<Vec<_>>();
            data.entry(parts[0]).or_insert(Vec::new()).push(parts[1]);
            data.entry(parts[1]).or_insert(Vec::new()).push(parts[0]);
        });
       data
}

fn is_small(cave: &str) -> bool {
    cave == cave.to_lowercase()
}
fn count_paths(caves: &HashMap<&str, Vec<&str>>, cave: &str, seen: &HashSet<String>) -> usize {
    if cave == "end" {
        return 1
    } else if is_small(cave) && seen.contains(cave) {
        return 0
    }
    let mut copy_seen: HashSet<String> = HashSet::new();
    for c in seen {
        copy_seen.insert(c.to_string());
    }
    copy_seen.insert(cave.to_string());
    let mut count = 0;
    for c in &caves[cave] {
        count += count_paths(caves, c, &copy_seen);
    }
    count
}

fn part1(input: &str) -> usize {

    let caves = parse_input(input);

    let mut count = 0;
    count += count_paths(&caves, "start", &HashSet::new());

    count
}

fn count_paths_2(caves: &HashMap<&str, Vec<&str>>, cave: &str, seen: &HashSet<String>, cannot_revisit_small: bool) -> usize {
    if cave == "end" {
        return 1
    } else if cave == "start" && seen.contains(cave) { // we can only go to start once
        return 0
    } else if is_small(cave) && seen.contains(cave) && cannot_revisit_small {
        return 0
    }

    let mut small_visited_twice: bool = cannot_revisit_small;
    if is_small(cave) && seen.contains(cave) {
        small_visited_twice = true;
    }

    // copy set so it is unique to each path
    let mut copy_seen: HashSet<String> = HashSet::new();
    for c in seen {
        copy_seen.insert(c.to_string());
    }

    copy_seen.insert(cave.to_string());

    let mut count = 0;
    for c in &caves[cave] {
        count += count_paths_2(caves, c, &copy_seen, small_visited_twice);
    }
    count
}

fn part2(input: &str) -> usize {

    let caves = parse_input(input);

    let mut count = 0;
    count += count_paths_2(&caves, "start", &HashSet::new(), false);

    count
}

fn main() {
    let path = Path::new("./inputs/day12.txt");
    let input = fs::read_to_string(path).expect("Unable to read file");

    let p1 = part1(&input);
    println!("{}", p1);
    let p2 = part2(&input);
    println!("{}", p2);
}
