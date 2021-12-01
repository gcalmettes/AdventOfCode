use std::path::Path;
use std::io::{Error};
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
    
    let mut ints = content.lines().map(|x| x.parse::<i32>().expect("parse error"))
    .collect::<Vec<i32>>();

    let iter = ints.windows(3);
    let mut count:i32 = 0;

    // Popping the first element of an iterator was givin me headache, so we'll just 
    // start by a very big number
    let mut current = 1444444444;

    for n in iter {
        let mut s:i32 = 0;
        for k in n {
            s = s + k;
        }
        if s > current {
            count = count + 1;
        }
        println!("-- {} {}", s, current);
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
