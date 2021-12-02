use std::path::Path;
use std::fmt;
use std::fs;

struct Sub {
    forward: u32,
    depth: u32,
    aim: u32
}

impl fmt::Display for Sub {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Forward: {}, Depth: {}", self.forward, self.depth)
    }
}

fn read_file(filepath: &str) -> String {
    let path = Path::new(&filepath);
    let content = fs::read_to_string(path).expect("Unable to read file");
    content

}

fn parse_input(content: &str) -> Vec<(&str, u32)> {
    let instructions = content
        .lines()
        .map(|entry| {
            let splitted: Vec<&str> = entry.split_whitespace().collect();
            let op: &str = splitted[0];
            let i: u32 = splitted[1].parse().unwrap();
            (op, i)
        })
        .collect();

    return instructions
}

fn part1() -> Sub {
    let input = read_file("./inputs/day02.txt");
    let ins = parse_input(&input);

    // Startint position
    let mut sub = Sub{
        forward: 0,
        depth: 0,
        aim: 0,
    };

    for (op, i) in ins {
        match op {
            "forward" => sub.forward += i,
            "up" => sub.depth -= i,
            "down" => sub.depth += i,
            _ => (),

        }
    }
    return sub
}


fn part2() -> Sub {
    let input = read_file("./inputs/day02.txt");
    let ins = parse_input(&input);

    // Startint position
    let mut sub = Sub{
        forward: 0,
        depth: 0,
        aim: 0
    };

    for (op, i) in ins {
        match op {
            "forward" => {
                sub.forward += i;
                sub.depth += i*sub.aim;
            },
            "up" => sub.aim -= i,
            "down" => sub.aim += i,
            _ => (),

        }
    }
    return sub
}
fn main() {
    let p1 = part1();
    println!("{}", p1.forward * p1.depth);
    let p2 = part2();
    println!("{}", p2.forward * p2.depth)

}
