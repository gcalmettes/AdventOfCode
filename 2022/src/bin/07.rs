use std::collections::HashMap;
use std::{path::PathBuf, str::FromStr};

fn parse_input(input: &str) -> HashMap<PathBuf, Vec<(&str, isize)>> {
    let mut filesystem = HashMap::new();
    let mut current_path = PathBuf::new();
    let commands = input
        .split('$')
        // file starts with $ so first block will be empty
        .skip(1)
        .map(|line| line.trim())
        .collect::<Vec<&str>>();

    for cmd_block in commands.iter() {
        match cmd_block.lines().next().unwrap() {
            "ls" => {
                // we list a dir, let's reference its content
                let items = cmd_block
                    .lines()
                    // skip the "ls" command itself
                    .skip(1)
                    .map(|output| {
                        let (dir_or_size, name) = output.split_once(" ").unwrap();
                        let size = if dir_or_size != "dir" {
                            dir_or_size.parse::<isize>().unwrap()
                        } else {
                            0
                        };
                        (name, size)
                    })
                    .collect::<Vec<(&str, isize)>>();
                filesystem
                    .entry(current_path.clone())
                    .or_insert(items.clone());
            }
            "cd .." => {
                // we go up, reflect that on the current path
                current_path.pop();
            }
            cd_into => {
                // we cd into a dir, reflect that on the current path
                let (_cd, name) = cd_into.split_once(" ").unwrap();
                current_path.push(name);
            }
        }
    }
    filesystem
}

fn compute_size(
    dir: PathBuf,
    filesystem: &HashMap<PathBuf, Vec<(&str, isize)>>,
    sizes: &mut HashMap<PathBuf, isize>,
) -> isize {
    if sizes.contains_key(&dir) {
        // already done
        return sizes[&dir];
    }
    // println!("-- COMPUTING SIZE for {:?} --", dir);
    let size = filesystem[&dir]
        .iter()
        .map(|&(name, s)| match s {
            0 => {
                let subdir = dir.join(name);
                // println!("-- ADDING SUBDIR {:?} --", subdir);
                compute_size(subdir, filesystem, sizes)
            }
            s => s,
        })
        .sum();
    // println!("  ** SIZE for {:?}: {}", dir, size);
    sizes.insert(dir.clone(), size);
    size
}

fn part1(filesystem: &HashMap<PathBuf, Vec<(&str, isize)>>) -> isize {
    let mut sizes: HashMap<PathBuf, isize> = HashMap::new();
    filesystem
        .keys()
        .map(|name| compute_size(name.to_path_buf(), filesystem, &mut sizes))
        .filter(|s| *s < 100000)
        .sum()
}

fn part2(filesystem: &HashMap<PathBuf, Vec<(&str, isize)>>) -> isize {
    let mut sizes: HashMap<PathBuf, isize> = HashMap::new();
    let fs_sizes = filesystem
        .keys()
        .map(|name| compute_size(name.to_path_buf(), filesystem, &mut sizes))
        .collect::<Vec<isize>>();

    const TOTAL_CAPACITY: isize = 70_000_000;
    const MINIMUM_NEEDED_FOR_UPDATE: isize = 30_000_000;
    let currently_available = TOTAL_CAPACITY - sizes[&PathBuf::from_str("/").unwrap()];
    let needed_to_free = MINIMUM_NEEDED_FOR_UPDATE - currently_available;
    fs_sizes
        .into_iter()
        .filter(|x| *x >= needed_to_free)
        .min()
        .unwrap()
}

#[aoc::main()]
fn main(input: &str) -> (isize, isize) {
    let fs = parse_input(input);
    let p1 = part1(&fs);
    let p2 = part2(&fs);
    (p1, p2)
}
