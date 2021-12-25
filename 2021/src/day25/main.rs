use std::path::Path;
use std::fs;


 fn parse_input(content: &str) -> Vec<Vec<char>> {
    content.lines()
        .map(|line| line.chars().collect())
        .collect()
 }

fn step(sea_floor: &mut Vec<Vec<char>>, delta_y: usize, delta_x: usize, x_size: usize, y_size: usize) -> bool {

    let mut new_sea_floor = vec![vec!['.'; x_size]; y_size];
    let mut has_moved = false;

    for y in 0..y_size {
        for x in 0..x_size {
            match sea_floor[y][x] {
                '>' => {
                    let next_x = (x + delta_x) % x_size;
                    if sea_floor[y][next_x] == '.' {
                        new_sea_floor[y][next_x] = '>';
                        has_moved = true;
                    } else {
                        new_sea_floor[y][x] = '>';
                    }
                },
                'v' => {
                    let next_y = (y + delta_y) % y_size;
                    if sea_floor[next_y][x] == '.' {
                        new_sea_floor[next_y][x] = 'v';
                        has_moved = true;
                    } else {
                        new_sea_floor[y][x] = 'v';
                    }
                }, 
                _ => {}
            }
        }
    }
    *sea_floor = new_sea_floor;
    has_moved
}


fn part1(input: &str) -> usize {

    let mut sea_floor = parse_input(input);
    let (y_size,x_size) = (sea_floor.len(), sea_floor[0].len());

    let mut n_steps = 0;
    loop {
        let move_east = step(&mut sea_floor, 0, 1, x_size, y_size);
        let move_south = step(&mut sea_floor, 1, 0, x_size, y_size);
        n_steps += 1;
        if !move_east && !move_south {
            break
        }
    }
    n_steps
}


 fn main() {
     let path = Path::new("./inputs/day25.txt");
     let input = fs::read_to_string(path).expect("Unable to read file");

     let p1 = part1(&input);
     println!("{}", p1);
 }
