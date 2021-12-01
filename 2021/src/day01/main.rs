use std::path::Path;
use std::fs;

fn part1() -> i32 {


    // let ex = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
    // let mut lines = ex.lines().map(|x| x.parse::<i32>().expect("parse error"))
    // .collect::<Vec<i32>>();
    
    let path = Path::new("./inputs/day01.txt");
    let content = fs::read_to_string(path).expect("Unable to read file");
    
    let mut ints = content.lines().map(|x| x.parse::<i32>().expect("parse error"))
    .collect::<Vec<i32>>();

    let mut count:i32 = 0;
    let mut current = ints.remove(0);

    for n in ints {
        if n > current {
            count = count + 1;
        }
        current = n;
    }
    return count
}

fn part2() -> i32 {
    let path = Path::new("./inputs/day01.txt");
    let content = fs::read_to_string(path).expect("Unable to read file");
    
    let ints = content.lines().map(|x| x.parse::<i32>().expect("parse error"))
    .collect::<Vec<i32>>();

    let mut windows = ints.windows(3);
    let mut count:i32 = 0;

    let first_window = windows.next().unwrap();
    let mut current = first_window.iter().sum();


    for n in windows {
        let mut s:i32 = 0;
        for k in n {
            s = s + k;
        }
        if s > current {
            count = count + 1;
        }
        current = s;
    }
    return count
}

fn main() {
    let p1 = part1();
    println!("{}", p1);
    let p2 = part2();
    println!("{}", p2);

}
