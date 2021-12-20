use std::path::Path;
use std::fs;
use std::collections::{HashMap, HashSet};

// fn parse_input(content: &str) -> (Vec<char>, HashMap<(usize, usize), char>) {
//     let (algorithm, image_data) = content.split_once("\n\n").unwrap();
//     let algorithm = algorithm.chars().collect::<Vec<char>>();
//     let mut image: HashMap<(usize, usize), char> = HashMap::new();
//     image_data.lines()
//         .enumerate()
//         .for_each(|(y, line)| line.chars()
//                   .enumerate()
//                   .for_each(|(x, c)| {image.insert((x, y), c);}));
//     (algorithm, image)
// }

// fn part1(input: &str) -> usize {
//     let (algo, image) = parse_input(input);
//     println!("{:?}", image);
//     0
// }

fn solve(input: &str, simulation_time: usize) -> u64 {
    let mut lines = input.lines();

    let enhancement = lines.next().unwrap();
    let enhancement: Vec<char> = enhancement.chars().collect();

    let _ = lines.next().unwrap();

    let mut image: HashSet<(i64, i64)> = HashSet::new();

    let mut min_x: i64 = 0;
    let mut min_y: i64 = 0;
    let mut max_x: i64 = 0;
    let mut max_y: i64 = 0;

    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                image.insert((x as i64, y as i64));
            }
            max_x = max_x.max(x as i64);
        }
        max_y = max_y.max(y as i64);
    }

    let mut background = '.';
    for _ in 0..simulation_time {
        let mut new_image : HashSet<(i64, i64)> = HashSet::new();
        for y in min_y-2..=max_y+2 {
            for x in min_x-2..=max_x+2 {
                let offset = sum_grid(x, y, &image, (min_x, min_y, max_x, max_y), background);
                let pixel = enhancement[offset];
                if pixel == '#' {
                    new_image.insert((x, y));
                }
            }
        }

        // everything that is from boundary outward will be changed
        if background == '.' {
            background = enhancement[0];
        } else {
            background = enhancement[511];
        }
        image = new_image;
        // adjusting boundary
        min_x -= 2;
        min_y -= 2;
        max_x += 2;
        max_y += 2;
    }

    image.len() as u64
}

fn sum_grid(x0: i64, y0: i64, image: &HashSet<(i64, i64)>, boundary: (i64, i64, i64, i64), background: char) -> usize {
    let mut grid_sum = 0;
    let (min_x, min_y, max_x, max_y) = boundary;
    for y in y0-1..=y0+1 {
        for x in x0-1..=x0+1 {
            grid_sum *= 2;
            let mut pixel = background;
            if min_x <= x && x <= max_x && min_y <= y && y <= max_y {
                if image.contains(&(x, y)) {
                    pixel = '#';
                } else {
                    pixel = '.';
                }
            }
            if pixel == '#' {
                grid_sum += 1;
            }
        }
    }
    grid_sum
}

fn main() {
    let path = Path::new("./inputs/day20.txt");
    let input = fs::read_to_string(path).expect("Unable to read file");

    let p1 = solve(&input, 2);
    println!("{}", p1);
    let p2 = solve(&input, 50);
    println!("{}", p2);
    // let p1 = part1(&input);
    // println!("{}", p1);
    // let p2 = part2(&input);
    // println!("{}", p2);
}
