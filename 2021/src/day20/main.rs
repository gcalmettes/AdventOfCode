use std::path::Path;
use std::fs;
use std::collections::HashSet;

struct Limits {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl Limits {
    fn contains(&self, x: isize, y: isize) -> bool {
        self.x_min <= x && x <= self.x_max && self.y_min <= y && y <= self.y_max
    }
}


fn parse_input(content: &str) -> (Vec<char>, HashSet<(isize, isize)>, isize, isize) {
    let (algorithm, image_data) = content.split_once("\n\n").unwrap();
    let algorithm = algorithm.chars().collect::<Vec<char>>();

    let mut image: HashSet<(isize, isize)> = HashSet::new();
    image_data.lines()
        .enumerate()
        .for_each(|(y, line)| line.chars()
                .enumerate()
                .for_each(|(x, c)| match c {
                    '#' => {image.insert((x as isize, y as isize));},
                      _ => {},
                  }));

    let y_max: isize = image_data.lines().count() as isize - 1;
    let x_max: isize = image_data.lines().next().unwrap().chars().count() as isize - 1;

    (algorithm, image, x_max, y_max)
}


fn get_idx(
    x_ref: isize,
    y_ref: isize,
    image: &HashSet<(isize, isize)>,
    limits: Limits,
    background: char
    ) -> usize {
    let mut sum = 0;
    for y in y_ref-1..=y_ref+1 {
        for x in x_ref-1..=x_ref+1 {
            sum *= 2;
            let mut pixel = background;
            if limits.contains(x, y) {
                if image.contains(&(x, y)) {
                    pixel = '#';
                } else {
                    pixel = '.';
                }
            }
            if pixel == '#' {
                sum += 1;
            }
        }
    }
    sum
}


fn parts(input: &str, step: usize) -> usize {
    
    let (algorithm, mut image, mut x_max, mut y_max) = parse_input(input);
    
    let mut x_min: isize = 0;
    let mut y_min: isize = 0;

    let mut background = '.';
    for _ in 0..step {
        let mut new_image : HashSet<(isize, isize)> = HashSet::new();
        for y in y_min-1..=y_max+1 {
            for x in x_min-1..=x_max+1 {
                let idx = get_idx(x, y, &image, Limits{x_min, y_min, x_max, y_max}, background);
                match algorithm[idx] {
                    '#' => {new_image.insert((x, y));},
                    _ => {},
                }
            }
        }

        // switch everything that is outside current image limits
        if background == '.' {
            background = algorithm[0];
        } else {
            background = algorithm[algorithm.len()-1];
        }
        image = new_image;

        // grow image
        x_min -= 1;
        y_min -= 1;
        x_max += 1;
        y_max += 1;
    }
    image.len()
}


fn main() {
    let path = Path::new("./inputs/day20.txt");
    let input = fs::read_to_string(path).expect("Unable to read file");

    let p1 = parts(&input, 2);
    println!("{}", p1);
    let p2 = parts(&input, 50);
    println!("{}", p2);
}
