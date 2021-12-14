use std::path::Path;
use std::fs;
use std::collections::HashMap;

struct Paper {
    dots: HashMap<(usize, usize), usize>,
}

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

impl Paper {

    fn show(&self) {
        let maxs: [usize; 2] = self.dots.keys()
            .cloned()
            .fold([0; 2], |mut acc, entry| {
                if entry.0 > acc[0] {
                    acc[0] = entry.0;
                }
                if entry.1 > acc[1] {
                    acc[1] = entry.1;
                }
                acc
            });
        
        for y in 0..maxs[1]+1 {
            for x in 0..maxs[0]+1 {
                if self.dots.keys().cloned().collect::<Vec<_>>().contains(&(x, y)) {
                    print!("##")
                } else {
                    print!("  ")
                }
            }
            print!("\n")
        }
    }

    fn fold_x(&mut self, ins: &Fold) {
        let mut value: usize = 0;
        if let Fold::X(i) = *ins {
            value = i;
        }

        let dots_to_move = self.dots.keys()
            .filter(|key| key.0 > value)
            .cloned()
            .collect::<Vec<_>>();

        dots_to_move.iter()
            .for_each(|ins| {
                let delta = ins.0 - value;
                self.dots.insert((ins.0 - 2 * delta, ins.1), 1);
                self.dots.remove(ins);
            })
    }

    fn fold_y(&mut self, ins: &Fold) {

        let mut value: usize = 0;
        if let Fold::Y(i) = *ins {
            value = i;
        }

        let dots_to_move = self.dots.keys()
            .filter(|key| key.1 > value)
            .cloned()
            .collect::<Vec<_>>();

        dots_to_move.iter()
            .for_each(|ins| {
                let delta = ins.1 - value;
                self.dots.insert((ins.0, ins.1 - 2 * delta), 1);
                self.dots.remove(ins);
            })
    }

    fn fold(&mut self, ins: &Fold) {
        match ins {
            Fold::X(_) => self.fold_x(ins),
            Fold::Y(_) => self.fold_y(ins),
        }
    }

    fn run(&mut self, ins: &Vec<Fold>) {
        for i in ins {
            self.fold(i);
        }
    }
}

fn parse_input(content: &str) -> (HashMap<(usize, usize), usize>, Vec<Fold>) {
    let parts = content
        .split("\n\n")
        .collect::<Vec<&str>>();
    
    let mut dots: HashMap<(usize, usize), usize> = HashMap::new();
    parts[0].lines()
        .for_each(|line| {
            let x_y: Vec<usize> = line.split(",")
                .map(|x| x.parse().unwrap())
                .collect();
            let x = x_y[0];
            let y = x_y[1];
            dots.insert((x, y), 1);
        });

    let ins = parts[1].lines()
        .map(|x| {
            let ins = x.split("fold along ").collect::<Vec<_>>();
            let ins = ins[1].split("=").collect::<Vec<_>>();
            if ins[0] == "x" {
                Fold::X(ins[1].parse().unwrap())
            } else {
                Fold::Y(ins[1].parse().unwrap())
            }
        })
    .collect::<Vec<_>>();
    
    (dots, ins)
}

fn part1(input: &str) -> usize {

    let (dots, ins) = parse_input(&input);
    let mut paper = Paper{dots};

    // Only run first fold
    paper.fold(&ins[0]);

    paper.dots.len()
}


fn part2(input: &str) {

    let (dots, ins) = parse_input(&input);

    let mut paper = Paper{dots};

    paper.run(&ins);
    paper.show();
}

fn main() {
    let path = Path::new("./inputs/day13.txt");
    let input = fs::read_to_string(path).expect("Unable to read file");

    let p1 = part1(&input);
    println!("{}", p1);
    let _p2 = part2(&input);
}
