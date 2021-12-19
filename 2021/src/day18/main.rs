use std::path::Path;
use std::fs;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Number {
    value: usize,
    depth: u8,
}

#[derive(Debug, Clone)]
struct Equation {
    numbers: Vec<Number>,
} 

impl Equation {
    fn explode(&mut self) -> bool {
        let mut new_numbers: Vec<Number> = self.numbers.clone();
        for i in 0..self.numbers.len() {
            let current_depth = self.numbers[i].depth;
            if current_depth > 4 {
                if i > 0 { // there is at least a number on the left
                    // The left number of the pair is the number at current pos
                    let left_number = &self.numbers[i];
                    let value = left_number.value;
                    // The number to update is the one directly on the left of left number (i-1)
                    new_numbers[i-1] = Number{
                        value: new_numbers[i-1].value + value,
                        depth: new_numbers[i-1].depth
                    };
                }
                if i < self.numbers.len() - 2 { // there is at least a number on the right
                    // The right number of the pair is the number at current pos + 1
                    let right_number = &self.numbers[i+1];
                    let value = right_number.value;
                    // The number to update is the one directly on the right of right number (i+2)
                    new_numbers[i+2] = Number{
                        value: new_numbers[i+2].value + value,
                        depth: new_numbers[i+2].depth
                    };
                }
                // Remove pair and insert zeroed Number at depth above
                new_numbers.splice(i..i+2, vec!(Number{
                    value: 0,
                    depth: current_depth -1,
                }));

                // We updated the tree
                self.numbers = new_numbers;
                return true
            }
        }
        // We did not change anything
        return false
    }

    fn split(&mut self) -> bool {
        let mut new_numbers: Vec<Number> = self.numbers.clone();
        for i in 0..self.numbers.len() {
            let current_number = &self.numbers[i];
            if current_number.value > 9 {
                let remainder = current_number.value % 2;
                new_numbers.splice(i..i+1, vec!(
                        Number{
                            value: current_number.value / 2,
                            depth: current_number.depth + 1,
                        },
                        Number{
                            value: current_number.value / 2 + remainder,
                            depth: current_number.depth + 1,
                        })
                );
            // We updated the tree
            self.numbers = new_numbers;
            return true
            }
        }
        // no split occured
        return false
    }

    fn compute_level_magnitude(&mut self, target_depth: u8) -> (bool, Equation) {
        let mut new_numbers: Vec<Number> = self.numbers.clone();

        for i in 0..self.numbers.len()-1 {
            if self.numbers[i].depth == target_depth {
                let left_number = &self.numbers[i];
                let right_number = &self.numbers[i+1];
                
                let pair_magnitude = 3 * left_number.value + 2 * right_number.value;

                new_numbers.splice(i..i+2, vec!(
                        Number{
                            value: pair_magnitude,
                            depth: left_number.depth - 1,
                        })
                    );

            return (true, Equation{numbers: new_numbers})
            }
        }
        return (false, Equation{numbers: new_numbers})
    }

    fn add(&mut self, eq: &Equation) {
        let mut left_numbers: Vec<Number> = self.numbers.clone();
        for i in 0..self.numbers.len() {
            left_numbers[i].depth += 1
        }
        let mut right_numbers: Vec<Number> = eq.numbers.clone();
        for i in 0..eq.numbers.len() {
            right_numbers[i].depth += 1
        }
        left_numbers.extend(right_numbers);
        self.numbers = left_numbers;
    }

    fn compute_magnitude(&mut self) -> usize {
        // get max depth (should be 4 if exploding/splitting has been done)
        let mut max_depth: u8 = 0;
        self.numbers.iter()
            .for_each(|n| {
                if n.depth > max_depth {
                    max_depth = n.depth;
                }
            });

        while max_depth > 0 {
            match self.compute_level_magnitude(max_depth) {
                (true, n) => {
                    self.numbers = n.numbers;
                },
                (false, _) => {
                    max_depth -= 1;
                },
            }
        }
        self.numbers[0].value
    }
}

fn parse_input(content: &str) -> Vec<Equation> {
    let mut equations: Vec<Equation> = Vec::new();
    content.lines()
        .for_each(|math| {
            let mut equation: Equation = Equation{
                numbers: Vec::new(),
            };
            let mut depth: u8 = 0;
            math.chars()
                .for_each(|c| match c {
                    ',' => (), // continue
                    '[' => {
                        depth += 1;
                    },
                    ']' => {
                        depth -= 1;
                    },
                    v => {
                        let number = Number{
                            value: v.to_string().parse().unwrap(),
                            depth,
                        };
                        equation.numbers.push(number);
                    },

                });
            equations.push(equation);
        });
    equations
}

fn part1(input: &str) -> usize {
    let equations = parse_input(input);
    let mut reduced_equation: Equation = equations[0].clone();
    equations.iter()
        .skip(1)
        .for_each(|eq| {
            reduced_equation.add(eq);
            while reduced_equation.explode() || reduced_equation.split() {
                continue
            }
        });

    reduced_equation.compute_magnitude()
}

fn part2(input: &str) -> usize {
    let equations = parse_input(input);
    equations.iter()
        .tuple_combinations()
        .flat_map(|(eq1, eq2)| {
            let mut reduced1 = eq1.clone();
            let mut reduced2 = eq2.clone();
            
            // compute first magnitude
            reduced1.add(eq2);
            while reduced1.explode() || reduced1.split() {
                continue
            }
            // compute second magnitude
            reduced2.add(eq1);
            while reduced2.explode() || reduced2.split() {
                continue
            }
            [reduced1.compute_magnitude(), reduced2.compute_magnitude()]
        })
        .max()
        .unwrap()
}

fn main() {
    let path = Path::new("./inputs/day18.txt");
    let input = fs::read_to_string(path).expect("Unable to read file");

    let p1 = part1(&input);
    println!("{}", p1);
    let p2 = part2(&input);
    println!("{}", p2);
}
