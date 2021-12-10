use std::path::Path;
use std::fs;
use std::collections::HashMap;


fn part1(input: &str) -> usize {
    let points = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);
    
    let score = input.lines()
        .map(|line| {
            let mut stack = Vec::new();
            for c in line.chars() {
                match c {
                    // if closing, try to match to its corresponding opening
                    ')' | ']' | '}' | '>' => match stack.pop() {
                        Some(o) => {
                            if o == '(' && c != ')' ||
                               o == '[' && c != ']' ||
                               o == '{' && c != '}' ||
                               o == '<' && c != '>' {
                                // println!("corrupted, opening was {} but encountered {} as closing", o, c);
                                match points.get(&c) {
                                    Some(p) => {
                                        // println!("-- points {}", p);
                                        return *p
                                    },
                                    None => {
                                        // println!("-- shouldn't be there");
                                        return 0
                                    },
                                };
                            } else {
                                // println!("Ok");
                            }
                        },
                        None => {
                            // println!("Not possible");
                            return 0
                        },
                    },
                    // else, it's an opening, add it to stack and continue
                    o => {
                        stack.push(o);
                    },
                }
            }
            // Everything went well, no penalty point
            return 0
        }).sum();
        score
}


fn part2(input: &str) -> usize {
    let points = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ]);
    
    let mut scores = input.lines()
        .map(|line| {
            let mut stack = Vec::new();
            for c in line.chars() {
                match c {
                    // if closing, try to match to its corresponding opening
                    ')' | ']' | '}' | '>' => match stack.pop() {
                        Some(o) => {
                            if o == '(' && c != ')' ||
                               o == '[' && c != ']' ||
                               o == '{' && c != '}' ||
                               o == '<' && c != '>' {
                                // println!("corrupted, opening was {} but encountered {} as closing", o, c);
                                return 0
                            } else {
                                // println!("Ok");
                            }
                        },
                        None => {
                            // println!("Not possible");
                            return 0
                        },
                    },
                    // else, it's an opening, add it to stack and continue
                    o => {
                        stack.push(o);
                    },
                }
            }

            if stack.len() > 0 {
                // println!("Incomplete");
                return stack.iter()
                    .rev()
                    .fold(0 as usize, |acc, cur| acc * 5 + points.get(cur).unwrap())
            }
            
            // Everything went well, no penalty point
            return 0
        })
        .filter(|v| *v > 0)
        .collect::<Vec<_>>();

        scores.sort();
        let mid = scores.len() / 2;
        scores[mid]
}
fn main() {
    let path = Path::new("./inputs/day10.txt");
    let input = fs::read_to_string(path).expect("Unable to read file");

    let p1 = part1(&input);
    println!("{}", p1);
    let p2 = part2(&input);
    println!("{}", p2);
}
